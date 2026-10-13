#[path = "../build_support.rs"]
mod build_support;
#[path = "../src/layout.rs"]
mod layout;

use build_support::{CHUNK_SIZE, Kind, discover, emit, pack};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

struct Fixture {
    base: PathBuf,
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/tree-fixtures");
        fs::create_dir_all(&base).unwrap();
        let base = base.canonicalize().unwrap();
        let root = base.join(format!(
            "{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self { base, root }
    }

    fn write(&self, path: &str, contents: &str) {
        let path = self.root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
    }

    fn corpus(&self, path: &str) {
        self.write(
            &format!("{path}/records.json"),
            r#"[{"id":"one","text":"Example","provenance":"Test fixture"}]"#,
        );
    }

    fn error(&self) -> String {
        match discover(&self.root) {
            Ok(_) => panic!("expected invalid tree"),
            Err(error) => error,
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Only delete the freshly allocated fixture inside the workspace target directory.
        let path = self.root.canonicalize().unwrap();
        assert!(path.starts_with(&self.base) && path != self.base);
        fs::remove_dir_all(path).unwrap();
    }
}

fn branches(node: &build_support::Node) -> &[build_support::Node] {
    match &node.kind {
        Kind::Branch(children) => children,
        _ => panic!("expected branch"),
    }
}

#[test]
fn discovers_nested_defaults_and_generates_registration() {
    let fixture = Fixture::new();
    fixture.corpus("poems/haiku/author");
    fixture.write(
        "math/algebra/generator.rs",
        "pub fn generate(_: &mut rand::rngs::StdRng) -> String { String::from(\"2 + 2 = 4\") }",
    );
    let tree = discover(&fixture.root).unwrap();
    let children = branches(&tree);
    assert_eq!(
        children.iter().map(|n| n.path.as_str()).collect::<Vec<_>>(),
        ["math", "poems"]
    );
    assert!(children.iter().all(|n| n.weight == 1));
    assert_eq!(
        branches(&branches(&children[1])[0])[0].path,
        "poems/haiku/author"
    );
    let out = fixture.root.join("compiled");
    fs::create_dir(&out).unwrap();
    emit(&tree, &out).unwrap();
    let generated = fs::read_to_string(out.join("families.rs")).unwrap();
    assert!(generated.contains("b\"mathalgebrapoemshaikuauthor\""));
    assert!(generated.contains("Source::Branch"));
    assert!(generated.contains("Source::Corpus"));
    assert!(generated.contains("Source::Generator"));
    assert!(generated.contains("generator.rs"));
    assert!(generated.contains("static CHILDREN:"));
    assert!(generated.contains("static RECORDS:"));
    assert!(generated.contains("static CHUNKS:"));
    let packed = pack(&tree).unwrap();
    assert_eq!(
        fs::read(out.join("chunks.bin")).unwrap(),
        packed.compressed_bytes
    );
    assert_eq!(packed.children, [1, 3, 2, 4, 5]);
    assert_eq!(
        packed.nodes[0].source,
        layout::Source::Branch {
            first_child: 0,
            child_count: 2,
            total_weight: 2,
        }
    );
    assert_eq!(packed.nodes[2].source, layout::Source::Generator(0));
    assert_eq!(packed.nodes[5].source, layout::Source::Corpus(0));
    let names = packed
        .names
        .iter()
        .map(|name| {
            std::str::from_utf8(
                &packed.name_bytes[name.offset as usize..name.offset as usize + name.len as usize],
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(names, ["", "math", "algebra", "poems", "haiku", "author"]);
}

fn write_records(fixture: &Fixture, path: &str, texts: &[&str]) {
    let records = texts.iter().enumerate().map(|(index, text)| {
        serde_json::json!({"id": index.to_string(), "text": text, "provenance": "Test fixture"})
    }).collect::<Vec<_>>();
    fixture.write(
        &format!("{path}/records.json"),
        &serde_json::to_string(&records).unwrap(),
    );
}

fn assert_packed_texts(packed: &build_support::PackedTree, texts: &[&str]) {
    use miniz_oxide::inflate::decompress_to_vec_zlib_with_limit;
    let mut next_offset = 0;
    let decoded = packed
        .chunks
        .iter()
        .map(|chunk| {
            assert_eq!(chunk.offset as usize, next_offset);
            next_offset += chunk.compressed_len as usize;
            let bytes = &packed.compressed_bytes[chunk.offset as usize..next_offset];
            let decoded =
                decompress_to_vec_zlib_with_limit(bytes, chunk.decoded_len as usize).unwrap();
            assert_eq!(decoded.len(), chunk.decoded_len as usize);
            decoded
        })
        .collect::<Vec<_>>();
    assert_eq!(next_offset, packed.compressed_bytes.len());
    assert_eq!(packed.records.len(), texts.len());
    for (record, text) in packed.records.iter().zip(texts) {
        assert_eq!(
            &decoded[record.chunk as usize]
                [record.offset as usize..record.offset as usize + record.len as usize],
            text.as_bytes()
        );
    }
}

#[test]
fn small_corpuses_share_chunks_and_keep_distinct_record_ranges() {
    let fixture = Fixture::new();
    write_records(&fixture, "a", &["前\n", "hello"]);
    write_records(&fixture, "b", &["世界"]);
    let packed = pack(&discover(&fixture.root).unwrap()).unwrap();
    assert_eq!(packed.chunks.len(), 1);
    assert_eq!(
        packed.corpuses,
        [
            layout::Corpus {
                first_record: 0,
                record_count: 2
            },
            layout::Corpus {
                first_record: 2,
                record_count: 1
            },
        ]
    );
    assert_eq!(packed.records[2].offset, 9);
    assert_packed_texts(&packed, &["前\n", "hello", "世界"]);
}

#[test]
fn chunks_respect_byte_limits_and_isolate_oversized_records() {
    let fixture = Fixture::new();
    let first = "x".repeat(CHUNK_SIZE - 1);
    let fill = "y".repeat(CHUNK_SIZE - 2);
    let oversized = "z".repeat(CHUNK_SIZE + 1);
    let texts = [
        first.as_str(),
        "é",
        fill.as_str(),
        oversized.as_str(),
        "tail",
    ];
    write_records(&fixture, "a", &texts);
    let packed = pack(&discover(&fixture.root).unwrap()).unwrap();
    assert_eq!(
        packed
            .chunks
            .iter()
            .map(|chunk| chunk.decoded_len as usize)
            .collect::<Vec<_>>(),
        [CHUNK_SIZE - 1, CHUNK_SIZE, CHUNK_SIZE + 1, 4]
    );
    assert_eq!(
        packed
            .records
            .iter()
            .map(|record| record.chunk)
            .collect::<Vec<_>>(),
        [0, 1, 1, 2, 3]
    );
    assert_eq!(packed.records[2].offset, 2);
    assert_packed_texts(&packed, &texts);
}

#[test]
fn generator_only_trees_have_no_payload_chunks() {
    let fixture = Fixture::new();
    fixture.write(
        "a/generator.rs",
        "pub fn generate(_: &mut rand::rngs::StdRng) -> String { \"x\".into() }",
    );
    let packed = pack(&discover(&fixture.root).unwrap()).unwrap();
    assert!(packed.corpuses.is_empty());
    assert!(packed.records.is_empty());
    assert!(packed.chunks.is_empty());
    assert!(packed.compressed_bytes.is_empty());
    assert_eq!(packed.generators.len(), 1);
}

#[test]
fn packed_sibling_totals_use_u64() {
    let fixture = Fixture::new();
    for path in ["a", "b"] {
        fixture.corpus(path);
        fixture.write(&format!("{path}/node.toml"), "weight = 4294967295");
    }
    let packed = pack(&discover(&fixture.root).unwrap()).unwrap();
    let layout::Source::Branch { total_weight, .. } = packed.nodes[0].source else {
        panic!("expected branch");
    };
    assert_eq!(total_weight, 2 * u64::from(u32::MAX));
}

#[test]
fn moving_a_subtree_preserves_local_weights_without_metadata_edits() {
    let fixture = Fixture::new();
    fixture.corpus("poems");
    fixture.corpus("algebra");
    fixture.write("algebra/node.toml", "weight = 3\n");
    let before = discover(&fixture.root).unwrap();
    assert_eq!(branches(&before)[0].path, "algebra");
    assert_eq!(branches(&before)[0].weight, 3);
    let old_path = fixture.root.join("algebra").canonicalize().unwrap();
    let parent = fixture.root.join("math");
    fs::create_dir(&parent).unwrap();
    let new_path = parent.canonicalize().unwrap().join("algebra");
    assert!(old_path.starts_with(&fixture.root) && new_path.starts_with(&fixture.root));
    fs::rename(old_path, new_path).unwrap();
    let after = discover(&fixture.root).unwrap();
    let math = &branches(&after)[0];
    assert_eq!(math.path, "math");
    assert_eq!(math.weight, 1);
    assert_eq!(branches(math)[0].path, "math/algebra");
    assert_eq!(branches(math)[0].weight, 3);
}

#[test]
fn rejects_ambiguous_and_empty_nodes() {
    let both = Fixture::new();
    both.corpus("leaf");
    both.write("leaf/generator.rs", "");
    assert!(both.error().contains("both records.json and generator.rs"));

    let mixed = Fixture::new();
    mixed.corpus("parent");
    mixed.corpus("parent/child");
    assert!(
        mixed
            .error()
            .contains("payload leaf cannot also contain child directories")
    );

    let empty = Fixture::new();
    empty.write("empty/README.md", "Documentation is not a payload.");
    assert!(empty.error().contains("empty node"));
}

#[test]
fn validates_weights_and_requires_selectable_children_even_under_disabled_nodes() {
    for invalid in ["weight = -1", "weight = 4294967296", "weigth = 2"] {
        let fixture = Fixture::new();
        fixture.corpus("leaf");
        fixture.write("leaf/node.toml", invalid);
        assert!(fixture.error().contains("node.toml"));
    }
    let fixture = Fixture::new();
    fixture.corpus("enabled");
    fixture.corpus("disabled/leaf");
    fixture.write("disabled/node.toml", "weight = 0");
    assert!(discover(&fixture.root).is_ok());
    fixture.write("disabled/leaf/node.toml", "weight = 0");
    assert!(fixture.error().contains("positive-weight child"));
}

#[test]
fn rejects_invalid_records_and_root_configuration() {
    let fixture = Fixture::new();
    fixture.corpus("leaf");
    fixture.write("node.toml", "weight = 2");
    assert!(fixture.error().contains("has no parent"));
    fs::remove_file(fixture.root.join("node.toml")).unwrap();
    for (records, message) in [
        (r#"[]"#, "corpus must not be empty"),
        (
            r#"[{"id":"one","text":"x","provenance":"p"},{"id":"one","text":"y","provenance":"p"}]"#,
            "duplicate record ID",
        ),
        (
            r#"[{"id":"one","text":"\u0000","provenance":"p"}]"#,
            "NUL character",
        ),
        (
            r#"[{"id":"one","text":"x","provenance":""}]"#,
            "missing provenance",
        ),
    ] {
        fixture.write("leaf/records.json", records);
        assert!(fixture.error().contains(message));
    }
}
