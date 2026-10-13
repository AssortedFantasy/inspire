#[path = "../build_support.rs"]
mod build_support;

use build_support::{Kind, discover, emit};
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
    assert!(generated.contains("poems/haiku/author"));
    assert!(generated.contains("Source::Branch"));
    assert!(generated.contains("Source::Corpus"));
    assert!(generated.contains("Source::Generator"));
    assert!(generated.contains("generator.rs"));
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
