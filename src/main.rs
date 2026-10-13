mod cli;
mod dashboard;
mod layout;
mod payload;
mod tree;

use layout::{Chunk, Corpus, Name, Node, Record, Source};
use rand::{SeedableRng, rngs::StdRng};
use std::io::{self, Write};

include!(concat!(env!("OUT_DIR"), "/families.rs"));

static TREE: tree::Tree<'static> = tree::Tree {
    nodes: NODES,
    children: CHILDREN,
    names: NAMES,
    name_bytes: NAME_BYTES,
};

static PAYLOADS: payload::Payloads<'static> = payload::Payloads {
    corpuses: CORPUSES,
    records: RECORDS,
    chunks: CHUNKS,
    compressed_bytes: COMPRESSED_BYTES,
    generators: GENERATORS,
};

fn run() -> Result<String, String> {
    let options = cli::parse(std::env::args().skip(1))?;
    match options.action {
        cli::Action::Help => return Ok(cli::HELP.into()),
        cli::Action::FullHelp => return Ok(cli::FULL_HELP.into()),
        cli::Action::Version => return Ok(format!("inspire {}", env!("CARGO_PKG_VERSION"))),
        cli::Action::List => return Ok(TREE.listing()),
        cli::Action::Dashboard => {
            dashboard::serve(options.port.unwrap_or(8765))?;
            return Ok(String::new());
        }
        cli::Action::Sample => (),
    }

    let start = match options.family.as_deref() {
        Some(path) => TREE
            .resolve(path)
            .ok_or_else(|| format!("unknown family path {path:?}; use --list to see the tree"))?,
        None => 0,
    };
    let mut rng = match options.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::try_from_os_rng().map_err(|e| format!("cannot obtain randomness: {e}"))?,
    };
    let leaf = TREE.sample_leaf(start, &mut rng);
    PAYLOADS.render(NODES[leaf].source, &mut rng)
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
        for corpus in CORPUSES {
            assert!(corpus.record_count > 0);
            for index in corpus.first_record as usize
                ..corpus.first_record as usize + corpus.record_count as usize
            {
                let text = PAYLOADS.record_text(index).unwrap();
                assert!(!text.trim().is_empty());
                assert!(!text.contains('\0'));
            }
        }
    }
}
