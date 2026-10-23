# Transport

The airport corpus contains 1,248 real sites from the public-domain
[OurAirports data](https://ourairports.com/data/), snapshot 2026-10-16.
Six leaves distinguish large, medium and small airports, heliports, seaplane
bases and balloonports. Names, municipality served, codes, elevation and landing
surface dimensions retain their source meaning. Missing codes and surfaces are
omitted. Only explicit IATA and ICAO fields receive those labels; the site's
internal identifier is never presented as an aviation code.

Each type has at most 240 sites. Selection takes one site per country per round,
with a deterministic hash ordering of countries and ordinary sites. An ordinary
site and the highest and lowest sites alternate at the front of each country's
queue; country-specific rotations retain all three kinds in partial rounds.
Sites need a known country and integer elevation. Closed sites and explicitly
marked duplicate or fictional names are excluded. All selected sites have equal
sampling weight, including the smaller balloonport collection. This is a bounded
collection of community metadata, not a representative inventory of world aviation.

Elevations are feet above mean sea level. Municipality means the place served,
which can differ from the site's physical location. The longest listed open
landing surface is joined by airport ID, with the text identifier cross-checked.
Lengths, widths and unstandardized surface codes are retained as supplied.
Absent surface data does not imply that a site has no runway or pad. The snapshot
and listed-open status are historical, with no live operational information.
The [source data dictionary](https://ourairports.com/help/data-dictionary.html)
documents these fields. No prose, images, frequencies or comments are imported.

## Rebuild

```sh
python -B families/transport/import_airports.py --download
python -B families/transport/import_airports.py --check
```

The default cache is `target/imports/ourairports`; `--cache` overrides it.
The importer verifies all three source SHA-256 checksums before writing.
`--download` fetches only missing files. Records include source IDs and URLs in
provenance; local leaf weights equal record counts.

Pinned repository revision: `b529623d2a0b09a9be78d37fc7a7a5e1e16e3451`.
Files come from
`https://raw.githubusercontent.com/davidmegginson/ourairports-data/b529623d2a0b09a9be78d37fc7a7a5e1e16e3451/`.

| File | SHA-256 |
| --- | --- |
| `airports.csv` | `0499dbd88de2fc735ac5c09f59796ccc3af544e17ffda807f793c0c6fe6c3d75` |
| `countries.csv` | `2a9dbee691125b0cdb8ceb5fe227c48c903f99c488963b8e53e2ab366521c639` |
| `runways.csv` | `eff294dc5c055e7bb48ef43cb3b564399f2741bbfbba0c18c085aea7b9ab988b` |
