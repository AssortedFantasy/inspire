# Formula and structure relationships

`composition/molecule-inventories` retains the original named catalog and its
computed empirical formulas and atom inventories. Element order is conventional;
empirical subscripts are reduced by their greatest common divisor. Tests check
composition, unsaturation where applicable, reduction, uniqueness and reproducibility.

`isomers/constitutional` has source-backed pairs with different atom connectivity.
`isomers/stereochemical` distinguishes geometry, chirality and epimerism explicitly.
Both cite stable PubChem CIDs and emit source structures. Matching formulas alone
are never used to claim a stereochemical relationship.
