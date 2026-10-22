# Astronomical bodies

Extrasolar planets are grouped by physical discovery technique: photometry,
dynamical measurements, resolved observations, gravitational microlensing, and
timing variations. Solar System planets, dwarf planets, and natural satellites
are separate datasets. Discovery techniques do not imply habitability or a
particular planet composition.

The downloaded NASA Exoplanet Archive **Planetary Systems (`PS`)** table is a
13 October 2026 snapshot of `default_flag=1` literature solutions. Each row
contains one published solution; it does not mix the archive's calculated
mass/radius infill from the composite table. Only parameters with limit flag
**0** are emitted: upper/lower bounds and unspecified flags are omitted rather
than printed as central estimates. Of 6,445 rows, three contain no remaining
selected numerical property and are omitted.

Reported mass and projected minimum mass `m·sin(i)` are distinct. Missing values
are omitted. No equilibrium-temperature value is substituted for a measured
surface temperature, and no habitability inference is made. Orbital eccentricity
is the value in the published solution; a reported zero can be a model choice.
Exoplanet and planetary values use four significant figures and an approximation marker. They are
literature estimates, not guaranteed exact values or measurements of an
individual physical surface. See the archive's
[column definitions](https://exoplanetarchive.ipac.caltech.edu/docs/API_PS_columns.html)
and [composite calculations](https://exoplanetarchive.ipac.caltech.edu/docs/pscp_calc.html).

Outputs start with the object's name, followed by one quantity per line.
Scientific notation omits padded exponents and redundant signs (`1.42e4`).
Definitions and source-selection details stay in documentation and provenance.

NASA/JPL Solar System Dynamics provides the other two tables. A mean radius is
the radius of an equal-volume sphere, not an equatorial radius or a diameter.
Sidereal rotation and orbital periods refer to fixed stars; a rotation period is
not a solar day. Satellite radii retain the source's central-value precision and
quoted sigma, so rounding does not shift the quoted interval. Densities computed
from uncertain GM/radius values and missing/upper-limit GM values are omitted.
The original tables' reference markers and uncertainties remain in cached HTML.

`sources.json` and `import_data.py` pin the exact source URLs, SQL selection, and
SHA-256 hashes. Raw downloads stay outside `families/`; runtime is offline.

```sh
python -B families/natural-world/astronomy/import_data.py --cache /path/to/cache
```

The importer fetches missing inputs and rejects changed checksums. It recognizes
only the snapshot's documented discovery methods and validates the table layouts.
Within extrasolar planets every retained named object has equal probability;
branch weights equal descendant record counts. Astronomy balances extrasolar and
Solar System topics equally, then balances the three Solar System object kinds.
Rare techniques remain explicitly selectable without dominating random output.

These are formatted factual numerical data, not copied article prose or imagery.
Each record credits NASA Exoplanet Archive or NASA/JPL Solar System Dynamics in
provenance, preserving its source URL, object ID, and pinned snapshot details.
Each exoplanet also attributes its default literature reference, preserving the
reference label and linked bibliographic URL in provenance. The archive is
operated by Caltech under contract with NASA's Exoplanet Exploration Program;
see its [acknowledgement guidance](https://exoplanetarchive.ipac.caltech.edu/docs/acknowledge.html)
and Christiansen et al. (2025), [doi:10.3847/PSJ/ade3c2](https://doi.org/10.3847/PSJ/ade3c2).
No JPL imagery is redistributed; its image-use terms are not mislabeled as a
license for numerical scientific data.
