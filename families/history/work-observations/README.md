# Work observations

Thirty short exchanges and observations from NASA's original Apollo 12
air-to-ground transcript (November 1969) retain the crew's working voices:
cross-checking a suspect timer, putting away a water gun, cleaning a hatch
seal, draining suit hoses, comparing a chart with the ground, bracing failed
Velcro, recovering a torn experiment foil, and talking through lost data.
Ordinary food, fatigue and quiet time sit alongside instrument and material
problems. These are verbatim source turns, not reconstructed dialogue.

Speaker labels expand the transcript's CDR, CMP, LMP and CC to Conrad, Gordon,
Bean and Houston. Each output gives the mission, date, elapsed mission time
and PDF page locator. Mission times use hours, minutes and seconds from launch.
The attribution time is the first selected turn. Some source transcription
errors and incomplete speech remain; they are part of the original record.

`sources.json` pins the NASA PDF and its SHA-256, plus the verified reuse pages.
The NASA-hosted PDF and Apollo Journals mirror were downloaded independently
and have identical bytes. The archive explicitly asserts no copyright in the
original NASA press transcript, while reserving copyright in its corrected
Journal transcript. Only the original is used; see `LICENSE.txt`.

`selections.json` pins every complete selected speaker turn by page, character
range and SHA-256 within whitespace-normalized PDF text. Its small explicit
OCR corrections were checked against the scanned pages. They repair OCR, not
the source's grammar or factual claims. Page numbers refer to PDF positions,
which sometimes differ from the document's printed page numbers. No prose is
paraphrased or synthesized. Exchanges retain their intervening speaker turns.

Replay with the pinned PDF text extractor:

```sh
uv run --cache-dir target/uv-cache families/history/work-observations/import_data.py --source-dir target/imports/work-observations --download
uv run --cache-dir target/uv-cache families/history/work-observations/import_data.py --source-dir target/imports/work-observations --check
```

Downloads and scan previews belong under ignored `target/`. Changed upstream
PDF bytes or changed extracted turns fail validation rather than silently
changing the collection. The branch and its single leaf have explicit local
weight 1; sibling history branches determine its share of the history family.
