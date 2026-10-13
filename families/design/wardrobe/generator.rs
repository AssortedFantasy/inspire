use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Weather {
    Warm,
    Mild,
    Rain,
    Cold,
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Occasion {
    Errands,
    Office,
    Dinner,
    Trail,
}

struct Palette {
    light: &'static str,
    dark: &'static str,
    accent: &'static str,
    leather: &'static str,
}
const PALETTES: [Palette; 5] = [
    Palette {
        light: "ivory",
        dark: "navy",
        accent: "rust",
        leather: "dark brown",
    },
    Palette {
        light: "stone",
        dark: "charcoal",
        accent: "forest green",
        leather: "black",
    },
    Palette {
        light: "cream",
        dark: "olive",
        accent: "terracotta",
        leather: "dark brown",
    },
    Palette {
        light: "pale blue",
        dark: "navy",
        accent: "burgundy",
        leather: "dark brown",
    },
    Palette {
        light: "light gray",
        dark: "black",
        accent: "cobalt",
        leather: "black",
    },
];

struct Outfit {
    weather: Weather,
    occasion: Occasion,
    temperature: i32,
    palette: usize,
    top: &'static str,
    bottom: &'static str,
    mid: Option<&'static str>,
    outer: Option<&'static str>,
    shoes: &'static str,
    accessory: &'static str,
}

fn choose<'a>(rng: &mut StdRng, options: &[&'a str]) -> &'a str {
    options[rng.random_range(0..options.len())]
}

fn outfit(rng: &mut StdRng, weather: Weather, occasion: Occasion) -> Outfit {
    let warm = weather == Weather::Warm;
    let (top, bottom, shoes) = match occasion {
        Occasion::Errands => (
            choose(
                rng,
                if warm {
                    &["cotton T-shirt", "linen short-sleeve shirt"]
                } else {
                    &["cotton long-sleeve T-shirt", "cotton oxford shirt"]
                },
            ),
            choose(
                rng,
                if warm {
                    &["cotton shorts", "lightweight chinos"]
                } else {
                    &["straight-leg jeans", "cotton chinos"]
                },
            ),
            if matches!(weather, Weather::Rain | Weather::Cold) {
                "water-resistant walking boots"
            } else {
                choose(rng, &["canvas sneakers", "leather sneakers"])
            },
        ),
        Occasion::Office => (
            choose(
                rng,
                if warm {
                    &["linen-cotton collared shirt", "short-sleeve knit polo"]
                } else {
                    &["cotton poplin shirt", "cotton oxford shirt"]
                },
            ),
            choose(
                rng,
                if warm {
                    &["lightweight tailored trousers", "linen-cotton trousers"]
                } else {
                    &["wool-blend trousers", "tailored cotton trousers"]
                },
            ),
            if matches!(weather, Weather::Rain | Weather::Cold) {
                "weather-resistant leather ankle boots"
            } else {
                choose(rng, &["leather loafers", "leather derby shoes"])
            },
        ),
        Occasion::Dinner => (
            choose(
                rng,
                if warm {
                    &["linen collared shirt", "fine-gauge cotton polo"]
                } else {
                    &["fine-gauge merino turtleneck", "cotton poplin shirt"]
                },
            ),
            choose(rng, &["tailored trousers", "pleated trousers"]),
            if matches!(weather, Weather::Rain | Weather::Cold) {
                "weather-resistant leather Chelsea boots"
            } else {
                choose(rng, &["leather loafers", "leather derby shoes"])
            },
        ),
        Occasion::Trail => (
            if warm {
                "short-sleeve moisture-wicking base layer"
            } else {
                "long-sleeve merino base layer"
            },
            if warm {
                choose(
                    rng,
                    &["quick-drying hiking shorts", "lightweight hiking trousers"],
                )
            } else {
                "quick-drying hiking trousers"
            },
            if matches!(weather, Weather::Rain | Weather::Cold) {
                "waterproof hiking boots"
            } else {
                "grippy trail shoes"
            },
        ),
    };
    let mid = match (weather, occasion) {
        (Weather::Cold, Occasion::Trail) => Some("insulating fleece"),
        (Weather::Cold, _) => Some("wool cardigan"),
        (Weather::Mild | Weather::Rain, Occasion::Trail) => Some("lightweight fleece"),
        (Weather::Mild | Weather::Rain, Occasion::Office | Occasion::Dinner) => {
            Some("unstructured blazer")
        }
        (Weather::Mild, Occasion::Errands) => Some("cotton overshirt"),
        _ => None,
    };
    let outer = match (weather, occasion) {
        (Weather::Rain, Occasion::Trail) => Some("hooded waterproof shell"),
        (Weather::Rain, _) => Some("hooded waterproof raincoat"),
        (Weather::Cold, Occasion::Trail) => Some("insulated wind-resistant hiking jacket"),
        (Weather::Cold, _) => Some("insulated coat"),
        _ => None,
    };
    let accessory = match (weather, occasion) {
        (Weather::Cold, Occasion::Trail) => "warm beanie, gloves, and wool hiking socks",
        (Weather::Cold, _) => "wool scarf, gloves, and warm socks",
        (Weather::Rain, Occasion::Trail) => "wool hiking socks and a waterproof pack cover",
        (Weather::Rain, _) => "compact umbrella and a water-resistant bag",
        (Weather::Warm, Occasion::Trail) => "sun hat and lightweight hiking socks",
        (Weather::Warm, _) => "sunglasses and a lightweight bag",
        (_, Occasion::Trail) => "wool hiking socks and a small daypack",
        _ => "plain socks and a simple bag",
    };
    Outfit {
        weather,
        occasion,
        temperature: rng.random_range(match weather {
            Weather::Warm => 24..=32,
            Weather::Mild => 13..=20,
            Weather::Rain => 10..=17,
            Weather::Cold => -5..=5,
        }),
        palette: rng.random_range(0..PALETTES.len()),
        top,
        bottom,
        mid,
        outer,
        shoes,
        accessory,
    }
}

