# Natural processes

Geological processes distinguish plate boundaries and volcanism from karst and
glaciation. Ocean records distinguish circulation, tides, and marine ecology.
Weather records distinguish suspended water/ice, precipitation, atmospheric
motion, and optical effects. These branches balance mechanisms rather than
record counts.

The original 48 source-grounded explanations retain their IDs, wording, and
provenance. A further 82 definitions were downloaded from the National Weather
Service's [glossary](https://forecast.weather.gov/glossary.php), not bulk-authored.
`import_weather.py` explicitly lists the selected terms; forecast operations,
warnings, equipment, acronyms, and purely cross-reference definitions are omitted.
`noaa_sources.json` pins all 26 alphabetical pages by exact URL and SHA-256,
retrieved 13 October 2026. Raw HTML stays outside the sampling tree.

The NWS [reuse policy](https://www.weather.gov/disclaimer) places its unmarked web
material in the public domain. The selected glossary definitions are identified
as incorporated NWS material, not subject to copyright protection; their editorial
changes are identified separately. No third-party images are used. Definitions
retain source wording after HTML/whitespace normalization and
correction of three evidenced UTF-8 encoding artifacts already present in the
source. Five bounded paraphrases remove operational guidance, clarify frost
deposition, and omit an overstrong restriction on sun dogs. Trailing "See also"
cross-references are removed when the definition is already self-contained.
Edits are explicit in `PARAPHRASES` and `clean_definition`; the raw source remains
checksum-verifiable. Multiple meanings are rejected for selected terms.

```sh
python -B families/natural-world/phenomena/import_weather.py --cache /path/to/cache
```

The importer downloads missing pinned HTML to that cache and refuses changed
inputs. Replaying replaces only imported `noaa-glossary-*` records, retaining the
earlier explanations. Source URLs and checksums accompany every imported record.
