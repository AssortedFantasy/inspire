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

---

## Follow-up: Four Gemini 4 Argon (Medium) reviews (IDs 501–700)

Four independent **Gemini 4 Argon (Medium)** reviewers (`review-11.md` through
`review-14.md`) subsequently evaluated the next 200 sequential draws from the
same `random.Random(20261102500)` stream (IDs `501`–`700`, 50 per reviewer)
under the identical blinded-first-impressions and source-verification protocol.

### Argon sample and verdict summary

- **200 draws** across all 26 root families and 137 distinct leaves (`107`
  corpus records, `93` generator outputs, `200` distinct texts).
- **Pooled Argon judgments:** `81` strong (`40.5%`), `76` okay (`38.0%`), and
  `43` weak (`21.5%`) — slightly more critical on the weak tail (`21.5%` vs.
  `9.2%` in batches 01–10), primarily penalizing leaf-level template collisions,
  unpruned random logic ASTs, and bare homework/UUID draws.

| Batch | IDs | Strong | Okay | Weak | Standout favorites |
| --- | --- | --- | --- | --- | --- |
| 11 | `501–550` | 15 | 23 | 12 | `507` equal-summary OLS pair, `504` Montagu Vienna stairs, `509` de Dillmont linen darning, `517` Rosenzweig–MacArthur thresholds, `529` Hearn *nazoraëru*, `532` M/M/4 pooled vs. split wait |
| 12 | `551–600` | 21 | 22 | 7 | `577` Vitruvius two-summer stone seasoning, `574` 14 kn vessel pushed 1 kn astern by 15 kn current, `570` M/M/3 pooled vs. split, `586` vertical-stack + outlier OLS pair, `590` Shamir on constant $f(x)=3$ |
| 13 | `601–650` | 12 | 25 | 13 | `629` Du Bois color-line cartography, `618` Tissandier walnut-shell *ludion*, `646` degenerate Lotka–Volterra equilibrium line ($x+y=1, \lambda=0$), `650` parabolic vs. perturbed OLS pair |
| 14 | `651–700` | 23 | 18 | 9 | `694` Mars projectile hitting upslope while still rising ($v_y > 0$), `667` mod-7 quadratic probing orbit saturation with tombstones, `653` walnut/flax 2/1 twill drawdown, `699` Meketre tomb bakery/brewery model |

### Concrete code, generator, and corpus findings surfaced by Argon

Every Argon reviewer inspected the relevant `generator.rs` and corpus files to
distinguish sampling luck from code/data defects:

