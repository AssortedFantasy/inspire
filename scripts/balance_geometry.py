"""Calibrate embedding geometry and aggregate leaf kernels without an NxN array.

Scales come only from train. Raw V-statistic kernel matrices are PSD; corrected
diagonals estimate population collisions and need not be PSD. Neither measures
artifact quality. RBF quadrature uses a reproducible leaf-stratified subsample.
"""

import argparse
from collections import defaultdict
import csv
import hashlib
import json
from pathlib import Path
import time

import numpy as np

from balance_embeddings import read_bank


QUANTILES = [0, .01, .1, .25, .5, .75, .9, .99, 1]


def summary(values):
    return {str(q): float(v) for q, v in zip(QUANTILES, np.quantile(values, QUANTILES))}


def load(root, split):
    rows = read_bank(root / f"{split}.jsonl")
    vectors = np.load(root / f"{split}.embeddings.npy", mmap_mode="r")
    manifest = json.loads((root / f"{split}.embeddings.json").read_text(encoding="utf-8"))
    digest = hashlib.sha256((root / f"{split}.jsonl").read_bytes()).hexdigest()
    if manifest["bank_sha256"] != digest or len(rows) != len(vectors):
        raise ValueError("bank/embedding alignment mismatch")
    groups = defaultdict(list)
    for i, row in enumerate(rows):
        if np.isfinite(vectors[i]).all():
            groups[row["path"]].append(i)
    return rows, vectors, groups


def calibration(rows, vectors, groups, seed):
    rng = np.random.default_rng(seed)
    paths = sorted(groups)
    # Equal leaf sampling prevents catalogue bank sizes from setting bandwidth.
    pairs = {}
    for label in ["within_leaf", "cross_leaf", "baseline_bank_iid"]:
        first, second = [], []
        if label == "baseline_bank_iid":
            valid = np.array([i for p in paths for i in groups[p]])
            weights = np.array([rows[i]["probability"] for i in valid], dtype=np.float64)
            weights /= weights.sum()
            first = rng.choice(valid,size=100000,p=weights).tolist()
            second = rng.choice(valid,size=100000,p=weights).tolist()
        while len(first) < 100000:
            i = int(rng.integers(len(paths)))
            j = i if label == "within_leaf" else int(rng.integers(len(paths)))
            if label == "cross_leaf" and i == j:
                continue
            a, b = groups[paths[i]], groups[paths[j]]
            if label == "within_leaf" and len(a) < 2:
                continue
            x, y = int(rng.choice(a)), int(rng.choice(b))
            if x == y:
                continue
            first.append(x)
            second.append(y)
        # Avoid a 100000x1024 temporary for all pair rows.
        cosine = np.concatenate([np.einsum("ij,ij->i", vectors[np.array(first[k:k+1024])], vectors[np.array(second[k:k+1024])]) for k in range(0,len(first),1024)])
        pairs[label] = np.clip(cosine, -1, 1)
    distance = np.sqrt(np.maximum(0, 2-2*pairs["within_leaf"]))
    positive = distance[distance > 1e-3]
    sigmas = {"fine": float(np.quantile(positive, .25)),
              "medium": float(np.quantile(positive, .75)),
              "broad": float(np.median(np.sqrt(np.maximum(0, 2-2*pairs["cross_leaf"]))))}
    return {"seed": seed, "calibration_population": "train only; within/cross use uniform leaves then conditional bank rows; baseline_bank_iid uses nominal probability over bank rows; distinct row positions within leaf",
            "pair_counts": {k:len(v) for k,v in pairs.items()}, "cosine_quantiles": {k:summary(v) for k,v in pairs.items()},
            "sigmas": sigmas, "rbf_formula": "exp(-||x-y||^2/(2*sigma^2))",
            "scale_rule": "fine=within positive distance q25, medium=within positive distance q75, broad=cross distance median"}


def corrected_diagonal(matrix, rows, groups, paths, selected, corpus_sizes):
    result = matrix.astype(np.float64).copy()
    for k, path in enumerate(paths):
        indexes = selected[path]
        weights = np.array([rows[i]["conditional_weight"] for i in indexes], dtype=np.float64)
        weights /= weights.sum()
        self_mass = float(weights @ weights)
        if self_mass >= 1-1e-12:
            continue
        off_diagonal = (result[k,k]-self_mass)/(1-self_mass)
        corpus_size = corpus_sizes.get(path) if rows[indexes[0]]["kind"] == "corpus" else None
        result[k,k] = 1/corpus_size + (1-1/corpus_size)*off_diagonal if corpus_size else off_diagonal
    return result


