use rand::{Rng, rngs::StdRng};

const EVERYDAY: usize = 0;
const DINNER: usize = 2;
const TRAIL: usize = 3;
const WARM: usize = 0;
const MILD: usize = 1;
const RAIN: usize = 2;
const COLD: usize = 3;

struct Palette {
    light: &'static str,
    dark: &'static str,
    accent: &'static str,
    leather: &'static str,
}

const PALETTES: &[Palette] = &[
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
    Palette {
        light: "white",
        dark: "indigo",
        accent: "ochre",
        leather: "tan",
    },
    Palette {
        light: "ecru",
        dark: "chocolate",
        accent: "sage",
        leather: "dark brown",
    },
    Palette {
        light: "oatmeal",
        dark: "slate",
        accent: "plum",
        leather: "black",
    },
    Palette {
        light: "sand",
        dark: "espresso",
        accent: "teal",
        leather: "dark brown",
    },
    Palette {
        light: "pearl gray",
        dark: "midnight blue",
        accent: "muted coral",
        leather: "black",
    },
    Palette {
        light: "white",
        dark: "black",
        accent: "tomato red",
        leather: "black",
    },
    Palette {
        light: "cream",
        dark: "dark brown",
        accent: "ochre",
        leather: "dark brown",
    },
];

struct Wardrobe {
    warm_tops: &'static [&'static str],
    tops: &'static [&'static str],
    warm_bottoms: &'static [&'static str],
    bottoms: &'static [&'static str],
    warm_dresses: &'static [&'static str],
    dresses: &'static [&'static str],
    warm_onepiece: &'static [&'static str],
    onepiece: &'static [&'static str],
    shoes: &'static [&'static str],
    warm_shoes: &'static [&'static str],
    boots: &'static [&'static str],
    light_layers: &'static [&'static str],
    knitwear: &'static [&'static str],
    bags: &'static [&'static str],
}