| ID(s) | Root seed(s) | Leaf path | Finding | Actionable fix |
| --- | --- | --- | --- | --- |
| `512`, `536`, `593`, `688` | `14620060181762999689`, `8297691499053210432`, `8992086442858998809`, `11851236082833746207` | `philosophy/logic/truth-tables`, `philosophy/logic/modal-kripke` | Confirmed AST generator flaw: `random_expr` / `random_formula` draw left and right operands of binary connectives independently without pruning identical subformulas, emitting degenerate `(q ∧ q)`, `◇¬(p ∧ p)`, and `□(q ∨ q)`. | Disallow identical left/right child expressions in binary connectives (`And`, `Or`, `Imp`, `Iff`) in both generators; compactly format dense Kripke relations `R`. |
| `626` | `9544776720666203506` | `cryptography/secret-sharing/shamir` | Re-confirmed pluralization bug at `generator.rs:54` when $k=2$: prints `"Two polynomials through these 1 shares"`. | Branch on `k - 1 == 1` (`"through this 1 share"`). |
| `676` | `1771844156593791866` | `games/abstract/impartial/subtraction` | Confirmed generator omission: `subtraction/generator.rs` computes `outcomes(40, &moves)` to balance N/P sampling, but prints only `Subtraction / Heap: 18 / Removals: 3 5 6`—discarding the N/P outcome, period, and winning moves that sibling leaves `nim` and `kayles` output. | Emit `N-position`/`P-position`, period, and winning removals in `subtraction/generator.rs`. |
| `634` | `9495404086545031986` | `design/textiles/knitting` | Un-staggered cable repeat prints 11 identical consecutive lines (`Round 2` through `Round 12: [p3, k6, p3, k6, p3]`). | Collapse consecutive identical plain rounds (e.g., `Rounds 2–12  [p3, k6, p3, k6, p3]`) or stagger cable crossings. |
| `575` | `13343442330048418717` | `cryptography/error-correction/hamming` | `mode == 0` in `hamming/generator.rs` emits an all-clean 5-word table where every syndrome is `000` and no error occurs. | Drop or sharply downweight `mode == 0` so every draw shows single-bit correction, parity-bit flip, double-error SECDED rejection, or burst interleaving. |
| `539`, `611` | `15651157310407989756`, `3636184415906016052` | `math/linear-algebra/.../orthogonal-projections` | Prints a raw $3\times 3$ rational projection matrix and vector product (`-152/17`), omitting the complementary orthogonal decomposition ($x = x_\parallel + x_\perp$) that its own test suite verifies. | Include the complementary subspace decomposition or geometric invariant in the output. |
| `502`, `534`, `557`, `565`, `604`, `617`, `647`, `657`, `665`, `681` | Multiple | `chemistry/reactions/*`, `chemistry/formulas/.../empirical-formulas`, `math/analysis/integrals/indefinite/logarithms`, `math/probability/.../finite-mass-functions` | Static 12–13 item arrays (`oxygenated-fuels`, `neutralization`, `metal-displacement`), GCD subscript reductions (`empirical-formulas`, 3 hits in Batch 14), and uniform constant-substitution drills (`logarithms`, `finite-mass-functions`) lack regime boundaries. | Upgrade to boundary/comparative regimes (e.g., oxygen-limited vs. excess combustion, titration equivalence/buffer cliffs, parameter-bifurcating integrals) or lower leaf weights. |
| `560`, `592`, `636`, `655`, `695` | `10722680462749208932`, `16996609619189405362`, `10912476706340342031`, `1834688937045230856`, `14618586432034031902` | `materials/catalogues/textiles`, `geography/.../rivers`, `language/lexicon/*`, `mechanisms/.../escapements/historical` | Corpus blemishes: `zorbeez` (infomercial brand noise in textiles); `Map extent center` GIS phrasing (`592`); circular antonym glosses (`636` `unfair — not fair`); unillustrated diagram letters `piece, B`, `lever, E, C`, `notch, E` (`655`, Brown no. 296); trivial ordinal definition `third (n.)` (`695`). | Add exclusions for `zorbeez`, ordinal/cardinal numerals, and circular `not X` antonym glosses; filter diagram-dependent Brown passages; soften `Map extent center` label. |

### Where Argon agreed with — and sharpened — the GPT-6.1-sol findings

1. **Zero-slop discipline and boundary-seeking generators are the core strength:**
   All four Argon reviewers independently praised the complete absence of LLM
   preamble and highlighted the exact same class of high-leverage outputs:
   equal-summary OLS pairs (`507`, `567`, `586`, `650`), pooled vs. split
   queues (`532`, `570`), ecological invasion/Hopf thresholds (`517`, `646`,
   `682`), physical/navigation reversals (`574`, `694`), hash-table probe orbit
   saturation (`667`), railway loop/overtake schedules (`514`, `541`, `591`,
   `606`), and tactile primary sources (Montagu, Equiano, Du Bois, Vitruvius,
   de Dillmont, Farmer, Tissandier, Hearn, Weekley).
