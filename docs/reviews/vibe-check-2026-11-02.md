# Inspiration vibe check — 2026-11-02

Ten independent `gpt-6.1-sol` agents at high reasoning effort reviewed 50
ordinary root draws each. The recurring impression was positive: varied,
specific material with little synthetic inspirational prose. The main weakness
was a narrower range of experiences than the topic tree suggests. Many subjects
arrive as catalogue labels, completed formal examples, or historical prose.

This is a qualitative review, not evidence that a different distribution improves
creative work. No content or sampling weights were changed.

## What works

The most praised generators expose a consequential relationship. Reviewers liked
elections where counting rules disagree, stable matchings that depend on who
proposes, interleaving that repairs an otherwise destructive burst, and several
co-critical paths that defeat an apparently useful schedule improvement. Their
structure can transfer to another problem without an instruction telling the
recipient what analogy to make.

Specific source prose supplies things the generators rarely do: bodily action,
voice, hesitation, social stakes, and material surroundings. Repeated favorites
included weaving a doll's cap, embroidery, cooking, shipboard work, crowded
markets, and conflicting family obligations. Craft instructions were especially
successful at joining physical structure with lived experience.

Reviewers consistently defended short ordinary entries. Sandstone, denim,
gingham, a car, a place, or an ingredient can provide ample associations at very
low cost. Brevity is not a defect, and unfamiliarity depends on the recipient.
Preserve these alongside technical artifacts and complete human source units.

Representative successes, reproducible at the reviewed revision:

| ID | Root seed | Material | Why it worked |
| --- | --- | --- | --- |
| 008 | `16495811135744455395` | Critical-path schedule | Shortening one critical task does nothing because other paths tie. |
| 202 | `7309282823854514033` | Election | Plurality, Borda, and IRV choose three different winners. |
| 446 | `9541595664750397729` | Quadratic hash probing | Insertion fails despite empty slots outside the probe orbit. |
| 009 | `6055638719987299845` | Doll's Tam O'Shanter | A woven square becomes an object through folding and fastening. |
| 141 | `17428457044352050236` | Poem after a tryst | Breath, steps, and night sound provide an uncommon emotional register. |

## Where breadth falls short

The clearest shared request was for more situated human material: requests,
refusals, complaints, care, maintenance, negotiation, humor, bodily sensation,
and ordinary work. Contemporary life was particularly scarce in these batches.
All ten reviewers distinguished subject breadth from experiential breadth.
Geographical names and cultural catalogue fields widen subject matter without
necessarily widening the voices or experiences represented.

This is not a claim that human or non-European voices are absent. Current
coverage includes Montagu correspondence, Equiano, Douglass, Du Bois, Sui Sin
Far, Persian anecdotes, Japanese tales, theatre directions, and domestic
practices. Extend and inspect those branches before creating duplicates.

Two checked branches illustrate a real distinction between a broad name and a
narrow mode: `society` contains seven economics/voting generators, and
`philosophy` contains three formal-logic generators. Those are useful, but they
do not provide the full range their names might suggest. An equal share per
top-level family is consequently not an equal share per kind of inspiration.

The [old suggestions](../archive/INSPIRATION_SUGGESTIONS.md) already contain
promising directions: everyday letters and petitions, correspondence forms,
occupations, visible repairs, naturalists' observations, dreams, and folk play.
Treat these as candidates for a coverage and source check, not as an instruction
to implement the entire backlog. Several technical proposals are already built;
the present reviews favor expanding modes of experience over accumulating more
formal architectures merely to fill out a list.

## Distribution and repetition

The stream used current weights, with no quotas, family filters, rejected draws,
or resampling. It contained:

- 500 draws across all 26 root families and 252 leaves.
- 276 corpus records and 224 generated outputs.
- 493 distinct texts: seven pairs repeated across batches, with no exact repeat
  inside any reviewer's 50.
- Between 20 and 24 families and between 42 and 48 leaves per batch.

Every non-random family has approximately 3.92% root probability; `random` has
1.96%. Observed counts ranged from 11 to 31 among the former, with 10 random
outputs. Local clusters varied: one reviewer saw seven music outputs, another
saw four philosophy outputs, and several noticed paired navigation, knitting,
nutrition, or imposition forms. These observations do not justify diagnosing a
global weighting defect from any single batch.

The concern is primarily repeated presentation and attention, not exact
duplication. Different subjects can repeatedly ask the recipient to read a solved
table. Conversely, two outputs from the same leaf can expose genuinely different
mechanisms. Long mythology and historical excerpts also occupy more attention
than their draw count suggests. For example, the 20 belief draws occupied
24,403 characters; the 31 geography draws occupied 870. Character count is only
a rough display-size measure, not a measure of attention or value.

Keep the current distribution as the baseline. The
[balance study](../balance-study.md) and subsequent
[blind comparison](../blind-stream-study.md) already demonstrate that improved
embedding/repetition metrics need not produce preferred streams. This review
compares no alternative distribution and supplies no evidence to overturn that.

## Concrete quality findings

