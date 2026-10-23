# Transport

Aviation, rail, navigation and maritime have equal weight within transport.
Airport type weights remain proportional to their record counts.

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

## Procedural leaves

`rail/timetables` constructs four small fictional working timetables: opposing
meets at a two-road loop, a 30/60-minute single-trainset shuttle, a pulse
interchange with a single-track branch, and a freight refuge overtake. Running
times, stops, release times, transfers and trainset turns are explicit integer
minutes. Each interstation track is one exclusive block; the pulse main line has
independent directional tracks and platforms, with a separate branch bay. Tests
reconstruct block occupancy intervals from the timetable and check every pair,
plus running times, minimum stops, transfers, stock continuity and overtaking
order. Release and turn margins are synthetic, not rules for a real railway.
Acceleration, braking curves, route locking, train lengths and terminal platform
allocation are outside the model. Network Rail's [planning account](https://www.networkrail.co.uk/industry-and-commercial/the-timetable/)
and [2027 Network Statement, timetable planning rules](https://www.networkrail.co.uk/wp-content/uploads/2016/12/2027-Network-Statement.pdf)
provide the concepts of running times, headways and conflicting movements; no
operational timetable or prescribed margin is copied.

`navigation/dead-reckoning` alternates local-plane vector calculations and
spherical route comparisons. Local legs have constant through-water velocity
and current, true bearings and no leeway. DR excludes current; the estimated
endpoint includes it. Steering cases enumerate both zero-cross-track headings
when distinct, including two forward solutions, a stalled resultant, reversed
progress, tangent feasibility and excessive cross-current. Spherical cases use
exactly 60 nmi per angular degree, compare the shorter great-circle arc with a
rhumb using the shorter longitude difference, and include a great-circle
midpoint. Equatorial and meridional equality, high latitudes, the date line,
opposite hemispheres and near-antipodes are sampled separately. Exact antipodes,
poles, an ellipsoid, geographic obstacles, variable currents and wind are absent.
Tests check signed displacement, vector closure, independent Cartesian and
forward-geodesic closure, midpoint distances and the shortest-arc inequality.
The primary reference is NGA's [American Practical Navigator](https://msi.nga.mil/Publications/APN),
especially [volume II, chapter 9, The Sailings](https://msi.nga.mil/api/publications/download?key=16693975%2FSFH00000%2FBowditch_Vol_2_LoRes.pdf&type=view).
Only mathematics and terminology are used, not passages or figures.

`maritime/buoyage` creates schematic narrowing channels, circular shoals with a
cardinal mark, and preferred-channel bifurcations. Lateral colours, unchanged
port-cylinder/starboard-cone shapes, modified lateral bands and Fl(2+1) lights,
and cardinal colours, topmarks and Q/VQ rhythms follow official
[IALA R1001 edition 2.0](https://www.iala.int/product/r1001/?download=true),
sections 2.1–2.2, tables 1–6. The official
[2017 edition](https://www.iala.int/content/uploads/woocommerce_uploads/2017/06/R1001-The-IALA-Maritime-Buoyage-System-Ed1.1-June-2017.pdf)
also documents these same mark characteristics. Regions are explicitly selected;
no country-to-region assignments are inferred. Conventional buoyage direction
is stated, and reversing travel reverses the vessel's passing sides. All channel
marks have fitted topmarks and are unlit. Cardinal marks use the same semantics
in both regions. Positions, shoals, banks, tracks and beam dimensions are invented.
Channel clearance is transverse to its straight axis, with linear banks between
paired marks. Shoal clearance buffers a circle by half the beam; fork clearance
buffers a point mark. These are geometric cross-sections without ship length,
depth, tide, squat or real chart information. Tests check sides under rotations
and reversed travel, minimum separation and correct fork geometry. No other
mark classes or light periods are invented.

Calculations retain full precision internally; displayed coordinates and
distances are rounded. Reproduce samples with `inspire --family PATH --seed N`.