2. **Shallow top-level branches create leaf-level hotspots:**
   While the balance study's atom cap (`0.02451%`) prevents any single *exact
   corpus string* from dominating, shallow top-level branches still funnel
   `0.98%`–`1.66%` of root probability into single generator leaves or tiny
   single-book corpora:
   - `philosophy/logic/*`: 3 leaves at `1.31%` each (`11` draws across the 200
     Argon samples).
   - `cryptography/*`: 4 leaves at `0.98%` each (`13` draws across the 200
     Argon samples, including four `Hamming(8,4)` tables in Batch 12 alone and
     three `rsa-toy` traces in Batch 14).
   - `transport/rail/timetables` and `transport/navigation/dead-reckoning`:
     `0.98%` each.
   - `procedures/craft`: `0.784%` across 36 records from a single book
     (de Dillmont's *Encyclopedia of Needlework*, 3 hits in Batch 11).
   - `procedures/experiments`: `0.784%` across 16 Tissandier records (`0.049%`
     per record, 3 hits in Batch 14).
3. **Expanding `society`, `philosophy`, and `procedures/craft`:**
   Like the first ten reviewers, all four Argon reviewers noted that `society`
   (7 economics/voting generators) and `philosophy` (3 formal-logic generators)
   are 100% formal generators in `tree.txt`, and recommended adding sourced
   legal/institutional/charter/paradox leaves alongside non-needlework crafts
   (joinery, dyeing, ceramics, bookbinding) and material failure/repair.

### Argon artifacts for Astra

All raw batches, frozen first impressions, full reviews, per-sample verdicts,
and summaries are stored in `target/vibe-check-2026-11-02/`:

- Batches & seeds: `batch-11.txt`..`batch-14.txt`, `batch-11.jsonl`..`batch-14.jsonl`, `samples-argon.jsonl`, `manifest-argon.json`
- Frozen first impressions: `first-impressions-11.md`..`first-impressions-14.md`
- Full reviews: `review-11.md`..`review-14.md`
- Verdicts & summaries: `assessments-11.json`..`assessments-14.json`, `summary-11.json`..`summary-14.json`, `aggregate-argon.json`

## Addendum: single Opus review

One Claude Opus reviewer read 60 fresh root draws (seeds `9000000 + 7919·i`,
`i = 1..60`, same binary), then checked suspicions with 200 seeds per leaf. The
broad verdict agrees with the panels. The truth-table flaw repeats Argon's
finding, and the panel noted inert bottleneck probes; the rates below are new.

Confirmed defects:

- Bulfinch OCR: `hy` for "by" (3), `bo your bees`, and seven broken hyphens
  (`sea- calves`, `sharp- pointed`, `death- warrant`, `daughters-in- law`).
  Fix in the importer if one exists, otherwise in `records.json`.
- `philosophy/logic/truth-tables`: 45/200 formulas contain an idempotent
  subterm such as `(s ∧ s)`. `¬((s ∧ s) ∨ ¬(s ∧ s))` reads as noise, not a
  boundary case.
- `random/geometry/shapes`: 47/200 are triangles that still print
  `Reflex vertices = 0`. Some are needles (`M 25 0 L 35 1 L 0 1 Z`, height 1).
  Omit the reflex line for triangles and reject extreme aspect ratios.
- `systems/flows/bottlenecks`: the capacity probe left maximum flow unchanged
  in 119/200 seeds. One nonbinding probe teaches something; at 60% it is the
  default. Probe a cut edge most of the time.

Polish:

- Attribution lines use three styles: `— Author`, `- Author`, and bare
  `Author, Title`. Pick one.
- Degenerate grid ranges (`rows 1–1`, `columns 6–6`) should print one index.
- Math definition records mix ASCII (`f:I->R`, `=>`) with Unicode (`∈`, `Σₖ`).

Breadth observation: every human-voiced passage in the 60 draws dates from
1716–1922, and non-European sources arrive through 19th-century English
translators. That follows from public-domain sourcing. US federal works offer
modern first-person voices without licensing trouble: NASA ASRS incident
narratives, Apollo air-to-ground transcripts, NTSB docket interviews, WPA and
Library of Congress oral histories, and NPS ranger field notes. ASRS fits the
project's taste especially well: short, procedural, mundane, with a near miss.
`literature/dialogue` currently has 0.11% effective weight.

Elections agree on every method in 57/200 seeds. That is a reasonable baseline
rate and needs no change. Seed 9007919 shows the generator at its best:
plurality, Borda, and IRV choose different winners, and Borda matches Condorcet.
