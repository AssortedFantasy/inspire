"""Counterfactual leaf-mass experiments; never writes source tree or node weights.

Inputs are collected by balance_sampling.py and balance_embeddings.py. Run:
  python scripts/balance_experiments.py --study target/balance-study
Only generator/large-corpus validation observations are genuinely held out. Exact
corpus collision probabilities use every record, including cross-leaf duplicates.
NumPy and SciPy are required; CVXPY/Clarabel accelerate KL variants; all experiment artifacts go under --study.
"""

import argparse
from collections import Counter, defaultdict
import csv
import hashlib
import json
from pathlib import Path
import random
import time

import numpy as np
from scipy import sparse
from scipy.optimize import minimize

LOCKS = ("people", "objects/cars", "geography/subdivisions", "natural-world/animals")


def jsonl(path):
    with path.open(encoding="utf-8") as source:
        for line in source:
            if line.strip():
                yield json.loads(line)


def save_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def groups(leaves):
    """Map free unit masses to leaves, retaining each homogeneous subtree's ratios."""
    paths = [leaf["path"] for leaf in leaves]
    baseline = np.array([leaf["probability"] for leaf in leaves])
    names = sorted(set(next((lock for lock in LOCKS if p == lock or p.startswith(lock + "/")), p)
                       for p in paths))
    index = {name: i for i, name in enumerate(names)}
    transform = np.zeros((len(leaves), len(names)))
    unit_baseline = np.zeros(len(names))
    for i, path in enumerate(paths):
        name = next((lock for lock in LOCKS if path == lock or path.startswith(lock + "/")), path)
        j = index[name]
        unit_baseline[j] += baseline[i]
        transform[i, j] = baseline[i]
    transform /= unit_baseline
    return baseline, unit_baseline, transform, names