def kernels(root, split, calibration_report, cap, seed):
    started = time.perf_counter()
    rows, vectors, groups = load(root, split)
    paths = sorted(groups)
    all_paths = {r["path"] for r in rows}
    missing = sorted(all_paths - set(paths))
    if missing:
        raise ValueError(f"entire leaves missing embeddings: {missing}")
    rng = np.random.default_rng(seed)
    selected = {p: sorted(rng.choice(groups[p], size=min(cap, len(groups[p])), replace=False).tolist()) for p in paths}
    indexes = [i for p in paths for i in selected[p]]
    X = np.asarray(vectors[indexes])
    counts = np.array([len(selected[p]) for p in paths])
    starts = np.r_[0, np.cumsum(counts)]
    weights = np.array([rows[i]["conditional_weight"] for i in indexes],dtype=np.float32)
    for a,b in zip(starts[:-1],starts[1:]):
        weights[a:b] /= weights[a:b].sum()
    centroids = []
    for p in paths:
        ii = groups[p]
        w = np.array([rows[i]["conditional_weight"] for i in ii],dtype=np.float64)
        w /= w.sum()
        centroids.append(w @ vectors[ii])
    centroids = np.array(centroids)
    cosine = centroids @ centroids.T
    matrices = {"cosine": cosine}
    for scale in calibration_report["sigmas"]:
        matrices["rbf_"+scale] = np.empty((len(paths),len(paths)),dtype=np.float32)
    for first_leaf in range(0,len(paths),16):
        last_leaf = min(first_leaf+16,len(paths))
        a,b = starts[first_leaf],starts[last_leaf]
        C = np.clip(X[a:b] @ X.T,-1,1)
        distance_squared = np.maximum(0,2-2*C)
        for scale, sigma in calibration_report["sigmas"].items():
            K = np.exp(-distance_squared/(2*sigma*sigma))
            column_reduced = np.add.reduceat(K*weights[None,:], starts[:-1], axis=1)
            value = np.add.reduceat(column_reduced*weights[a:b,None], starts[first_leaf:last_leaf]-a, axis=0)
            matrices["rbf_"+scale][first_leaf:last_leaf] = value
    corpus_sizes = {}
    for p in paths:
        if rows[groups[p][0]]["kind"] == "corpus":
            corpus_sizes[p] = len(json.loads((Path("families")/p/"records.json").read_text(encoding="utf-8")))
    for key in list(matrices):
        matrix = matrices[key]
        matrix[:] = (matrix+matrix.T)/2
        groups_used = groups if key == "cosine" else selected
        matrices["corrected_"+key] = corrected_diagonal(matrix, rows, groups, paths, groups_used, corpus_sizes)
    np.savez_compressed(root/f"{split}.leaf-kernels.npz",paths=np.array(paths),**matrices)
    metadata = {"split":split,"seed":seed,"leaf_count":len(paths),"bank_rows":len(vectors),
                "valid_embedding_rows":sum(len(ii) for ii in groups.values()),
                "rbf_cap_per_leaf":cap,"rbf_rows":len(indexes),"sigmas":calibration_report["sigmas"],
                "conditional_weights": "renormalized within retained bank rows, then within RBF subsample",
                "rbf_sample_ids":[rows[i]["sample_id"] for i in indexes],
                "diagonal_correction":"corpus:1/N+(1-1/N)*offdiag; generator:offdiag; raw V-stat retains sample self-pairs",
                "seconds":time.perf_counter()-started}
    (root/f"{split}.leaf-kernels.json").write_text(json.dumps(metadata,indent=2),encoding="utf-8")
    print(json.dumps({k:v for k,v in metadata.items() if k != "rbf_sample_ids"}),flush=True)
    return rows,vectors,groups


