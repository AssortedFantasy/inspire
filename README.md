# Inspire

`inspire` is a small, offline CLI that provides natural-language entropy for AI agents.

Post-trained models suffer from RL mode collapse: ask an agent to make an open-ended or creative choice a dozen times and it will converge on the same handful of tropes, names, palettes, and structures. `inspire` breaks that symmetry by giving the agent a fast, zero-dependency command it can call to draw a random string. Such as a museum garment, a Markov chain, a literary quotation, a dice roll, a place, or a weaving draft—before deciding.

Everything is compiled into a single standalone binary with no network calls or runtime files.

## Install

Build and install with Cargo:

```sh
cargo install --path . --locked
```

Then add this to your project or global `AGENTS.md`:

```markdown
When making a creative or open-ended decision, run `inspire` in the shell (multiple times if helpful). It prints a random, task-unrelated text sample for entropy.
```

## Examples

```console
$ ./inspire
THE cobweb pretends to catch dew-drops and catches flies.
- Rabindranath Tagore, Stray Birds (1916)

$ ./inspire
5d8, keep 3 highest: [4, 8, 5, 8, 3] => [8, 8, 5] = 21

$ ./inspire
Travelers in Hilly Countryside — c. 1650
Aelbert Cuyp (Dutch, 1620–1691), artist
oil on wood
Culture - Netherlands

$ ./inspire
Misère Nim (last counter loses)
Heaps: H1=1 H2=1 H3=1 H4=1 H5=9
N-position
Winning reductions: H5 9→1

$ ./inspire
Shōchikubai — Three Friends of Winter
Pine — longevity
Bamboo — perseverance
Plum — renewal
Japanese kimono motifs
- Victoria and Albert Museum

$ ./inspire
Nakuru - Kenya
```

For CLI flags (`--family`, `--seed`, `--list`, `--json`) and a list of all 26 topic families, see [docs/USAGE.md](docs/USAGE.md).

## License and Data

The `inspire` source code and procedural generators are licensed under the [MIT License](LICENSE).

In addition to procedural generators, the binary embeds curated text and factual records drawn from public-domain texts, open government databases, and permissively licensed datasets (including CC0, CC BY 4.0, WordNet, Unicode, BSD, and ColorBrewer). Those source datasets retain their respective licenses and attributions. Run `inspire --data-licenses` or see [DATA_SOURCES.md](DATA_SOURCES.md) for full provenance and redistribution notices.
