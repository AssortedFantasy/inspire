#[derive(Debug, PartialEq)]
pub enum Action {
    Sample,
    Help,
    FullHelp,
    Version,
    List,
}

#[derive(Debug)]
pub struct Options {
    pub action: Action,
    pub family: Option<String>,
    pub seed: Option<u64>,
}

pub const HELP: &str = "Generates a bit of inspiration";

pub const FULL_HELP: &str = "
Usage: inspire [--family PATH] [--seed INTEGER]
       inspire --list

Options:
  --family PATH   Sample a branch or leaf, e.g. poems/haiku (use / on all platforms)
  --seed INTEGER  Reproduce a sample with an unsigned 64-bit seed
  --list          Show the tree, local weights, and effective probabilities
  -h, --help      Show a brief description
  --help-full     Show detailed help
  -V, --version   Show the version

Prints one stimulus to stdout. No network access or persistent history.
Seeds reproduce results for the same build and arguments.";

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut args = args.into_iter();
    let mut options = Options {
        action: Action::Sample,
        family: None,
        seed: None,
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
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
            "--list" | "-h" | "--help" | "--help-full" | "-V" | "--version" => {
                if options.action != Action::Sample {
                    return Err("choose only one informational flag".into());
                }
                options.action = match arg.as_str() {
                    "--list" => Action::List,
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
        ] {
            assert!(parse(args.into_iter().map(str::to_owned)).is_err());
        }
    }
}
