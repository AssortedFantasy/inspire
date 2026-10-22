use std::process::{Command, Output};

fn invoke(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_inspire"))
        // Prove that the executable does not need the repository as its CWD.
        .current_dir(std::env::temp_dir())
        .args(args)
        .output()
        .expect("run inspire")
}

#[test]
fn seeded_families_work_standalone_and_reproduce() {
    let list = invoke(&["--list"]);
    assert!(list.status.success());
    let list = String::from_utf8(list.stdout).unwrap();
    // Follow discovery so regrouping or reweighting source directories needs no
    // corresponding changes to a hard-coded list in this test.
    for line in list.lines() {
        let family = line.split_whitespace().next().unwrap();
        let args = ["--family", family, "--seed", "42"];
        let first = invoke(&args);
        let second = invoke(&args);
        assert!(first.status.success(), "{:?}", first.stderr);
        assert!(second.status.success());
        assert!(first.stderr.is_empty());
        assert_eq!(first.stdout, second.stdout);
        let text = String::from_utf8(first.stdout).unwrap();
        assert!(!text.trim().is_empty());
    }
}

#[test]
fn default_sampling_supports_seeded_and_os_random_runs() {
    let seeded = invoke(&["--seed", "0"]);
    assert!(seeded.status.success());
    assert!(!seeded.stdout.is_empty());
    assert_eq!(seeded.stdout, invoke(&["--seed", "0"]).stdout);
    let random = invoke(&[]);
    assert!(random.status.success(), "{:?}", random.stderr);
    assert!(!random.stdout.is_empty());
}

#[test]
fn json_sampling_matches_plain_cli_for_identical_seeds_and_scopes() {
    let listing = String::from_utf8(invoke(&["--list"]).stdout).unwrap();
    let mut scopes = vec![""];
    scopes.extend(listing.lines().filter_map(|line| {
        let path = line.split_whitespace().next().unwrap();
        (!path.contains('/')).then_some(path)
    }));
    // Include explicit leaves to check record indexes and generator metadata.
    for kind in ["[corpus]", "[generator]"] {
        scopes.push(
            listing
                .lines()
                .find(|line| line.contains(kind))
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap(),
        );
    }
    for scope in scopes {
        for seed in ["0", "42", "18446744073709551615"] {
            let mut args = vec!["--seed", seed];
            if !scope.is_empty() {
                args.extend(["--family", scope]);
            }
            let plain = invoke(&args);
            args.push("--json");
            let encoded = invoke(&args);
            assert!(plain.status.success(), "{:?}", plain.stderr);
            assert!(encoded.status.success(), "{:?}", encoded.stderr);
            let sample: serde_json::Value = serde_json::from_slice(&encoded.stdout).unwrap();
            assert_eq!(
                plain.stdout,
                format!("{}\n", sample["text"].as_str().unwrap()).as_bytes()
            );
            let path = sample["path"].as_str().unwrap();
            assert!(scope.is_empty() || path == scope || path.starts_with(&format!("{scope}/")));
            if sample["kind"] == "record" {
                let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("families")
                    .join(path)
                    .join("records.json");
                let records: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(source).unwrap()).unwrap();
                let index = sample["record_index"].as_u64().unwrap() as usize;
                assert_eq!(records[index]["text"], sample["text"]);
            } else {
                assert_eq!(sample["kind"], "generator");
                assert!(sample["record_index"].is_null());
            }
        }
    }
}

#[test]
fn listing_and_help_respect_the_cli_contract() {
    let list = invoke(&["--list"]);
    assert!(list.status.success());
    let list = String::from_utf8(list.stdout).unwrap();
    assert!(!list.trim().is_empty());
    for line in list.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        assert_eq!(fields.len(), 4);
        assert!(["[branch]", "[corpus]", "[generator]"].contains(&fields[1]));
        fields[2]
            .strip_prefix("weight=")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let probability = fields[3]
            .strip_prefix("effective=")
            .unwrap()
            .strip_suffix('%')
            .unwrap()
            .parse::<f64>()
            .unwrap();
        assert!((0.0..=100.0).contains(&probability));
    }
    for flag in ["--help", "-h"] {
        let help = invoke(&[flag]);
        assert!(help.status.success());
        assert!(help.stderr.is_empty());
        assert_eq!(help.stdout, b"Generates a bit of inspirational text.\n");
    }
    let full_help = invoke(&["--help-full"]);
    assert!(full_help.status.success());
    assert!(full_help.stderr.is_empty());
    assert!(
        String::from_utf8(full_help.stdout)
            .unwrap()
            .contains("Usage:")
    );
}

#[test]
fn dataset_notices_are_available_without_the_source_checkout() {
    let output = invoke(&["--data-licenses"]);
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("Dataset sources and redistribution"));
    assert!(text.contains("family READMEs"));

    fn check_licenses(directory: &std::path::Path, notices: &str) {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                check_licenses(&path, notices);
            } else if matches!(
                path.file_name().and_then(|name| name.to_str()),
                Some("LICENSE" | "LICENSE.txt" | "LICENSE.md")
            ) {
                let license = std::fs::read_to_string(&path).unwrap();
                assert!(notices.contains(license.trim_end()), "{}", path.display());
            }
        }
    }
    check_licenses(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("families"),
        &text,
    );
}

#[test]
fn invalid_input_uses_stderr_and_fails() {
    for args in [
        vec!["--family", "//missing"],
        vec!["--family", "invalid\\path"],
        vec!["--seed", "invalid"],
        vec!["--unknown"],
    ] {
        let output = invoke(&args);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}
