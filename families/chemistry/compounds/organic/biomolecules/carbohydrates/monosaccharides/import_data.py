"""Regenerate monosaccharide child datasets from pinned PubChem responses."""
import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def import_data(download=False):
    command = [sys.executable, "-B", str(ROOT.parent / "import_data.py"), "--monosaccharides-only"]
    if download:
        command.append("--download")
    subprocess.run(command, check=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    import_data(parser.parse_args().download)
