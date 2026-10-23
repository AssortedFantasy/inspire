use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::{self, Write};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Item {
    key: i32,
    ticket: usize,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", (b'A' + self.ticket as u8) as char, self.key)
    }
}

#[derive(Clone, Copy, Debug)]
enum Order {
    Min,
    Max,
}

impl Order {
    fn before(self, a: Item, b: Item) -> bool {
        let priority = match self {
            Self::Min => a.key.cmp(&b.key),
            Self::Max => b.key.cmp(&a.key),
        };
        priority.then_with(|| a.ticket.cmp(&b.ticket)).is_lt()
    }

    fn name(self) -> &'static str {
        match self {
            Self::Min => "min",
            Self::Max => "max",
        }
    }

    fn best(self, key: i32, distance: i32) -> i32 {
        match self {
            Self::Min => key - distance,
            Self::Max => key + distance,
        }
    }
}

#[derive(Clone, Debug)]
struct Heap {
    items: Vec<Item>,
    arity: usize,
    order: Order,
}

impl Heap {
    fn empty(arity: usize, order: Order) -> Self {
        Self {
            items: Vec::new(),
            arity,
            order,
        }
    }

    fn sift_up(&mut self, mut index: usize) -> Vec<usize> {
        let mut path = vec![index];
        while index > 0 {
            let parent = (index - 1) / self.arity;
            if !self.order.before(self.items[index], self.items[parent]) {
                break;
            }
            self.items.swap(index, parent);
            index = parent;
            path.push(index);
        }
        path
    }

    fn sift_down(&mut self, mut index: usize) -> Vec<usize> {
        let mut path = vec![index];
        loop {
            let first = index * self.arity + 1;
            let end = (first + self.arity).min(self.items.len());
            let mut best = index;
            for child in first..end {
                if self.order.before(self.items[child], self.items[best]) {
                    best = child;
                }
            }
            if best == index {
                break;
            }
            self.items.swap(index, best);
            index = best;
            path.push(index);
        }
        path
    }

    fn floyd(items: Vec<Item>, arity: usize, order: Order) -> (Self, usize) {
        let mut heap = Self {
            items,
            arity,
            order,
        };
        let mut swaps = 0;
        if heap.items.len() > 1 {
            let last_parent = (heap.items.len() - 2) / arity;
            for parent in (0..=last_parent).rev() {
                swaps += heap.sift_down(parent).len() - 1;
            }
        }
        (heap, swaps)
    }

    fn inserted(items: &[Item], arity: usize, order: Order) -> (Self, usize) {
        let mut heap = Self::empty(arity, order);
        let swaps = items.iter().map(|&item| heap.push(item).len() - 1).sum();
        (heap, swaps)
    }

    fn push(&mut self, item: Item) -> Vec<usize> {
        self.items.push(item);
        self.sift_up(self.items.len() - 1)
    }

    fn pop(&mut self) -> Option<(Item, Vec<usize>)> {
        if self.items.is_empty() {
            return None;
        }
        let item = self.items.swap_remove(0);
        let path = if self.items.is_empty() {
            Vec::new()
        } else {
            self.sift_down(0)
        };
        Some((item, path))
    }

    fn change(&mut self, ticket: usize, key: i32) -> (Item, Vec<usize>) {
        let index = self
            .items
            .iter()
            .position(|item| item.ticket == ticket)
            .unwrap();
        let old = self.items[index];
        self.items[index].key = key;
        let path = if index > 0
            && self
                .order
                .before(self.items[index], self.items[(index - 1) / self.arity])
        {
            self.sift_up(index)
        } else {
            self.sift_down(index)
        };
        (old, path)
    }
}

