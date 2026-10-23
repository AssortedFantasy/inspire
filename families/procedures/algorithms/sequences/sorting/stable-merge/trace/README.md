# Stable merge traces

Five input regimes have equal probability: ordinary mixed keys, all-equal keys,
disjoint runs, alternating runs, and a singleton tied with an interior element of
a longer run. Either side can be the shorter or lower run. Item letters retain
source identity, so equal-key left-before-right stability remains visible.

Tests compare merged identities with a stable sort of the concatenated inputs,
check source consumption order, and verify each constructed regime's shape.
