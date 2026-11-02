# Finite automata

Six output forms mix direct recognizers and traces, state minimization,
Boolean product machines, shortest distinguishing words, finite codebooks,
and NFA subset construction. Languages include overlapping binary patterns,
binary-value and population residues, bounded balanced words, and a chosen
bit counted from the right. Products include compatible and incompatible
residue constraints; state expansion introduces equivalent states and an
unreachable component. Finite codebooks share tails as well as prefixes.

Tables, reachability, quotient classes, witnesses, and acceptance results are
computed. Minimization displays the original machine and quotient classes;
the quotient table is derivable from those classes. Equivalence comparisons
specify B as a change to A. Subset tables use at most three bits of suffix
memory (eight states). Partition refinement removes unreachable states before constructing
the quotient. Witness searches traverse the state or pair graph in breadth
first order. Subset construction starts from a nondeterministic machine that
guesses the selected bit and counts the remaining symbols.

Tests compare recognizers with string operations, integer arithmetic, prefix
balances, direct bit indexing, and exact codebook membership. Independent
pair-graph searches check every equivalence class and minimal-state pair;
exhaustive short words check products and shortest witnesses.

The alphabet is binary and transitions are total. Binary residue machines
allow leading zeros and assign the empty word value zero. Balance machines
use 0 as an opening symbol and 1 as a closing symbol. Displayed state labels
have no meaning beyond the accompanying machine. These are small exact
machines, not a general regular-expression parser or Unicode lexer.

The implementation and emitted examples are original. The algorithmic
reference is Cornell CS 4120,
[Automating Lexical Analysis](https://www.cs.cornell.edu/courses/cs4120/2022sp/notes/leximpl/),
especially the sections on subset construction, reachable-state minimization,
and distinguishing suffixes. No source prose or code is embedded.
