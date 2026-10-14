# Recursive review handoff

Own the assigned node under `families/history`. Read `families/AGENTS.md`,
this family's README, and these instructions. Do not edit other families,
shared Cargo/tests/docs, run the main Cargo build, commit, or push.

Every directory node has its own agent. A branch delegates exactly one
unfinished immediate child at a time; single-child branches also delegate.
The same agent resumes after callbacks, preserving its existing child-agent
identities. Keep at most five live agents in this family subtree.

To release capacity, dispatch the next child with its complete node path,
parent canonical agent and these instructions, then finish your turn with a
CHECKPOINT. A finished child calls `collaboration.followup_task` on that SAME
parked parent with its fully validated result, then returns FINAL. The resumed
parent integrates it and advances its sibling queue. Root history is
`/root/history`; notify it with `send_message` only when it is active. Never
followup the global `/root` manager. On a slot-limit failure, park and notify
`/root` with the exact agent identity that must resume.

A terminal leaf reviews **every actual record**, not just sample counts.
Verify its coherent historical topic, source and precise narrator/period/
chapter context; identify incomplete, unintelligible or dependent fragments,
unsupported synthesized claims, chapter synopses, notes, transitions, omitted
illustrations, misleading cultural generalizations and repetitive filler.
Old primary-source wording is quotation, not a modern verified fact. Keep
useful real content and preserve original 72 IDs unless a verified correction
justifies a change. Historical laws are a legal collection, not evidence that
every case was enforced; autobiographies and outsiders' accounts retain their
perspective and uncertainty.

Imported records derive from `selections.json`, `sources.json` and the pinned
HTML files under `target/imports/history`. Reproduce normalization through
`import_data.Paragraphs` and `import_data.clean` using Python **-B**. To improve
an excerpt, adjust its explicit contiguous source offsets, locator or leaf
and recalculate `text_sha256`, or remove it. Never invent connecting sentences
or silently rewrite quotation. Document new removals in `exclusions.json`.
Do not edit only `records.json` for imported excerpts: replay must retain your
change. Preserve nonempty nodes; if a split needs refactoring, coordinate with
your parent and ensure every resulting final leaf has its own reviewer.
Do not alter the global parser casually: it changes other leaves' hashes.

After editing, run these source checks from repository root:

```sh
python -B families/history/import_data.py
python -B families/history/import_data.py --check
python -B families/history/test_import.py
```

The source schema is exactly `id`, `text`, `provenance`; nonblank, unique IDs,
no NUL, distinct meaningful texts, no empty or mixed-payload nodes. Runtime
stays offline. Keep full source notices and emitted attribution. Default
sibling weights balance topics; only change them for an explicit reason.

Once fully reviewed, append one entry to `families/history/review_manifest.json`
with `path` relative to `families` (starting `history/`), your distinct canonical
`reviewer`, substantive `checks`, optional `corrections`, `status: completed`
and the **current** `records` count. No branch may impersonate a leaf reviewer.
Branches inspect their descendants' ledger and corrections before callback;
they report COMPLETE only after every final descendant leaf is reviewed.
The family manager updates final counts/docs and runs the coverage checker.
