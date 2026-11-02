use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Entry {
    key: u32,
    value: i32,
}

#[derive(Clone, Copy)]
struct Hash {
    capacity: usize,
    a: u32,
    b: u32,
}

impl Hash {
    fn home(self, key: u32) -> usize {
        ((u64::from(self.a) * u64::from(key) + u64::from(self.b)) % self.capacity as u64) as usize
    }

    fn key_at(self, home: usize, multiple: u32) -> u32 {
        let residue = (0..self.capacity as u32)
            .find(|&key| self.home(key) == home)
            .unwrap();
        residue + multiple * self.capacity as u32
    }

    fn description(self) -> String {
        format!("h(k) = ({}k + {}) mod {}", self.a, self.b, self.capacity)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Search {
    value: Option<i32>,
    path: Vec<usize>,
}

struct Write {
    old: Option<i32>,
    slot: Option<usize>,
    path: Vec<usize>,
    steals: Vec<(u32, u32, usize)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    Empty,
    Deleted,
    Live(Entry),
}

#[derive(Clone, Copy)]
enum Probing {
    Linear,
    Quadratic,
}

struct OpenTable {
    hash: Hash,
    probing: Probing,
    slots: Vec<Slot>,
    len: usize,
}

impl OpenTable {
    fn new(hash: Hash, probing: Probing) -> Self {
        Self {
            hash,
            probing,
            slots: vec![Slot::Empty; hash.capacity],
            len: 0,
        }
    }

    fn orbit(&self, key: u32) -> Vec<usize> {
        // For an odd prime, these are all the distinct square offsets.
        let count = match self.probing {
            Probing::Linear => self.hash.capacity,
            Probing::Quadratic => self.hash.capacity.div_ceil(2),
        };
        (0..count)
            .map(|i| {
                let offset = match self.probing {
                    Probing::Linear => i,
                    Probing::Quadratic => i * i,
                };
                (self.hash.home(key) + offset) % self.hash.capacity
            })
            .collect()
    }

    fn get(&self, key: u32) -> Search {
        let mut path = Vec::new();
        for index in self.orbit(key) {
            path.push(index);
            match self.slots[index] {
                Slot::Live(entry) if entry.key == key => {
                    return Search {
                        value: Some(entry.value),
                        path,
                    };
                }
                Slot::Empty => break,
                _ => {}
            }
        }
        Search { value: None, path }
    }

    fn put(&mut self, key: u32, value: i32) -> Write {
        let mut path = Vec::new();
        let mut vacant = None;
        for index in self.orbit(key) {
            path.push(index);
            match self.slots[index] {
                Slot::Live(entry) if entry.key == key => {
                    self.slots[index] = Slot::Live(Entry { key, value });
                    return Write {
                        old: Some(entry.value),
                        slot: Some(index),
                        path,
                        steals: Vec::new(),
                    };
                }
                Slot::Deleted => {
                    vacant.get_or_insert(index);
                }
                Slot::Empty => {
                    vacant.get_or_insert(index);
                    break;
                }
                _ => {}
            }
        }
        if let Some(index) = vacant {
            self.slots[index] = Slot::Live(Entry { key, value });
            self.len += 1;
        }
        Write {
            old: None,
            slot: vacant,
            path,
            steals: Vec::new(),
        }
    }

    fn remove(&mut self, key: u32) -> Search {
        let found = self.get(key);
        if found.value.is_some() {
            self.slots[*found.path.last().unwrap()] = Slot::Deleted;
            self.len -= 1;
        }
        found
    }
}

struct RobinHood {
    hash: Hash,
    slots: Vec<Option<Entry>>,
    len: usize,
}

impl RobinHood {
    fn new(hash: Hash) -> Self {
        Self {
            hash,
            slots: vec![None; hash.capacity],
            len: 0,
        }
    }

    fn distance(&self, key: u32, index: usize) -> usize {
        (index + self.hash.capacity - self.hash.home(key)) % self.hash.capacity
    }

    fn get(&self, key: u32) -> Search {
        let mut path = Vec::new();
        for distance in 0..self.hash.capacity {
            let index = (self.hash.home(key) + distance) % self.hash.capacity;
            path.push(index);
            match self.slots[index] {
                Some(entry) if entry.key == key => {
                    return Search {
                        value: Some(entry.value),
                        path,
                    };
                }
                Some(entry) if self.distance(entry.key, index) >= distance => {}
                _ => break,
            }
        }
        Search { value: None, path }
    }

    fn put(&mut self, key: u32, value: i32) -> Write {
        if self.len == self.hash.capacity {
            let found = self.get(key);
            let slot = found.value.map(|_| *found.path.last().unwrap());
            if let Some(index) = slot {
                self.slots[index] = Some(Entry { key, value });
            }
            return Write {
                old: found.value,
                slot,
                path: found.path,
                steals: Vec::new(),
            };
        }
        let mut incoming = Entry { key, value };
        let mut distance = 0;
        let mut index = self.hash.home(key);
        let mut inserted_at = None;
        let mut path = Vec::new();
        let mut steals = Vec::new();
        for _ in 0..self.hash.capacity {
            path.push(index);
            match self.slots[index] {
                None => {
                    self.slots[index] = Some(incoming);
                    self.len += 1;
                    return Write {
                        old: None,
                        slot: Some(inserted_at.unwrap_or(index)),
                        path,
                        steals,
                    };
                }
                Some(entry) if entry.key == incoming.key => {
                    self.slots[index] = Some(incoming);
                    return Write {
                        old: Some(entry.value),
                        slot: Some(index),
                        path,
                        steals,
                    };
                }
                Some(entry) => {
                    let resident_distance = self.distance(entry.key, index);
                    if resident_distance < distance {
                        steals.push((incoming.key, entry.key, index));
                        inserted_at.get_or_insert(index);
                        self.slots[index] = Some(incoming);
                        incoming = entry;
                        distance = resident_distance;
                    }
                }
            }
            index = (index + 1) % self.hash.capacity;
            distance += 1;
        }
        unreachable!("a nonfull Robin Hood table has a vacant slot")
    }

    fn remove(&mut self, key: u32) -> (Search, Vec<(u32, usize, usize)>) {
        let found = self.get(key);
        let mut shifts = Vec::new();
        if found.value.is_some() {
            let mut hole = *found.path.last().unwrap();
            self.slots[hole] = None;
            self.len -= 1;
            for _ in 0..self.hash.capacity {
                let next = (hole + 1) % self.hash.capacity;
                let Some(entry) = self.slots[next] else {
                    break;
                };
                if self.distance(entry.key, next) == 0 {
                    break;
                }
                shifts.push((entry.key, next, hole));
                self.slots[hole] = Some(entry);
                self.slots[next] = None;
                hole = next;
            }
        }
        (found, shifts)
    }
}

struct Chains {
    hash: Hash,
    buckets: Vec<Vec<Entry>>,
    move_to_front: bool,
    len: usize,
}

impl Chains {
    fn new(hash: Hash, move_to_front: bool) -> Self {
        Self {
            hash,
            buckets: vec![Vec::new(); hash.capacity],
            move_to_front,
            len: 0,
        }
    }

    fn locate(&self, key: u32) -> (Option<usize>, Vec<u32>) {
        let mut visited = Vec::new();
        for (index, entry) in self.buckets[self.hash.home(key)].iter().enumerate() {
            visited.push(entry.key);
            if entry.key == key {
                return (Some(index), visited);
            }
        }
        (None, visited)
    }

    fn get(&mut self, key: u32) -> (Option<i32>, Vec<u32>, bool) {
        let (index, visited) = self.locate(key);
        let bucket = &mut self.buckets[self.hash.home(key)];
        let value = index.map(|index| bucket[index].value);
        let moved = self.move_to_front && index.is_some_and(|index| index > 0);
        if moved {
            let entry = bucket.remove(index.unwrap());
            bucket.insert(0, entry);
        }
        (value, visited, moved)
    }

    fn put(&mut self, key: u32, value: i32) -> (Option<i32>, Vec<u32>) {
        let (index, visited) = self.locate(key);
        let bucket = &mut self.buckets[self.hash.home(key)];
        let old = index.map(|index| bucket[index].value);
        if let Some(index) = index {
            bucket[index].value = value;
        } else {
            bucket.insert(0, Entry { key, value });
            self.len += 1;
        }
        (old, visited)
    }

    fn remove(&mut self, key: u32) -> (Option<i32>, Vec<u32>) {
        let (index, visited) = self.locate(key);
        let bucket = &mut self.buckets[self.hash.home(key)];
        let old = index.map(|index| bucket.remove(index).value);
        if old.is_some() {
            self.len -= 1;
        }
        (old, visited)
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Put(u32, i32),
    Get(u32),
    Remove(u32),
}

fn random_hash(rng: &mut StdRng, capacities: &[usize]) -> Hash {
    let capacity = capacities[rng.random_range(0..capacities.len())];
    Hash {
        capacity,
        a: rng.random_range(1..capacity as u32),
        b: rng.random_range(0..capacity as u32),
    }
}

fn put_operation(rng: &mut StdRng, key: u32) -> Operation {
    Operation::Put(key, rng.random_range(-25..=99))
}

fn mixed_tail(rng: &mut StdRng, keys: &[u32], capacity: usize) -> Vec<Operation> {
    (0..rng.random_range(0..=1))
        .map(|_| {
            let key = if rng.random_bool(0.75) {
                keys[rng.random_range(0..keys.len())]
            } else {
                rng.random_range(0..capacity as u32 * 16)
            };
            match rng.random_range(0..4) {
                0..=1 => put_operation(rng, key),
                2 => Operation::Get(key),
                _ => Operation::Remove(key),
            }
        })
        .collect()
}

fn value_text(value: Option<i32>) -> String {
    value.map_or_else(|| "absent".into(), |value| value.to_string())
}

fn path_text<T: std::fmt::Display>(path: &[T]) -> String {
    format!(
        "[{}]",
        path.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" → ")
    )
}

fn write_text(write: &Write, failure: &str) -> String {
    match (write.slot, write.old) {
        (Some(slot), Some(old)) => format!("replaces {old} at {slot}"),
        (Some(slot), None) => format!("new at {slot}"),
        (None, _) => failure.into(),
    }
}

fn open_artifact(rng: &mut StdRng, probing: Probing) -> String {
    let hash = random_hash(rng, &[5, 7]);
    let home = if rng.random_bool(0.65) {
        hash.capacity - rng.random_range(1..=2)
    } else {
        rng.random_range(0..hash.capacity)
    };
    let count = match probing {
        Probing::Linear => rng.random_range(3..hash.capacity),
        Probing::Quadratic => hash.capacity.div_ceil(2),
    };
    let first = rng.random_range(1..=6);
    let keys: Vec<_> = (0..count + 3)
        .map(|i| hash.key_at(home, first + i as u32))
        .collect();
    let mut operations: Vec<_> = keys[..count]
        .iter()
        .map(|&key| put_operation(rng, key))
        .collect();
    if matches!(probing, Probing::Quadratic) {
        operations.push(put_operation(rng, keys[count]));
    }
    operations.push(Operation::Remove(keys[rng.random_range(0..count - 1)]));
    operations.push(put_operation(rng, keys[count - 1]));
    operations.push(Operation::Get(keys[count + 2]));
    operations.push(put_operation(rng, keys[count + 1]));
    operations.extend(mixed_tail(rng, &keys, hash.capacity));

    let (name, formula, failure) = match probing {
        Probing::Linear => (
            "Linear probing with tombstones",
            format!(
                "p_i(k) = (h(k) + i) mod {}, i = 0..{}",
                hash.capacity,
                hash.capacity - 1
            ),
            "table full",
        ),
        Probing::Quadratic => (
            "Quadratic probing with tombstones",
            format!(
                "p_i(k) = (h(k) + i²) mod {}, i = 0..{}",
                hash.capacity,
                hash.capacity / 2
            ),
            "probe orbit full",
        ),
    };
    let mut lines = vec![name.into(), hash.description(), formula];
    let mut table = OpenTable::new(hash, probing);
    let initial_count = count;
    for (step, operation) in operations.into_iter().enumerate() {
        if step == initial_count {
            lines.push(format!(
                "Initial [{}]",
                table
                    .slots
                    .iter()
                    .map(|slot| match slot {
                        Slot::Empty => "_".into(),
                        Slot::Deleted => "×".into(),
                        Slot::Live(e) => format!("{}={}", e.key, e.value),
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        let text = match operation {
            Operation::Put(key, value) => {
                let write = table.put(key, value);
                format!(
                    "put({key}, {value}) → {}    probes {}",
                    write_text(&write, failure),
                    path_text(&write.path)
                )
            }
            Operation::Get(key) => {
                let found = table.get(key);
                format!(
                    "get({key}) → {}    probes {}",
                    value_text(found.value),
                    path_text(&found.path)
                )
            }
            Operation::Remove(key) => {
                let found = table.remove(key);
                format!(
                    "remove({key}) → {}    probes {}",
                    value_text(found.value),
                    path_text(&found.path)
                )
            }
        };
        if step >= initial_count {
            lines.push(format!("{}. {text}", step + 1 - initial_count));
        }
    }
    lines.push(format!("Slots ({} live)", table.len));
    for (index, slot) in table.slots.iter().enumerate() {
        let content = match slot {
            Slot::Empty => "empty".into(),
            Slot::Deleted => "deleted".into(),
            Slot::Live(entry) => format!(
                "{}={}    home {}",
                entry.key,
                entry.value,
                hash.home(entry.key)
            ),
        };
        lines.push(format!("{index:>2}  {content}"));
    }
    lines.join("\n")
}

fn robin_artifact(rng: &mut StdRng) -> String {
    let hash = random_hash(rng, &[5, 7]);
    let home = if rng.random_bool(0.7) {
        hash.capacity - 2
    } else {
        rng.random_range(0..hash.capacity)
    };
    let homes = [
        home,
        home,
        (home + 1) % hash.capacity,
        home,
        (home + 4) % hash.capacity,
        home,
    ];
    let first = rng.random_range(1..=6);
    let keys: Vec<_> = homes
        .iter()
        .enumerate()
        .map(|(i, &home)| hash.key_at(home, first + i as u32))
        .collect();
    let mut operations: Vec<_> = keys[..5]
        .iter()
        .map(|&key| put_operation(rng, key))
        .collect();
    operations.push(Operation::Get(keys[3]));
    operations.push(Operation::Remove(keys[1]));
    operations.push(put_operation(rng, keys[2]));
    operations.push(put_operation(rng, keys[5]));
    operations.extend(mixed_tail(rng, &keys, hash.capacity));
    let mut table = RobinHood::new(hash);
    let mut lines = vec![
        "Robin Hood insertion, backward-shift deletion".into(),
        hash.description(),
    ];
    let initial_count = 5;
    for (step, operation) in operations.into_iter().enumerate() {
        if step == initial_count {
            lines.push(format!(
                "Initial [{}]",
                table
                    .slots
                    .iter()
                    .map(|slot| slot
                        .map_or_else(|| "_".into(), |e| format!("{}={}", e.key, e.value)))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        let text = match operation {
            Operation::Put(key, value) => {
                let write = table.put(key, value);
                let mut text = format!(
                    "put({key}, {value}) → {}    probes {}",
                    write_text(&write, "table full"),
                    path_text(&write.path)
                );
                for (incoming, resident, slot) in write.steals {
                    text.push_str(&format!(
                        "\n   slot {slot}: {incoming} displaces {resident}"
                    ));
                }
                text
            }
            Operation::Get(key) => {
                let found = table.get(key);
                format!(
                    "get({key}) → {}    probes {}",
                    value_text(found.value),
                    path_text(&found.path)
                )
            }
            Operation::Remove(key) => {
                let (found, shifts) = table.remove(key);
                let mut text = format!(
                    "remove({key}) → {}    probes {}",
                    value_text(found.value),
                    path_text(&found.path)
                );
                for (shifted, from, to) in shifts {
                    text.push_str(&format!("\n   move {shifted}: {from} → {to}"));
                }
                text
            }
        };
        if step >= initial_count {
            lines.push(format!("{}. {text}", step + 1 - initial_count));
        }
    }
    lines.push(format!("Slots ({} live)", table.len));
    for (index, slot) in table.slots.iter().enumerate() {
        let content = slot.map_or_else(
            || "empty".into(),
            |entry| {
                format!(
                    "{}={}    home {}    displacement {}",
                    entry.key,
                    entry.value,
                    hash.home(entry.key),
                    table.distance(entry.key, index)
                )
            },
        );
        lines.push(format!("{index:>2}  {content}"));
    }
    lines.join("\n")
}

fn chain_artifact(rng: &mut StdRng) -> String {
    let hash = random_hash(rng, &[5, 7]);
    let home = rng.random_range(0..hash.capacity);
    let count = rng.random_range(3..=5);
    let first = rng.random_range(1..=5);
    let mut keys: Vec<_> = (0..count)
        .map(|i| hash.key_at(home, first + i as u32))
        .collect();
    for i in 1..=rng.random_range(1..=2) {
        keys.push(hash.key_at((home + i) % hash.capacity, first + count as u32 + i as u32));
    }
    let mut operations: Vec<_> = keys.iter().map(|&key| put_operation(rng, key)).collect();
    operations.push(Operation::Get(keys[0]));
    operations.push(put_operation(rng, keys[1]));
    operations.push(Operation::Remove(keys[count / 2]));
    operations.push(Operation::Get(keys[count - 1]));
    operations.extend(mixed_tail(rng, &keys, hash.capacity));
    let move_to_front = rng.random_bool(0.5);
    let mut table = Chains::new(hash, move_to_front);
    let policy = if move_to_front {
        "Successful get moves its entry to the front"
    } else {
        "Successful get preserves chain order"
    };
    let mut lines = vec![
        "Separate chaining, new entries at the head".into(),
        hash.description(),
        policy.into(),
    ];
    let initial_count = keys.len();
    for (step, operation) in operations.into_iter().enumerate() {
        if step == initial_count {
            lines.push(format!(
                "Initial [{}]",
                table
                    .buckets
                    .iter()
                    .map(|bucket| format!(
                        "[{}]",
                        bucket
                            .iter()
                            .map(|e| format!("{}={}", e.key, e.value))
                            .collect::<Vec<_>>()
                            .join(" ")
                    ))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }
        let text = match operation {
            Operation::Put(key, value) => {
                let (old, visited) = table.put(key, value);
                let action = old.map_or_else(|| "new head".into(), |old| format!("replaces {old}"));
                format!(
                    "put({key}, {value}) → {action}    bucket {}    compared {}",
                    hash.home(key),
                    path_text(&visited)
                )
            }
            Operation::Get(key) => {
                let (value, visited, moved) = table.get(key);
                format!(
                    "get({key}) → {}{}    bucket {}    compared {}",
                    value_text(value),
                    if moved { ", moved to head" } else { "" },
                    hash.home(key),
                    path_text(&visited)
                )
            }
            Operation::Remove(key) => {
                let (value, visited) = table.remove(key);
                format!(
                    "remove({key}) → {}    bucket {}    compared {}",
                    value_text(value),
                    hash.home(key),
                    path_text(&visited)
                )
            }
        };
        if step >= initial_count {
            lines.push(format!("{}. {text}", step + 1 - initial_count));
        }
    }
    lines.push(format!("Buckets ({} live)", table.len));
    for (index, bucket) in table.buckets.iter().enumerate() {
        let content = if bucket.is_empty() {
            "empty".into()
        } else {
            bucket
                .iter()
                .map(|entry| format!("{}={}", entry.key, entry.value))
                .collect::<Vec<_>>()
                .join(" → ")
        };
        lines.push(format!("{index:>2}  {content}"));
    }
    lines.join("\n")
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..4) {
        0 => open_artifact(rng, Probing::Linear),
        1 => open_artifact(rng, Probing::Quadratic),
        2 => robin_artifact(rng),
        _ => chain_artifact(rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeMap;

    fn hash(capacity: usize) -> Hash {
        Hash {
            capacity,
            a: 1,
            b: 0,
        }
    }

    fn check_open(table: &OpenTable, reference: &BTreeMap<u32, i32>) {
        let mut actual = BTreeMap::new();
        for (index, slot) in table.slots.iter().enumerate() {
            if let Slot::Live(entry) = slot {
                assert_eq!(actual.insert(entry.key, entry.value), None);
                let path = table.get(entry.key);
                assert_eq!(path.value, Some(entry.value));
                assert_eq!(path.path.last(), Some(&index));
                assert!(
                    path.path[..path.path.len() - 1]
                        .iter()
                        .all(|&i| table.slots[i] != Slot::Empty)
                );
            }
        }
        assert_eq!(&actual, reference);
        assert_eq!(table.len, reference.len());
    }

    fn check_robin(table: &RobinHood, reference: &BTreeMap<u32, i32>) {
        let mut actual = BTreeMap::new();
        for (index, entry) in table.slots.iter().enumerate() {
            if let Some(entry) = entry {
                assert_eq!(actual.insert(entry.key, entry.value), None);
                let found = table.get(entry.key);
                assert_eq!(found.value, Some(entry.value));
                assert_eq!(found.path.last(), Some(&index));
                assert_eq!(found.path.len() - 1, table.distance(entry.key, index));
            }
        }
        assert_eq!(&actual, reference);
        assert_eq!(table.len, reference.len());
        for key in 0..table.hash.capacity as u32 * 6 {
            assert_eq!(table.get(key).value, reference.get(&key).copied());
        }
    }

    fn check_chains(table: &Chains, reference: &BTreeMap<u32, i32>) {
        let mut actual = BTreeMap::new();
        for (index, bucket) in table.buckets.iter().enumerate() {
            for entry in bucket {
                assert_eq!(table.hash.home(entry.key), index);
                assert_eq!(actual.insert(entry.key, entry.value), None);
            }
        }
        assert_eq!(&actual, reference);
        assert_eq!(table.len, reference.len());
    }

    #[test]
    fn tombstones_do_not_hide_updates_or_break_wrapped_clusters() {
        let mut table = OpenTable::new(hash(7), Probing::Linear);
        for key in [6, 13, 20] {
            assert!(table.put(key, key as i32).slot.is_some());
        }
        assert_eq!(table.remove(13).value, Some(13));
        let updated = table.put(20, 91);
        assert_eq!(updated.old, Some(20));
        assert_eq!(updated.path, [6, 0, 1]);
        assert_eq!(table.len, 2);
        assert_eq!(table.put(27, 27).slot, Some(0));
        check_open(&table, &BTreeMap::from([(6, 6), (20, 91), (27, 27)]));
    }

    #[test]
    fn every_tombstone_and_full_table_paths_terminate() {
        let mut table = OpenTable::new(hash(7), Probing::Linear);
        for key in 0..7 {
            table.put(key, key as i32);
        }
        assert_eq!(table.put(14, 5).slot, None);
        assert_eq!(table.put(3, 30).old, Some(3));
        for key in 0..7 {
            assert!(table.remove(key).value.is_some());
        }
        assert_eq!(table.get(14).path.len(), 7);
        let inserted = table.put(14, 5);
        assert_eq!(inserted.path.len(), 7);
        assert_eq!(inserted.slot, Some(0));
        check_open(&table, &BTreeMap::from([(14, 5)]));
    }

    #[test]
    fn quadratic_orbit_can_be_full_while_other_slots_are_empty() {
        for capacity in [5, 7, 11, 13] {
            let mut table = OpenTable::new(hash(capacity), Probing::Quadratic);
            let count = capacity.div_ceil(2);
            let orbit = table.orbit(0);
            let mut distinct = orbit.clone();
            distinct.sort_unstable();
            distinct.dedup();
            assert_eq!(distinct.len(), count);
            for n in 0..count {
                assert!(table.put((n * capacity) as u32, n as i32).slot.is_some());
            }
            let extra = (count * capacity) as u32;
            assert_eq!(table.put(extra, 100).slot, None);
            assert!(table.slots.contains(&Slot::Empty));
            table.remove(0);
            let update = table.put(((count - 1) * capacity) as u32, 90);
            assert_eq!(update.old, Some((count - 1) as i32));
            assert_eq!(table.put(extra, 100).slot, Some(0));
            assert_eq!(table.get(extra).value, Some(100));
        }
    }

    #[test]
    fn robin_hood_steals_and_backshifts_across_zero() {
        let mut table = RobinHood::new(hash(7));
        for key in [5, 12, 6] {
            table.put(key, key as i32);
        }
        let inserted = table.put(19, 19);
        assert_eq!(inserted.steals, [(19, 6, 0)]);
        table.put(2, 2);
        let (_, shifts) = table.remove(12);
        assert_eq!(shifts, [(19, 0, 6), (6, 1, 0)]);
        assert_eq!(table.slots[2], Some(Entry { key: 2, value: 2 }));
        check_robin(&table, &BTreeMap::from([(2, 2), (5, 5), (6, 6), (19, 19)]));
    }

    #[test]
    fn full_robin_table_rejects_without_losing_displaced_entries() {
        let mut table = RobinHood::new(hash(7));
        let mut reference = BTreeMap::new();
        for key in [6, 13, 20, 27, 2, 9, 16] {
            table.put(key, key as i32);
            reference.insert(key, key as i32);
        }
        let before = table.slots.clone();
        assert_eq!(table.put(34, 34).slot, None);
        assert_eq!(table.slots, before);
        assert_eq!(table.put(20, 200).old, Some(20));
        reference.insert(20, 200);
        check_robin(&table, &reference);
        for key in [6, 13, 20, 27, 2, 9, 16] {
            assert_eq!(table.remove(key).0.value, reference.remove(&key));
            check_robin(&table, &reference);
        }
    }

    #[test]
    fn chained_head_middle_tail_removal_and_move_to_front() {
        let mut table = Chains::new(hash(5), true);
        for key in [1, 6, 11, 16] {
            table.put(key, key as i32);
        }
        let (value, visited, moved) = table.get(1);
        assert_eq!((value, visited, moved), (Some(1), vec![16, 11, 6, 1], true));
        assert_eq!(table.put(11, 110).0, Some(11));
        assert_eq!(table.len, 4);
        assert_eq!(table.remove(1).0, Some(1));
        assert_eq!(table.remove(11).0, Some(110));
        assert_eq!(table.remove(6).0, Some(6));
        assert_eq!(table.remove(21).0, None);
        check_chains(&table, &BTreeMap::from([(16, 16)]));
    }

    #[test]
    fn seeded_mixed_traces_match_reference_maps_after_every_operation() {
        for seed in 0..40 {
            let mut rng = StdRng::seed_from_u64(seed);
            let hash = random_hash(&mut rng, &[5, 7, 11]);
            let mut linear = OpenTable::new(hash, Probing::Linear);
            let mut quadratic = OpenTable::new(hash, Probing::Quadratic);
            let mut robin = RobinHood::new(hash);
            let mut chains = Chains::new(hash, seed % 2 == 0);
            let mut references: [BTreeMap<u32, i32>; 4] = std::array::from_fn(|_| BTreeMap::new());
            for _ in 0..220 {
                let key = rng.random_range(0..hash.capacity as u32 * 6);
                match rng.random_range(0..4) {
                    0..=1 => {
                        let value = rng.random_range(-100..=100);
                        for (table, reference) in [&mut linear, &mut quadratic]
                            .into_iter()
                            .zip(references.iter_mut())
                        {
                            let write = table.put(key, value);
                            if write.slot.is_some() {
                                assert_eq!(write.old, reference.insert(key, value));
                            } else {
                                assert!(!reference.contains_key(&key));
                                assert!(
                                    table
                                        .orbit(key)
                                        .iter()
                                        .all(|&index| matches!(table.slots[index], Slot::Live(_)))
                                );
                            }
                        }
                        let write = robin.put(key, value);
                        if write.slot.is_some() {
                            assert_eq!(write.old, references[2].insert(key, value));
                        } else {
                            assert_eq!(robin.len, hash.capacity);
                            assert!(!references[2].contains_key(&key));
                        }
                        assert_eq!(chains.put(key, value).0, references[3].insert(key, value));
                    }
                    2 => {
                        assert_eq!(linear.get(key).value, references[0].get(&key).copied());
                        assert_eq!(quadratic.get(key).value, references[1].get(&key).copied());
                        assert_eq!(robin.get(key).value, references[2].get(&key).copied());
                        assert_eq!(chains.get(key).0, references[3].get(&key).copied());
                    }
                    _ => {
                        assert_eq!(linear.remove(key).value, references[0].remove(&key));
                        assert_eq!(quadratic.remove(key).value, references[1].remove(&key));
                        let (found, shifts) = robin.remove(key);
                        assert_eq!(found.value, references[2].remove(&key));
                        for (shifted, from, to) in shifts {
                            assert_eq!(
                                robin.distance(shifted, from),
                                robin.distance(shifted, to) + 1
                            );
                        }
                        assert_eq!(chains.remove(key).0, references[3].remove(&key));
                    }
                }
                check_open(&linear, &references[0]);
                check_open(&quadratic, &references[1]);
                check_robin(&robin, &references[2]);
                check_chains(&chains, &references[3]);
            }
        }
    }
}
