# SQL

`queries` emits complete SQLite schema/data statements, a query, and its computed
result. Seven equally likely scenarios vary query structure as well as data:

- Outer joins place a threshold in `ON`, `WHERE`, or `WHERE ... OR id IS NULL`.
  Inputs include a customer with only failing matches and one with no matches.
- Anti-joins compare nullable `NOT IN`, `NOT EXISTS`, and a non-null subquery.
  Empty, all-null, duplicate, overlapping, and disjoint exclusions are sampled.
- Aggregates include null grouping keys, all-null groups, cancelling sums,
  duplicates, `HAVING`, and empty grouped or ungrouped input.
- Windows vary cumulative `ROWS`/`RANGE`, tied ranks, or lag and a sliding frame.
  Explicit ID tie-breakers make positional windows deterministic.
- Compound queries mix bag union, union, intersection, and difference, including
  duplicate nulls, identical inputs, and empty inputs.
- Recursive queries compute set reachability through cycles or bounded walk
  multiplicities through parallel edges, diamonds, loops, and disconnected parts.
- Relational division includes duplicate skills, near misses, extra skills, and
  an empty requirement set. Division inputs have at most five candidates, three
  required skills, and one duplicate skill row per candidate.

The small evaluators implement these selected query forms, not an SQL parser or
database. All cells are bounded integers or NULL; every result has an explicit
ordering. There is no text collation, affinity conversion, floating-point
aggregation, query planning, transaction model, or claim of cross-engine
portability. `UNION` reachability deduplicates node rows; `UNION ALL` walks use an
explicit depth bound. No source data or source prose is embedded.

Tests cover outer-join filter boundaries, the `NOT IN` truth matrix, aggregate
null handling, peer windows and partition resets, bag/set multiplicity, recursive
walk counts, and vacuous division. Reachability is compared with an independent
Floyd–Warshall closure over every directed graph on three vertices.

## References

- SQLite [SELECT](https://www.sqlite.org/lang_select.html), for outer joins,
  grouping, compound operators, null duplicate elimination, and output ordering.
- SQLite [expressions](https://www.sqlite.org/lang_expr.html), for three-valued
  predicates, `IN`/`NOT IN`, and correlated `EXISTS`.
- SQLite [aggregate functions](https://www.sqlite.org/lang_aggfunc.html), for
  `COUNT`, `DISTINCT`, and null/empty `SUM` behavior.
- SQLite [window functions](https://www.sqlite.org/windowfunctions.html), for
  peers, positional frames, ranks, and `LAG`.
- SQLite [WITH](https://www.sqlite.org/lang_with.html), for recursive set/bag
  semantics and termination through deduplication or a depth predicate.
