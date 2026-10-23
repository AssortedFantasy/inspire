use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Write;
use std::net::Ipv4Addr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Block {
    net: u32,
    prefix: u8,
}

impl Block {
    fn new(address: u32, prefix: u8) -> Self {
        Self {
            net: address & mask(prefix),
            prefix,
        }
    }

    fn size(self) -> u64 {
        1_u64 << (32 - self.prefix)
    }
    fn end(self) -> u64 {
        u64::from(self.net) + self.size() - 1
    }
    fn cidr(self) -> String {
        format!("{}/{}", ip(self.net), self.prefix)
    }
}

fn mask(prefix: u8) -> u32 {
    if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix)
    }
}

fn ip(address: u32) -> String {
    Ipv4Addr::from(address).to_string()
}

fn cover(mut first: u64, last: u64) -> Vec<Block> {
    let mut blocks = Vec::new();
    while first <= last {
        let alignment = if first == 0 {
            32
        } else {
            first.trailing_zeros().min(32)
        };
        let remaining = 63 - (last - first + 1).leading_zeros();
        let power = alignment.min(remaining);
        blocks.push(Block::new(first as u32, (32 - power) as u8));
        first += 1_u64 << power;
    }
    blocks
}

fn minimum_prefix(hosts: u32) -> u8 {
    let needed = u64::from(hosts) + 2;
    (32 - needed.next_power_of_two().trailing_zeros()) as u8
}

fn row(out: &mut String, block: Block) {
    let last = block.end() as u32;
    match block.prefix {
        32 => {
            writeln!(out, "{:<19} host {}", block.cidr(), ip(block.net)).unwrap();
        }
        31 => {
            writeln!(
                out,
                "{:<19} endpoints {} – {}",
                block.cidr(),
                ip(block.net),
                ip(last)
            )
            .unwrap();
        }
        _ => {
            writeln!(
                out,
                "{:<19} {:<15} {:<15} {}",
                block.cidr(),
                ip(block.net + 1),
                ip(last - 1),
                ip(last)
            )
            .unwrap();
        }
    }
}

fn uniform(rng: &mut StdRng, parent: Block) -> String {
    let extra = rng.random_range(1..=3).min(30 - parent.prefix);
    let prefix = parent.prefix + extra;
    let child_size = 1_u32 << (32 - prefix);
    let mut out = format!(
        "{} → {} equal /{} subnets\nMask {}\n\nsubnet              first host      last host       broadcast\n",
        parent.cidr(),
        1 << extra,
        prefix,
        ip(mask(prefix))
    );
    for i in 0..(1 << extra) {
        row(&mut out, Block::new(parent.net + i * child_size, prefix));
    }
    out.trim_end().to_owned()
}

fn vlsm(rng: &mut StdRng, parent: Block) -> String {
    let power = 32 - parent.prefix;
    let mut sizes: Vec<u8> = match rng.random_range(0..3) {
        0 => vec![power - 1, power - 2, power - 3],
        1 => vec![power - 2, power - 2, power - 3, power - 3],
        _ => vec![power - 2, power - 3, power - 3, power - 4],
    };
    sizes.shuffle(rng);
    let mut requests: Vec<_> = sizes
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            let capacity = (1_u32 << p) - 2;
            let hosts = if p == 2 || rng.random_bool(0.5) {
                capacity
            } else {
                (1_u32 << (p - 1)) - 1
            };
            ((b'A' + i as u8) as char, hosts, minimum_prefix(hosts))
        })
        .collect();
    let mut out = format!("{}\nLAN requests", parent.cidr());
    for &(name, hosts, _) in &requests {
        write!(out, "  {name}={hosts}").unwrap();
    }
    out.push_str("\n\nLAN  hosts  subnet              first host      last host       broadcast\n");
    requests.sort_by_key(|r| (r.2, r.0));
    let mut cursor = u64::from(parent.net);
    for (name, hosts, prefix) in requests {
        let block = Block::new(cursor as u32, prefix);
        write!(out, "{name}    {hosts:<5}  ").unwrap();
        row(&mut out, block);
        cursor += block.size();
    }
    if cursor <= parent.end() {
        let free = cover(cursor, parent.end())
            .iter()
            .map(|b| b.cidr())
            .collect::<Vec<_>>()
            .join(", ");
        write!(out, "\nFree  {free}").unwrap();
    }
    out.trim_end().to_owned()
}

fn links(rng: &mut StdRng, parent: Block) -> String {
    let count = rng.random_range(3..=6);
    let mut out = format!(
        "{}\n\nlink  /31                   endpoint A      endpoint B\n",
        parent.cidr()
    );
    for i in 0..count {
        let block = Block::new(parent.net + 2 * i, 31);
        writeln!(
            out,
            "{:<5} {:<21} {:<15} {}",
            i + 1,
            block.cidr(),
            ip(block.net),
            ip(block.net + 1)
        )
        .unwrap();
    }
    out.push_str("\nDirected broadcast  none");
    out
}

