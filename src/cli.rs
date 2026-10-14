#[derive(Debug, PartialEq)]
pub enum Action {
    Sample,
    Help,
    FullHelp,
    Version,
    List,
    DataLicenses,
    Dashboard,
}

#[derive(Debug)]
pub struct Options {
    pub action: Action,
    pub family: Option<String>,
    pub seed: Option<u64>,
    pub port: Option<u16>,
    pub json: bool,
}

pub const HELP: &str = "Generates a bit of inspiration";

pub const FULL_HELP: &str = "
Usage: inspire [--family PATH] [--seed INTEGER] [--json]
       inspire --list
       inspire --dashboard [--port INTEGER]
       inspire --data-licenses

Options:
  --family PATH   Sample a branch or leaf, e.g. games/chance/dice (use / on all platforms)
  --seed INTEGER  Reproduce a sample with an unsigned 64-bit seed
  --json          Emit sample text, selected leaf, and record index as JSON
  --list          Show the tree, local weights, and effective probabilities
  --data-licenses Show source attribution and bundled dataset licenses
  --dashboard     Serve the output dashboard locally (Ctrl+C to stop)
  --port INTEGER  Dashboard port (default 8765; 0 chooses an available port)
  -h, --help      Show a brief description
  --help-full     Show detailed help
  -V, --version   Show the version

Prints one stimulus to stdout.";

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut args = args.into_iter();
    let mut options = Options {
        action: Action::Sample,
        family: None,
        seed: None,
        port: None,
        json: false,
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--json" => {
                if options.json {
                    return Err("--json specified twice".into());
                }
                options.json = true;
            }
            "--family" => {
                if options.family.is_some() {
                    return Err("--family specified twice".into());
                }
                options.family = Some(
                    args.next()
                        .filter(|v| !v.starts_with('-') && !v.is_empty())
                        .ok_or("--family requires a path")?,
                );
            }
            "--seed" => {
                if options.seed.is_some() {
                    return Err("--seed specified twice".into());
                }
                options.seed = Some(
                    args.next()
                        .ok_or("--seed requires an unsigned 64-bit integer")?
                        .parse()
                        .map_err(|_| "--seed requires an unsigned 64-bit integer")?,
                );
            }
            "--port" => {
                if options.port.is_some() {
                    return Err("--port specified twice".into());
                }
                options.port = Some(
                    args.next()
                        .ok_or("--port requires an integer from 0 to 65535")?
                        .parse()
                        .map_err(|_| "--port requires an integer from 0 to 65535")?,
                );
            }
            "--dashboard" | "--list" | "--data-licenses" | "-h" | "--help" | "--help-full"
            | "-V" | "--version" => {
                if options.action != Action::Sample {
                    return Err("choose only one informational flag".into());
                }
                options.action = match arg.as_str() {
                    "--list" => Action::List,
                    "--data-licenses" => Action::DataLicenses,
                    "--dashboard" => Action::Dashboard,
                    "-h" | "--help" => Action::Help,
                    "--help-full" => Action::FullHelp,
                    _ => Action::Version,
                };
            }
            _ => return Err(format!("unknown argument {arg:?}")),
        }
    }
    if options.action != Action::Sample && (options.family.is_some() || options.seed.is_some()) {
        return Err("--family and --seed apply only when sampling".into());
    }
    if options.port.is_some() && options.action != Action::Dashboard {
        return Err("--port applies only with --dashboard".into());
    }
    if options.json && options.action != Action::Sample {
        return Err("--json applies only when sampling".into());
    }
    Ok(options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_arguments_are_errors() {
        for args in [
            vec!["--seed"],
            vec!["--seed", "-1"],
            vec!["--seed", "18446744073709551616"],
            vec!["--family"],
            vec!["--family", "--list"],
            vec!["--wat"],
            vec!["--list", "--seed", "1"],
            vec!["--port", "0"],
            vec!["--dashboard", "--seed", "1"],
            vec!["--dashboard", "--list"],
            vec!["--dashboard", "--port", "65536"],
            vec!["--dashboard", "--port"],
            vec!["--json", "--list"],
            vec!["--json", "--json"],
        ] {
            assert!(parse(args.into_iter().map(str::to_owned)).is_err());
        }
    }
}
