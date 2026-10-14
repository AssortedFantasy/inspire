# Fish image measurements

287 depiction-associated, dimensionless ratios from the author-deposited
[FISHMORPH v1.1](https://rodare.hzdr.de/record/1316): 278 body-elongation (`BEl`)
ratios and nine positive caudal-peduncle (`CPt`) ratios for flatfishes whose `BEl`
is missing. Exact, unique scientific-binomial joins use the parent branch's
599-species GBIF breadth sample. IDs preserve the trait and FishBase species code.

The [primary paper](https://aureletoussaint.github.io/homepage/data/2021_Brosse_GEB.pdf)
Figure 1 defines `BEl` as standard length (snout to caudal-fin base) divided by
maximum body depth, and `CPt` as maximum caudal-fin depth divided by minimum
caudal-peduncle depth. Ratios are formatted to three significant digits. No
absolute size, behavioral inference or imputation is included. Three flatfishes
with zero `CPt` and one with missing measurements are omitted. The source's
special-case conventions for other traits are not extended to these records.

The methods select one lateral-view illustration per species; these outputs
describe that source-associated depiction, not a population mean, an invariant
species ratio, or within-species variation. Drawings need not identify a concrete
individual. Neither individual identity nor sex is asserted. The 265 photographs
and 22 drawings retain the full source illustration citation inline; 45 references
marked `Unpublished` explicitly identify that source label and an unidentified
image. Original illustrations are not redistributed or independently remeasured.

FISHMORPH attribution: Brosse, Charpin, Su, Toussaint, Herrera-R, Tedesco and
Villéger (2021), doi:10.14278/rodare.1316; associated methods doi:10.1111/geb.13395,
sections 2.1.1, 2.1.2, 2.2.2 and 4. Common names and species associations come from
GBIF Backbone Taxonomy, GBIF Secretariat. Both datasets are CC BY 4.0; provenance records their attribution. Full license inherited from
[the animals branch](../../../LICENSE.txt).

Shared checksum-pinned inputs and reproduction are documented in the
[parent README](../README.md), [GBIF manifest](../sources.json) and
[FISHMORPH manifest](../traits-sources.json); regeneration uses
`../import_data.py`. Runtime remains offline.