fn range_cover(rng: &mut StdRng, parent: Block) -> String {
    let half = parent.size() / 2;
    let start = u64::from(parent.net) + half - rng.random_range(1..=half / 2);
    let end = u64::from(parent.net) + half + rng.random_range(0..half / 2);
    let blocks = cover(start, end);
    let mut out = format!(
        "{} – {}\n{} addresses\n\nprefix              addresses\n",
        ip(start as u32),
        ip(end as u32),
        end - start + 1
    );
    for block in blocks {
        writeln!(out, "{:<19} {:>6}", block.cidr(), block.size()).unwrap();
    }
    out.trim_end().to_owned()
}

fn exclude(rng: &mut StdRng, parent: Block) -> String {
    let extra = rng.random_range(2..=4);
    let prefix = parent.prefix + extra;
    let size = 1_u64 << (32 - prefix);
    let index = rng.random_range(0..(1_u64 << extra));
    let removed = Block::new((u64::from(parent.net) + index * size) as u32, prefix);
    let mut remainder = Vec::new();
    if removed.net > parent.net {
        remainder.extend(cover(u64::from(parent.net), u64::from(removed.net) - 1));
    }
    if removed.end() < parent.end() {
        remainder.extend(cover(removed.end() + 1, parent.end()));
    }
    let mut out = format!(
        "{} minus {}\n\nRemaining prefixes\n",
        parent.cidr(),
        removed.cidr()
    );
    for block in remainder {
        writeln!(out, "{}", block.cidr()).unwrap();
    }
    write!(out, "\n{} addresses remain", parent.size() - removed.size()).unwrap();
    out
}

pub fn generate(rng: &mut StdRng) -> String {
    let prefix = rng.random_range(18..=26);
    let address =
        (10_u32 << 24) | (rng.random_range(0..256_u32) << 16) | rng.random_range(0..65536_u32);
    let parent = Block::new(address, prefix);
    match rng.random_range(0..5) {
        0 => uniform(rng, parent),
        1 => vlsm(rng, parent),
        2 => links(rng, parent),
        3 => range_cover(rng, parent),
        _ => exclude(rng, parent),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn masks_and_sizes_include_address_space_boundaries() {
        for p in 0..=32 {
            let b = Block::new(u32::MAX, p);
            assert_eq!(u64::from(b.net) + b.size(), 1_u64 << 32);
            assert_eq!(b.net.count_ones(), u32::from(p));
            assert_eq!(b.net & !mask(p), 0);
        }
        assert_eq!(cover(0, u64::from(u32::MAX)), vec![Block::new(0, 0)]);
    }

    #[test]
    fn greedy_covers_are_exact_aligned_and_cannot_merge() {
        for first in 0..128_u64 {
            for last in first..256 {
                let blocks = cover(first, last);
                let mut cursor = first;
                for b in &blocks {
                    assert_eq!(u64::from(b.net), cursor);
                    assert_eq!(cursor % b.size(), 0);
                    cursor += b.size();
                }
                assert_eq!(cursor, last + 1);
                for pair in blocks.windows(2) {
                    let a = pair[0];
                    let b = pair[1];
                    assert!(!(a.prefix == b.prefix && u64::from(a.net) % (2 * a.size()) == 0));
                }
                // Independent dynamic programming finds the minimum number of CIDR blocks.
                let mut best = vec![usize::MAX; (last - first + 2) as usize];
                best[0] = 0;
                for x in first..=last {
                    for power in 0..=8 {
                        let width = 1 << power;
                        if x % width == 0 && x + width <= last + 1 {
                            let to = (x + width - first) as usize;
                            best[to] = best[to].min(best[(x - first) as usize] + 1);
                        }
                    }
                }
                assert_eq!(blocks.len(), *best.last().unwrap());
            }
        }
    }

    #[test]
    fn lan_prefix_is_the_smallest_that_meets_request() {
        for hosts in 1..=65534 {
            let prefix = minimum_prefix(hosts);
            let capacity = (1_u64 << (32 - prefix)) - 2;
            assert!(capacity >= u64::from(hosts));
            assert!((1_u64 << (31 - prefix)) - 2 < u64::from(hosts));
        }
    }

    #[test]
    fn vlsm_constructions_stay_inside_parent() {
        let mut rng = StdRng::seed_from_u64(43);
        for p in 18..=26 {
            for _ in 0..128 {
                let parent = Block::new(0x0a31_abcd, p);
                let text = vlsm(&mut rng, parent);
                for line in text
                    .lines()
                    .filter(|l| l.starts_with(|c: char| c.is_ascii_uppercase()) && l.contains('/'))
                {
                    for token in line.split_whitespace() {
                        if let Some((address, prefix)) = token.trim_end_matches(',').split_once('/')
                        {
                            let net = u32::from(address.parse::<Ipv4Addr>().unwrap());
                            let block = Block::new(net, prefix.parse().unwrap());
                            assert!(block.net >= parent.net && block.end() <= parent.end());
                        }
                    }
                }
            }
        }
    }
}