const WARDROBES: &[Wardrobe] = &[
    Wardrobe {
        warm_tops: &[
            "cotton jersey T-shirt",
            "linen camp shirt",
            "cotton seersucker shirt",
            "cotton knit polo",
        ],
        tops: &[
            "cotton oxford shirt",
            "cotton chambray shirt",
            "long-sleeve cotton jersey shirt",
            "cotton flannel shirt",
        ],
        warm_bottoms: &[
            "cotton twill shorts",
            "wide-leg linen trousers",
            "cropped cotton chinos",
            "cotton poplin midi skirt",
        ],
        bottoms: &[
            "straight-leg denim jeans",
            "cotton twill chinos",
            "corduroy trousers",
            "cotton twill midi skirt",
        ],
        warm_dresses: &[
            "linen shirt dress",
            "cotton seersucker midi dress",
            "cotton jersey T-shirt dress",
        ],
        dresses: &["long-sleeve cotton shirt dress", "wool jersey midi dress"],
        warm_onepiece: &[
            "wide-leg linen jumpsuit",
            "short-sleeve cotton poplin jumpsuit",
        ],
        onepiece: &["cotton twill coveralls", "long-sleeve denim jumpsuit"],
        shoes: &[
            "canvas sneakers",
            "leather sneakers",
            "flat leather loafers",
        ],
        warm_shoes: &[
            "canvas sneakers",
            "leather fisherman sandals",
            "flat leather sandals",
        ],
        boots: &[
            "water-resistant leather lace-up boots",
            "water-resistant leather ankle boots",
        ],
        light_layers: &[
            "cotton overshirt",
            "unlined cotton chore jacket",
            "light cotton cardigan",
        ],
        knitwear: &["wool cardigan", "wool crew-neck sweater"],
        bags: &[
            "canvas crossbody bag",
            "canvas tote",
            "small nylon backpack",
        ],
    },
    Wardrobe {
        warm_tops: &[
            "cotton poplin shirt",
            "linen collared shirt",
            "short-sleeve cotton knit polo",
            "cotton seersucker blouse",
        ],
        tops: &[
            "cotton poplin shirt",
            "cotton oxford shirt",
            "cotton jersey mock-neck top",
        ],
        warm_bottoms: &[
            "tailored linen trousers",
            "lightweight cotton gabardine trousers",
            "cotton poplin midi skirt",
        ],
        bottoms: &[
            "tailored wool trousers",
            "cotton gabardine trousers",
            "wool tweed trousers",
            "wool midi skirt",
        ],
        warm_dresses: &["cotton poplin shirt dress", "short-sleeve linen midi dress"],
        dresses: &["long-sleeve wool jersey dress", "cotton twill shirt dress"],
        warm_onepiece: &[
            "tailored short-sleeve linen jumpsuit",
            "cotton poplin jumpsuit with a waist tie",
        ],
        onepiece: &[
            "tailored wool-blend jumpsuit",
            "cotton twill collared jumpsuit",
        ],
        shoes: &[
            "leather loafers",
            "leather derby shoes",
            "leather ballet flats",
        ],
        warm_shoes: &[
            "leather loafers",
            "leather ballet flats",
            "leather derby shoes",
        ],
        boots: &[
            "weather-resistant leather ankle boots",
            "weather-resistant leather Chelsea boots",
        ],
        light_layers: &["unstructured cotton blazer", "fine-gauge wool cardigan"],
        knitwear: &[
            "fine-gauge wool cardigan",
            "fine-gauge wool crew-neck sweater",
        ],
        bags: &[
            "leather tote",
            "structured leather shoulder bag",
            "canvas briefcase",
        ],
    },
    Wardrobe {
        warm_tops: &[
            "linen collared shirt",
            "fine-gauge cotton polo",
            "cotton poplin blouse",
            "cotton jersey boat-neck top",
        ],
        tops: &[
            "fine-gauge merino turtleneck",
            "cotton poplin shirt",
            "cotton blouse",
            "cotton jersey boat-neck top",
        ],
        warm_bottoms: &[
            "wide-leg linen trousers",
            "pleated cotton trousers",
            "linen midi skirt",
        ],
        bottoms: &[
            "tailored wool trousers",
            "pleated wool-blend trousers",
            "wool midi skirt",
        ],
        warm_dresses: &[
            "linen wrap dress",
            "cotton poplin midi dress",
            "sleeveless linen column dress",
        ],
        dresses: &[
            "long-sleeve wool jersey wrap dress",
            "fine-gauge wool midi dress",
        ],
        warm_onepiece: &[
            "wide-leg linen jumpsuit",
            "sleeveless cotton twill jumpsuit",
        ],
        onepiece: &[
            "tailored wool-blend jumpsuit",
            "long-sleeve cotton gabardine jumpsuit",
        ],
        shoes: &[
            "leather loafers",
            "leather derby shoes",
            "flat leather ankle-strap shoes",
        ],
        warm_shoes: &[
            "leather loafers",
            "flat leather ankle-strap shoes",
            "leather strappy sandals",
        ],
        boots: &[
            "weather-resistant leather Chelsea boots",
            "weather-resistant leather ankle boots",
        ],
        light_layers: &["unstructured linen blazer", "fine-gauge wool cardigan"],
        knitwear: &["fine-gauge wool cardigan", "fine-gauge wool V-neck sweater"],
        bags: &[
            "small structured leather bag",
            "folded leather clutch",
            "compact leather shoulder bag",
        ],
    },
    Wardrobe {
        warm_tops: &[
            "short-sleeve moisture-wicking polyester base layer",
            "lightweight merino short-sleeve base layer",
        ],
        tops: &[
            "long-sleeve merino base layer",
            "long-sleeve moisture-wicking polyester base layer",
        ],
        warm_bottoms: &[
            "quick-drying nylon ripstop hiking shorts",
            "lightweight nylon hiking trousers",
            "lightweight polyester hiking trousers",
        ],
        bottoms: &[
            "nylon ripstop hiking trousers",
            "quick-drying polyester hiking trousers",
        ],
        warm_dresses: &[],
        dresses: &[],
        warm_onepiece: &[],
        onepiece: &[],
        shoes: &["grippy trail shoes", "lightweight hiking shoes"],
        warm_shoes: &["grippy trail shoes", "lightweight hiking shoes"],
        boots: &["waterproof hiking boots", "waterproof trail shoes"],
        light_layers: &[
            "lightweight polar fleece",
            "lightweight wool zip-neck layer",
        ],
        knitwear: &["insulating polar fleece", "insulating wool zip-neck layer"],
        bags: &["small hiking daypack"],
    },
];

fn pick<'a, T>(rng: &mut StdRng, choices: &'a [T]) -> &'a T {
    &choices[rng.random_range(0..choices.len())]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Role {
    Top,
    Bottom,
    Onepiece,
    Middle,
    Outer,
    Shoes,
    Legwear,
    Bag,
    Accessory,
}

struct Item {
    role: Role,
    text: String,
}

#[derive(Default)]
struct Outfit {
    items: Vec<Item>,
    skirt: bool,
    matched_set: bool,
}

impl Outfit {
    fn add(&mut self, role: Role, color: &str, garment: &str) {
        self.items.push(Item {
            role,
            text: format!("{color} {garment}"),
        });
    }

