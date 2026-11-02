# Materials

482 literal labels from the CC0 Corpora collection, revision
`2e7adec11a0561696c236ae1fca82e88be173d42`. Seven leaves cover building materials,
decorative stones, textile vocabulary, fibres, metals, packaging and technical
textiles. All leaves use record-count weights, so each retained label has the
same share within the family. The family has top-level weight 2.

These are names and vocabulary, not invented technical profiles. The source
includes trade names, regional stone names, textile structures and broad classes;
it is not a mineralogical or engineering classification. No hardness, composition,
performance or safe-use properties are inferred from a name.

Duplicate labels go to their first listed topic. Incomplete adjectives,
manufacturing operations and a place-name entry are excluded: `conductive`,
`carding`, `desizing`, `lenzing, grimsby`, `modified atmosphere`, `cushioning`
and `natural fiber`. The duplicate `hemp` remains in food's herb catalogue.
`zorbeez` is also excluded as a finished household cleaning-product brand rather
than a textile material name. Its manufacturer's [contemporaneous filing](https://www.sec.gov/Archives/edgar/data/1125532/000135448807002011/vertical10qsbv5a.htm)
identifies a product line of chamois-like cloths. This is a specific scope
exclusion, not a general ban on trade names or unfamiliar terms. Other source
spellings are retained. Missing properties remain absent.

```sh
python -B scripts/import_catalogues.py --family materials --download
python -B scripts/import_catalogues.py --family materials --check
```

The standard-library importer verifies pinned SHA-256 checksums in
`scripts/catalogue_sources.json`; caches stay in `target/imports/corpora`.
Record provenance retains exact source files, fields and list indexes.
See the [pinned source lists](https://github.com/dariusk/corpora/tree/2e7adec11a0561696c236ae1fca82e88be173d42/data/materials)
and bundled `LICENSE.md`.
