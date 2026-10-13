use crate::layout;
use miniz_oxide::deflate::compress_to_vec_zlib;
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Settings {
    weight: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { weight: 1 }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    id: String,
    text: String,
    provenance: String,
}

pub struct Node {
    pub path: String,
    pub weight: u32,
    pub kind: Kind,
}

pub enum Kind {
    Branch(Vec<Node>),
    Corpus(Vec<Record>),
    Generator(PathBuf),
}

pub const CHUNK_SIZE: usize = 64 * 1024;

#[derive(Default)]
pub struct PackedTree {
    pub nodes: Vec<layout::Node>,
    pub children: Vec<u32>,
    pub names: Vec<layout::Name>,
    pub name_bytes: Vec<u8>,
    pub corpuses: Vec<layout::Corpus>,
    pub records: Vec<layout::Record>,
    pub chunks: Vec<layout::Chunk>,
    pub compressed_bytes: Vec<u8>,
    pub generators: Vec<PathBuf>,
}

fn narrow(value: usize, label: &str) -> Result<u32, String> {
    u32::try_from(value).map_err(|_| format!("{label} exceeds the packed layout's u32 limit"))
}

fn extended_len(current: usize, extra: usize, label: &str) -> Result<usize, String> {
    let len = current
        .checked_add(extra)
        .ok_or_else(|| format!("{label} overflows usize"))?;
    narrow(len, label)?;
    Ok(len)
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

pub fn discover(root: &Path) -> Result<Node, String> {
    if root.join("node.toml").exists() {
        return Err(
            "families/node.toml has no parent: place weights inside child directories".into(),
        );
    }
    let node = visit(root, String::new())?;
    if !matches!(node.kind, Kind::Branch(_)) {
        return Err("families/ must be a branch containing child directories".into());
    }
    Ok(node)
}

fn visit(directory: &Path, path: String) -> Result<Node, String> {
    let display = directory.display();
    let mut directories = Vec::new();
    for entry in fs::read_dir(directory).map_err(|e| format!("{display}: {e}"))? {
        let entry = entry.map_err(|e| format!("{display}: {e}"))?;
        let kind = entry.file_type().map_err(|e| format!("{display}: {e}"))?;
        if kind.is_symlink() {
            return Err(format!(
                "{}: symlinks are not supported in the family tree",
                entry.path().display()
            ));
        }
        if kind.is_dir() {
            directories.push(entry.path());
        }
    }
    directories.sort();
    let settings_path = directory.join("node.toml");
    let settings: Settings = if settings_path.exists() {
        toml::from_str(&read(&settings_path)?)
            .map_err(|e| format!("{}: {e}", settings_path.display()))?
    } else {
        Settings::default()
    };
    let records_path = directory.join("records.json");
    let generator_path = directory.join("generator.rs");
    let has_records = records_path.is_file();
    let has_generator = generator_path.is_file();
    if has_records && has_generator {
        return Err(format!(
            "{display}: cannot contain both records.json and generator.rs"
        ));
    }
    if (has_records || has_generator) && !directories.is_empty() {
        return Err(format!(
            "{display}: a payload leaf cannot also contain child directories; move the payload into a child"
        ));
    }
    let kind = if has_records {
        let records: Vec<Record> = serde_json::from_str(&read(&records_path)?)
            .map_err(|e| format!("{}: {e}", records_path.display()))?;
        if records.is_empty() {
            return Err(format!("{display}: corpus must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for record in &records {
            if record.id.trim().is_empty() || !ids.insert(&record.id) {
                return Err(format!(
                    "{display}: empty or duplicate record ID: {}",
                    record.id
                ));
            }
            if record.text.trim().is_empty() || record.text.contains('\0') {
                return Err(format!(
                    "{display}/{}: empty text or NUL character",
                    record.id
                ));
            }
            if record.provenance.trim().is_empty() {
                return Err(format!("{display}/{}: missing provenance", record.id));
            }
        }
        Kind::Corpus(records)
    } else if has_generator {
        Kind::Generator(generator_path)
    } else {
        if directories.is_empty() {
            return Err(format!(
                "{display}: empty node; add child directories, records.json, or generator.rs"
            ));
        }
        let mut children = Vec::new();
        for child in directories {
            let name = child
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| format!("{}: directory name must be UTF-8", child.display()))?;
            if !name
                .bytes()
                .next()
                .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                || !name
                    .bytes()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            {
                return Err(format!(
                    "{}: directory names must start with a lowercase ASCII letter or digit and contain only lowercase letters, digits, or hyphens",
                    child.display()
                ));
            }
            let child_path = if path.is_empty() {
                name.to_owned()
            } else {
                format!("{path}/{name}")
            };
            children.push(visit(&child, child_path)?);
        }
        if children.iter().all(|child| child.weight == 0) {
            return Err(format!(
                "{display}: every branch must have a positive-weight child, even when the branch itself is disabled"
            ));
        }
        Kind::Branch(children)
    };
    Ok(Node {
        path,
        weight: settings.weight,
        kind,
    })
}

pub fn pack(tree: &Node) -> Result<PackedTree, String> {
    let mut packer = Packer {
        tree: PackedTree::default(),
        decoded: Vec::new(),
    };
    packer.node(tree)?;
    packer.flush()?;
    Ok(packer.tree)
}

struct Packer {
    tree: PackedTree,
    decoded: Vec<u8>,
}

impl Packer {
    fn node(&mut self, node: &Node) -> Result<u32, String> {
        let index = narrow(self.tree.nodes.len(), "node index")?;
        extended_len(self.tree.nodes.len(), 1, "node count")?;
        self.tree.nodes.push(layout::Node {
            weight: node.weight,
            source: layout::Source::Generator(0),
        });
        let name = node.path.rsplit('/').next().unwrap();
        let offset = narrow(self.tree.name_bytes.len(), "name offset")?;
        let len = narrow(name.len(), "name length")?;
        extended_len(self.tree.name_bytes.len(), name.len(), "name bytes")?;
        self.tree.name_bytes.extend_from_slice(name.as_bytes());
        self.tree.names.push(layout::Name { offset, len });

        let source = match &node.kind {
            Kind::Branch(children) => {
                let first_child = narrow(self.tree.children.len(), "child offset")?;
                let child_count = narrow(children.len(), "child count")?;
                let end = extended_len(self.tree.children.len(), children.len(), "child indexes")?;
                // Reserve the parent's whole range before visiting nested branches.
                self.tree.children.resize(end, 0);
                let mut total_weight = 0_u64;
                for (position, child) in children.iter().enumerate() {
                    let child_index = self.node(child)?;
                    self.tree.children[first_child as usize + position] = child_index;
                    total_weight = total_weight
                        .checked_add(u64::from(child.weight))
                        .ok_or("sibling weights overflow u64")?;
                }
                layout::Source::Branch {
                    first_child,
                    child_count,
                    total_weight,
                }
            }
            Kind::Corpus(records) => {
                let index = narrow(self.tree.corpuses.len(), "corpus index")?;
                extended_len(self.tree.corpuses.len(), 1, "corpus count")?;
                let first_record = narrow(self.tree.records.len(), "record offset")?;
                let record_count = narrow(records.len(), "corpus record count")?;
                for record in records {
                    self.record(record.text.as_bytes())?;
                }
                self.tree.corpuses.push(layout::Corpus {
                    first_record,
                    record_count,
                });
                layout::Source::Corpus(index)
            }
            Kind::Generator(path) => {
                let index = narrow(self.tree.generators.len(), "generator index")?;
                extended_len(self.tree.generators.len(), 1, "generator count")?;
                self.tree.generators.push(path.clone());
                layout::Source::Generator(index)
            }
        };
        self.tree.nodes[index as usize].source = source;
        Ok(index)
    }

    fn record(&mut self, bytes: &[u8]) -> Result<(), String> {
        let len = narrow(bytes.len(), "record length")?;
        extended_len(self.tree.records.len(), 1, "record count")?;
        if bytes.len() > CHUNK_SIZE.saturating_sub(self.decoded.len()) {
            self.flush()?;
        }
        let chunk = narrow(self.tree.chunks.len(), "chunk index")?;
        let offset = narrow(self.decoded.len(), "record byte offset")?;
        self.tree
            .records
            .push(layout::Record { chunk, offset, len });
        self.decoded.extend_from_slice(bytes);
        if self.decoded.len() >= CHUNK_SIZE {
            self.flush()?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        if self.decoded.is_empty() {
            return Ok(());
        }
        extended_len(self.tree.chunks.len(), 1, "chunk count")?;
        let compressed = compress_to_vec_zlib(&self.decoded, 9);
        let offset = narrow(self.tree.compressed_bytes.len(), "chunk byte offset")?;
        let compressed_len = narrow(compressed.len(), "compressed chunk length")?;
        let decoded_len = narrow(self.decoded.len(), "decoded chunk length")?;
        extended_len(
            self.tree.compressed_bytes.len(),
            compressed.len(),
            "compressed bytes",
        )?;
        self.tree.chunks.push(layout::Chunk {
            offset,
            compressed_len,
            decoded_len,
        });
        self.tree.compressed_bytes.extend_from_slice(&compressed);
        self.decoded.clear();
        Ok(())
    }
}

pub fn emit(tree: &Node, out: &Path) -> Result<(), String> {
    let packed = pack(tree)?;
    let mut declarations = String::new();
    let mut generators = Vec::new();
    for (index, path) in packed.generators.iter().enumerate() {
        declarations.push_str(&format!(
            "#[path = {path:?}]\nmod generator_{index};\n",
            path = path.to_str().ok_or("generator path must be UTF-8")?
        ));
        generators.push(format!("generator_{index}::generate"));
    }
    let nodes = packed
        .nodes
        .iter()
        .map(|node| {
            format!(
                "Node {{ weight: {}, source: Source::{:?} }},",
                node.weight, node.source
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(out.join("chunks.bin"), &packed.compressed_bytes)
        .map_err(|e| format!("{}: {e}", out.display()))?;
    fs::write(out.join("families.rs"), format!(
        "{declarations}\nstatic NODES: &[Node] = &[\n{nodes}\n];\n\
         static CHILDREN: &[u32] = &{:?};\n\
         static NAMES: &[Name] = &{:?};\n\
         static NAME_BYTES: &[u8] = b{:?};\n\
         static CORPUSES: &[Corpus] = &{:?};\n\
         static RECORDS: &[Record] = &{:?};\n\
         static CHUNKS: &[Chunk] = &{:?};\n\
         static COMPRESSED_BYTES: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/chunks.bin\"));\n\
         static GENERATORS: &[payload::Generator] = &[{}];\n",
        packed.children, packed.names,
        std::str::from_utf8(&packed.name_bytes).map_err(|e| e.to_string())?,
        packed.corpuses, packed.records, packed.chunks, generators.join(", ")
    )).map_err(|e| format!("{}: {e}", out.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_lengths_reject_u32_overflow() {
        assert_eq!(narrow(u32::MAX as usize, "length").unwrap(), u32::MAX);
        assert!(extended_len(u32::MAX as usize, 1, "length").is_err());
        assert!(extended_len(usize::MAX, 1, "length").is_err());
    }
}
