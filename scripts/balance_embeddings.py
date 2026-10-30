"""Embed JSONL sample banks with a content/config addressed, resumable cache.

Raw texts are sent verbatim. Harrier rejects rather than truncates long inputs;
those rows remain NaN and are explicitly listed in the manifest. Cache and bank
outputs live under target/, never in the embedded sampling tree.
"""

import argparse
import base64
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import random
import sqlite3
import string
import time
import uuid

import numpy as np
import requests


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def read_bank(path):
    return [json.loads(line) for line in Path(path).read_text(encoding="utf-8").splitlines() if line]


def write_noise_bank(output, seed=521904):
    """Create labelled controls, outside the sampling tree, for metric diagnostics."""
    output.mkdir(parents=True, exist_ok=True)
    rng = random.Random(seed)
    rows = []
    specifications = [("hex-bytes", "0123456789abcdef", 256),
                      ("decimal-digits", string.digits, 128),
                      ("ascii-noise", string.ascii_letters+string.digits, 128),
                      ("uuid", "", 0)]
    for label, alphabet, length in specifications:
        for i in range(8):
            text = str(uuid.UUID(int=rng.getrandbits(128),version=4)) if label == "uuid" else "".join(rng.choice(alphabet) for _ in range(length))
            rows.append({"sample_id":f"noise-probe:{label}:{i}", "split":"diagnostic", "path":"noise-probe/"+label,
                         "kind":"synthetic-diagnostic", "text":text, "conditional_weight":1/8, "probability":1/32, "seed":seed})
    bank = output/"noise-probe.jsonl"
    bank.write_text("".join(json.dumps(row)+"\n" for row in rows),encoding="utf-8")
    return bank


def subset_bank(bank, output, cap, seed=718203):
    """Write a reproducible conditional subset for a separate mode experiment."""
    target = output / bank.name
    if bank.resolve() == target.resolve():
        raise ValueError("subsampling requires a separate --output directory")
    output.mkdir(parents=True, exist_ok=True)
    groups = defaultdict(list)
    for row in read_bank(bank):
        groups[row["path"]].append(row)
    rng = np.random.default_rng(seed)
    selected = []
    for path in sorted(groups):
        rows = groups[path]
        indexes = sorted(rng.choice(len(rows),size=min(cap,len(rows)),replace=False))
        sample = [dict(rows[i]) for i in indexes]
        mass = sum(row["probability"] for row in rows)
        normalizer = sum(row["conditional_weight"] for row in sample)
        for row in sample:
            row["conditional_weight"] /= normalizer
            row["probability"] = mass*row["conditional_weight"]
        selected.extend(sample)
    target.write_text("".join(json.dumps(row,ensure_ascii=False)+"\n" for row in selected),encoding="utf-8")
    return target


