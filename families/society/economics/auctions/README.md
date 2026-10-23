# Single-item auctions

Each sample fixes three to six bidders' integer private values and submitted
bids, then computes allocations, bidder payments, seller revenue and
quasilinear utilities under three sealed-bid rules. The same bids are reused
to isolate the payment-rule difference; they are not claimed to be equilibrium
strategies. Values are synthetic private values, with no distributional or
independence assumption.

The largest bid meeting the public reserve wins. Equality meets the reserve.
A displayed priority order, fixed before bids, resolves equal bids.

- First-price charges the winner its bid and charges everyone else zero.
- Second-price charges the winner the larger of the reserve and the largest
  opposing bid, and charges everyone else zero.
- All-pay charges every submitted bid regardless of allocation. The reserve
  is an allocation threshold, with no refunds if the item remains unsold.
  This last convention is explicit in every sample.

Utilities are value received minus payment. Seller revenue is the sum of
payments; no seller value or social-cost model is assumed. Zero bids may win
at a zero reserve. Losing all-pay bidders with positive bids have negative
utility.

Nine equally likely constructions cover ordinary shaded bids, an inefficient
allocation without overbidding, an unsold item despite a value above reserve,
one eligible bidder, sale exactly at reserve with zero winner surplus, tied
highest bids, negative total all-pay buyer utility without overbidding, a
winning overbid, and unrestricted profiles. Bidder labels and tie priority
are shuffled independently. No common-value model, winner's-curse inference,
English clock or Dutch stopping model is included.

Tests exhaust small bids, reserves and priority orders against sorted order
statistics, check transfer and utility conservation, exhaustively check the
second-price truthful-bid inequality, and verify each constructed regime.

Primary sources:

- [Glenn Ellison, MIT 14.271, Lecture 22: Auctions, Part 1](https://ocw.mit.edu/courses/14-271-industrial-organization-i-fall-2022/mit14_271_f22_lec22slides.pdf), slides 3, 4, 7 and 13: private values, utility and the three payment rules.
- [Tim Roughgarden and Joshua R. Wang, *Minimizing Regret with Multiple Reserves*](https://theory.stanford.edu/~tim/papers/multres.pdf), section 1: reserve-price auction conventions.

No source text or implementation is embedded. Instances and code are original.
