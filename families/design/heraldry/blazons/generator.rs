use rand::{Rng, rngs::StdRng};

struct Tincture {
    name: &'static str,
    plain: &'static str,
}

const TINCTURES: [Tincture; 7] = [
    Tincture {
        name: "Or",
        plain: "gold",
    },
    Tincture {
        name: "argent",
        plain: "silver",
    },
    Tincture {
        name: "gules",
        plain: "red",
    },
    Tincture {
        name: "azure",
        plain: "blue",
    },
    Tincture {
        name: "sable",
        plain: "black",
    },
    Tincture {
        name: "vert",
        plain: "green",
    },
    Tincture {
        name: "purpure",
        plain: "purple",
    },
];

struct Charge {
    single: &'static str,
    plural: &'static str,
    plain: &'static str,
    plain_plural: &'static str,
}

const CHARGES: [Charge; 8] = [
    Charge {
        single: "mullet",
        plural: "mullets",
        plain: "five-pointed star",
        plain_plural: "five-pointed stars",
    },
    Charge {
        single: "annulet",
        plural: "annulets",
        plain: "ring",
        plain_plural: "rings",
    },
    Charge {
        single: "crescent",
        plural: "crescents",
        plain: "crescent with horns upward",
        plain_plural: "crescents with horns upward",
    },
    Charge {
        single: "escallop",
        plural: "escallops",
        plain: "scallop shell",
        plain_plural: "scallop shells",
    },
    Charge {
        single: "lozenge",
        plural: "lozenges",
        plain: "diamond",
        plain_plural: "diamonds",
    },
    Charge {
        single: "lion rampant",
        plural: "lions rampant",
        plain: "upright lion facing left",
        plain_plural: "upright lions facing left",
    },
    Charge {
        single: "eagle displayed",
        plural: "eagles displayed",
        plain: "eagle with spread wings and head turned left",
        plain_plural: "eagles with spread wings and heads turned left",
    },
    Charge {
        single: "fleur-de-lys",
        plural: "fleurs-de-lys",
        plain: "fleur-de-lys",
        plain_plural: "fleurs-de-lys",
    },
];

struct Ordinary {
    name: &'static str,
    plain: &'static str,
}

const ORDINARIES: [Ordinary; 9] = [
    Ordinary {
        name: "fess",
        plain: "horizontal band",
    },
    Ordinary {
        name: "pale",
        plain: "vertical band",
    },
    Ordinary {
        name: "bend",
        plain: "diagonal band from upper left to lower right",
    },
    Ordinary {
        name: "bend sinister",
        plain: "diagonal band from upper right to lower left",
    },
    Ordinary {
        name: "chevron",
        plain: "upward-pointing chevron",
    },
    Ordinary {
        name: "chief",
        plain: "band across the top",
    },
    Ordinary {
        name: "cross",
        plain: "cross reaching the shield edges",
    },
    Ordinary {
        name: "saltire",
        plain: "diagonal cross reaching the shield edges",
    },
    Ordinary {
        name: "bordure",
        plain: "border",
    },
];

enum Field {
    Solid(usize),
    Fur(usize),                         // ermine, counter-ermine, vair
    Divided(usize, usize, usize),       // per pale, per fess, quarterly
    Striped(bool, usize, usize, usize), // vertical, count, first, second
}

impl Field {
    fn text(&self) -> String {
        match *self {
            Self::Solid(t) => TINCTURES[t].name.to_owned(),
            Self::Fur(i) => ["ermine", "counter-ermine", "vair"][i].to_owned(),
            Self::Divided(i, a, b) => format!(
                "{} {} and {}",
                ["per pale", "per fess", "quarterly"][i],
                TINCTURES[a].name,
                TINCTURES[b].name
            ),
            Self::Striped(vertical, n, a, b) => format!(
                "{} of {n} {} and {}",
                if vertical { "paly" } else { "barry" },
                TINCTURES[a].name,
                TINCTURES[b].name
            ),
        }
    }

