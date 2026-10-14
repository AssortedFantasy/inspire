"""Regenerate monosaccharide child datasets from pinned PubChem responses."""
import argparse
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def import_data(download=False):
    manifest = {}
    for child in ("aldoses", "ketoses"):
        command = [sys.executable, "-B", str(ROOT / child / "import_data.py")]
        if download:
            command.append("--download")
        subprocess.run(command, check=True)
        data = json.loads((ROOT / child / "source_manifest.json").read_text(encoding="utf-8"))
        for key, value in data.items():
            if key in manifest and manifest[key] != value:
                raise ValueError(f"Conflicting source pin: {key}")
            manifest[key] = value
    (ROOT / "source_manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    return manifest


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    import_data(parser.parse_args().download)
