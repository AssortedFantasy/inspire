# Animals

Species records follow two biological branches. Vertebrates separate mammals,
birds, amphibians, reptiles, bony fishes and cartilaginous fishes. Invertebrates
separate insects, arachnids, crustaceans and molluscs.
The hierarchy groups related organisms and separates evidence kinds. Insects
have distinct species-name and study-observation children.

Names lead each output. Classification and traits use one field per line;
dataset credits, source URLs and detailed interpretation stay in provenance.
Study records retain units, methods and available sample context. Fish image
ratios identify the measurement and whether it came from a photograph or drawing.

## Source and selection

The vertebrate nomenclature comes from the [GBIF Backbone Taxonomy](https://www.gbif.org/dataset/d7dddbf4-2cf0-4f39-9b2a-bb099caae36c),
GBIF Secretariat, CC BY 4.0. Provenance carries attribution and stable record keys. The
legacy backbone is a frozen management taxonomy, superseded by Catalogue of
Life Extended Release; these records describe that pinned snapshot and do not
assert that its nomenclature represents current taxonomic consensus.

`import_data.py` downloads accepted species through the official API and retains
species with an English common name, a scientific binomial and family-level
classification. Known extinct species, synonyms, placeholder names and common
name codes are excluded. Stable GBIF keys identify records. Family caps use a
stable SHA-256 ordering of IDs to reduce domination by large families; useful
previously present species are retained when they match downloaded common names.
Download limits are documented explicitly in each manifest; the collections are
curated samples, not complete inventories. Independent page snapshots download
with at most four concurrent requests.

Vertebrate records identify the organism and its class, order and family. Habitat
categories are included only when explicitly present in the source. Leaf
documentation describes additional primary trait datasets and the units,
missing-value handling and exact-name joins used for enrichment. No geographic
range, behavior or conservation status is inferred from a name or a taxon.

Marine invertebrates use [Cefas' benthic trait matrix V2](https://doi.org/10.14466/CefasDataHub.123)
under OGL v3.0. These are explicitly genus-level profiles from Northwest Europe,
with only strongly supported trait categories (confidence score 3 of 3).
Duplicate genus rows must have identical evidence before deduplication; a zero
score is never treated as evidence of absence. Categories describe a regional
genus-level synthesis, allowing variation among species and contexts. The
matrix's broad `exoskeleton` category is rendered as hard skeletal structures,
Original independently sourced species names remain separate records.

Terrestrial invertebrate observations come from [AnimalTraits v1.0.7](https://doi.org/10.5281/zenodo.6468938),
CC0, and retain study context and physical units. Observation values describe
the sampled individuals or groups; they are not species maxima. Full CC BY 4.0,
CC0 and OGL v3.0 license texts accompany the importers and are embedded in the
CLI's offline data-license output.

## Reproduction

Each leaf's `sources.json` records exact input URLs, SHA-256 checksums, request
limits, selection caps and source counts. Raw API responses are cached under
`inspire-animal-taxonomy` in the operating system temporary directory, outside
the sampling tree. The importer validates pinned hashes; `--replay` additionally
requires all input URLs to be present in the manifest and makes no network calls.
For example, the base bird corpus can be replayed with:

```sh
python families/natural-world/animals/import_data.py --taxon Aves --output families/natural-world/animals/vertebrates/birds --pages 30 --per-family 40 --replay
```

Use the exact recorded arguments and follow each leaf's enrichment instructions.
The upstream API is mutable: hashes detect changed source data but cannot restore
an expired snapshot after the temporary cache is lost. An intentional source
upgrade must create and review a new manifest. The compiled executable uses only
committed records and performs no network requests.

The `animals` topic has weight 1 alongside other natural-world topics. Its
descendant leaves and branches have weights equal to their descendant record
counts, so selection within animals is uniform per record. Adding data requires
updating the affected descendant weights.

The previous direct `animals/{mammals,birds,amphibians,reptiles}` paths migrate to
`animals/vertebrates/...`. The former mixed `animals/invertebrates` corpus is
split into its biological groups.