    fn plain(&self) -> String {
        match *self {
            Self::Solid(t) => format!("{} field", TINCTURES[t].plain),
            Self::Fur(0) => "silver field with black ermine spots".to_owned(),
            Self::Fur(1) => "black field with silver ermine spots".to_owned(),
            Self::Fur(_) => "field of alternating blue and silver vair bells".to_owned(),
            Self::Divided(i, a, b) => {
                let a = TINCTURES[a].plain;
                let b = TINCTURES[b].plain;
                match i {
                    0 => format!("field with a {a} left half and a {b} right half"),
                    1 => format!("field with a {a} upper half and a {b} lower half"),
                    _ => format!(
                        "quartered field, {a} at upper left and lower right, {b} in the other quarters"
                    ),
                }
            }
            Self::Striped(vertical, n, a, b) => format!(
                "field of {n} {} stripes, alternating {} and {} from the {}",
                if vertical { "vertical" } else { "horizontal" },
                TINCTURES[a].plain,
                TINCTURES[b].plain,
                if vertical { "left" } else { "top" }
            ),
        }
    }
}

enum Charges {
    None,
    Single(usize, usize),
    Three(usize, usize, bool), // charge, tincture, in fess rather than 2 and 1
    Ordinary(usize, usize, &'static str),
    OnOrdinary(usize, usize, usize, usize), // ordinary, tincture, charge, tertiary tincture
    Counterchanged(usize),
}

struct Arms {
    field: Field,
    charges: Charges,
}

fn metal(rng: &mut StdRng) -> usize {
    rng.random_range(0..2)
}
fn color(rng: &mut StdRng) -> usize {
    rng.random_range(2..7)
}
fn opposite(rng: &mut StdRng, t: usize) -> usize {
    if t < 2 { color(rng) } else { metal(rng) }
}

fn arms(rng: &mut StdRng) -> Arms {
    let regime = rng.random_range(0..7);
    let field = rng.random_range(0..7);
    match regime {
        0 => Arms {
            field: Field::Solid(field),
            charges: Charges::Ordinary(
                rng.random_range(0..ORDINARIES.len()),
                opposite(rng, field),
                ["", " wavy", " engrailed", " indented"][rng.random_range(0..4)],
            ),
        },
        1 => Arms {
            field: Field::Solid(field),
            charges: if rng.random_bool(0.5) {
                Charges::Single(rng.random_range(0..CHARGES.len()), opposite(rng, field))
            } else {
                Charges::Three(
                    rng.random_range(0..CHARGES.len()),
                    opposite(rng, field),
                    rng.random_bool(0.4),
                )
            },
        },
        2 => {
            let ordinary = opposite(rng, field);
            Arms {
                field: Field::Solid(field),
                charges: Charges::OnOrdinary(
                    rng.random_range(0..6),
                    ordinary,
                    rng.random_range(0..5),
                    opposite(rng, ordinary),
                ),
            }
        }
        3 => Arms {
            field: Field::Divided(rng.random_range(0..2), metal(rng), color(rng)),
            charges: Charges::Counterchanged(rng.random_range(0..CHARGES.len())),
        },
        4 => {
            let a = rng.random_range(0..7);
            let b = if a < 2 {
                1 - a
            } else {
                2 + (a - 2 + rng.random_range(1..5)) % 5
            };
            // Both field parts have one class, so the charge contrasts with each.
            Arms {
                field: Field::Divided(rng.random_range(0..3), a, b),
                charges: Charges::Single(rng.random_range(0..CHARGES.len()), opposite(rng, a)),
            }
        }
        5 => {
            let fur = rng.random_range(0..3);
            let charge = match fur {
                0 => color(rng),
                1 => metal(rng),
                _ => [0, 2, 4, 5, 6][rng.random_range(0..5)],
            };
            Arms {
                field: Field::Fur(fur),
                charges: Charges::Single(rng.random_range(0..CHARGES.len()), charge),
            }
        }
        _ => Arms {
            field: Field::Striped(
                rng.random_bool(0.5),
                2 * rng.random_range(3..=5),
                metal(rng),
                color(rng),
            ),
            charges: Charges::None,
        },
    }
}

fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
}

fn article(word: &str) -> &'static str {
    if matches!(word.as_bytes()[0], b'a' | b'e' | b'i' | b'o' | b'u') {
        "an"
    } else {
        "a"
    }
}

