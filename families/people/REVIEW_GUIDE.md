# Staged people review

One agent owns each node; each branch manages its immediate children sequentially.
Every final corpus leaf has its own dedicated reviewer. Parents park after dispatch
and resume through a child callback, keeping one unfinished immediate child each.
Do not spawn outside your node or consume the last two global callback slots.

## Branch agents

1. Read root/family rules, people README, this guide, and your actual immediate
   directories. Review their taxonomy and descendant-count weights.
2. Spawn exactly one immediate child with full task context, canonical parent name
   and these instructions. Store your queue/current child in `review_state.json`
   within your own branch. Then return a parked checkpoint in FINAL.
3. A completed child invokes `collaboration.followup_task` on its SAME parent with
   its completed checks/corrections and ledger location, then immediately FINALs.
   Do not ask the root to create a replacement parent.
4. On callback, integrate the child ledger and advance to the next immediate node.
   If callbacks temporarily fill slots, wait for the finished child to FINAL or park
   and notify the parent of your exact canonical resume identity.
5. Once every child actually completes, aggregate their leaf entries into your
   branch's `review_manifest.json`. Run `python -B scripts/check_reviews.py --family
   people/<your-branch>` and report node taxonomy/weight checks. Callback your same
   parent, then FINAL. Root-active reporting uses `send_message`, not root callback.

## Leaf agents

Read **every emitted profile** in your leaf; do not treat a few seeds or a schema
check as a semantic review. Original profiles are in people/curated.json. Imported
profiles are pinned by people/selection.json and genuinely downloaded AgeDataset
V1 source CSVs in target/people-sources/. All source checksums and exact URLs are
in people/sources.json. A convenience extracted copy is review-selected.json in
the same cache; the CSVs are the authoritative source.

Check every record's identity, English metadata fidelity, leaf role, historical
dates, source attribution and provenance. Inspect self-contained actual output:
recognizable names, useful field/context, ambiguous homonyms, malformed names,
false occupation classification, organisations/fictional figures, unsupported
source claims, current-status phrasing, broken dates, vague fragments and filler.
The source's inferred date/gender/death columns are deliberately unused. Dates in
the actual English description remain source metadata; assess suspect cases.
Primary sources must support factual corrections. Browse primary entity or
institutional records for suspicious identities, role claims, dates and names.
Do not claim independent biographical verification of every fact if you only
checked all rows against the downloaded dataset. Describe actual checks precisely.

Keep each original ID/profile unless a specific verified correction is justified.
Globally unique QIDs prevent duplicate people even when displayed names differ.
Classify cross-disciplinary people by one explicit browsing role; mention source
bias and genuine coverage limits. Do not bulk-author new biographies or add filler.

Record reproducible corrections in people/corrections.json, with QID keys and
`reason` and primary `source` URL; supported changes are `name`, `description`, or
`exclude: true`. This shared family input is safe to edit because leaf work is
serialized. Document every change and use literal concise descriptions. Imported
selection is pinned, so exclusions remove the reviewed person without silently
replacing them with an unreviewed candidate. Do not change selection.json or move
people into another leaf without coordinating its dedicated review with managers.

Run `python -B families/people/import_data.py` after correction changes; it rebuilds
all family payloads and all descendant-count weights using pinned inputs. Do not
run concurrent importers. Verify your leaf's every record, count, unique IDs,
corrected roles and emitted CC BY 4.0 creator/source/license attribution. Preserve
the independently sourced notices of original profiles. Do not alter global files,
main Cargo files, or commit.

Write your own leaf `review_manifest.json` only when substantive review is done:

```json
[
  {
    "path": "people/arts/literature/poets",
    "reviewer": "/root/people/arts/literature/poets",
    "status": "completed",
    "records": 126,
    "checks": ["Precise actual checks, including all rows reviewed"],
    "corrections": ["Actual fixes or a clear no-corrections finding"],
    "limitations": ["Remaining source/coverage limits"]
  }
]
```

Use your **exact actual canonical agent name**, not the illustrative name. Run the
review-coverage checker for your leaf. Notify the SAME parked parent using
`followup_task` with ledger path, actual changes, count and checks, then immediately
FINAL so its callback slot is released. A source-reference URL without evidence of
reading it is not source verification. Mark completed only after actual work.