| ID | Root seed | Finding | Status |
| --- | --- | --- | --- |
| 038 | `7466860082023399348` | Art creator credit ends in literal `None`. | Confirmed emitted artifact; importer only excludes an entire creator field equal to `None`. |
| 235 | `5927216502589681193` | Art culture field says `Itay`. | Confirmed emitted spelling; source versus transformation origin not established. |
| 209 | `1464333357240701027` | Escapement description refers to absent lettered parts. | Standalone-context weakness; not a demonstrated mechanical error. |
| 338 | `14290528955931918548` | Governor excerpt begins “Another arrangement” and refers to “as shown.” | Same extraction issue; some causal content remains useful. |
| 434 | `4233356914706140085` | Edda excerpt opens with orphaned “he said.” | Confirmed excerpt-boundary blemish. |
| 272 | `6889462128303159859` | Fable retains an unresolved `[24]` footnote marker. | Confirmed extraction artifact. |
| 403 | `5886760792912253832` | Shamir output says “these 1 shares.” | Confirmed generator grammar bug. |
| 328 | `4353165393740252133` | `Degrees` is a sorted sequence beside numbered vertices. | Importer confirms sorting; clarify the label, not the mathematics. |

Reviewers also flagged less self-contained word-history and catalogue records.
Suspicions about astronomical dates/estimates were not authoritatively verified
and are not counted as factual defects. The review did not certify calculations.

Several technically coherent draws felt inert: a completely serial project
schedule, RSA encryption of zero with substantial setup, a remote capacity
increase behind an obvious source bottleneck, or a nonbinding control whose
unchanged result is printed repeatedly. Current generators already include
richer regimes. Review the cost and frequency of these cases within their
existing leaves; do not remove all clean baselines or force every draw to be a
paradox. A compact ordinary case can be more useful than an elaborate novelty.

There were useful differences of emphasis. Reviewer 04 wanted notation previews
and an exposure audit of small fixed corpora; reviewer 02 suggested broader
belief traditions; reviewer 10 prioritized suspicious source fields. Some
reviewers wanted recognition labels for opaque syntax, while all defended
terse ordinary entries. These are proposals to assess individually, not a
reason to restore taxonomy headers or blanket explanatory text.

## Recommended order of work

1. Repair concrete extraction and formatting issues. Update importers and
   records together; retain source wording and provenance. Select complete
   image-independent source units where possible.
2. Add a small, source-backed collection of everyday requests, disputes,
   workarounds, care, or sensory observations. Check adjacent existing records
   first, and seek different speakers and social positions. Do not fill the gap
   with LLM-written dialogue templates.
3. Improve structural yield within existing generators. Reduce redundant
   representations and low-information table rows; retain useful ordinary
   cases beside boundaries and counterexamples.
4. Improve review access to encoded artifacts where useful. ABC, SVG, weaving,
   and palettes can require rendering or specialist literacy. Several reviewers
   explicitly could not judge musical quality from notation. That is a review
   limitation, not evidence the artifacts are poor.
5. Test any proposed exposure change against current sampling in fresh blinded
   streams and actual creative tasks. Compare kinds of material as well as
   family counts; do not replace the old embedding objective with an unvalidated
   new scalar score.

## Method and reproduction

Reviewed source: `221002bee1f4212b3ab331ce722f4044aca6bee2`, clean working tree.
`cargo build --release --locked` succeeded before sampling. Binary SHA-256:
`3873df7355416245a39223bdaa0c153c12e42b7d9c34e77312d7ed62fc2c89ad`.

Python `random.Random(20261102500)` generated 500 successive
`getrandbits(64)` seeds. Each invoked `target/release/inspire.exe --seed SEED
--json`. IDs 001–050 went to reviewer 01, 051–100 to reviewer 02, and so on.
Sample-bank SHA-256:
`ae7cd95442bbb9c3c57ad69a4e4b6d402bacc1fc144dfd45ec218a856467c2d4`.

Each agent first read only the full sample texts and froze initial impressions,
then read paths, seeds, both historical documents requested by the user, and
selected current documentation/source to qualify apparent gaps. Reviewers saw
no other batches or verdicts and generated no extra samples. Each produced a
report and one sample-specific assessment for every assigned output.
Validation confirmed exactly 50 ordered assessments per reviewer and all 500
IDs once. Pooled judgments were 166 strong, 288 okay, and 46 weak. Each item
had one reviewer and the categories were not calibrated between reviewers;
these counts describe their impressions, not an estimated objective defect rate.

Preparation scripts, manifest, all 500 texts/seeds/paths, frozen first
impressions, full reports, and per-sample judgments remain under ignored
`target/vibe-check-2026-11-02/`. They are local review artifacts, not embedded
payloads. Reproduce one item with the root seed above and no `--family` flag.

These are ten batches reviewed by the same model with the same prompt, not ten
diverse human perspectives or ten evaluations of each item. The prompt explicitly
solicited strengths, weaknesses, experiential breadth, and gaps. Agreement may
reflect shared model preferences. The study measures neither downstream creative
benefit nor coverage of a formally defined latent space.
