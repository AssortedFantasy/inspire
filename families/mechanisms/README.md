# Mechanisms

Transmission, lifting, linkages, fluid machinery, feedback, and oscillation.
Leaves separate historical passages, factual relations, and procedural models.

The historical corpus contains 105 verbatim passages from Henry T. Brown’s
*Five Hundred and Seven Mechanical Movements*, 21st edition (1908), in 25 leaves.
Excerpts must convey their operation without an illustration. Diagram letters
remain when the prose identifies the parts and their mechanical relationships;
isolated labels and unsupported cross-references are excluded.
The author and work follow each passage. Period terminology remains intact.
Another 54 records in 16 leaves contain equations, classifications, and measured
experimental data with primary references in provenance.

Seventeen generators emit kinematic configurations, gear diagrams, transfer
functions, motion programs, and compact traces. They include variable-length
compound trains with idlers, planetary operating modes, cam rise/dwell/return
programs, Geneva indexing, parallel capillaries, thermostat hysteresis, actuator
windup, and all three damping regimes. Oscillators vary initial motion and
representation, including energy, poles, impulse responses, and forced power.
Units and geometry remain with the artifact. Shared `number.rs` keeps numeric
precision compact.

Tests cover linkage closure, gear meshes and power, hydraulic volume/work,
velocity-profile integration, cam continuity, Geneva tangency, thermostat
thresholds, actuator bounds, and differential-equation and energy balances.

## Sources and replay

Brown’s original book is public domain. Its scan is at
[Internet Archive](https://archive.org/details/fivehundredseven00browiala).
The scan’s OCR download is
`https://archive.org/download/fivehundredseven00browiala/fivehundredseven00browiala_djvu.txt`
(SHA256 `f5bd6199cd2091b094acb8e291c2ca3e50cfb542f38f66d76be18b06353a69b3`).

Paragraph transcriptions come from
[507movements.com](https://507movements.com/about.html), which identifies its
static text as the original public-domain book. Modern animations, images,
JavaScript, site commentary, and layout are excluded. `brown_selection.json`
pins each URL, byte checksum, category, and exact excerpt boundaries. The
importer normalizes whitespace, requires unique boundaries and complete
sentence endings, and rejects starts inside a sentence (except after Brown’s
colon-and-dash introduction). Paired movement numbers retain their attribution.

```sh
python -B families/mechanisms/import_brown.py --source-dir target/imports/mechanisms --download
python -B families/mechanisms/import_brown.py --source-dir target/imports/mechanisms --check
```

Source mismatches fail before records are written. Replay reproduces the
historical corpus byte for byte; source caches stay outside the sampling tree.

The November 2026 standalone-text review replaced the lever, dead-beat, and
three-legged gravity selections (296, 303, 310) with operative passages on
cylinder, three-legged pendulum, and Mudge gravity escapements (294–295, 306,
309). The short Bloxam selection (312) was removed: it identifies lifting and
stopping parts but omits how the pendulum receives its impulse. The six retained
escapements cover duplex, cylinder, single-pin, three-legged, detached, and
gravity action. No historical leaf was removed.

The water-wheel governor passage (163) now begins with its three-pulley
arrangement, omitting the dependent “Another arrangement” introduction. Its
remaining “as shown” supplies no missing information: the sentence itself says
the belt runs on the loose pulley at the proper speed, then describes both
directions of gate correction. Other retained figure cues are similarly
redundant, such as the nut “shown on the middle of the screw” in the Persian
drill (112). Named lettered parts remain intact in mechanisms such as the
steam traps, gas meter, paddle-wheel linkage, and epicyclic trains.

The equations and generated models are original implementations. References
include Robert L. Williams II’s
[Ohio University kinematics notes](https://people.ohio.edu/williams/html/PDF/NotesBook3011.pdf),
[MIT Engineering Dynamics](https://ocw.mit.edu/courses/2-003sc-engineering-dynamics-fall-2011/),
[MIT Dynamics and Control II](https://ocw.mit.edu/courses/2-004-dynamics-and-control-ii-spring-2008/),
and Matthew Peet’s
[hybrid systems lecture](https://control.asu.edu/Classes/MAE509/509Lecture19.pdf).
No course text or code is imported.

The five primary branches retain equal topic weights. Subbranches balance
mechanism types and representations so corpus size does not dominate selection.
