"""Render all SVG trial outputs unchanged; retain failures as labeled placeholders.

Requires Pillow, Node, and sharp (set NODE_PATH if sharp is not locally installed).
python -B scripts/creative_render.py --base target/creative-validation
"""

import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path
from xml.etree import ElementTree

from PIL import Image, ImageDraw


def extract_svg(text):
    """Remove a surrounding Markdown fence only; never repair generated SVG."""
    text = text.strip()
    match = re.fullmatch(r"```(?:svg|xml)?\s*\n(.*)\n```", text, re.DOTALL | re.IGNORECASE)
    return match.group(1).strip() if match else text


def validate_svg(text):
    if re.search(r"<!DOCTYPE|<!ENTITY|<\?xml-stylesheet", text, re.IGNORECASE):
        raise ValueError("External declarations are not allowed")
    root = ElementTree.fromstring(text)
    if root.tag != "{http://www.w3.org/2000/svg}svg":
        raise ValueError("Missing SVG root or namespace")
    for node in root.iter():
        if node.tag.rsplit("}", 1)[-1].lower() in {"script", "foreignobject"}:
            raise ValueError("Active or embedded external content is not rendered")
        for key, value in node.attrib.items():
            name = key.rsplit("}", 1)[-1].lower()
            if name.startswith("on"):
                raise ValueError("Event handler is not allowed")
            local = value.strip().startswith("#")
            raster_data = re.fullmatch(r"data:image/(?:png|jpeg|webp);base64,[A-Za-z0-9+/=\s]+", value.strip())
            if name in {"href", "src"} and not (local or raster_data):
                raise ValueError("Nonlocal resource reference")
    for target in re.findall(r"url\s*\(\s*([^)]*)\)", text, re.IGNORECASE):
        if not target.strip(" \t\r\n\"'").startswith("#"):
            raise ValueError("Nonlocal CSS resource reference")
    if re.search(r"@import", text, re.IGNORECASE):
        raise ValueError("CSS import is not allowed")
    return root


def render(base):
    manifest = json.loads((base / "manifest.json").read_text(encoding="utf-8"))
    destination = base / "rendered"
    destination.mkdir(exist_ok=True)
    pending, audit = [], {}
    for job in manifest["jobs"]:
        if job["task"] != "svg":
            continue
        record = json.loads((base / "records" / f"{job['id']}.json").read_text(encoding="utf-8"))
        text = record["output_text"]
        svg = extract_svg(text)
        entry = {"id": job["id"], "output_sha256": hashlib.sha256(text.encode()).hexdigest(),
                 "fence_removed": svg != text.strip(), "output": str((destination / f"{job['id']}.png").resolve())}
        audit[job["id"]] = entry
        (destination / f"{job['id']}.svg").write_text(svg, encoding="utf-8")
        try:
            root = validate_svg(svg)
            try:
                viewbox = [float(x) for x in root.attrib.get("viewBox", "").replace(",", " ").split()]
                entry["requested_viewbox"] = viewbox == [0, 0, 400, 400]
            except ValueError:
                entry["requested_viewbox"] = False
            pending.append({"id": job["id"], "svg": svg, "output": entry["output"]})
        except (ValueError, ElementTree.ParseError) as error:
            entry.update(status="failed", reason=str(error))
    jobs_path = destination / "render-jobs.json"
    jobs_path.write_text(json.dumps(pending, ensure_ascii=False), encoding="utf-8")
    result = subprocess.run(["node", str(Path(__file__).with_suffix(".cjs")), str(jobs_path)],
                            check=True, capture_output=True, text=True, encoding="utf-8")
    for line in result.stdout.splitlines():
        row = json.loads(line)
        audit[row["id"]].update(row)
    for entry in audit.values():
        if entry["status"] != "rendered":
            image = Image.new("RGB", (400, 400), "white")
            ImageDraw.Draw(image).text((135, 190), "No rendered image", fill="black")
            image.save(entry["output"])
        entry["png_sha256"] = hashlib.sha256(Path(entry["output"]).read_bytes()).hexdigest()
    versions = subprocess.check_output(["node", "-e", "console.log(JSON.stringify(require('sharp').versions))"], text=True)
    report = {"renderer": "sharp/librsvg", "versions": json.loads(versions), "canvas": [400, 400], "background": "white",
              "failure_policy": "No repairs or regeneration; placeholder occupies the original position.",
              "records": list(audit.values())}
    (destination / "audit.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"total": len(audit), "rendered": sum(x["status"] == "rendered" for x in audit.values()),
                      "failures": [x["id"] for x in audit.values() if x["status"] != "rendered"]}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", type=Path, default=Path("target/creative-validation"))
    render(parser.parse_args().base)
