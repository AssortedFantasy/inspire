# Aldoses

Thirty fixed PubChem identities represent D and L glyceraldehyde, the two
aldotetroses, four aldopentoses and eight aldohexoses. Carbon-count branches
separate distinct structural families. Pentoses and hexoses additionally separate
open-chain aldehydes from cyclic six-membered hemiacetals (pyranoses).

The source's representation is retained: name resolution can return a cyclic
form for one sugar and an open-chain form for another. These selected CIDs do
not enumerate all solution forms. Every cyclic record explicitly leaves the
anomeric configuration unspecified; no alpha/beta claim is made. D/L denotes
configuration rather than the sign of optical rotation. Original D-glucose
CID 5793 and D-ribose CID 10975657 remain present as pyranose representations.
Deoxy/amino sugars, sugar alcohols and sugar acids are excluded.

`python -B import_data.py --download` retrieves development-only sources into
`target/chemistry-sources/`. Without `--download`, every cached response must
match the exact URL and SHA-256 in `source_manifest.json`. The two original
cached responses retain their original keys and checksums. The importer checks
CnH2nOn formulas, neutral charge, molar masses, independent SMILES
valence inventories, ring/aldehyde consistency, stereo-centre counts and
IUPAC configuration tuples, plus record ID/text uniqueness. Its restricted
SMILES parser rejects chemistry outside the selected neutral C/O scope.

Records contain factual identity, numerical properties and PubChem computed
names/structures with original formatting. Each credits PubChem/NCBI/NLM.
Third-party descriptive annotations, safety prose and images are excluded.
NCBI policy distinguishes government information from potentially protected
contributed material; this is not a blanket public-domain claim for PubChem.
See [chemistry notices](../../../../../../LICENSE.md) and the
[primary NCBI policy](https://www.ncbi.nlm.nih.gov/home/about/policies/).
