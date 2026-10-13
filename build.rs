mod build_support;

use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=families");
    println!("cargo:rerun-if-changed=build_support.rs");
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let tree =
        build_support::discover(&root.join("families")).unwrap_or_else(|error| panic!("{error}"));
    build_support::emit(&tree, &out).unwrap_or_else(|error| panic!("{error}"));
}
