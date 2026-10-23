# Markov queues

M/M/1, pooled M/M/c, and finite-capacity M/M/1/K queues. Samples cover light load,
near saturation, infinite queues at or above their stability boundary, finite
queues above offered service capacity, and pooling versus random splitting.

Arrivals are Poisson and services are independent exponential variables. Servers
are identical, service is FCFS, and K counts everyone in the system. Full finite
queues reject arrivals permanently. There is no abandonment or retrial traffic.
The printed `ρ = λ/(cμ)` is offered traffic intensity, not finite-queue utilization.

Infinite queues use the geometric/Erlang-C stationary formulas only when ρ < 1.
Finite queues normalize birth-death weights directly, including at ρ = 1. Waiting
and system times use admitted throughput in Little's law. Admission-conditional
delay uses the occupancy seen by Poisson arrivals. The pooling comparison splits
each arrival independently and uniformly among c separate M/M/1 queues.

Tests independently normalize a birth-death chain, check local probability flow
balance and completion rates, reconstruct delay and waits from arrival states,
and verify uniform occupancy for the finite critical-load case.

Source: Bertsekas and Gallager,
[Data Networks, Chapter 3](https://web.mit.edu/dimitrib/www/Queueing_Data_Nets.pdf),
especially Sections 3.2–3.4 and Problem 3.21. All parameters are synthetic; no
textbook examples or datasets are embedded.
