# Chemistry

Chemical identities, structural relationships and reaction classes. Runtime is
offline; PubChem downloads happen only during development.

- `elements/`: 118 element identities and neutral-atom configurations, grouped
  into main-group, transition, lanthanide and actinide branches.
- `compounds/`: 146 PubChem compounds organized by structure, functional group
  and inorganic class. Outputs contain a name, formula, molar mass and molecular
  SMILES. Ionic solids use familiar formula-unit notation without molecular SMILES.
- `formulas/`: 34 empirical reductions, 15 constitutional-isomer comparison sets
  and 26 stereochemical pairs from pinned source structures.
- `reactions/`: 87 reviewed equation templates in ten leaves: two combustion
  classes, neutralization, weak equilibria, ionic precipitation, carbonate gas
  evolution, metal displacement, aqueous electron transfer, thermal decomposition
  and gas equilibria. Outputs contain the equation, with a fuel identity or a
  concise heating/catalyst condition where useful.

Top-level weights balance chemical topics. Within `elements` and `compounds`,
weights follow descendant record counts so each identity samples uniformly.

## Reproduction

```sh
python -B families/chemistry/import_data.py --download
python -B families/chemistry/import_isomers.py --download
python -B families/chemistry/generate_reactions.py
python -B families/chemistry/verify_data.py
```

Downloads are cached under `target/chemistry-sources/`. `source_manifest.json`
pins complete response SHA-256 hashes and exact URLs. Without `--download`, the
importers require the cache and perform no network I/O. Changed upstream inputs
are rejected until explicitly reviewed and repinned. Raw data and intermediate
files remain outside the tree; the executable embeds the generated outputs.

The base importer also runs the amino-acid and carbohydrate importers, then
rebuilds identity weights. It preserves the source selection used by isomer
comparisons without writing corpus files over branches containing child leaves.

## Chemical conventions

Selections are deliberate examples, not exhaustive inventories or random IDs.
Ionic solids use formula units, not molecules. Molar mass is not labelled exact
or monoisotopic mass. SMILES come directly from pinned PubChem structures,
including available stereo information. Superheavy element configurations are
marked with the source's calculated/predicted qualification. Atomic weights and
inadequately qualified physical properties from the periodic table are omitted.

Constitutional comparisons exclude carbohydrates. Stereochemical pairs
distinguish E/Z geometry, enantiomers, epimers and other diastereomers. Aldose
relationships use fully specified open-chain R/S assignments on matching
backbones; cyclic structures with unspecified anomers are excluded.

Every reaction has independent Rust atom-inventory, charge-conservation,
positive-coefficient and primitive-ratio tests. All 28 combustion fuels are
preserved, with names distinguishing isomeric fuels that share a formula.
Equations use their smallest integer coefficients and are never multiplied by a
random extent. Phase labels, equilibrium arrows and relevant conditions remain
part of the chemical artifact.

References inspected include PubChem PUG REST responses, NIST compound identities
(https://wtt-lite.nist.gov/wtt-lite/help/lite.html), and established chemical
equations. Textbook references check factual chemistry; prose and datasets are
not copied. Current OpenStax pages restrict AI ingestion and use CC BY-NC-SA,
so they are not import sources. See `LICENSE.md` for embedded data notices.
