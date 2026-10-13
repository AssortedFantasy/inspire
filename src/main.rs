mod cli;
mod tree;

use miniz_oxide::inflate::decompress_to_vec_zlib_with_limit;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::io::{self, Write};
use tree::{Node, Source};

include!(concat!(env!("OUT_DIR"), "/families.rs"));

fn render(node: &Node, rng: &mut StdRng) -> Result<String, String> {
    match node.source {
        Source::Branch { .. } => unreachable!("sample a leaf before rendering"),
        Source::Generator(generate) => Ok(generate(rng)),
        Source::Corpus {
            bytes,
            count,
            decoded_len,
        } => {
            let selected = rng.random_range(0..count) as usize;
            let decoded = decompress_to_vec_zlib_with_limit(bytes, decoded_len)
                .map_err(|e| format!("cannot decompress {}: {e:?}", node.path))?;
            let text = std::str::from_utf8(&decoded).map_err(|e| e.to_string())?;
            text.split('\0')
                .nth(selected)
                .map(str::to_owned)
                .ok_or_else(|| format!("invalid corpus index for {}", node.path))
        }
    }
}

fn run() -> Result<String, String> {
    let options = cli::parse(std::env::args().skip(1))?;
    match options.action {
        cli::Action::Help => return Ok(cli::HELP.into()),
        cli::Action::FullHelp => return Ok(cli::FULL_HELP.into()),
        cli::Action::Version => return Ok(format!("inspire {}", env!("CARGO_PKG_VERSION"))),
        cli::Action::List => return Ok(tree::listing(NODES)),
        cli::Action::Sample => (),
    }

    let start = match options.family.as_deref() {
        Some(path) => NODES
            .iter()
            .position(|node| node.path == path)
            .ok_or_else(|| format!("unknown family path {path:?}; use --list to see the tree"))?,
        None => 0,
    };
    let mut rng = match options.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::try_from_os_rng().map_err(|e| format!("cannot obtain randomness: {e}"))?,
    };
    render(tree::sample_leaf(NODES, start, &mut rng), &mut rng)
}

fn main() {
    match run() {
        Ok(output) => {
            if let Err(error) = writeln!(io::stdout().lock(), "{output}")
                && error.kind() != io::ErrorKind::BrokenPipe
            {
                eprintln!("inspire: {error}");
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("inspire: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_embedded_corpus_is_valid() {
        for node in NODES {
            if let Source::Corpus {
                bytes,
                count,
                decoded_len,
            } = node.source
            {
                let decoded = decompress_to_vec_zlib_with_limit(bytes, decoded_len).unwrap();
                assert_eq!(decoded.len(), decoded_len);
                let text = std::str::from_utf8(&decoded).unwrap();
                let records: Vec<_> = text.split('\0').collect();
                assert_eq!(records.len() as u64, count);
                assert!(records.iter().all(|record| !record.trim().is_empty()));
            }
        }
    }
}
