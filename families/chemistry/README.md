# Chemistry

The 35 leaves separate chemical identities, structural relationships and reaction
classes. Runtime is offline; PubChem downloads happen only during development.

- `elements/`: 118 element identities and neutral-atom configurations, grouped
  into main-group, transition, lanthanide and actinide branches.
- `compounds/`: 106 selected PubChem compounds organized by structure, functional
  group and inorganic class. Stable CIDs accompany formulas, reduced empirical
  ratios, calculated molar masses, atom inventories and structure SMILES.
- `formulas/`: the original molecular-inventory generator plus distinct
  constitutional-isomer and stereochemical-pair corpora.
- `reactions/`: 87 reviewed equation templates in ten leaves: two combustion
  classes, neutralization, weak equilibria, ionic precipitation, carbonate gas
  evolution, metal displacement, aqueous electron transfer, thermal decomposition
  and gas equilibria. Species specify phases and conditions.

`formulas` and `reactions` remain selectable branches; their former mixed payloads
moved into descendants. Weights balance chemical topics, not dataset sizes.
Records contain concrete objects and facts rather than assignments.

## Reproduction

```sh
python -B families/chemistry/import_data.py --download
python -B families/chemistry/import_isomers.py --download
python -B families/chemistry/generate_reactions.py
```

Downloads are cached under `target/chemistry-sources/`. `source_manifest.json`
pins complete response SHA-256 hashes and exact URLs. Without `--download`, the
importers require the cache and perform no network I/O. Changed upstream inputs
are rejected until explicitly reviewed and repinned. Raw data and intermediate
files remain outside the tree; the executable embeds the generated outputs.

## Chemical conventions and limitations

Selections are deliberate examples, not exhaustive inventories or random IDs.
Ionic solids use formula units, not molecules. Molar mass is not labelled exact
or monoisotopic mass. SMILES come directly from pinned PubChem structures,
including available stereo information. Superheavy element configurations are
labelled theoretical. Atomic weights and inadequately qualified physical
properties from the periodic table are omitted.

Constitutional pairs exclude carbohydrates: sugar epimers may have identical
connectivity while differing in stereochemistry. Four explicitly selected
stereochemical pairs distinguish E/Z geometry, enantiomers and C4 epimers. D/L
labels specify configuration; optical-rotation sign is a separate property.

Every reaction has independent Rust atom-inventory, charge-conservation,
positive-coefficient and primitive-ratio tests. Conditions limit precipitation
to supersaturation without excess complexing ligands, metal displacement to
appropriate exposed surfaces, and redox chemistry to the specified medium.
Random forward extents give stoichiometric reactant consumption and product
formation; actual yields depend on reaction conditions, kinetics and equilibrium.

The original 56-molecule inventory and all 28 combustion fuels are preserved.
Older mixed equations moved into their reaction classes; silver chloride now
uses a net ionic equation. Sodium/chlorine formation was omitted from the
refactored redox branch, which focuses on displacement and aqueous transfer.

References inspected include PubChem PUG REST responses, NIST compound identities
(https://wtt-lite.nist.gov/wtt-lite/help/lite.html), and established chemical
equations. Textbook references check factual chemistry; prose and datasets are
not copied. Current OpenStax pages restrict AI ingestion and use CC BY-NC-SA,
so they are not import sources. See `LICENSE.md` for embedded data notices.
