# Critical-path schedules

Activity-on-node DAGs with integer durations mix chains, nearly tied diamond
branches, three co-critical branches, nested forks and joins, multiple starts
and finishes, and random ordered DAGs. Networks have at most eleven tasks.

Dependencies are finish-to-start with zero lag. Time starts at day zero, all
tasks are required, and parallel tasks have unlimited resources. Durations are
fixed. A forward longest-path pass finds earliest starts; a backward pass finds
latest starts against the earliest possible project finish. Slack is total
float. Every critical source-to-terminal path is enumerated. A selected task is
then shortened by one day and the entire schedule is recomputed.

Tests independently enumerate all source-to-terminal paths and certify project
duration, the complete critical-path set, and each task's slack from the longest
path through that task. They also verify precedence in both schedules and check
the effect of a one-day delay to every task.

Source: C. Hendrickson and T. Au, Carnegie Mellon University,
[Project Management for Construction, chapter 10](https://www.cmu.edu/cee/projects/PMbook/10_Fundamental_Scheduling_Procedures.html),
critical paths, activity-on-node schedules, and total float.

Projects are synthetic. No task lists or source prose are copied.
