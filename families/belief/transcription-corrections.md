# Bulfinch transcription repairs

`transcription-corrections.json` freezes twelve narrow replacements in the pinned
Project Gutenberg 3327 transcription. The importer applies them after excerpt
selection, so frozen character spans continue to refer to the original cleaned
source. Counts, IDs, source order and excerpt boundaries are unchanged.

The independent [1913 Bulfinch edition, Project Gutenberg 56644](https://www.gutenberg.org/cache/epub/56644/pg56644-images.html)
was checked on 2026-11-02. Its corresponding passages corroborate the three `hy`
readings as `by` (Perseus, Sphinx, Aristaeus), and `bo your bees` as `to your bees`.
The eight space repairs join the existing hyphen to its continuation: `sharp-pointed`,
`lookers-on`, `daughters-in-law`, `Sea-Nymphs`, `death-warrant`, `sea-calves`,
`wide-spread`, and `harp-string`. The corroborating edition sometimes prints an
open compound (sharp pointed, death warrant, wide spread); we retain the pinned
edition's existing hyphen and remove only the spurious intervening space.

These are transcription repairs, not modernization or replacement with another
edition. Other unusual source wording remains untouched; this pass is not a
complete collation of either edition.