def neighbors(root, split, rows, vectors, groups, seed):
    rng = np.random.default_rng(seed)
    # Search a leaf-balanced pool of actual outputs; report cross-leaf and same-leaf.
    selected = [i for p in sorted(groups) for i in rng.choice(groups[p],size=min(8,len(groups[p])),replace=False)]
    X = np.asarray(vectors[selected])
    wanted = [p for p in sorted(groups) if p.startswith(("random/", "chemistry/", "people/", "math/", "computing/", "language/names"))]
    records = []
    for p in wanted:
        index = groups[p][0]
        scores = X @ vectors[index]
        order = np.argsort(-scores)
        other = next((int(j) for j in order if rows[selected[j]]["path"] != p),None)
        same = next((int(j) for j in order if rows[selected[j]]["path"] == p and rows[selected[j]]["text"] != rows[index]["text"]),None)
        def entry(j):
            if j is None:
                return None
            row = rows[selected[j]]
            return {"sample_id":row["sample_id"],"path":row["path"],"text":row["text"],"cosine":float(scores[j])}
        records.append({"sample_id":rows[index]["sample_id"],"path":p,"text":rows[index]["text"],
                        "nearest_cross_leaf":entry(other),"nearest_distinct_within_leaf":entry(same)})
    (root/f"{split}.neighbors.json").write_text(json.dumps({"pool_size":len(selected),"pool_cap_per_leaf":8,"records":records},indent=2,ensure_ascii=False),encoding="utf-8")


def noise_diagnostics(root):
    """Compare labelled synthetic noise controls with the nominal train mixture."""
    if not (root/"noise-probe.embeddings.npy").exists():
        return
    rows, vectors, groups = load(root,"train")
    controls = read_bank(root/"noise-probe.jsonl")
    Z = np.load(root/"noise-probe.embeddings.npy")
    weights = np.array([r["probability"] for r in rows],dtype=np.float64)
    weights /= weights.sum()
    cosine = np.clip(Z @ vectors.T,-1,1)
    config = json.loads((root/"geometry-calibration.json").read_text(encoding="utf-8"))
    leaf_kernels = np.load(root/"train.leaf-kernels.npz")
    paths = leaf_kernels["paths"].tolist()
    probability = np.array([sum(rows[i]["probability"] for i in groups[p]) for p in paths])
    probability /= probability.sum()
    metrics = {"cosine":cosine@weights}
    for label,sigma in config["sigmas"].items():
        metrics["rbf_"+label] = np.exp(-(2-2*cosine)/(2*sigma*sigma))@weights
    report = {"interpretation":"lower expected similarity to baseline means greater marginal distinctiveness under this kernel; no quality inference",
              "controls_are_actual_tree_leaves":False,"records":[]}
    for path in sorted({r["path"] for r in controls}):
        ii = [i for i,r in enumerate(controls) if r["path"] == path]
        local = Z[ii]@Z[ii].T
        a,b = np.triu_indices(len(ii),1)
        values = {"path":path,"within_cosine_quantiles":summary(local[a,b]),"metrics":{}}
        for key, scores in metrics.items():
            value = float(np.mean(scores[ii]))
            leaf_scores = leaf_kernels[key]@probability
            values["metrics"][key] = {"mean_similarity_to_baseline":value,
                "fraction_actual_leaves_with_lower_similarity":float(np.mean(leaf_scores < value))}
        index = ii[0]
        nearest = int(np.argmax(cosine[index]))
        values["example"] = {"text":controls[index]["text"],"nearest_actual_output":rows[nearest],"cosine":float(cosine[index,nearest])}
        report["records"].append(values)
    (root/"noise-diagnostics.json").write_text(json.dumps(report,indent=2,ensure_ascii=False),encoding="utf-8")