    fn render(&self) -> String {
        self.items
            .iter()
            .map(|item| {
                let mut chars = item.text.chars();
                let first = chars.next().expect("garment is nonempty");
                first.to_uppercase().collect::<String>() + chars.as_str()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn assemble(rng: &mut StdRng, context: usize, weather: usize) -> Outfit {
    let wardrobe = &WARDROBES[context];
    let palette = pick(rng, PALETTES);
    let warm = weather == WARM;
    let trail = context == TRAIL;
    let color_route = rng.random_range(0..3);
    let main = if color_route == 2 {
        palette.accent
    } else {
        palette.dark
    };
    let top_color = if color_route == 1 {
        palette.dark
    } else {
        palette.light
    };
    let mut outfit = Outfit::default();
    let silhouette = if trail { 0 } else { rng.random_range(0..10) };

    match silhouette {
        0..=4 => {
            let top = pick(
                rng,
                if warm {
                    wardrobe.warm_tops
                } else {
                    wardrobe.tops
                },
            );
            let bottom = pick(
                rng,
                if warm {
                    wardrobe.warm_bottoms
                } else {
                    wardrobe.bottoms
                },
            );
            outfit.add(Role::Top, top_color, top);
            outfit.add(Role::Bottom, palette.dark, bottom);
            outfit.skirt = bottom.contains("skirt");
        }
        5..=6 => {
            let dress = pick(
                rng,
                if warm {
                    wardrobe.warm_dresses
                } else {
                    wardrobe.dresses
                },
            );
            outfit.add(Role::Onepiece, main, dress);
            outfit.skirt = true;
        }
        7 => {
            let onepiece = pick(
                rng,
                if warm {
                    wardrobe.warm_onepiece
                } else {
                    wardrobe.onepiece
                },
            );
            outfit.add(Role::Onepiece, main, onepiece);
        }
        _ => {
            outfit.matched_set = true;
            if context == EVERYDAY && warm {
                outfit.add(Role::Top, main, "linen camp shirt");
                outfit.add(
                    Role::Bottom,
                    main,
                    pick(rng, &["linen shorts", "wide-leg linen trousers"]),
                );
            } else {
                outfit.add(
                    Role::Top,
                    palette.light,
                    if context == DINNER {
                        "fine-gauge cotton knit top"
                    } else {
                        "cotton jersey T-shirt"
                    },
                );
                let fabric = if warm {
                    "linen"
                } else if context == EVERYDAY {
                    "cotton twill"
                } else {
                    "wool"
                };
                let jacket = if context == EVERYDAY {
                    "chore jacket"
                } else {
                    "unstructured blazer"
                };
                outfit.add(Role::Middle, main, &format!("{fabric} {jacket}"));
                outfit.add(Role::Bottom, main, &format!("{fabric} trousers"));
            }
        }
    }

    if !warm && !outfit.matched_set {
        // One-piece garments get an open front layer, rather than a second neckline.
        let onepiece = outfit.items.iter().any(|item| item.role == Role::Onepiece);
        let middle = if onepiece {
            "wool cardigan"
        } else {
            pick(
                rng,
                if weather == COLD {
                    wardrobe.knitwear
                } else {
                    wardrobe.light_layers
                },
            )
        };
        if weather == COLD || trail || rng.random_bool(0.65) {
            let color = if color_route == 0 {
                palette.accent
            } else {
                palette.dark
            };
            outfit.add(Role::Middle, color, middle);
        }
    }
    if weather == RAIN {
        outfit.add(
            Role::Outer,
            if trail { palette.accent } else { palette.dark },
            if trail {
                "hooded waterproof hiking shell"
            } else {
                "hooded waterproof raincoat"
            },
        );
    } else if weather == COLD {
        outfit.add(
            Role::Outer,
            palette.dark,
            if trail {
                "insulated wind-resistant hiking jacket"
            } else {
                "insulated wool overcoat"
            },
        );
    }

    if outfit.skirt && !warm {
        outfit.add(Role::Legwear, palette.dark, "opaque tights");
    }
    let shoes = if weather == COLD || weather == RAIN {
        wardrobe.boots
    } else if warm {
        wardrobe.warm_shoes
    } else {
        wardrobe.shoes
    };
    outfit.add(
        Role::Shoes,
        if trail { palette.dark } else { palette.leather },
        pick(rng, shoes),
    );

    let bag_color = if warm && color_route == 0 {
        palette.accent
    } else if trail {
        palette.dark
    } else {
        palette.leather
    };
    let bag = if weather == RAIN && !trail {
        "water-resistant shoulder bag"
    } else {
        pick(rng, wardrobe.bags)
    };
    outfit.add(Role::Bag, bag_color, bag);
    match weather {
        WARM if trail => {
            outfit.add(Role::Accessory, palette.accent, "brimmed sun hat");
            outfit.add(Role::Accessory, palette.light, "lightweight hiking socks");
        }
        WARM if context == EVERYDAY && rng.random_bool(0.5) => {
            outfit.add(Role::Accessory, palette.leather, "sunglasses")
        }
        RAIN if trail => {
            outfit.add(Role::Accessory, palette.accent, "waterproof pack cover");
            outfit.add(Role::Accessory, palette.light, "wool hiking socks");
        }
        RAIN => outfit.add(Role::Accessory, palette.dark, "compact umbrella"),
        COLD => {
            outfit.add(
                Role::Accessory,
                palette.accent,
                if trail { "wool beanie" } else { "wool scarf" },
            );
            outfit.add(
                Role::Accessory,
                palette.dark,
                if trail {
                    "insulated gloves"
                } else {
                    "lined leather gloves"
                },
            );
            if trail {
                outfit.add(Role::Accessory, palette.light, "wool hiking socks");
            }
        }
        MILD if trail => outfit.add(Role::Accessory, palette.light, "wool hiking socks"),
        _ => {}
    }
    outfit
}

pub fn generate(rng: &mut StdRng, context: usize, weather: usize) -> String {
    assemble(rng, context, weather).render()
}

#[cfg(test)]
pub fn check(context: usize, weather: usize) {
    use rand::SeedableRng;
    use std::collections::HashSet;

    let mut rng = StdRng::seed_from_u64(31);
    let mut outputs = HashSet::new();
    let mut silhouettes = HashSet::new();
    for _ in 0..500 {
        let outfit = assemble(&mut rng, context, weather);
        let count = |role| outfit.items.iter().filter(|item| item.role == role).count();
        assert_eq!(count(Role::Shoes), 1);
        assert_eq!(count(Role::Bag), 1);
        assert_eq!(count(Role::Onepiece) + count(Role::Top), 1);
        assert_eq!(count(Role::Onepiece) + count(Role::Bottom), 1);
        assert_eq!(count(Role::Legwear) == 1, outfit.skirt && weather != WARM);
        assert_eq!(count(Role::Outer) == 1, weather == RAIN || weather == COLD);
        if weather == WARM {
            assert!(count(Role::Middle) == 0 || outfit.matched_set);
        }
        if weather == COLD {
            assert_eq!(count(Role::Middle), 1);
            assert!(!outfit.items.iter().any(|item| item.text.contains("shorts")));
            assert!(outfit.items.iter().any(|item| item.text.contains("gloves")));
        }
        if weather == RAIN || weather == COLD {
            let footwear = &outfit
                .items
                .iter()
                .find(|item| item.role == Role::Shoes)
                .unwrap()
                .text;
            assert!(
                WARDROBES[context]
                    .boots
                    .iter()
                    .any(|boots| footwear.ends_with(boots))
            );
            let outerwear = &outfit
                .items
                .iter()
                .find(|item| item.role == Role::Outer)
                .unwrap()
                .text;
            assert!(outerwear.contains(if weather == RAIN {
                "waterproof"
            } else {
                "insulated"
            }));
        }
        if outfit.matched_set {
            let upper_role = if context == EVERYDAY && weather == WARM {
                Role::Top
            } else {
                Role::Middle
            };
            let upper = &outfit
                .items
                .iter()
                .find(|item| item.role == upper_role)
                .unwrap()
                .text;
            let bottom = &outfit
                .items
                .iter()
                .find(|item| item.role == Role::Bottom)
                .unwrap()
                .text;
            let fabric = ["linen", "cotton twill", "wool"]
                .into_iter()
                .find(|fabric| upper.contains(fabric))
                .unwrap();
            assert!(bottom.contains(fabric));
            assert!(PALETTES.iter().any(|palette| {
                [palette.dark, palette.accent].iter().any(|color| {
                    let prefix = format!("{color} ");
                    upper.starts_with(&prefix) && bottom.starts_with(&prefix)
                })
            }));
        }
        if context == TRAIL {
            assert_eq!(count(Role::Onepiece), 0);
            assert!(!outfit.skirt);
        } else {
            silhouettes.insert((count(Role::Onepiece), outfit.skirt, outfit.matched_set));
        }
        let text = outfit.render();
        assert!(!text.ends_with('\n'));
        assert!(text.lines().all(|line| !line.trim().is_empty()));
        assert_eq!(text.lines().count(), outfit.items.len());
        outputs.insert(text);
    }
    assert!(outputs.len() > 200);
    if context != TRAIL {
        assert!(silhouettes.len() >= 4);
    }
}
