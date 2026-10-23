use rand::{Rng, rngs::StdRng};
use std::{collections::BTreeMap, fmt::Write};

fn interleavings(left: usize, right: usize) -> Vec<Vec<usize>> {
    fn visit(remaining: [usize; 2], prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if remaining == [0, 0] {
            out.push(prefix.clone());
            return;
        }
        for thread in 0..2 {
            if remaining[thread] == 0 {
                continue;
            }
            let mut next = remaining;
            next[thread] -= 1;
            prefix.push(thread);
            visit(next, prefix, out);
            prefix.pop();
        }
    }
    let mut out = Vec::new();
    visit([left, right], &mut Vec::new(), &mut out);
    out
}

fn increment_run(start: i32, delta: [i32; 2], order: &[usize]) -> (i32, Vec<String>) {
    let mut memory = start;
    let mut local = [0; 2];
    let mut pc = [0; 2];
    let mut trace = Vec::new();
    for &thread in order {
        let action = match pc[thread] {
            0 => {
                local[thread] = memory;
                format!("r{thread} = load(x) → {memory}")
            }
            1 => {
                local[thread] += delta[thread];
                format!("r{thread} += {} → {}", delta[thread], local[thread])
            }
            2 => {
                memory = local[thread];
                format!("store(x, r{thread}) → x={memory}")
            }
            _ => unreachable!(),
        };
        trace.push(format!("T{thread}  {action}"));
        pc[thread] += 1;
    }
    (memory, trace)
}