def exact_matrices(study, leaves):
    """Known corpus distributions plus independent generator collision estimates.

The sparse V-stat distribution Gram matrix is PSD. Replacing generator diagonals
by unbiased pair estimates removes 1/n self-match bias but need not retain PSD.
Cross-leaf duplicate text is aggregated literally, without Unicode normalization.
"""
    cache = study / "exact-matrices.npz"
    provenance = {"version": 2, "paths": [leaf["path"] for leaf in leaves],
                  "corpus_sha256": hashlib.sha256((study / "corpus.jsonl").read_bytes()).hexdigest(),
                  "generators_sha256": hashlib.sha256((study / "generators.jsonl").read_bytes()).hexdigest()}
    manifest = study / "exact-matrices.manifest.json"
    if (cache.exists() and (study / "validation.generator-exposures.npz").exists()
            and manifest.exists() and json.loads(manifest.read_text()) == provenance):
        data = np.load(cache)
        exposures = sparse.load_npz(study / "corpus-exposures.npz")
        return dict(data), exposures
    indexes = {leaf["path"]: i for i, leaf in enumerate(leaves)}
    n = len(leaves)
    corpus = {}
    rows, cols, values = [], [], []
    text_labels = []
    for row, item in enumerate(jsonl(study / "corpus.jsonl")):
        shares = defaultdict(float)
        for member in item["members"]:
            i = indexes[member["path"]]
            shares[i] += member["probability"] / leaves[i]["probability"]
        corpus[item["text_sha256"]] = dict(shares)
        text_labels.append({"text_sha256": item["text_sha256"], "text": item["text"]})
        for i, share in shares.items():
            rows.append(row)
            cols.append(i)
            values.append(share)
    exposures = sparse.csr_matrix((values, (rows, cols)), shape=(len(corpus), n))
    sparse.save_npz(study / "corpus-exposures.npz", exposures)
    save_json(study / "corpus-exposure-texts.json", text_labels)
    cc = (exposures.T @ exposures).toarray()
    generated = {split: defaultdict(Counter) for split in ("train", "validation")}
    for row in jsonl(study / "generators.jsonl"):
        generated[row["split"]][indexes[row["path"]]][row["text_sha256"]] += 1
    result = {"corpus": cc}
    diagnostics = {}
    for split, by_leaf in generated.items():
        empirical = defaultdict(dict)
        sample_sizes = np.zeros(n, dtype=int)
        generator_observed_max = np.zeros(n)
        for i, counts in by_leaf.items():
            total = sum(counts.values())
            sample_sizes[i] = total
            generator_observed_max[i] = max(counts.values()) / total
            for text, count in counts.items():
                empirical[text][i] = count / total
        rr, jj, vv = [], [], []
        for r, (text, distribution) in enumerate(empirical.items()):
            for i, share in distribution.items():
                rr.append(r)
                jj.append(i)
                vv.append(share)
        gen = sparse.csr_matrix((vv, (rr, jj)), shape=(len(empirical), n))
        mixed_rows, mixed_cols, mixed_values = list(rr), list(jj), list(vv)
        for r, text in enumerate(empirical):
            for i, share in corpus.get(text, {}).items():
                mixed_rows.append(r)
                mixed_cols.append(i)
                mixed_values.append(share)
        mixed = sparse.csr_matrix((mixed_values, (mixed_rows, mixed_cols)), shape=(len(empirical), n))
        sparse.save_npz(study / (split + ".generator-exposures.npz"), mixed)
        matrix_v = cc + (gen.T @ gen).toarray()
        # Corpus-generator collisions use the full corpus and sampled generator.
        for text, distribution in empirical.items():
            if text in corpus:
                for i, a in distribution.items():
                    for j, b in corpus[text].items():
                        matrix_v[i, j] += a * b
                        matrix_v[j, i] += a * b
        matrix_u = matrix_v.copy()
        for i, total in enumerate(sample_sizes):
            if total > 1:
                matrix_u[i, i] = (total * matrix_v[i, i] - 1) / (total - 1)
        result[split] = matrix_u
        result[split + "_v"] = matrix_v
        result[split + "_generator_observed_max"] = generator_observed_max
        diagnostics[split] = {
            "sample_sizes": sorted(set(sample_sizes[sample_sizes > 0].tolist())),
            "leaves_with_observed_repeats": int(np.count_nonzero(np.diag(matrix_u) > 0)
                                                   - np.count_nonzero(np.diag(cc) > 0)),
            "generator_pair_collisions": {leaves[i]["path"]: int(sum(c * (c - 1) // 2 for c in counts.values()))
                                          for i, counts in by_leaf.items() if max(counts.values()) > 1},
        }
    np.savez_compressed(cache, **result)
    save_json(study / "exact-diagnostics.json", diagnostics)
    save_json(manifest, provenance)
    return result, exposures


def optimize(matrix, prior, kl=0., upper=None, family_matrix=None, family_bounds=None,
             exposure_matrix=None, exposure_cap=None):
    """Simplex constrained quadratic, optional KL, envelope and corpus text cap."""
    matrix = (matrix + matrix.T) / 2
    upper = np.ones(len(prior)) if upper is None else upper.copy()
    if exposure_matrix is not None:
        atoms = sparse.csr_matrix(exposure_matrix)
        counts = np.diff(atoms.indptr)
        single = np.flatnonzero(counts == 1)
        positions = atoms.indptr[single]
        np.minimum.at(upper, atoms.indices[positions], exposure_cap / atoms.data[positions])
        exposure_matrix = atoms[counts > 1]
        if not exposure_matrix.shape[0]:
            exposure_matrix = None
            exposure_cap = None
    if kl and not np.any(matrix) and family_bounds is None:
        caps = upper.copy()
        if exposure_matrix is not None:
            atoms = sparse.csr_matrix(exposure_matrix)
            if np.max(np.diff(atoms.indptr), initial=0) > 1:
                atoms = None
            if atoms is not None:
                np.minimum.at(caps, atoms.indices, exposure_cap / atoms.data)
        else:
            atoms = True
        if atoms is not None:
            if caps.sum() < 1 - 1e-12:
                raise ValueError("Infeasible caps")
            low, high = 0., 1.
            while np.minimum(caps, high * prior).sum() < 1:
                high *= 2
            for _ in range(100):
                middle = (low + high) / 2
                if np.minimum(caps, middle * prior).sum() < 1:
                    low = middle
                else:
                    high = middle
            solution = np.minimum(caps, high * prior)
            return solution, [{"success": True, "message": "Analytic capped KL projection",
                               "iterations": 100, "objective": float(kl * np.sum(
                                   solution[solution > 0] * np.log(solution[solution > 0] / prior[solution > 0])))}]
    conic_failure = None
    # Entropic regularization is stiff when baseline masses span many orders.
    # An optional conic solver avoids hundreds of dense SLSQP iterations.
    try:
        import cvxpy as cp
        from scipy.linalg import eigvalsh
        if eigvalsh(matrix, subset_by_index=[0, 0])[0] >= -1e-9:
            x = cp.Variable(len(prior))
            constraints_cp = [x >= 0, x <= upper, cp.sum(x) == 1]
            if family_bounds is not None:
                lo, hi = family_bounds
                constraints_cp += [family_matrix @ x >= lo, family_matrix @ x <= hi]
            if exposure_matrix is not None:
                constraints_cp += [exposure_matrix @ x <= exposure_cap]
            objective_cp = .5 * cp.quad_form(x, cp.psd_wrap(matrix))
            if kl:
                objective_cp += kl * cp.sum(cp.kl_div(x, prior))
            problem = cp.Problem(cp.Minimize(objective_cp), constraints_cp)
            solver_name = "CLARABEL"
            try:
                problem.solve(solver=solver_name, max_iter=200, tol_gap_abs=1e-9, tol_feas=1e-9)
            except cp.error.SolverError:
                if kl:
                    raise
                solver_name = "OSQP"
                problem.solve(solver=solver_name, max_iter=100000, eps_abs=1e-9, eps_rel=1e-8,
                              polishing=True)
            if problem.status in ("optimal", "optimal_inaccurate") and x.value is not None:
                solution = np.maximum(x.value, 0)
                solution /= solution.sum()
                return solution, [{"success": problem.status == "optimal",
                                   "message": solver_name + " " + problem.status,
                                   "iterations": problem.solver_stats.num_iters,
                                   "objective": float(problem.value)}]
    except Exception as error:
        conic_failure = type(error).__name__ + ": " + str(error)
        print("Conic solver unavailable; falling back to SLSQP: " + conic_failure, flush=True)
    constraints = [{"type": "eq", "fun": lambda x: x.sum() - 1,
                    "jac": lambda x: np.ones_like(x)}]
    if family_bounds is not None:
        lo, hi = family_bounds
        constraints.extend([
            {"type": "ineq", "fun": lambda x: family_matrix @ x - lo,
             "jac": lambda x: family_matrix},
            {"type": "ineq", "fun": lambda x: hi - family_matrix @ x,
             "jac": lambda x: -family_matrix},
        ])
    selected = []
    start = prior.copy()
    trace = []
    def objective(x):
        positive = np.maximum(x, 1e-300)
        regularizer = np.sum(np.where(x > 0, x * np.log(positive / prior), 0.))
        return .5 * x @ matrix @ x + kl * regularizer
    def gradient(x):
        return matrix @ x + kl * (np.log(np.maximum(x, 1e-300) / prior) + 1)
    for iteration in range(12):
        active = list(constraints)
        if selected:
            hot = exposure_matrix[selected].toarray()
            active.append({"type": "ineq", "fun": lambda x: exposure_cap - hot @ x,
                           "jac": lambda x: -hot})
        result = minimize(objective, start, jac=gradient, method="SLSQP",
                          bounds=[(1e-14 if kl else 0., float(cap)) for cap in upper],
                          constraints=active, options={"maxiter": 150, "ftol": 1e-10})
        trace.append({"success": bool(result.success), "message": result.message,
                      "iterations": result.nit, "objective": float(result.fun), "conic_fallback": conic_failure})
        start = np.maximum(result.x, 0)
        start /= start.sum()
        if exposure_cap is None:
            break
        exposure = np.asarray(exposure_matrix @ start).ravel()
        violating = np.flatnonzero(exposure > exposure_cap * (1 + 1e-7))
        if not len(violating):
            break
        selected = sorted(set(selected).union(violating[np.argsort(exposure[violating])[-128:]].tolist()))
    return start, trace


def stream(leaves, weights, study, name, size=80):
    """Actual uniform corpus draws and independent heldout generator seed draws."""
    rng = random.Random(2026103007)
    generated = defaultdict(list)
    for row in jsonl(study / "generators.jsonl"):
        if row["split"] == "validation":
            generated[row["path"]].append(row)
    corpus_cache = {}
    root = Path(__file__).resolve().parents[1]
    result = []
    for position in range(size):
        i = rng.choices(range(len(leaves)), weights=weights, k=1)[0]
        leaf = leaves[i]
        path = leaf["path"]
        if leaf["kind"] == "corpus":
            if path not in corpus_cache:
                corpus_cache[path] = json.loads((root / "families" / path / "records.json").read_text(encoding="utf-8"))
            record_index = rng.randrange(len(corpus_cache[path]))
            record = corpus_cache[path][record_index]
            item = {"text": record["text"], "record_id": record["id"], "record_index": record_index}
        else:
            row = rng.choice(generated[path])
            item = {"text": row["text"], "seed": row["seed"], "sample_id": row["sample_id"]}
        result.append({"variant": name, "position": position, "path": path, "kind": leaf["kind"], **item})
    return result


def input_signature(study):
    return {name: hashlib.sha256((study / name).read_bytes()).hexdigest() for name in
            ("inventory.json", "train.leaf-kernels.npz", "validation.leaf-kernels.npz", "exact-matrices.manifest.json")}


def run(study, resume=False):
    started = time.time()
    inventory = json.loads((study / "inventory.json").read_text(encoding="utf-8"))
    if inventory["metadata"].get("collection_in_progress") or not (study / "sampling-summary.json").exists():
        raise RuntimeError("Sample bank is incomplete; wait for sampling-summary.json and finalized inventory")
    leaves = inventory["leaves"]
    p0, q0, transform, unit_names = groups(leaves)
    exact, corpus_exposures = exact_matrices(study, leaves)
    signature = input_signature(study)
    generator_exposures = sparse.load_npz(study / "validation.generator-exposures.npz")
    train_generator_exposures = sparse.load_npz(study / "train.generator-exposures.npz")
    train = dict(np.load(study / "train.leaf-kernels.npz"))
    validation = dict(np.load(study / "validation.leaf-kernels.npz"))
    paths = [leaf["path"] for leaf in leaves]
    for bank in (train, validation):
        assert list(bank["paths"]) == paths, "kernel and inventory leaf order differ"
    families = sorted(set(path.split("/")[0] for path in paths))
    family_leaf = np.array([[float(path.startswith(family + "/")) for path in paths] for family in families])
    family_units = family_leaf @ transform
    family_baseline = family_leaf @ p0
    corpus_units = corpus_exposures @ sparse.csr_matrix(transform)
    collision = transform.T @ exact["train"] @ transform
    collision_scale = float(p0 @ exact["train"] @ p0)
    semantics = {key: transform.T @ train[key] @ transform for key in
                 ("cosine", "rbf_fine", "rbf_medium", "rbf_broad")}
    multiscale = sum(semantics[key] for key in ("rbf_fine", "rbf_medium", "rbf_broad")) / 3
    semantic_scale = float(q0 @ multiscale @ q0)
    corrected_keys = ["corrected_" + key for key in ("rbf_fine", "rbf_medium", "rbf_broad")]
    corrected = sum(transform.T @ train[key] @ transform for key in corrected_keys) / 3
    corrected_scale = float(q0 @ corrected @ q0)
    known_cap = float(np.max(corpus_exposures @ p0))
    variants = [("current", q0, {"kind": "baseline"}, [])]
    stream_cache = {}
    progress = []
    def publish(name, q, trace):
        p = transform @ q
        directory = study / "streams"
        directory.mkdir(exist_ok=True)
        existing = directory / (name + ".jsonl")
        output = list(jsonl(existing)) if resume and existing.exists() else stream(leaves, p, study, name)
        stream_cache[name] = output
        with (directory / (name + ".jsonl")).open("w", encoding="utf-8") as target:
            for row in output:
                target.write(json.dumps(row, ensure_ascii=False) + "\n")
        progress.append({"variant": name, "collision_validation": float(p @ exact["validation"] @ p),
                         "expected_matching_pairs_1000": float(499500 * p @ exact["validation"] @ p),
                         "effective_leaves": float(1 / (p @ p)), "solver": trace,
                         "semantic_corrected_validation": {key: float(p @ validation[key] @ p) for key in corrected_keys},
                         "family_mass": dict(zip(families, (family_leaf @ p).tolist())),
                         "top_leaves": [{"path": paths[i], "mass": float(p[i])} for i in np.argsort(p)[-15:][::-1]]})
        save_json(study / "progressive-results.json", progress)
    publish("current", q0, [])
    combined = multiscale / semantic_scale + .6 * collision / collision_scale
    specifications = [
        ("min_kl_corpus_cap_half", np.zeros_like(multiscale),
         {"kl": 1., "exposure_matrix": corpus_units, "exposure_cap": known_cap / 2}),
        ("min_kl_training_observed_cap_quarter", np.zeros_like(multiscale),
         {"kl": 1., "exposure_matrix": sparse.vstack([corpus_units, sparse.load_npz(
             study / "train.generator-exposures.npz") @ sparse.csr_matrix(transform)]),
          "exposure_cap": known_cap / 4}),
        ("min_kl_corpus_cap_quarter", np.zeros_like(multiscale),
         {"kl": 1., "exposure_matrix": corpus_units, "exposure_cap": known_cap / 4}),
        ("exact_only", 2 * collision / collision_scale, {}),
        ("raw_cosine_only", semantics["cosine"] / float(q0 @ semantics["cosine"] @ q0), {}),
        ("rbf_only", multiscale / semantic_scale, {}),
        ("rbf_exact_0.3_kl_0.1", combined, {"kl": .1, "lambda_scaled": .3}),
        ("rbf_fine_only", semantics["rbf_fine"] / float(q0 @ semantics["rbf_fine"] @ q0), {}),
        ("rbf_broad_only", semantics["rbf_broad"] / float(q0 @ semantics["rbf_broad"] @ q0), {}),
    ]
    for strength in (.001, .003, .01, .03, .1, .3, 1., 3., 10.):
        specifications.append((f"rbf_exact_{strength:g}", multiscale / semantic_scale
                               + 2 * strength * collision / collision_scale, {"lambda_scaled": strength}))
    for kl in (.01, .5):
        specifications.append((f"rbf_exact_0.3_kl_{kl:g}", combined, {"kl": kl, "lambda_scaled": .3}))
    specifications.extend([
        ("rbf_exact_0.01_kl_0.03", multiscale / semantic_scale + .02 * collision / collision_scale,
         {"kl": .03, "lambda_scaled": .01}),
        ("rbf_exact_0.03_kl_0.03", multiscale / semantic_scale + .06 * collision / collision_scale,
         {"kl": .03, "lambda_scaled": .03}),
        ("rbf_exact_0.003_kl_0.03", multiscale / semantic_scale + .006 * collision / collision_scale,
         {"kl": .03, "lambda_scaled": .003}),
        ("rbf_exact_0.3_family_0.5_2", combined,
         {"family_bounds": (family_baseline * .5, family_baseline * 2), "lambda_scaled": .3}),
        ("rbf_exact_0.3_corpus_cap", combined,
         {"exposure_matrix": corpus_units, "exposure_cap": known_cap, "lambda_scaled": .3}),
        ("corrected_rbf_only", corrected / corrected_scale, {}),
        ("corrected_rbf_exact_0.3", corrected / corrected_scale + .6 * collision / collision_scale,
         {"lambda_scaled": .3}),
    ])
    cached = {}
    if resume and (study / "optimization-progress.json").exists():
        previous = json.loads((study / "optimization-progress.json").read_text())
        if previous.get("input_signature") != signature:
            raise ValueError("Cannot resume: bank or kernels changed")
        cached = {row["variant"]: row for row in previous["completed"]}
    for name, matrix, options in specifications:
        print(f"Optimizing {name}", flush=True)
        solve_options = {key: value for key, value in options.items() if key != "lambda_scaled"}
        if name in cached:
            q = np.array(cached[name]["unit_probabilities"])
            trace = cached[name]["solver"]
        else:
            q, trace = optimize(matrix, q0, family_matrix=family_units, **solve_options)
        metadata = {key: (list(value) if isinstance(value, tuple) else value)
                    for key, value in options.items() if key not in ("exposure_matrix", "family_bounds")}
        metadata["family_envelope"] = "0.5 to 2 times current" if "family_bounds" in options else None
        variants.append((name, q, metadata, trace))
        publish(name, q, trace)
        save_json(study / "optimization-progress.json", {
            "input_signature": signature,
            "completed": [{"variant": label, "options": settings, "solver": history,
                           "unit_probabilities": weights.tolist()}
                          for label, weights, settings, history in variants],
            "elapsed_seconds": time.time() - started})
    rows, weight_rows, streams = [], [], []
    details = {}
    for name, q, options, trace in variants:
        p = transform @ q
        exposure = np.asarray(corpus_exposures @ p).ravel()
        family_mass = family_leaf @ p
        row = {"variant": name, "collision_validation": float(p @ exact["validation"] @ p),
               "collision_train": float(p @ exact["train"] @ p),
               "collision_corpus_exact": float(p @ exact["corpus"] @ p),
               "collision_validation_v": float(p @ exact["validation_v"] @ p),
               "max_corpus_text_mass": float(exposure.max()),
               "max_generator_observed_atom": float(np.max(generator_exposures @ p)),
               "effective_leaves_inverse_simpson": float(1 / (p @ p)),
               "effective_leaves_entropy": float(np.exp(-np.sum(p[p > 0] * np.log(p[p > 0])))),
               "active_leaves_1e-8": int(np.count_nonzero(p > 1e-8)),
               "total_variation_from_current": float(.5 * np.sum(np.abs(p - p0))),
               "family_total_variation": float(.5 * np.sum(np.abs(family_mass - family_baseline))),
               "kl_from_current": float(np.sum(p[p > 0] * np.log(p[p > 0] / p0[p > 0]))),
               "expected_matching_pairs_1000": float(499500 * (p @ exact["validation"] @ p)),
               "max_leaf_mass": float(p.max()), "max_family_ratio": float(np.max(family_mass / family_baseline)),
               "random_mass": float(family_mass[families.index("random")]),
               "solver_success": all(step["success"] for step in trace)}
        for key in ("cosine", "rbf_fine", "rbf_medium", "rbf_broad", *corrected_keys):
            row[key + "_validation"] = float(p @ validation[key] @ p)
            row[key + "_train"] = float(p @ train[key] @ p)
        row["collision_change_percent"] = 100 * (row["collision_validation"] / float(p0 @ exact["validation"] @ p0) - 1)
        for scale in ("fine", "medium", "broad"):
            key = "corrected_rbf_" + scale + "_validation"
            reference = float(p0 @ validation["corrected_rbf_" + scale] @ p0)
            row["semantic_" + scale + "_change_percent"] = 100 * (row[key] / reference - 1)
        row["max_known_or_observed_text_mass"] = max(row["max_corpus_text_mass"], row["max_generator_observed_atom"])
        row["max_training_known_or_observed_text_mass"] = max(row["max_corpus_text_mass"],
                                                                  float(np.max(train_generator_exposures @ p)))
        rows.append(row)
        for i, leaf in enumerate(leaves):
            weight_rows.append({"variant": name, "path": leaf["path"], "kind": leaf["kind"],
                                "current_probability": float(p0[i]), "probability": float(p[i]),
                                "ratio": float(p[i] / p0[i])})
        details[name] = {"options": options, "solver": trace,
                         "family_masses": dict(zip(families, family_mass.tolist())),
                         "top_leaves": [{"path": paths[i], "mass": float(p[i]), "current": float(p0[i])}
                                        for i in np.argsort(p)[-20:][::-1]],
                         "hottest_corpus_text_row": int(exposure.argmax())}
        streams.extend(stream_cache[name])
    save_json(study / "weights.json", {name: {entry["path"]: entry["probability"] for entry in weight_rows
                                                   if entry["variant"] == name} for name, *_ in variants})
    summary_keys = ("variant", "expected_matching_pairs_1000", "collision_change_percent",
                    "semantic_fine_change_percent", "semantic_medium_change_percent",
                    "semantic_broad_change_percent", "max_corpus_text_mass", "max_generator_observed_atom",
                    "max_known_or_observed_text_mass", "max_training_known_or_observed_text_mass",
                    "effective_leaves_inverse_simpson", "active_leaves_1e-8", "random_mass",
                    "total_variation_from_current", "family_total_variation", "solver_success")
    summary = [{key: row[key] for key in summary_keys} for row in rows]
    save_json(study / "summary.json", summary)
    with (study / "summary.tsv").open("w", newline="", encoding="utf-8") as output:
        writer = csv.DictWriter(output, fieldnames=list(summary[0]), delimiter="\t")
        writer.writeheader()
        writer.writerows(summary)
    family_rows = [{"variant": name, "family": family, "probability": details[name]["family_masses"][family],
                    "current_probability": float(family_baseline[i]),
                    "ratio": details[name]["family_masses"][family] / float(family_baseline[i])}
                   for name, *_ in variants for i, family in enumerate(families)]
    for filename, data in (("metrics.csv", rows), ("weights.csv", weight_rows),
                           ("summary.csv", summary), ("family-shifts.csv", family_rows)):
        with (study / filename).open("w", newline="", encoding="utf-8") as output:
            writer = csv.DictWriter(output, fieldnames=list(data[0]))
            writer.writeheader()
            writer.writerows(data)
    with (study / "streams.jsonl").open("w", encoding="utf-8") as output:
        for row in streams:
            output.write(json.dumps(row, ensure_ascii=False) + "\n")
    save_json(study / "results.json", {"metadata": inventory["metadata"], "elapsed_seconds": time.time() - started,
              "locks": LOCKS, "unit_count": len(q0), "leaf_count": len(leaves),
              "scales": {"collision": collision_scale, "semantic": semantic_scale,
                         "corrected_semantic": corrected_scale, "corpus_text_cap": known_cap},
              "matrix_diagnostics": {"rbf_v_min_eigenvalue": float(np.linalg.eigvalsh(multiscale)[0]),
                                     "rbf_corrected_min_eigenvalue": float(np.linalg.eigvalsh(corrected)[0]),
                                     "collision_u_min_eigenvalue": float(np.linalg.eigvalsh(collision)[0])},
              "metrics": rows, "variants": details})
    print(f"Saved {len(rows)} variants in {study}; elapsed {time.time()-started:.1f}s", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--study", type=Path, default=Path("target/balance-study"))
    parser.add_argument("--resume", action="store_true", help="Reuse completed optimization variants from this unchanged bank")
    args = parser.parse_args()
    run(args.study, args.resume)


if __name__ == "__main__":
    main()
