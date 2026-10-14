# Vertebrates

Species records are separated into mammals, birds, amphibians, reptiles,
bony fish, and cartilaginous fish. Bony fish here means the ray-finned and
non-tetrapod lobe-finned fish represented by the source taxonomy, not the
cladistic group containing all terrestrial vertebrates.

Each leaf contains downloaded nomenclature and classification from the
GBIF Backbone Taxonomy, with measured biological traits when a compatible,
reusable source is available. Scientific names, ecological categories, and
trait values follow the pinned source versions; they are not claims that
the taxonomy is current or that a species has invariant measurements.
Missing extinction flags do not establish that a species is living.

The reptile leaf groups the source's accepted Squamata, Testudines,
Crocodylia, and Sphenodontia classes. The retired Reptilia key is not used;
each record prints the source's actual classification.

The seven leaves contain 1,966 species profiles and 287 image measurements.
Of the species profiles, 1,475 include additional source-backed biological
values or ecological categories. The original four leaves contained 80 names.

| Leaf | Records | Profiles with additional traits | Trait source |
| --- | ---: | ---: | --- |
| mammals | 425 | 312 | PanTHERIA 1.0 WR05 |
| birds | 453 | 413 | AVONET v7 |
| amphibians | 212 | 202 | AmphiBIO v1 |
| reptiles | 237 | 219 | ReptTraits v1-2 |
| bony-fish/species | 599 | 289 | FISHMORPH v1.1 maximum lengths |
| bony-fish/image-measurements | 287 | 287 | FISHMORPH v1.1 image geometry |
| cartilaginous-fish | 40 | 40 | Hutchings et al. species summaries |

The image leaf records ratios measured from one source-selected photograph
or drawing associated with a species. These are separate from species-level
properties: they neither identify an individual nor establish invariant
species proportions. Missing and zero-coded geometry is omitted.

Leaf READMEs document exact versions, source terms, selection and exclusions.
Download manifests pin source bytes, including methods and units. Importers
support offline replay from their development caches; runtime reads only the
embedded records. CC BY 4.0 and CC0 notices are inherited from `../LICENSE.txt`.

The original mammals, birds, amphibians, and reptiles leaves moved intact
from `animals/<leaf>` to `animals/vertebrates/<leaf>` before expansion.
Source manifests pin development-time downloads outside the sampling tree.
Every leaf and this branch are weighted by descendant record count.
