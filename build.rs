mod build_support;
#[path = "src/layout.rs"]
mod layout;

use std::{env, fs, path::Path, path::PathBuf};

fn append_licenses(path: &Path, root: &Path, notices: &mut String) {
    let mut entries = fs::read_dir(path)
        .expect("read validated family directory")
        .map(|entry| entry.expect("read family entry").path())
        .collect::<Vec<_>>();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            append_licenses(&entry, root, notices);
        } else if matches!(
            entry.file_name().and_then(|name| name.to_str()),
            Some("LICENSE" | "LICENSE.txt" | "LICENSE.md")
        ) {
            notices.push_str("\n\n---\n");
            notices.push_str(
                &entry
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
            notices.push_str("\n\n");
            notices.push_str(&fs::read_to_string(entry).expect("dataset license must be UTF-8"));
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=families");
    println!("cargo:rerun-if-changed=build_support.rs");
    println!("cargo:rerun-if-changed=DATA_SOURCES.md");
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let tree =
        build_support::discover(&root.join("families")).unwrap_or_else(|error| panic!("{error}"));
    build_support::emit(&tree, &out).unwrap_or_else(|error| panic!("{error}"));
    let mut notices = fs::read_to_string(root.join("DATA_SOURCES.md")).unwrap();
    append_licenses(&root.join("families"), &root, &mut notices);
    fs::write(out.join("data-licenses.txt"), notices).unwrap();
}
