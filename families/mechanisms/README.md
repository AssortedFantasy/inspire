# Mechanisms

Mechanism principles, historical descriptions, and concrete idealized models.
Transmission, lifting, reciprocating motion, indexing, cams, fluid machinery,
feedback, and oscillation occupy separate branches. Historical passages are
separate from modern equations and numeric traces.

The original 54 IDs remain in 16 smaller principle datasets. Another 151 records
are selected original-text passages from Henry T. Brown's *Five Hundred and Seven
Mechanical Movements*, 21st edition (1908), organized into 24 datasets. These
are historical descriptions, retaining period terminology and claims. Short
labels, descriptions requiring their absent illustration, circular references,
unsupported historical dates, and known erroneous formulas were excluded.
Named schematic parts remain where their operation is explained in the passage.

Thirteen generators produce compound and planetary gear trains, hydraulic
presses, laminar tube flow, slider-crank and four-bar positions, cycloidal cam
rise, proportional and sampled PI feedback, three distinct damping regimes,
and forced sinusoidal response. Each states its geometry, sign convention,
units, initial conditions, and applicable idealizations. Tests verify closure of
the rod lengths, mesh relations, pressure/volume/work balance, integrated
velocity profiles, differential-equation residuals, cam derivatives, and
actuator bounds. Numeric traces are rounded displays of the unrounded model.

## Sources and replay

Brown's original book is public domain; its original scan is at
[Internet Archive](https://archive.org/details/fivehundredseven00browiala).
The scan's OCR download is
`https://archive.org/download/fivehundredseven00browiala/fivehundredseven00browiala_djvu.txt`
(SHA256 `f5bd6199cd2091b094acb8e291c2ca3e50cfb542f38f66d76be18b06353a69b3`).
Selected clean paragraph transcriptions come from
[507movements.com](https://507movements.com/about.html), which identifies its
static text as the original public-domain book. Modern animations, images,
JavaScript, site commentary, and layout are excluded. The checked-in
`brown_selection.json` pins each input URL, byte checksum, category, and exact
excerpt boundaries. Whitespace is normalized; terminal punctuation is supplied
when an excerpt ends partway through a sentence.

```sh
python -B families/mechanisms/import_brown.py --source-dir target/imports/mechanisms --download
python -B families/mechanisms/import_brown.py --source-dir target/imports/mechanisms --check
```

A source mismatch fails before producing records. The importer reproduces
committed historical records byte for byte; raw sources remain outside the
sampling tree. The other 54 records are independently worded factual
explanations, with NASA, NIST, Ohio University, APMonitor, and siphon research
references in provenance. They do not reproduce modern textbook prose.

The generated equations and explanatory copy are original implementations of
classical models. Useful primary equation references are Robert L. Williams II's
[Ohio University kinematics notes](https://people.ohio.edu/williams/html/PDF/NotesBook3011.pdf),
[MIT Engineering Dynamics](https://ocw.mit.edu/courses/2-003sc-engineering-dynamics-fall-2011/),
and [MIT Dynamics and Control II](https://ocw.mit.edu/courses/2-004-dynamics-and-control-ii-spring-2008/).
No course text or code is imported.

The five primary branches retain equal topic weights. Subbranches balance
mechanism types and representation (principle, history, concrete model), rather
than allowing the size of the historical corpus to dominate selection.
