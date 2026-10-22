# Physical geography

Twenty topic leaves contain 3,765 named features from the pinned Natural Earth
1:10m snapshot in [`../import_data.py`](../import_data.py). Sources are downloaded
at development time; the executable embeds the records and runs offline.
Natural Earth data is [public domain](https://www.naturalearthdata.com/about/terms-of-use/).
Each record retains its source filename and stable NE_ID, or a river dissolve ID.
Child weights match descendant record counts, giving every feature an equal
share within physical geography. Small waterfalls and depressions collections
therefore receive proportionate sampling rather than repeating a handful of names.

Rebuild with `python -B families/geography/import_physical.py --source-dir target/imports/geography`.
The shared importer verifies every input's SHA-256 before parsing it.

Land topics distinguish coasts, islands, landscapes and relief. The ranges topic
includes related highland complexes, explicitly labeled as highlands rather than
mountain ranges. Water topics
distinguish inland waters from marine waters. Related valleys and gorges share a
leaf while retaining their precise type in the text. Marine coastal waters retain
the source's bay, gulf, fjord, inlet or sound label; sounds are not automatically
claimed to be passages. Only straits and channels belong to `marine/passages`.
Ocean and sea labels have separate leaves. Alkaline lakes remain separate from
ordinary source-class `Lake` records; neither class is equated with freshwater
or salinity. Source river centerlines crossing lakes are excluded.

Names retain the source spelling with whitespace normalized. All-capital map
labels use title case. Conventional, type-specific map abbreviations are expanded
(`Mts.`, `Ra.`, `Cord.`, `Plat.`, `I.`, `Is.`, `Pen.`); translated `NAME_EN`
values are not applied wholesale because several identify unrelated features.
The mixed `Pen/cape` polygon class is accepted only when the labels explicitly
identify a peninsula. Duplicate source IDs and duplicate emitted text are removed,
except for proven source ID collisions: the distinct Flores islands and Peruvian
versus northern Andean Cordillera Central/Occidental retain geographic suffixes
in their record IDs and the original source NE_ID in provenance.

Coordinates are approximate map positions rounded to 0.1 degrees, with trailing
zeroes omitted. They are never precise
summits, surveyed coordinates or geographic centroids. Capes and waterfalls use
GeoJSON point geometry: stale latitude/longitude attributes put several Antarctic
capes near New Zealand and place Victoria Falls incorrectly. Mountains' coordinate
attributes agree with their geometry. Lake and river text labels the mapped
extent center; a bounding-box center can fall outside the feature.
All pieces sharing a river dissolve ID contribute to its extent, including the
split Loire and Maas geometries. Conflicting names for one dissolve ID fail import.
Records sharing a name can still represent distinct features or mapped river
sections; their approximate map locations provide context.

The importer corrects landform/region errors against these references, recording
the reference in each corrected record's provenance:

- [Norway's official tourism site](https://www.visitnorway.com/places-to-go/fjord-norway/scenic-route-hardangervidda/):
  Hardangervidda is a plateau, rather than a mountain range.
- [Québec's geographical names commission](https://toponymie.gouv.qc.ca/ct/ToposWeb/Fiche.aspx?no_seq=34705):
  Les Laurentides is a mountain range, rather than a plain.
- [National Geographic's geographic reference](https://education.nationalgeographic.org/resource/peninsula/):
  the Iberian Peninsula is a peninsula, rather than a plateau.
- [Israel Nature and Parks Authority's map](https://static-en.parks.org.il/wp-content/uploads/2018/09/Orange_2016.pdf):
  the Negev's source `Africa` region is corrected to Asia.
- [USGS Antarctic atlas](https://pubs.usgs.gov/pp/p1386b/p1386b.pdf):
  Queen Maud Mountains spelling is corrected; a second row with the same identity
  is excluded.
- [UNESCO's Tassili n'Ajjer inscription](https://whc.unesco.org/en/list/179/):
  Tassili n'Ajjer is a plateau, rather than a desert.

Conservative exclusions avoid questionable cartographic labels becoming factual
classifications. The desert class omits Jabal as Sawdä’, Caatingas, Punjab,
Makgadikgadi Pan, Etosha Pan, Salar de Uyuni, White Sands and Big Bend. The plains
class omits Jabal Nafüsah and the Sahel. The range class omits Vatnajökull, an
[ice cap](https://www.vatnajokulsthjodgardur.is/en/melting-glaciers/vatnajokullglacier).
The duplicate Cape Lopatka row at 71.9°N is excluded; the Kamchatka row remains.
The source's geographic labels are a curated map collection, not an exhaustive
gazetteer. No source altitude, current water extent, political sovereignty or
unsupported physical description is asserted.

Dedicated recursive leaf reviews inspect every emitted record and its relevant
source rows. Doubtful identities and classifications receive targeted primary
checks; this is not independent verification of every geographical fact.
The corrections table in the importer and individual record provenance retain
the supporting references. Reviews corrected 28 single islands mislabeled as
archipelagos, recovered distinct places lost to reused IDs, and removed mainland
regions, ecological belts, reef complexes, ice domes and ambiguous labels from
incompatible topics. Physical type migrations retain source identities and do
not create a separate leaf for each uncommon cartographic category.

Additional verified material includes Qattara and Turpan depressions and Death
Valley. Cape positions and names receive corrections from national mapping and
navigation publications. Peak names include repaired Galdhøpiggen,
Hvannadalshnjúkur and Volcán Villarrica, and the restored name Kuwohi. Chapada dos
Veadeiros, Caribou Mountains, Birch Mountains, Serra da Ibiapaba and Hadhramaut
Plateau are emitted as plateaus. Haruj remains a plateau despite also being a
volcanic field; a second geological type does not establish a classification error.
