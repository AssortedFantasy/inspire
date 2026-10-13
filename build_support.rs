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

pub fn emit(tree: &Node, out: &Path) -> Result<(), String> {
    let mut nodes = Vec::new();
    let mut declarations = String::new();
    emit_node(tree, out, &mut nodes, &mut declarations)?;
    fs::write(
        out.join("families.rs"),
        format!(
            "{declarations}\nstatic NODES: &[Node] = &[\n{}\n];\n",
            nodes.join("\n")
        ),
    )
    .map_err(|e| format!("{}: {e}", out.display()))
}

fn emit_node(
    node: &Node,
    out: &Path,
    nodes: &mut Vec<String>,
    declarations: &mut String,
) -> Result<usize, String> {
    let index = nodes.len();
    nodes.push(String::new());
    let source = match &node.kind {
        Kind::Branch(children) => {
            let mut indexes = Vec::new();
            let mut total = 0_u64;
            for child in children {
                indexes.push(emit_node(child, out, nodes, declarations)?);
                total = total
                    .checked_add(u64::from(child.weight))
                    .ok_or("sibling weights overflow u64")?;
            }
            format!("Source::Branch {{ children: &{indexes:?}, total: {total} }}")
        }
        Kind::Corpus(records) => {
            let payload = records
                .iter()
                .map(|r| r.text.as_str())
                .collect::<Vec<_>>()
                .join("\0");
            let filename = format!("corpus_{index}.zlib");
            fs::write(
                out.join(&filename),
                compress_to_vec_zlib(payload.as_bytes(), 9),
            )
            .map_err(|e| e.to_string())?;
            format!(
                "Source::Corpus {{ bytes: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{filename}\")), count: {}, decoded_len: {} }}",
                records.len(),
                payload.len()
            )
        }
        Kind::Generator(path) => {
            declarations.push_str(&format!(
                "#[path = {path:?}]\nmod generator_{index};\n",
                path = path.to_str().ok_or("generator path must be UTF-8")?
            ));
            format!("Source::Generator(generator_{index}::generate)")
        }
    };
    nodes[index] = format!(
        "Node {{ path: {:?}, weight: {}, source: {source} }},",
        node.path, node.weight
    );
    Ok(index)
}