def rescore_modes(root, weights_path):
    """Evaluate fixed candidate probabilities; never refit them in another mode."""
    from scipy.stats import spearmanr

    primary = np.load(root/"validation.leaf-kernels.npz")
    alternate = np.load(root/"document-mode/validation.leaf-kernels.npz")
    paths = primary["paths"].tolist()
    if paths != alternate["paths"].tolist():
        raise ValueError("mode path orders differ")
    if weights_path.exists():
        candidates = json.loads(weights_path.read_text(encoding="utf-8"))
        vectors = {name:np.array([weights[p] for p in paths]) for name,weights in candidates.items()}
    else:
        from balance_experiments import groups

        inventory = json.loads((root/"inventory.json").read_text(encoding="utf-8"))
        leaves = inventory["leaves"]
        if [r["path"] for r in leaves] != paths:
            raise ValueError("inventory path order differs")
        _,_,transform,_ = groups(leaves)
        progress = json.loads((root/"optimization-progress.json").read_text(encoding="utf-8"))
        vectors = {r["variant"]:transform@np.array(r["unit_probabilities"]) for r in progress["completed"]}
    results = []
    keys = [k for k in primary.files if k != "paths"]
    for mode, kernels in [("similarity",primary),("document",alternate)]:
        baseline = vectors["current"]
        for key in keys:
            matrix = kernels[key]
            initial = float(baseline@matrix@baseline)
            for name,p in vectors.items():
                energy = float(p@matrix@p)
                results.append({"variant":name,"mode":mode,"kernel":key,"energy":energy,
                                "relative_reduction":1-energy/initial})
    rankings = []
    names = list(vectors)
    for key in keys:
        a = [next(r["energy"] for r in results if r["variant"]==name and r["mode"]=="similarity" and r["kernel"]==key) for name in names]
        b = [next(r["energy"] for r in results if r["variant"]==name and r["mode"]=="document" and r["kernel"]==key) for name in names]
        rankings.append({"kernel":key,"spearman_rho":float(spearmanr(a,b).statistic),
                         "best_similarity":names[int(np.argmin(a))],"best_document":names[int(np.argmin(b))]})
    # Isolate the task-mode change from the smaller document cosine bank.
    source_rows,source_vectors,_ = load(root,"validation")
    source_index = {r["sample_id"]:i for i,r in enumerate(source_rows)}
    matched_groups = defaultdict(list)
    for row in read_bank(root/"document-mode/validation.jsonl"):
        matched_groups[row["path"]].append(source_index[row["sample_id"]])
    centroids = []
    for path in paths:
        indexes = matched_groups[path]
        w = np.array([source_rows[i]["conditional_weight"] for i in indexes])
        centroids.append((w/w.sum())@source_vectors[indexes])
    centroids = np.array(centroids)
    matched_cosine = centroids@centroids.T
    corpus_sizes = {p:len(json.loads((Path("families")/p/"records.json").read_text(encoding="utf-8")))
                    for p in paths if source_rows[matched_groups[p][0]]["kind"]=="corpus"}
    matched_matrices = {"cosine":matched_cosine,"corrected_cosine":corrected_diagonal(
        matched_cosine,source_rows,matched_groups,paths,matched_groups,corpus_sizes)}
    matched_rankings = []
    for key,matrix in matched_matrices.items():
        initial = float(vectors["current"]@matrix@vectors["current"])
        a = []
        for name,p in vectors.items():
            energy = float(p@matrix@p)
            a.append(energy)
            results.append({"variant":name,"mode":"similarity_matched16","kernel":key,"energy":energy,"relative_reduction":1-energy/initial})
        b = [next(r["energy"] for r in results if r["variant"]==name and r["mode"]=="document" and r["kernel"]==key) for name in names]
        matched_rankings.append({"kernel":key,"spearman_rho":float(spearmanr(a,b).statistic),
                                 "best_similarity_matched16":names[int(np.argmin(a))],"best_document":names[int(np.argmin(b))]})
    with (root/"mode-sensitivity.csv").open("w",encoding="utf-8",newline="") as stream:
        writer = csv.DictWriter(stream,fieldnames=list(results[0]))
        writer.writeheader()
        writer.writerows(results)
    report = {"candidate_count":len(vectors),"fit_in_alternate_mode":False,"rankings":rankings,"matched_cosine_rankings":matched_rankings,"results":results,
              "caveat":"mode-specific train-calibrated RBF scales; primary full cosine and an additional matched16 cosine comparison both reported"}
    (root/"mode-sensitivity.json").write_text(json.dumps(report,indent=2),encoding="utf-8")
    print(json.dumps({k:v for k,v in report.items() if k != "results"}),flush=True)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root",type=Path,default=Path("target/balance-study"))
    parser.add_argument("--splits",nargs="+",default=["train","validation"])
    parser.add_argument("--cap",type=int,default=16)
    parser.add_argument("--seed",type=int,default=718203)
    parser.add_argument("--noise-only",action="store_true")
    parser.add_argument("--rescore-modes",action="store_true")
    parser.add_argument("--weights",type=Path)
    args=parser.parse_args()
    if args.rescore_modes:
        rescore_modes(args.root,args.weights or args.root/"weights.json")
        return
    if args.noise_only:
        noise_diagnostics(args.root)
        return
    calibration_path=args.root/"geometry-calibration.json"
    if "train" in args.splits:
        report=calibration(*load(args.root,"train"),args.seed)
        calibration_path.write_text(json.dumps(report,indent=2),encoding="utf-8")
    else:
        report=json.loads(calibration_path.read_text(encoding="utf-8"))
    for split in args.splits:
        rows,vectors,groups=kernels(args.root,split,report,args.cap,args.seed)
        neighbors(args.root,split,rows,vectors,groups,args.seed)


if __name__ == "__main__":
    main()