def run(args):
    started = time.perf_counter()
    args.output.mkdir(parents=True, exist_ok=True)
    session = requests.Session()
    session.trust_env = False
    status = session.get(args.base_url + "/status", timeout=30)
    status.raise_for_status()
    status = status.json()
    config = {"model": status["model"], "revision": status["revision"],
              "input_type": args.input_type, "instruction": None,
              "dimensions": status["dimensions"], "dtype": status.get("dtype"),
              "high_precision": status.get("high_precision")}
    config_hash = digest(json.dumps(config, sort_keys=True))
    db = sqlite3.connect(args.output / "embedding-cache.sqlite3")
    db.execute("CREATE TABLE IF NOT EXISTS embeddings (config TEXT, text_hash TEXT, vector BLOB, error TEXT, PRIMARY KEY(config,text_hash))")
    db.execute("CREATE TABLE IF NOT EXISTS configs (hash TEXT PRIMARY KEY, json TEXT)")
    db.execute("INSERT OR IGNORE INTO configs VALUES (?,?)", (config_hash, json.dumps(config, sort_keys=True)))
    db.commit()
    stats = Counter()
    errors = []
    timings = []

    def embed(items):
        if not items:
            return
        payload = {"input": [text for _, text in items], "model": config["model"],
                   "input_type": config["input_type"], "dimensions": config["dimensions"],
                   "encoding_format": "base64"}
        begin = time.perf_counter()
        response = None
        for attempt in range(3):
            try:
                response = session.post(args.base_url + "/v1/embeddings", json=payload, timeout=300)
                if response.status_code < 500:
                    break
            except requests.RequestException:
                if attempt == 2:
                    raise
            stats["retries"] += 1
            time.sleep(attempt + 1)
        stats["requests"] += 1
        if response.status_code == 400 and "tokens" in response.text:
            if len(items) > 1:
                middle = len(items) // 2
                embed(items[:middle])
                embed(items[middle:])
            else:
                key, text = items[0]
                error = response.text
                db.execute("INSERT OR REPLACE INTO embeddings VALUES (?,?,NULL,?)", (config_hash, key, error))
                db.commit()
                errors.append({"text_sha256": key, "characters": len(text), "error": error})
                stats["rejected_overlength"] += 1
            return
        response.raise_for_status()
        result = response.json()
        if result.get("model") != config["model"]:
            raise ValueError("service returned a different model")
        if len(result["data"]) != len(items):
            raise ValueError("service returned wrong batch size")
        for entry in result["data"]:
            key, _ = items[entry["index"]]
            value = entry["embedding"]
            vector = np.frombuffer(base64.b64decode(value), dtype="<f4").copy() if isinstance(value, str) else np.asarray(value, dtype=np.float32)
            if vector.shape != (config["dimensions"],) or not np.isfinite(vector).all():
                raise ValueError("invalid service vector")
            norm = float(np.linalg.norm(vector))
            if not 0.99 < norm < 1.01:
                raise ValueError(f"unexpected vector norm {norm}")
            vector /= norm
            db.execute("INSERT OR REPLACE INTO embeddings VALUES (?,?,?,NULL)", (config_hash, key, vector.tobytes()))
        db.commit()
        stats["embedded_unique_texts"] += len(items)
        stats["reported_tokens"] += result.get("usage", {}).get("prompt_tokens", 0)
        timings.append({"batch_size": len(items), "seconds": time.perf_counter() - begin,
                        "tokens": result.get("usage", {}).get("prompt_tokens")})
        if stats["embedded_unique_texts"] % 4096 < len(items):
            print(json.dumps({"embedded_unique_texts": stats["embedded_unique_texts"], "seconds": round(time.perf_counter()-started, 2)}), flush=True)

    for bank in args.banks:
        rows = read_bank(bank)
        unique = {digest(row["text"]): row["text"] for row in rows}
        known = {r[0] for r in db.execute("SELECT text_hash FROM embeddings WHERE config=?", (config_hash,))}
        pending = [(key, text) for key, text in unique.items() if key not in known]
        stats["cache_hits_unique"] += len(unique) - len(pending)
        # Cap characters as well as record count to avoid enormous mixed batches.
        batch, characters = [], 0
        for item in pending:
            if batch and (len(batch) >= args.batch_size or characters + len(item[1]) > args.batch_characters):
                embed(batch)
                batch, characters = [], 0
            batch.append(item)
            characters += len(item[1])
        embed(batch)
        vectors = np.full((len(rows), config["dimensions"]), np.nan, dtype=np.float32)
        failures = []
        values = {r[0]: (r[1], r[2]) for r in db.execute("SELECT text_hash,vector,error FROM embeddings WHERE config=?", (config_hash,))}
        for index, row in enumerate(rows):
            raw, error = values[digest(row["text"])]
            if raw is None:
                failures.append({"row": index, "sample_id": row["sample_id"], "path": row["path"], "characters": len(row["text"]),
                                 "probability": row.get("probability"), "conditional_weight": row.get("conditional_weight"), "error": error})
            else:
                vectors[index] = np.frombuffer(raw, dtype=np.float32)
        name = Path(bank).stem
        np.save(args.output / f"{name}.embeddings.npy", vectors)
        manifest = {"bank": str(bank), "bank_sha256": hashlib.sha256(Path(bank).read_bytes()).hexdigest(),
                    "rows": len(rows), "unique_texts": len(unique), "config": config, "config_hash": config_hash,
                    "service_status": status, "service_base_url": args.base_url,
                    "row_order": "identical to input JSONL", "valid_rows": len(rows)-len(failures),
                    "truncated_rows": 0, "rejected_rows": failures,
                    "rejected_baseline_probability_mass": sum(x.get("probability") or 0 for x in failures),
                    "rejections_by_root_family": dict(Counter(x["path"].split("/")[0] for x in failures)),
                    "sample_ids": [r["sample_id"] for r in rows]}
        (args.output / f"{name}.embeddings.json").write_text(json.dumps(manifest, indent=2), encoding="utf-8")
        print(json.dumps({"bank_complete": name, "rows": len(rows), "rejected": len(failures), "seconds": round(time.perf_counter()-started, 2)}), flush=True)
    report = {"config": config, "service_base_url": args.base_url,
              "stats": dict(stats), "seconds": time.perf_counter()-started,
              "errors": errors, "batches": timings, "truncated_rows": 0}
    (args.output / "embedding-run.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    run_name = "-".join(Path(p).stem for p in args.banks)
    (args.output / f"embedding-run-{run_name}.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    with (args.output / "embedding-runs.jsonl").open("a", encoding="utf-8") as stream:
        stream.write(json.dumps({"banks": [str(p) for p in args.banks], **report})+"\n")
    db.close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("banks", nargs="*", type=Path)
    parser.add_argument("--output", type=Path, default=Path("target/balance-study"))
    parser.add_argument("--base-url", default="http://noesis.internal:8088")
    parser.add_argument("--input-type", choices=["similarity", "document"], default="similarity")
    parser.add_argument("--batch-size", type=int, default=256)
    parser.add_argument("--batch-characters", type=int, default=100000)
    parser.add_argument("--noise-probe",action="store_true",help="generate and embed 32 reproducible labelled noise controls")
    parser.add_argument("--cap-per-leaf",type=int,help="write a seeded subset in a separate output directory before embedding")
    arguments = parser.parse_args()
    if arguments.cap_per_leaf is not None:
        if arguments.cap_per_leaf < 1:
            parser.error("--cap-per-leaf must be positive")
        arguments.banks = [subset_bank(p,arguments.output,arguments.cap_per_leaf) for p in arguments.banks]
    if arguments.noise_probe:
        arguments.banks.append(write_noise_bank(arguments.output))
    if not arguments.banks:
        parser.error("at least one bank or --noise-probe is required")
    run(arguments)
