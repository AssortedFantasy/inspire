# Molecular formulas

Samples 56 named molecules from a fixed catalog. Outputs molecular and empirical
formulas, atom counts, and a randomized molecule-count exercise. Some samples
compare two actual isomers with the same molecular formula. This does not infer
a structure from a formula or assemble arbitrary element combinations.

Names and formulas were checked against the
[NIST Web Thermo Tables compound list](https://wtt-lite.nist.gov/wtt-lite/help/lite.html).
Conventional ordering is used for NH3 and SO2. The empirical formula uses
alphabetical element ordering and reduces atom counts by their greatest common
divisor. NIST's factual names/formulas are used; all explanatory prose and computed
examples are original. Tests check catalog uniqueness, compositions, integer
unsaturation for the C/H/N/O subset, reduction, and seeded reproducibility.
