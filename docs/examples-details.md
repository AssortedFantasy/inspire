# Example details

[Back to examples](examples.md).

Generated November 2, 2026 with `gpt-6.1-terra`: ten short poems at medium
reasoning and ten SVGs at high reasoning. Each group has five independent calls
with Inspire and five without. All twenty outputs are shown in generation order
within each group, without selection or editing; surrounding SVG fences are removed.

## Prompts

Every call starts fresh with this developer message:

```text
Complete the requested creative task. Return only the finished work.
```

Without Inspire, the user message is just the task prompt:

```text
Write a short poem of 4–8 lines.
```

```text
Draw an animal on a skateboard as a self-contained SVG with viewBox="0 0 400 400". Use no scripts or external resources. Return only the SVG.
```

With Inspire, the user message is:

```text
Use this random text as inspiration. You may use or ignore it.

<inspiration>
{complete Inspire output}
</inspiration>

{task prompt}
```

Each inspired call receives one unselected root draw from
`inspire --seed SEED --json`, using only its `text` field in the prompt.

## Run settings

Responses API; `max_output_tokens: 16384`, `service_tier: "default"`,
`store: false`. No temperature or top-p override, tools, or prior conversation.
The [request and output archive](data/examples.json) contains every exact request,
output, Inspire draw, seed, response model, and token count.

Inspire source: `dd294b8380189dae9eb0cf3a6330071bc0f89264`.

## Inspire draws

Numbers match the rows in each comparison.

### Poem 1

`--seed 1223725163901051235` · `arts/painting/works`

```text
Drawing of an Elephant — c. 1700
ink on paper
Culture - India, Mughal School, early 18th Century
```

### Poem 2

`--seed 17869591926096135607` · `systems/reliability/fault-trees`

```text
TOP = AND(A, B, C)
Independent basic events on one demand
P(A) = 0.04
P(B) = 0.02
P(C) = 0.03
Minimal cut sets: {A, B, C}
P(TOP) = 3/125000 ≈ 2.4e-5
```

### Poem 3

`--seed 11624963402158238667` · `food/nutrition/vegetables-and-vegetable-products`

```text
Asparagus, raw
Per 100 g edible portion
Energy 20 kcal
Protein 2.2 g
Fat 0.12 g
Carbohydrate 3.88 g
Fibre 2.1 g
Sodium 2 mg
```

### Poem 4

`--seed 2940616870255189056` · `philosophy/primary-texts/james`

```text
The naturalist answer is that the environment kills as well as sustains us, and that the tendency of raw experience to extinguish the experient himself is lessened just in the degree in which the elements in it that have a practical bearing upon life are analyzed out of the continuum and verbally fixed and coupled together, so that we may know what is in the wind for us and get ready to react in time. Had pure experience, the naturalist says, been always perfectly healthy, there would never have arisen the necessity of isolating or verbalizing any of its terms. We should just have experienced inarticulately and unintellectually enjoyed. This leaning on 'reaction' in the naturalist account implies that, whenever we intellectualize a relatively pure experience, we ought to do so for the sake of redescending to the purer or more concrete level again; and that if an intellect stays aloft among its abstract terms and generalized relations, and does not reinsert itself with its conclusions into some particular point of the immediate stream of life, it fails to finish out its function and leaves its normal race unrun.

— William James, Essays in Radical Empiricism, The Thing and Its Relations, II (1912)
```

### Poem 5

`--seed 7648719065386770608` · `philosophy/logic/syllogisms`

```text
Some M are not P.
Some S are M.
∴ No S are P.
OIE–1
Invalid even assuming S, P and M are all nonempty.
Countermodel
D = {a0, a1}
S = {a0, a1}
P = {a1}
M = {a0, a1}
```

### SVG 1

`--seed 14062307532474499100` · `philosophy/primary-texts/wollstonecraft`

```text
Men, in general, seem to employ their reason to justify prejudices, which they have imbibed, they cannot trace how, rather than to root them out. The mind must be strong that resolutely forms its own principles; for a kind of intellectual cowardice prevails which makes many men shrink from the task, or only do it by halves. Yet the imperfect conclusions thus drawn, are frequently very plausible, because they are built on partial experience, on just, though narrow, views.

— Mary Wollstonecraft, A Vindication of the Rights of Woman, chapter 1 (1792)
```

### SVG 2

`--seed 11985304782249288717` · `literature/verse/english/early-modern/shakespeare/sonnet-quatrains`

```text
Not from the stars do I my judgement pluck;
And yet methinks I have astronomy,
But not to tell of good or evil luck,
Of plagues, of dearths, or seasons’ quality;
- William Shakespeare, Sonnet 14 (1609), lines 1–4 [excerpt]
```

### SVG 3

`--seed 14202904163762105680` · `literature/narrative/anecdotes/persian/sadi/gulistan`

```text
An evil-disposed man is a captive in the hands of an enemy (namely, himself); for wherever he may go he cannot escape from the grasp of that enemy's vengeance:—Let a wicked man ascend up to heaven, that he may escape from the grasp of calamity; even thither would the hand of his own evil heart follow him with misfortune.
- Sa’di; English translation by James Ross, The Gulistan (1900 revised anthology edition), Chapter VIII, anecdote XXII
```

### SVG 4

`--seed 7475801933202892818` · `arts/drawing/works`

```text
Architectural Studies (verso) [partially visible on recto] — c. 1580
Paolo Veronese (Italian, 1528–1588), artist
pen and brown ink
Culture - Italy, Venice
```

### SVG 5

`--seed 9517153576612642625` · `chemistry/compounds/organic/biomolecules/carbohydrates/monosaccharides/aldoses/hexoses/pyranoses`

```text
D-allose (C6H12O6)
Pyranose, anomer unspecified
Molar mass: 180.16 g/mol
SMILES: C([C@@H]1[C@H]([C@H]([C@H](C(O1)O)O)O)O)O
```
