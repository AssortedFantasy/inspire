# Amino acids

Twenty standard protein amino acids, with glycine achiral and the remaining
identities selected as their L forms. The original eight PubChem CIDs are
preserved. The scope excludes selenocysteine, pyrrolysine, nonprotein amino acids,
peptides, salts and unspecified stereoisomer mixtures.

Side-chain groups are disjoint editorial categories: `aliphatic` (six),
`aromatic` (three), `hydroxyl` (two), `sulfur` (two), `amides` (two),
`acidic` (two), and `basic` (three). Tyrosine is grouped with aromatic rings,
histidine with basic side chains, and methionine with sulfur compounds. These
are structural groupings, not claims of a single charge or polarity at all pH.
Sibling weights follow record counts so all twenty identities sample uniformly.

Each record uses the neutral PubChem structural representation; amino acids in
solution have pH-dependent protonation and can be zwitterionic. L/D does not
specify optical rotation or universally imply S/R. Cysteine is (2R),
isoleucine (2S,3S), and threonine (2S,3R) in these selections. Other chiral
entries are (2S); glycine has no stereocentre.

```sh
python -B families/chemistry/compounds/organic/biomolecules/amino-acids/import_data.py --download
python -B families/chemistry/compounds/organic/biomolecules/amino-acids/import_data.py
```

`source_manifest.json` pins the full PUG REST responses. Missing inputs download
to `target/chemistry-sources`; changed URLs or hashes fail. The importer checks
CID uniqueness, neutral structure, stereochemical IUPAC names and composition.
Records contain the name, formula, molar mass and stereochemical SMILES.
Runtime has no network dependency. Provenance acknowledges NCBI/NLM;
chemistry/LICENSE.md carries the embedded source notice. Only factual identities
and computed properties are redistributed,
without third-party descriptive annotations or prose.