fn increments(rng: &mut StdRng) -> String {
    let start = rng.random_range(-5..=12);
    let mut delta = [rng.random_range(1..=5), rng.random_range(1..=5)];
    if rng.random_bool(0.35) {
        delta[1] = -delta[1];
    }
    let mut outcomes = BTreeMap::<i32, Vec<Vec<String>>>::new();
    for order in interleavings(3, 3) {
        let (value, trace) = increment_run(start, delta, &order);
        outcomes.entry(value).or_default().push(trace);
    }
    let choices: Vec<_> = outcomes.iter().collect();
    let (&value, traces) = choices[rng.random_range(0..choices.len())];
    let trace = &traces[rng.random_range(0..traces.len())];
    let mut text = format!(
        "SC atomic loads/stores, x={start}\nT0 adds {}, T1 adds {}",
        delta[0], delta[1]
    );
    for (i, event) in trace.iter().enumerate() {
        write!(text, "\n{}  {event}", i + 1).unwrap();
    }
    write!(
        text,
        "\nFinal x={value}\nSerial result {}\nAll possible final values: {}",
        start + delta[0] + delta[1],
        outcomes
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
    .unwrap();
    text
}

#[derive(Clone, Default)]
struct LockRun {
    pc: [usize; 2],
    owner: [Option<usize>; 2],
    trace: Vec<String>,
}

fn lock_runs(orders: [[usize; 2]; 2]) -> Vec<LockRun> {
    fn visit(state: LockRun, orders: [[usize; 2]; 2], out: &mut Vec<LockRun>) {
        let mut advanced = false;
        for (thread, order) in orders.iter().enumerate() {
            let pc = state.pc[thread];
            if pc == 4 {
                continue;
            }
            let lock = order[if pc < 2 { pc } else { 3 - pc }];
            if pc < 2 && state.owner[lock].is_some() {
                continue;
            }
            let mut next = state.clone();
            let name = (b'A' + lock as u8) as char;
            let action = if pc < 2 {
                next.owner[lock] = Some(thread);
                "lock"
            } else {
                assert_eq!(next.owner[lock], Some(thread));
                next.owner[lock] = None;
                "unlock"
            };
            next.trace.push(format!("T{thread}  {action}({name})"));
            next.pc[thread] += 1;
            visit(next, orders, out);
            advanced = true;
        }
        if !advanced {
            out.push(state);
        }
    }
    let mut out = Vec::new();
    visit(LockRun::default(), orders, &mut out);
    out
}

fn locks(rng: &mut StdRng) -> String {
    let first = usize::from(rng.random_bool(0.5));
    let second = if rng.random_bool(0.65) {
        1 - first
    } else {
        first
    };
    let orders = [[first, 1 - first], [second, 1 - second]];
    let runs = lock_runs(orders);
    let deadlock = first != second && rng.random_bool(0.6);
    let choices: Vec<_> = runs
        .iter()
        .filter(|run| (run.pc != [4, 4]) == deadlock)
        .collect();
    let run = choices[rng.random_range(0..choices.len())];
    let mut text = String::from("Blocking mutexes A and B, initially free");
    for (thread, order) in orders.iter().enumerate() {
        let a = (b'A' + order[0] as u8) as char;
        let b = (b'A' + order[1] as u8) as char;
        write!(
            text,
            "\nT{thread}: lock({a}), lock({b}), unlock({b}), unlock({a})"
        )
        .unwrap();
    }
    text.push('\n');
    for (i, event) in run.trace.iter().enumerate() {
        write!(text, "\n{}  {event}", i + 1).unwrap();
    }
    if deadlock {
        text.push_str("\nDeadlock");
        for (thread, order) in orders.iter().enumerate() {
            let lock = order[run.pc[thread]];
            let name = (b'A' + lock as u8) as char;
            write!(
                text,
                "\nT{thread} waits for {name}, held by T{}",
                run.owner[lock].unwrap()
            )
            .unwrap();
        }
    } else {
        text.push_str("\nBoth threads completed, both mutexes free");
    }
    text
}

#[derive(Clone, Default)]
struct BufferRun {
    pc: [usize; 2],
    memory: [u8; 2],
    pending: [Option<u8>; 2],
    read: [Option<u8>; 2],
    trace: Vec<String>,
}

fn buffer_runs(values: [u8; 2], buffered: bool) -> Vec<BufferRun> {
    fn visit(state: BufferRun, values: [u8; 2], buffered: bool, out: &mut Vec<BufferRun>) {
        if state.pc == [2, 2] && state.pending == [None, None] {
            out.push(state);
            return;
        }
        for (thread, &value) in values.iter().enumerate() {
            let own = if thread == 0 { 'x' } else { 'y' };
            let other = if thread == 0 { 'y' } else { 'x' };
            if state.pc[thread] < 2 {
                let mut next = state.clone();
                let event = if state.pc[thread] == 0 {
                    if buffered {
                        next.pending[thread] = Some(value);
                        format!("T{thread}  buffer {own}={value}")
                    } else {
                        next.memory[thread] = value;
                        format!("T{thread}  store {own}={value}")
                    }
                } else {
                    let read = state.memory[1 - thread];
                    next.read[thread] = Some(read);
                    format!("T{thread}  r{thread}=load({other}) → {read}")
                };
                next.pc[thread] += 1;
                next.trace.push(event);
                visit(next, values, buffered, out);
            }
            if let Some(value) = state.pending[thread] {
                let mut next = state.clone();
                next.memory[thread] = value;
                next.pending[thread] = None;
                next.trace.push(format!("T{thread}  flush {own}={value}"));
                visit(next, values, buffered, out);
            }
        }
    }
    let mut out = Vec::new();
    visit(BufferRun::default(), values, buffered, &mut out);
    out
}

fn buffers(rng: &mut StdRng) -> String {
    let values = [rng.random_range(1..=9), rng.random_range(1..=9)];
    let buffered = rng.random_bool(0.75);
    let runs = buffer_runs(values, buffered);
    let mut outcomes = BTreeMap::<[Option<u8>; 2], Vec<&BufferRun>>::new();
    for run in &runs {
        outcomes.entry(run.read).or_default().push(run);
    }
    let choices: Vec<_> = outcomes.values().collect();
    let group = choices[rng.random_range(0..choices.len())];
    let run = group[rng.random_range(0..group.len())];
    let model = if buffered {
        "TSO store buffers"
    } else {
        "Sequential consistency"
    };
    let mut text = format!(
        "{model}, x=y=0\nT0: store(x, {}), r0=load(y)\nT1: store(y, {}), r1=load(x)\n",
        values[0], values[1]
    );
    for (i, event) in run.trace.iter().enumerate() {
        write!(text, "\n{}  {event}", i + 1).unwrap();
    }
    let reads = |pair: &[Option<u8>; 2]| format!("({}, {})", pair[0].unwrap(), pair[1].unwrap());
    write!(
        text,
        "\n(r0, r1) = {}\nAllowed pairs: {}",
        reads(&run.read),
        outcomes.keys().map(reads).collect::<Vec<_>>().join(", ")
    )
    .unwrap();
    text
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..3) {
        0 => increments(rng),
        1 => locks(rng),
        _ => buffers(rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn exhaustive_split_increment_outcomes() {
        let orders = interleavings(3, 3);
        assert_eq!(orders.len(), 20);
        for deltas in [[1, 1], [3, 2], [2, -3]] {
            let actual: BTreeSet<_> = orders
                .iter()
                .map(|order| increment_run(7, deltas, order).0)
                .collect();
            let expected = [7 + deltas[0], 7 + deltas[1], 7 + deltas[0] + deltas[1]]
                .into_iter()
                .collect();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn consistent_lock_order_terminates_and_inversion_can_deadlock() {
        for order in [[0, 1], [1, 0]] {
            for run in lock_runs([order, order]) {
                assert_eq!(run.pc, [4, 4]);
                assert_eq!(run.owner, [None, None]);
            }
        }
        let runs = lock_runs([[0, 1], [1, 0]]);
        assert!(runs.iter().any(|run| run.pc == [4, 4]));
        let deadlocks: Vec<_> = runs.iter().filter(|run| run.pc != [4, 4]).collect();
        assert!(!deadlocks.is_empty());
        for run in deadlocks {
            assert_eq!(run.pc, [1, 1]);
            assert_eq!(run.owner, [Some(0), Some(1)]);
        }
    }

    #[test]
    fn sb_zero_zero_requires_buffering() {
        let outcomes = |buffered| {
            buffer_runs([3, 7], buffered)
                .iter()
                .map(|run| {
                    assert_eq!(run.memory, [3, 7]);
                    assert_eq!(run.pending, [None, None]);
                    [run.read[0].unwrap(), run.read[1].unwrap()]
                })
                .collect::<BTreeSet<_>>()
        };
        assert_eq!(
            outcomes(false),
            [[0, 3], [7, 0], [7, 3]].into_iter().collect()
        );
        assert_eq!(
            outcomes(true),
            [[0, 0], [0, 3], [7, 0], [7, 3]].into_iter().collect()
        );
    }
}