fn array(items: &[Item]) -> String {
    format!(
        "[{}]",
        items
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn movement(path: &[usize]) -> String {
    if path.len() < 2 {
        String::new()
    } else {
        format!(
            "  sift {}",
            path.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("→")
        )
    }
}

fn insert_line(text: &mut String, heap: &mut Heap, item: Item) {
    let path = heap.push(item);
    write!(
        text,
        "\ninsert {item} → {}{}",
        array(&heap.items),
        movement(&path)
    )
    .unwrap();
}

fn pop_line(text: &mut String, heap: &mut Heap) {
    match heap.pop() {
        Some((item, path)) => write!(
            text,
            "\npop → {item}  {}{}",
            array(&heap.items),
            movement(&path)
        ),
        None => write!(text, "\npop → empty  []"),
    }
    .unwrap();
}

fn change_line(text: &mut String, heap: &mut Heap, ticket: usize, key: i32) {
    let (old, path) = heap.change(ticket, key);
    write!(
        text,
        "\nchange {} to {key} → {}{}",
        old,
        array(&heap.items),
        movement(&path)
    )
    .unwrap();
}

fn header(arity: usize, order: Order) -> String {
    let shape = match arity {
        2 => "Binary",
        3 => "Ternary",
        4 => "4-ary",
        _ => unreachable!(),
    };
    format!("{shape} {}-heap, FIFO ties", order.name())
}

fn items(keys: &[i32]) -> Vec<Item> {
    keys.iter()
        .enumerate()
        .map(|(ticket, &key)| Item { key, ticket })
        .collect()
}

fn random_keys(rng: &mut StdRng, count: usize) -> Vec<i32> {
    match rng.random_range(0..4) {
        0 => (0..count).map(|_| rng.random_range(-2..=2)).collect(),
        1 => {
            let mut keys: Vec<_> = (0..count as i32).map(|n| 2 * n - 7).collect();
            keys.shuffle(rng);
            keys
        }
        2 => (0..count)
            .map(|n| if n % 2 == 0 { n as i32 } else { -(n as i32) })
            .collect(),
        _ => (0..count).map(|_| rng.random_range(-30..=50)).collect(),
    }
}

fn compare_builds(rng: &mut StdRng, arity: usize, order: Order) -> String {
    let count = rng.random_range(7..=13);
    let input = items(&random_keys(rng, count));
    let (mut floyd, floyd_swaps) = Heap::floyd(input.clone(), arity, order);
    let (mut inserts, insert_swaps) = Heap::inserted(&input, arity, order);
    let mut text = header(arity, order);
    write!(
        text,
        "\nInput {}\nFloyd build → {}  ({floyd_swaps} swaps)\nRepeated insert → {}  ({insert_swaps} swaps)",
        array(&input),
        array(&floyd.items),
        array(&inserts.items)
    )
    .unwrap();
    for _ in 0..rng.random_range(3..=5) {
        let (a, _) = floyd.pop().unwrap();
        let (b, _) = inserts.pop().unwrap();
        debug_assert_eq!(a, b);
        write!(
            text,
            "\npop → {a}\n  Floyd {}\n  Insert {}",
            array(&floyd.items),
            array(&inserts.items)
        )
        .unwrap();
    }
    let item = Item {
        ticket: count,
        key: rng.random_range(-12..=12),
    };
    floyd.push(item);
    inserts.push(item);
    write!(
        text,
        "\ninsert {item}\n  Floyd {}\n  Insert {}",
        array(&floyd.items),
        array(&inserts.items)
    )
    .unwrap();
    text
}

pub fn generate(rng: &mut StdRng) -> String {
    let arity = rng.random_range(2..=4);
    let order = if rng.random_bool(0.5) {
        Order::Min
    } else {
        Order::Max
    };
    let regime = rng.random_range(0..6);
    if regime == 0 {
        return compare_builds(rng, arity, order);
    }
    let mut text = header(arity, order);
    if regime == 1 {
        let mut heap = Heap::empty(arity, order);
        text.push_str("\n[]");
        let count = rng.random_range(6..=10);
        let mut key = rng.random_range(-5..=20);
        for ticket in 0..count {
            key = order.best(key, rng.random_range(1..=7));
            insert_line(&mut text, &mut heap, Item { key, ticket });
        }
        pop_line(&mut text, &mut heap);
        pop_line(&mut text, &mut heap);
        return text;
    }
    let count = match arity {
        2 => [6, 7, 8, 14, 15, 16][rng.random_range(0..6)],
        3 => [3, 4, 5, 12, 13, 14][rng.random_range(0..6)],
        _ => [4, 5, 6, 8, 9, 10][rng.random_range(0..6)],
    };
    let keys = if regime == 2 {
        (0..count).map(|n| order.best(4, -(n as i32) * 3)).collect()
    } else if regime == 3 {
        (0..count).map(|_| rng.random_range(-1..=1)).collect()
    } else {
        random_keys(rng, count)
    };
    let input = items(&keys);
    let (mut heap, _) = Heap::floyd(input.clone(), arity, order);
    write!(
        text,
        "\nInput {}\nFloyd build → {}",
        array(&input),
        array(&heap.items)
    )
    .unwrap();
    if regime == 2 {
        for _ in 0..count.min(rng.random_range(5..=8)) {
            pop_line(&mut text, &mut heap);
        }
        insert_line(
            &mut text,
            &mut heap,
            Item {
                key: order.best(4, 1),
                ticket: count,
            },
        );
    } else if regime == 3 {
        pop_line(&mut text, &mut heap);
        let root_key = heap.items[0].key;
        insert_line(
            &mut text,
            &mut heap,
            Item {
                key: root_key,
                ticket: count,
            },
        );
        let ticket = heap.items.last().unwrap().ticket;
        change_line(&mut text, &mut heap, ticket, root_key);
        for _ in 0..count.min(6) {
            pop_line(&mut text, &mut heap);
        }
    } else if regime == 4 {
        let ticket = heap.items.last().unwrap().ticket;
        let extreme = order.best(heap.items[0].key, rng.random_range(4..=12));
        change_line(&mut text, &mut heap, ticket, extreme);
        let root = heap.items[0].ticket;
        let worst = heap
            .items
            .iter()
            .map(|item| item.key)
            .reduce(|a, b| match order {
                Order::Min => a.max(b),
                Order::Max => a.min(b),
            })
            .unwrap();
        change_line(&mut text, &mut heap, root, order.best(worst, -7));
        pop_line(&mut text, &mut heap);
        insert_line(
            &mut text,
            &mut heap,
            Item {
                key: extreme,
                ticket: count,
            },
        );
        pop_line(&mut text, &mut heap);
    } else {
        let mut ticket = count;
        for _ in 0..rng.random_range(7..=10) {
            match rng.random_range(0..3) {
                0 if !heap.items.is_empty() => pop_line(&mut text, &mut heap),
                1 if !heap.items.is_empty() => {
                    let selected = heap.items[rng.random_range(0..heap.items.len())].ticket;
                    change_line(&mut text, &mut heap, selected, rng.random_range(-25..=35));
                }
                _ => {
                    insert_line(
                        &mut text,
                        &mut heap,
                        Item {
                            key: rng.random_range(-25..=35),
                            ticket,
                        },
                    );
                    ticket += 1;
                }
            }
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeMap;

    fn invariant(heap: &Heap) {
        for child in 1..heap.items.len() {
            assert!(
                !heap
                    .order
                    .before(heap.items[child], heap.items[(child - 1) / heap.arity])
            );
        }
        let mut tickets: Vec<_> = heap.items.iter().map(|item| item.ticket).collect();
        tickets.sort_unstable();
        tickets.dedup();
        assert_eq!(tickets.len(), heap.items.len());
    }

    fn reference_pop(reference: &mut BTreeMap<usize, i32>, order: Order) -> Option<Item> {
        let item = reference
            .iter()
            .map(|(&ticket, &key)| Item { ticket, key })
            .reduce(|a, b| if order.before(a, b) { a } else { b });
        if let Some(item) = item {
            reference.remove(&item.ticket);
        }
        item
    }

    #[test]
    fn builds_and_drains_at_level_boundaries() {
        let mut rng = StdRng::seed_from_u64(0x0048_4541_5053);
        for arity in 2..=4 {
            for order in [Order::Min, Order::Max] {
                for count in 0..=66 {
                    let input = items(&random_keys(&mut rng, count));
                    let (mut floyd, _) = Heap::floyd(input.clone(), arity, order);
                    let (mut inserted, _) = Heap::inserted(&input, arity, order);
                    let mut reference: BTreeMap<_, _> =
                        input.iter().map(|i| (i.ticket, i.key)).collect();
                    loop {
                        invariant(&floyd);
                        invariant(&inserted);
                        let expected = reference_pop(&mut reference, order);
                        assert_eq!(floyd.pop().map(|pair| pair.0), expected);
                        assert_eq!(inserted.pop().map(|pair| pair.0), expected);
                        if expected.is_none() {
                            break;
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn changes_preserve_identity_and_fifo_ties() {
        for order in [Order::Min, Order::Max] {
            let input = items(&[8, 8, 8, 8, 8, 8, 8]);
            let (mut heap, _) = Heap::floyd(input, 2, order);
            heap.change(6, order.best(8, 10));
            assert_eq!(heap.items[0].ticket, 6);
            heap.change(6, 8);
            invariant(&heap);
            let drained: Vec<_> = std::iter::from_fn(|| heap.pop().map(|p| p.0.ticket)).collect();
            assert_eq!(drained, (0..7).collect::<Vec<_>>());
        }
    }

    #[test]
    fn random_mutations_match_priority_queue_reference() {
        for arity in 2..=4 {
            for order in [Order::Min, Order::Max] {
                let mut rng = StdRng::seed_from_u64(110 + arity as u64);
                let mut heap = Heap::empty(arity, order);
                let mut reference = BTreeMap::new();
                let mut ticket = 0;
                for _ in 0..1000 {
                    match rng.random_range(0..3) {
                        0 if !reference.is_empty() => {
                            let selected = heap.items[rng.random_range(0..heap.items.len())].ticket;
                            let key = rng.random_range(-10..=10);
                            heap.change(selected, key);
                            reference.insert(selected, key);
                        }
                        1 => {
                            assert_eq!(
                                heap.pop().map(|p| p.0),
                                reference_pop(&mut reference, order)
                            );
                        }
                        _ => {
                            let key = rng.random_range(-10..=10);
                            heap.push(Item { key, ticket });
                            reference.insert(ticket, key);
                            ticket += 1;
                        }
                    }
                    invariant(&heap);
                    assert_eq!(heap.items.len(), reference.len());
                    for item in &heap.items {
                        assert_eq!(reference.get(&item.ticket), Some(&item.key));
                    }
                }
            }
        }
    }

    #[test]
    fn root_replacement_reaches_bottom_and_both_build_methods_are_distinct() {
        let input = items(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let (mut heap, _) = Heap::floyd(input, 2, Order::Min);
        let (_, path) = heap.change(0, 100);
        assert_eq!(path.len(), 4);
        invariant(&heap);
        let input = items(&[9, 8, 7, 6, 5, 4, 3, 2, 1]);
        let (floyd, floyd_swaps) = Heap::floyd(input.clone(), 2, Order::Min);
        let (inserted, inserted_swaps) = Heap::inserted(&input, 2, Order::Min);
        assert_ne!(floyd.items, inserted.items);
        assert!(floyd_swaps < inserted_swaps);
    }
}