impl Outfit {
    fn render(&self) -> String {
        let p = &PALETTES[self.palette];
        let occasion = match self.occasion {
            Occasion::Errands => "Everyday errands",
            Occasion::Office => "Business-casual office",
            Occasion::Dinner => "Smart-casual dinner",
            Occasion::Trail => "Day hike on maintained trails",
        };
        let weather = match self.weather {
            Weather::Warm => "warm and dry",
            Weather::Mild => "mild and dry",
            Weather::Rain => "steady rain",
            Weather::Cold => "cold and dry",
        };
        let shoe_color = if self.occasion == Occasion::Trail {
            p.dark
        } else {
            p.leather
        };
        let mut lines = vec![
            format!("{occasion}; {weather}, {} C.", self.temperature),
            format!("Palette: {}, {}, and {}.", p.light, p.dark, p.accent),
            format!("Top: {} {}.", p.light, self.top),
            format!("Bottom: {} {}.", p.dark, self.bottom),
        ];
        if let Some(mid) = self.mid {
            lines.push(format!("Middle layer: {} {mid}.", p.accent));
        }
        if let Some(outer) = self.outer {
            lines.push(format!("Outer layer: {} {outer}.", p.dark));
        }
        lines.push(format!("Footwear: {shoe_color} {}.", self.shoes));
        lines.push(format!("Accessories: {}.", self.accessory));
        if self.mid.is_none() {
            let accent_item = match (self.weather, self.occasion) {
                (Weather::Warm, Occasion::Trail) => "sun hat",
                (Weather::Rain, Occasion::Trail) => "pack cover",
                _ => "bag",
            };
            lines.push(format!("Accent: {} {accent_item}.", p.accent));
        }
        lines.join("\n")
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let weather =
        [Weather::Warm, Weather::Mild, Weather::Rain, Weather::Cold][rng.random_range(0..4)];
    let occasion = [
        Occasion::Errands,
        Occasion::Office,
        Occasion::Dinner,
        Occasion::Trail,
    ][rng.random_range(0..4)];
    outfit(rng, weather, occasion).render()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn every_weather_occasion_combination_has_appropriate_layers_and_footwear() {
        let mut rng = StdRng::seed_from_u64(31);
        for weather in [Weather::Warm, Weather::Mild, Weather::Rain, Weather::Cold] {
            for occasion in [
                Occasion::Errands,
                Occasion::Office,
                Occasion::Dinner,
                Occasion::Trail,
            ] {
                for _ in 0..50 {
                    let o = outfit(&mut rng, weather, occasion);
                    let text = o.render();
                    if weather == Weather::Warm {
                        assert!(o.mid.is_none() && o.outer.is_none());
                        assert!(o.temperature >= 24);
                    }
                    if weather == Weather::Cold {
                        assert!(o.mid.is_some() && o.outer.unwrap().contains("insulated"));
                        assert!(!o.bottom.contains("shorts"));
                        assert!(o.accessory.contains("gloves"));
                    }
                    if weather == Weather::Rain {
                        assert!(o.outer.unwrap().contains("waterproof"));
                        assert!(o.shoes.contains("resistant") || o.shoes.contains("waterproof"));
                    }
                    if occasion == Occasion::Trail {
                        assert!(o.shoes.contains("trail") || o.shoes.contains("hiking"));
                        assert!(o.top.contains("base layer"));
                    }
                    if matches!(occasion, Occasion::Office | Occasion::Dinner) {
                        assert!(o.bottom.contains("trousers"));
                        assert!(o.shoes.contains("leather"));
                    }
                    let p = &PALETTES[o.palette];
                    for color in [p.light, p.dark, p.accent] {
                        assert!(text.contains(color));
                    }
                    assert!(!text.ends_with('\n'));
                }
            }
        }
    }
}
