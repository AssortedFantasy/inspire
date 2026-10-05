# Research for `./inspire`: external stimuli, structured variety, and repeated-use coverage

Project-focused source notes, 4 October 2026. This is a companion to the earlier, broader `REASEARCH.md`, not an implementation specification. Sources are independent entries rather than a unified literature review.

The object of interest is a small external tool that gives an otherwise unchanged LLM varied material before creative decisions. The questions are what material to supply, how to preserve meaningful differences, how to distribute it across repeated calls, and whether it changes the resulting decisions. Decoder modifications and model training are outside this scope.

Evidence labels distinguish **direct LLM experiments**, **human studies**, **generative-design practice**, and **transferable algorithms/resources**. Applying a human finding or a generator technique to an LLM is a proposal, not an established result. “Project question” identifies an inference or experiment suggested by the source. Reported experiments have not been reproduced here.

## 1. Kate Compton — So you want to build a generator… (2016)

[Author’s essay](https://galaxykate0.tumblr.com/post/139774965871/so-you-want-to-build-a-generator)

**Generative-design practice.** Compton distinguishes mathematically different artifacts from perceptually distinctive ones: her “10,000 Bowls of Oatmeal” problem. She also separates choosing different ingredients from varying the structure that combines them. A huge space of parameter settings can still produce one recognizable kind of artifact.

Her treatment includes weighted selection, shuffled decks, grammars, and constraints. These solve different problems; obtaining more combinations is not the same as obtaining more expressive range.

**Project question:** does the CLI have many genuinely different ways of constructing inspiration, or one recognizable template with many substitutions? Evaluate batches, not just a particularly striking sample. This is design guidance about generators, not evidence that human-perceived stimulus differences will necessarily change an LLM’s decisions.

## 2. Cox et al. — Directed Diversity: Leveraging Language Embedding Distances for Collective Creativity in Crowd Ideation (CHI 2021)

[Paper](https://arxiv.org/abs/2101.06030) · [Full text](https://arxiv.org/pdf/2101.06030)

**Human studies.** The system selects inspiration phrases using embedding relationships instead of independently drawing arbitrary phrases. The experiments concern motivational messages for physical activity.

An important counterexample appears in the results: random phrases reduced ideation dispersion, more strongly with three phrases than one; directed three-phrase prompts improved dispersion relative to no prompt. Greater irrelevance slightly reduced diversity. There was no significant difference in self-assessed ideation quality. Stimulus distance, intelligibility, adoption, and resulting idea diversity were measured separately.

**Project question:** compare coherent or deliberately selected stimuli against random bundles at matched lengths. More unrelated content is not automatically a stronger intervention. The human results motivate this comparison but cannot establish the same response in a language model. See especially the results around printed page 14.

## 3. Oulipo — the constraint repertoire

[Official constraint index](https://oulipo.net/fr/contraintes)

**Generative-design practice.** This is a repertoire of literary operations and forms, not just subjects: the index includes lipograms, palindromes, S+7, branching structures, constrained vocabularies, and transformations between texts.

Its relevance is the variety of operations that can organize language. Changing the procedure can change what is generated more fundamentally than changing the nouns in a fixed procedure. The index is a source of mechanisms to investigate, not a controlled evaluation of their creative benefits.

**Project question:** should some inspiration families be produced by different formal operations, rather than all being short descriptive prose? Distinguish a constraint used to construct the inspiration from a constraint imposed on the agent’s final work. A constrained poem can supply a pattern without obliging a software design to obey that poem’s literal restrictions.

## 4. Agrawal and Goyal — Addressing LLM Diversity by Infusing Random Concepts (2026)

[Paper](https://arxiv.org/abs/2601.18053) · [Full text and ablations](https://arxiv.org/html/2601.18053v1)

**Direct LLM experiments.** Random words or sentences are prepended to repeated list-generation questions. Results differ substantially by model; some gains are small. This evaluates the distribution of named list items, not architectural, narrative, or aesthetic decisions.

Two details matter here. Increasing the number of words from one through two, five, and ten did not produce a consistent monotonic improvement. Appendix A.1 also reports gains from random character strings. The results therefore do not establish a universal hierarchy in which meaningful words always outperform meaningless strings.

**Project question:** retain cheap character and word baselines. A carefully curated stimulus system should demonstrate an advantage over those baselines, not assume one. Test stimulus length independently from stimulus structure and use the actual target models.

## 5. Emily Short — Expressive Range in Tarot Decks (2018)

[Author’s essay](https://emshort.blog/2018/05/03/favorite-tarot-decks/)

**Generative-design practice.** Short treats a card deck as an authored procedural vocabulary. Random dealing supplies combinations, but the names, imagery, suits, and accompanying interpretations determine which kinds of meaning are available. She distinguishes changing the artwork from substantially changing that expressive range.

This is particularly relevant to a collection intended to traverse human experience: a deck can have many attractive entries yet repeatedly frame everything through the same concerns or mood.

**Project question:** audit what kinds of situations, relationships, tensions, and attitudes the corpus can express—not merely how many topics it mentions. Alternative subcollections could encode genuinely different conceptual vocabularies. The essay concerns interpretation and narrative design; it does not demonstrate that tarot imagery, or a textual substitute for it, improves LLM performance.

## 6. Hope et al. — Scaling Creative Inspiration with Fine-Grained Functional Aspects of Ideas (CHI 2022)

[Paper](https://arxiv.org/abs/2102.09761) · [Full text](https://arxiv.org/html/2102.09761)

**Human studies and retrieval-system research.** The authors represent product descriptions through fine-grained purposes and mechanisms. This permits matching one aspect while excluding or changing another, instead of reducing an entire description to one undifferentiated similarity score.

Their human evaluations favor this representation over the tested retrieval baselines. The headline relative improvements concern particular search and inspiration-selection measurements, not a general percentage increase in creativity.

**Project question:** preserve relationships and mechanisms in some stimuli, rather than retaining only entity names or topics. Something can be distant in subject matter but analogous in function. For a task-agnostic CLI, that suggests useful corpus annotations; it does not require adding a live, task-conditioned retrieval engine. Compare raw material against a mechanism annotation—the annotation itself might help transfer or prematurely narrow interpretation.

## 7. Darius Kazemi and contributors — Corpora

[Repository and documentation](https://github.com/dariusk/corpora)

**Reusable resource.** Corpora supplies small, machine-readable JSON collections intended for bots and generative projects. The project deliberately favors manageable lists over exhaustive dictionaries or replacements for comprehensive APIs. Its documentation also supplies the collection’s licensing terms.

This is useful as a prototype source that does not require another LLM call to invent the random material.

**Project question:** keep source collections separate when sampling. Uniform selection from one concatenated file gives a collection probability proportional to its size; selecting a collection first allows deliberate weighting. That is a sampling-design observation, not a finding of the repository. Individual lists still need inspection: lexical variety and convenient JSON formatting do not establish conceptual breadth, factual reliability, or cultural representativeness.

## 8. Young et al. — Improving Structural Diversity of Blackbox LLMs via Chain-of-Specification Prompting (2024)

[Paper](https://arxiv.org/abs/2408.06186) · [Full text](https://arxiv.org/html/2408.06186v1)

**Direct LLM experiments.** The user supplies a mapping from outputs to structural features. The model first generates a specification, then an artifact satisfying it; specifications can be refined through multiple levels. Experiments include poetry, code, and coding problems.

The distinction is concrete: different wording can preserve the same structure, while different structures can reuse vocabulary. Higher temperature sometimes reduced structural diversity by making specifications harder to follow. Results depend on instruction-following capability.

**Project question:** separate sampling a structural direction from rendering an inspiration item. This is also a useful evaluation precedent: measure the features the project cares about, not only generic embedding distances. However, a task-specific specification is stronger and more prescriptive than an unrelated poem or fact. Its results do not prove that the latter will transfer equally well.

## 9. Siangliulue et al. — Providing timely examples improves the quantity and quality of generated ideas (2015)

[Author-hosted paper](https://joelchan.me/assets/pdf/2015-cc-timing.pdf)

**Human experiment.** Ninety-seven participants received examples on request, when a system inferred they were stuck, at regular intervals, or not at all. Requested examples produced the most novel ideas; inferred-impasse delivery produced the most ideas. Regularly scheduled examples produced fewer ideas than no examples.

This distinguishes access to inspiration from the timing of its delivery. It also separates novelty from idea count: the same delivery policy did not maximize both.

**Project question:** “invoke before creative decisions” needs an operational interpretation. Compare calls at genuine branching points against calls during routine execution or refinement. A clock can schedule stimulus selection, but a useful moment for intervention may depend on the work. These are human results; no inference about an LLM’s subjective experience of interruption is needed or warranted.

## 10. Kate Compton — Tracery

[Project](https://tracery.io/) · [Implementation](https://github.com/galaxykate/tracery)

**Generative tool.** Tracery is a grammar-based system for producing text and other artifacts. Authored alternatives can expand recursively, allowing a generator to encode several kinds of structure rather than merely substitute values into one fixed string.

The project demonstrates a practical route to an inexpensive, local stimulus generator whose runtime need not itself use a language model. The examples are generative artifacts, not experiments measuring their effects on downstream LLMs.

**Project question:** use distinct production rules for distinct stimulus families, with compatible components where needed. Do not infer broad meaning-space coverage from the number of grammar derivations. A grammar can remain highly formulaic, and repeated surface wrapping can make different contents feel like the same kind of instruction. Inspect generated batches as well as the rules.

## 11. Chan, Dow, and Schunn — Do the best design ideas (really) come from conceptually distant sources of inspiration? (2015)

[Author-hosted paper](https://joelchan.me/assets/pdf/2015-desstudies-nearfar-in-press.pdf)

**Observational human study.** The authors examine hundreds of design concepts across a dozen challenges on an innovation platform that records inspiration links. Citing sources was associated with greater creativity, but conceptually closer sources appeared more beneficial than farther ones.

The study challenges a simple “maximize distance” prescription. It does not establish that near inspiration is universally superior: the setting, similarity measure, and nonrandom choice of inspirations limit causal interpretation.

**Project question:** maximum distance from the task, maximum distance from previous stimuli, and maximum diversity among resulting decisions are different objectives. A distance-aware selector should not silently treat them as interchangeable. Include near, intermediate, and remote material in experiments rather than assuming the most remote tail is the most valuable.

## 12. Ge et al. — Scaling Synthetic Data Creation with 1,000,000,000 Personas (2024)

[Paper](https://arxiv.org/abs/2406.20094) · [Persona Hub](https://github.com/tencent-ailab/persona-hub) · [Released dataset](https://huggingface.co/datasets/proj-persona/PersonaHub)

**Direct LLM method and reusable resource.** Persona Hub derives persona descriptions from web material and uses them to condition synthetic-data generation. A persona supplies interests, knowledge, or priorities that a generic instruction omits.

The headline collection size should not be interpreted as a count of independent perspectives, representative people, or publicly downloadable records. Released material and the complete claimed collection must be distinguished.

**Project question:** compare a person’s concrete practice, concern, or constraint against a bare occupation or famous-author label. Persona conditioning may be one stimulus family without becoming the whole system. Audit repetition and stereotypes in the source material, and test whether the resulting decisions differ beyond vocabulary and voice.

## 13. Project Gutenberg — machine-readable catalogs and source texts

[Catalog and bulk-access documentation](https://www.gutenberg.org/ebooks/offline_catalogs.html)

**Reusable resource.** Gutenberg provides machine-readable catalog data and bulk-access routes. Its documentation recommends those routes rather than scraping the website. A consequential metadata detail: the catalog release date is the Gutenberg edition’s release date, not necessarily the original work’s publication date.

**Project question:** sample actual human-authored passages and forms, not only model-generated summaries of famous works. Preserve work identifiers and provenance so an excerpt can be traced. Choose authors or works before passages when that matches the intended distribution; uniform sampling over all text otherwise favors long works. Era-based sampling requires better historical metadata than upload dates. Corpus availability is not evidence of balanced cultural coverage, and redistribution requires checking the relevant source terms and rights.

## 14. Brian Eno and Peter Schmidt — Oblique Strategies

[Official description](https://enoshop.co.uk/products/oblique-strategies)

**Creative practice.** Oblique Strategies is a deck of concise provocations and process interventions. Its object is not simply to add a new subject but to disturb a habitual approach to ongoing work.

The distinction matters for an inspiration CLI: an operation on attention or process is a different intervention from an evocative fact, an author name, or a piece of prose. The official page describes the intended use; it is not experimental evidence of effectiveness, and its promotional claims should not be treated as measurements.

**Project question:** compare process-changing cues with content-changing cues. Does an agent actually reconsider its decision procedure, or merely mention the cue while producing its usual solution? Treat the deck as a design precedent, not as an automatically redistributable dataset.

## 15. Chan et al. — On the Benefits and Pitfalls of Analogies for Innovative Design (2011)

[Author-hosted paper](https://joelchan.me/assets/pdf/2011-jmd-distancecommonness.pdf)

**Human experiment.** Senior engineering students received patent-derived examples varied by analogical distance, commonness, and picture-versus-text presentation. Far-field and less-common examples had positive effects on novelty and variability in solution quality; the far-field, less-common combination produced more novel concepts than the no-example control.

“Variability in quality” is not the same as uniformly higher quality. The paper also reports different process patterns for distance and commonness, rather than treating them as one novelty variable.

**Project question:** distinguish unfamiliarity from distance. A familiar phenomenon from another field and an obscure example from the current field are different treatments. This is useful counterevidence to simplistic near-only recommendations, just as the later observational work cautions against far-only recommendations. Neither settles how a black-box LLM responds.

## 16. Suh et al. — Luminate (CHI 2024)

[Paper](https://arxiv.org/abs/2310.12953) · [Project](https://luminate-research.github.io/)

**LLM-based interaction research.** Luminate exposes a design space through dimensions and their possible values, generates alternatives, and lets people explore them rather than repeatedly revising a single answer. Its professional-writing study involved fourteen participants.

The important contribution is making the represented alternatives inspectable. A user can see what differs, rather than receiving an undifferentiated collection of outputs.

**Project question:** create several independent ways of describing inspiration diversity—form, register, emotional stance, scale, mechanism, and so forth—and inspect what each misses. Do not assume a model-generated taxonomy exhausts the space. The project need not inherit Luminate’s interface or generate a Cartesian product on every invocation; offline organization of a stimulus corpus may be sufficient.

## 17. PoetryDB — structured access to poems

[Repository and API documentation](https://github.com/thundercomb/poetrydb)

**Reusable resource.** PoetryDB exposes authors, titles, poem lines, line counts, and random retrieval. Provenance is available for records where it has been recorded. Its documentation demonstrates recombining lines from a constrained set of poems.

This supports more than feeding famous author names into the model: actual language, lineation, imagery, and rhetorical movement can become the stimulus. The API’s existence does not establish that the collection represents poetry broadly.

**Project question:** compare intact short units, arbitrary fragments, and recombinations. Fragment boundaries may remove the very relationship that makes a passage interesting. Sampling authors first versus poems first produces different exposure, especially for heavily represented authors. Keep source and transformation metadata internally, and distinguish intact quotation from a newly assembled text.

## 18. Chan and Schunn — The importance of iteration in creative conceptual combination (2015)

[Author-hosted paper](https://joelchan.me/assets/pdf/2015-cognition-openideo-diversity-inpress.pdf)

**Observational human study.** The authors trace conceptual genealogies on an innovation platform. Combining distant concepts had approximately zero average direct effect in their models, while later concepts building on those combinations showed more consistently positive effects.

This supports investigating indirect and delayed contributions of an inspiration, rather than assuming every useful stimulus immediately produces a strong finished idea. The observational design does not isolate iteration as a randomized treatment.

**Project question:** distinguish initial perturbation from subsequent development. Test whether a stimulus yields a useful branch after ordinary refinement, not only whether the first response is polished. Conversely, retain the extra effort in the comparison: a cue requiring many repair steps may not suit a lightweight workflow. This does not justify building an elaborate search loop before the simple intervention is tested.

## 19. Design Heuristics — an authored repertoire of design operations

[Project](https://www.designheuristics.com/)

**Research-derived creative tool; project description inspected.** The project presents 77 design strategies. Each card contains a description, an abstract depiction, and two sketches showing the heuristic in consumer products.

The useful feature is the representation: an operation is paired with both abstraction and concrete instantiation. This differs from a naked abstract instruction and from a single example whose incidental details may dominate.

**Project question:** compare operation-only, example-only, and operation-plus-example stimuli at matched token budgets. Some inspiration may work because it supplies a transferable transformation rather than an unfamiliar topic. The inspected project page does not provide enough experimental detail to attach an effect size or claim this card format is superior for LLMs. It also does not imply that every creative domain should inherit a product-design vocabulary.

## 20. NIST — combinatorial coverage and covering arrays

[Project, papers, and tools](https://csrc.nist.gov/projects/automated-combinatorial-testing-for-software)

**Transferable algorithmic perspective.** Combinatorial testing covers interactions among selected input factors without enumerating every complete configuration. NIST supplies work on covering arrays, coverage measurement, and sequence testing.

**Project question:** when stimuli have several explicit attributes, independent random draws can leave combinations unvisited even if each attribute appears often. Covering arrays suggest scheduling low-order combinations deliberately—for example, pairs of form and mechanism—rather than relying on a larger random seed.

The guarantee concerns the supplied factors and coverage definition, not latent human culture or downstream LLM behavior. Invalid combinations need constraints. Too many weak factors can create impressive-looking coverage without useful variation. For a small first corpus, ordinary shuffled categories may be an adequate baseline; this literature becomes relevant when interaction coverage is the actual bottleneck.

## 21. John Rising — Can an AI Model Give You a Different Idea? (2026)

[Practitioner experiment](https://bymorning.ai/journal/idea-mode-collapse)

**Direct LLM experiment, with substantial measurement limitations.** Rising normalizes outputs to short core-idea descriptions before comparing them. Personas changed presentation but provided little advantage over neutral prompt paraphrases at matched request budgets.

The automated comparisons are imperfect: a check on one hundred human-labeled pairs achieved 74% agreement, with Cohen’s kappa around 0.48. This makes the findings directional rather than exact counts of semantic modes.

**Project question:** an inspiration generator can appear successful because names, metaphors, or voice change while the underlying design stays fixed. Evaluate both the original artifact and a representation that suppresses those changes. Do not replace all judgments with a single normalization model, since it may erase important differences. Persona labels remain a treatment to test, not a reliably stronger alternative to simpler perturbations.

## 22. Wang et al. — Multilingual Prompting for Improving LLM Generation Diversity (EMNLP 2025)

[Paper](https://arxiv.org/abs/2505.15229) · [Publication](https://aclanthology.org/2025.emnlp-main.324/)

**Direct LLM experiments.** The method varies linguistic and cultural cues, generates under those contexts, and pools the results. The study reports diversity improvements relative to several prompting and sampling baselines. Matching cultural cues to language reduced hallucinated culturally specific material in the tested setting.

**Project question:** linguistic form, cultural context, and language identity are separable variables. The CLI can investigate culturally and historically varied material while still emitting English; that English-only transfer is a separate hypothesis, not the same demonstrated intervention.

Do not treat a country name as a complete cultural perspective or a balanced set of labels as representative human sampling. Compare results in a common semantic representation so that the evaluation does not reward a mere change of output language.

## 23. Sobol sequences and stratified sampling

[SciPy Sobol documentation](https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.qmc.Sobol.html)

**Transferable sampling method.** Sobol sequences distribute points with low discrepancy in a specified numerical space. The documentation warns that their balance properties depend on the sampling scheme: arbitrary thinning, skipping, or unsuitable sample counts can destroy those properties.

**Project question:** structured coverage can be obtained through the sequence design, not by repeatedly changing the PRNG seed. However, a numeric coordinate becomes useful only through its mapping to a meaningful stimulus attribute. Quantizing arbitrary coordinates into a ragged hierarchy does not automatically preserve the property one cares about.

Use this as a comparison for explicitly defined continuous or product spaces, not as a claim to avoid semantic concentration in an LLM. For categorical families, a shuffled bag is a simpler reference point. A wall-clock schedule and a call-index schedule also behave differently when invocation rates are uneven.

## 24. Misaki and Akiba — String Seed of Thought (2025/2026)

[Paper](https://arxiv.org/abs/2510.21150) · [Full text and prompts](https://arxiv.org/html/2510.21150v3)

**Direct LLM experiments.** SSoT asks a model to generate and manipulate a random-looking string when selecting a response. It evaluates probabilistic instruction following and diversity-aware generation. This is not the same as simply supplying an external random string.

An implementation detail matters for this project: an external-tool baseline suffered malformed tool calls. Its weaker result therefore cannot be cleanly interpreted as evidence against genuine external randomness. The paper also examines external random strings.

**Project question:** isolate whether a cue is available, whether the invocation succeeds, and whether the model uses it. Compare direct insertion with a successful tool invocation before attributing differences to randomness quality. Do not require the model to simulate a random-number generator when the CLI can provide one. Conversely, a valid random number does not guarantee a varied semantic interpretation.

## 25. Ippolito et al. — Creative Writing with an AI-Powered Writing Assistant: Perspectives from Professional Writers (2022)

[Paper](https://arxiv.org/abs/2211.05030) · [Wordcraft workshop](https://magenta.withgoogle.com/wordcraft-writers-workshop)

**Qualitative human–AI study.** Thirteen professional writers used Wordcraft. Some surprising or surreal suggestions were productive as inspiration despite being unsuitable as finished prose. Writers also encountered dull repetitions and difficulty maintaining their own voice.

This distinguishes the quality of a stimulus as an artifact from its usefulness in another creative process. Selection effort matters: many weak suggestions can contain a valuable one without making the overall workflow efficient.

**Project question:** do not judge every CLI output as a miniature polished work. A useful unresolved tension, odd structure, or suggestive fragment may be enough. But do not declare arbitrary nonsense inspirational by definition. Its value must appear in what follows, and the cost of rejecting or interpreting it belongs in the evaluation.

## 26. Shaib et al. — Detection and Measurement of Syntactic Templates in Generated Text (EMNLP 2024)

[Paper](https://arxiv.org/abs/2407.00211) · [Publication](https://aclanthology.org/2024.emnlp-main.368/)

**Empirical analysis and measurement.** Abstracting words into part-of-speech sequences exposes recurring constructions even when vocabulary changes. The paper distinguishes structural regularity from verbatim repetition; changing generation settings does not eliminate every kind of template concentration.

**Project question:** inspect the inspiration corpus’s forms as well as its topics. A collection generated by one model through one prompt may contain different facts wrapped in a repeated rhetorical construction. This is especially relevant when planning to synthesize a large corpus offline.

Syntactic variety is not semantic variety, and repeated syntax is not necessarily undesirable. Use template analysis to discover an otherwise hidden restriction, not as an objective that rewards awkward phrasing. The same limitation applies to downstream outputs: different syntax can still encode the same creative decision.

## 27. Benji Smith — Marcus, Marcus, Marcus! (2026)

[Experiment](https://machinecreativity.substack.com/p/marcus-marcus-marcus-ai-randomness) · [Code and data](https://github.com/benjismith/ai-randomness)

**Direct LLM practitioner experiment.** Smith repeatedly asks for names while varying instructions and random prefixes. Prefixes include random characters and words from a curated English list. Some word-prefix configurations substantially increase the variety of names, but changing prefixes does not prevent collapse in every configuration.

The word list’s exclusions and the surrounding explanations are part of the treatment. Pooling configurations can hide failures in individual combinations of model and prompt.

**Project question:** this remains a useful minimal reference experiment, but expand beyond name counts. Test whether cue meaning changes aesthetic, structural, or functional choices in the intended agent workflow. Keep the tool-output framing fixed when evaluating the corpus, and keep the corpus fixed when evaluating framing. Otherwise, an apparent benefit of content may actually come from the instruction that accompanies it.

## 28. Gonçalves, Cardoso, and Badke-Schaub — Inspiration choices that matter (2016)

[Open-access paper](https://www.cambridge.org/core/journals/design-science/article/inspiration-choices-that-matter-the-selection-of-external-stimuli-during-ideation/12BB4E73354F3072475B34E9EC63F78F)

**Human protocol study.** Thirty-one design master’s students completed ideation tasks under differing access to external stimuli. Think-aloud protocols and interviews distinguish keyword formulation, searching, and selecting. Unlimited availability did not ensure use: four of ten participants in that condition did not use the tool, whereas all eleven with limited access did.

This is not a controlled estimate of the superiority of scarcity; the observed behavior also reflects how participants interpreted access and timing.

**Project question:** emitting a stimulus, reading it, and adopting it are distinct events. The AGENTS.md instruction and the model’s discretion to ignore material are part of the intervention. Investigate whether a small coherent item is incorporated more effectively than a large menu. An acknowledgement or citation by the model is not sufficient evidence that its decision changed.

## 29. Mouret and Clune — Illuminating search spaces by mapping elites (2015)

[Paper](https://arxiv.org/abs/1504.04909)

**Transferable search architecture.** MAP-Elites maintains good solutions across a space of user-defined behavioral descriptors, rather than selecting only one globally best candidate. It separates a quality score from the attributes along which alternatives should remain available.

**Project question:** offline corpus development could retain effective stimuli from different behavioral regions instead of keeping only those with the highest average creativity rating. This is especially relevant if feedback shows that one stimulus family dominates selection while many others disappear.

Descriptor choice is the limiting assumption: an archive can cover its own grid yet miss important distinctions. For this project, an input-topic archive and an archive of downstream decision effects are different objects. The latter requires actual evaluation. There is no need to run evolutionary search on every CLI call, and the paper does not validate this application.

## 30. Tevet and Berant — Evaluating the Evaluation of Diversity in Natural Language Generation (EACL 2021)

[Paper](https://arxiv.org/abs/2004.02990) · [Publication](https://aclanthology.org/2021.eacl-main.25/)

**Measurement research.** The paper evaluates whether diversity metrics detect controlled changes in the intended kind of variation. Humans outperformed the studied automatic metrics at judging content diversity; the tested decoder changes mainly altered form rather than meaning.

**Project question:** validate the measurement against differences the tool is intended to cause. A stimulus generator should not be optimized solely against a metric that rewards new nouns or longer outputs. Conversely, a metric that ignores mood or register may miss real aesthetic benefits.

Construct separate checks for task validity, substantive decisions, and presentation. Measure repeated-use saturation as well as a small batch. These are proposed evaluation choices, not a claim that this older study establishes a definitive metric for current agents. Preserve artifacts so disagreements between representations can be inspected.