fn render(a: &Arms) -> String {
    let (blazon, plain) = match a.charges {
        Charges::None => (String::new(), String::new()),
        Charges::Single(c, t) => (
            format!(
                ", {} {} {}",
                article(CHARGES[c].single),
                CHARGES[c].single,
                TINCTURES[t].name
            ),
            format!(" bearing a {} {}", TINCTURES[t].plain, CHARGES[c].plain),
        ),
        Charges::Three(c, t, in_fess) => (
            format!(
                ", three {}{} {}",
                CHARGES[c].plural,
                if in_fess { " in fess" } else { "" },
                TINCTURES[t].name
            ),
            format!(
                " bearing three {} {}, {}",
                TINCTURES[t].plain,
                CHARGES[c].plain_plural,
                if in_fess {
                    "in a horizontal row"
                } else {
                    "two above and one below"
                }
            ),
        ),
        Charges::Ordinary(o, t, line) => (
            format!(", a {}{line} {}", ORDINARIES[o].name, TINCTURES[t].name),
            format!(
                " bearing a {} {}{}",
                TINCTURES[t].plain,
                ORDINARIES[o].plain,
                match line {
                    " wavy" => " with wavy edges",
                    " engrailed" => " with inward-scalloped edges",
                    " indented" => " with zigzag edges",
                    _ => "",
                }
            ),
        ),
        Charges::OnOrdinary(o, t, c, u) => (
            format!(
                ", on a {} {} three {} {}",
                ORDINARIES[o].name, TINCTURES[t].name, CHARGES[c].plural, TINCTURES[u].name
            ),
            format!(
                " bearing a {} {} charged with three {} {}",
                TINCTURES[t].plain,
                ORDINARIES[o].plain,
                TINCTURES[u].plain,
                CHARGES[c].plain_plural
            ),
        ),
        Charges::Counterchanged(c) => (
            format!(
                ", {} {} counterchanged",
                article(CHARGES[c].single),
                CHARGES[c].single
            ),
            format!(
                " bearing {} {}, its tinctures exchanged across the field division",
                article(CHARGES[c].plain),
                CHARGES[c].plain
            ),
        ),
    };
    format!(
        "{}.\n{}.",
        capitalize(a.field.text() + &blazon),
        capitalize(a.field.plain() + &plain)
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    render(&arms(rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn contrasts(t: usize, field: &Field) -> bool {
        match *field {
            Field::Solid(f) => (t < 2) != (f < 2),
            Field::Divided(_, a, b) => (t < 2) != (a < 2) && (t < 2) != (b < 2),
            Field::Fur(0) => t >= 2,
            Field::Fur(1) => t < 2,
            Field::Fur(_) => t != 1 && t != 3,
            Field::Striped(_, _, _, _) => false,
        }
    }

    #[test]
    fn overlays_contrast_at_every_supported_field_layer() {
        let mut outputs = HashSet::new();
        let mut regimes = HashSet::new();
        for seed in 0..2048 {
            let a = arms(&mut StdRng::seed_from_u64(seed));
            match a.charges {
                Charges::None => {
                    assert!(matches!(a.field, Field::Striped(_, _, _, _)));
                    regimes.insert(0);
                }
                Charges::Single(_, t) | Charges::Three(_, t, _) | Charges::Ordinary(_, t, _) => {
                    assert!(contrasts(t, &a.field));
                    regimes.insert(1);
                }
                Charges::OnOrdinary(_, t, _, u) => {
                    assert!(contrasts(t, &a.field));
                    assert!((t < 2) != (u < 2));
                    regimes.insert(2);
                }
                Charges::Counterchanged(_) => {
                    let Field::Divided(_, first, second) = a.field else {
                        panic!("divided field")
                    };
                    for (background, charge) in [(first, second), (second, first)] {
                        assert_ne!(background, charge);
                        assert!((background < 2) != (charge < 2));
                    }
                    regimes.insert(3);
                }
            }
            if let Field::Divided(_, a, b) | Field::Striped(_, _, a, b) = a.field {
                assert_ne!(a, b);
            }
            outputs.insert(render(&a));
        }
        assert_eq!(regimes.len(), 4);
        assert!(outputs.len() > 500);
    }

    #[test]
    fn fur_conventions_use_background_class_and_vair_constituents() {
        assert!(!contrasts(0, &Field::Fur(0)));
        assert!(contrasts(2, &Field::Fur(0)));
        assert!(!contrasts(2, &Field::Fur(1)));
        assert!(contrasts(1, &Field::Fur(1)));
        for t in 0..7 {
            assert_eq!(contrasts(t, &Field::Fur(2)), t != 1 && t != 3);
        }
    }
}
