"""Build coordinated outfits using a reviewed subset of Corpora fabric vocabulary.

Run from any directory: python -B families/design/wardrobe/import_fabrics.py
The pinned download lives outside the sampling tree. Outfit rules are original;
the downloaded list provides vocabulary, not garment or performance claims.
"""

from pathlib import Path
import hashlib
import json
import urllib.request

ROOT = Path(__file__).resolve().parents[3]
REVISION = "2e7adec11a0561696c236ae1fca82e88be173d42"
URL = f"https://raw.githubusercontent.com/dariusk/corpora/{REVISION}/data/materials/fabrics.json"
SHA256 = "6f4314342350c91f1457f3c20c0f37cf0c2149c12306c556302798a3ccae9684"
ACCEPTED = {
    "cotton", "linen", "wool", "nylon", "polyester", "poplin", "oxford",
    "denim", "canvas", "corduroy", "seersucker", "chambray", "twill",
    "jersey", "flannel", "gabardine", "polar fleece", "ripstop", "tweed",
}

PALETTES = [
    ("ivory", "navy", "rust", "dark brown"),
    ("stone", "charcoal", "forest green", "black"),
    ("cream", "olive", "terracotta", "dark brown"),
    ("pale blue", "navy", "burgundy", "dark brown"),
    ("light gray", "black", "cobalt", "black"),
    ("white", "indigo", "ochre", "tan"),
    ("ecru", "chocolate", "sage", "dark brown"),
    ("oatmeal", "slate", "plum", "black"),
    ("sand", "espresso", "teal", "dark brown"),
    ("pearl gray", "midnight blue", "muted coral", "black"),
]

# Garments within a row have matching formality and seasonal weight. Outer layers
# have enough room for the named middle layer; no unexplained performance claims
# are inferred from a fabric name.
CONTEXTS = {
    "everyday": {
        "title": "Everyday errands",
        "warm_top": ["cotton jersey T-shirt", "linen short-sleeve shirt", "cotton seersucker camp shirt", "cotton polo"],
        "top": ["cotton long-sleeve jersey shirt", "cotton oxford shirt", "cotton chambray shirt", "cotton flannel shirt"],
        "warm_bottom": ["cotton twill shorts", "lightweight linen trousers", "lightweight cotton chinos", "cotton midi skirt"],
        "bottom": ["straight-leg denim jeans", "cotton twill chinos", "corduroy trousers", "lined cotton midi skirt with opaque tights"],
        "dry_shoes": ["canvas sneakers", "leather sneakers", "flat leather loafers"],
        "wet_shoes": ["water-resistant walking boots", "water-resistant leather ankle boots"],
        "mild_mid": ["cotton overshirt", "light cotton cardigan", "unlined chore jacket"],
        "cold_mid": ["wool cardigan", "wool crew-neck sweater"],
    },
    "office": {
        "title": "Business-casual office",
        "warm_top": ["linen-cotton collared shirt", "cotton poplin shirt", "short-sleeve cotton knit polo", "cotton seersucker blouse"],
        "top": ["cotton poplin shirt", "cotton oxford shirt", "fine-gauge wool crew-neck sweater", "cotton jersey mock-neck top"],
        "warm_bottom": ["lightweight tailored cotton trousers", "linen-cotton trousers", "cotton poplin midi skirt"],
        "bottom": ["wool-blend tailored trousers", "tailored wool tweed trousers", "tailored cotton gabardine trousers", "wool midi skirt with opaque tights"],
        "dry_shoes": ["leather loafers", "leather derby shoes", "flat leather lace-up shoes"],
        "wet_shoes": ["weather-resistant leather ankle boots"],
        "mild_mid": ["unstructured cotton blazer", "fine-gauge wool cardigan"],
        "cold_mid": ["fine-gauge wool cardigan", "unstructured wool blazer"],
    },
    "dinner": {
        "title": "Smart-casual dinner",
        "warm_top": ["linen collared shirt", "fine-gauge cotton polo", "cotton poplin blouse", "cotton jersey boat-neck top"],
        "top": ["fine-gauge merino turtleneck", "cotton poplin shirt", "fine-gauge wool crew-neck sweater", "cotton blouse"],
        "warm_bottom": ["lightweight tailored linen trousers", "pleated cotton trousers", "linen midi skirt"],
        "bottom": ["tailored wool trousers", "pleated wool-blend trousers", "wool midi skirt with opaque tights"],
        "dry_shoes": ["leather loafers", "leather derby shoes", "flat leather ankle-strap shoes"],
        "wet_shoes": ["weather-resistant leather Chelsea boots"],
        "mild_mid": ["unstructured blazer", "fine-gauge wool cardigan"],
        "cold_mid": ["fine-gauge wool cardigan", "unstructured wool blazer"],
    },
    "trail": {
        "title": "Day hike on maintained trails",
        "warm_top": ["short-sleeve moisture-wicking polyester base layer", "lightweight merino short-sleeve base layer"],
        "top": ["long-sleeve merino base layer", "long-sleeve moisture-wicking polyester base layer"],
        "warm_bottom": ["quick-drying nylon ripstop hiking shorts", "lightweight nylon hiking trousers", "lightweight polyester hiking trousers"],
        "bottom": ["quick-drying nylon ripstop hiking trousers", "quick-drying polyester hiking trousers"],
        "dry_shoes": ["grippy trail shoes", "lightweight hiking shoes"],
        "wet_shoes": ["waterproof hiking boots", "waterproof trail shoes"],
        "mild_mid": ["lightweight polar fleece", "lightweight wool zip-neck layer"],
        "cold_mid": ["insulating polar fleece", "insulating wool sweater"],
    },
}

WEATHER = {
    "warm": ("warm and dry", 24, 32),
    "mild": ("mild and dry", 13, 20),
    "rain": ("steady rain", 10, 17),
    "cold": ("cold and dry", 0, 8),
}


def rust_strings(items):
    return "&[" + ", ".join(json.dumps(s) for s in items) + "]"


def source(context, weather):
    c = CONTEXTS[context]
    trail = context == "trail"
    label, low, high = WEATHER[weather]
    middle = (c["cold_mid"] if weather == "cold" else c["mild_mid"])
    has_middle = weather != "warm" and not (weather == "rain" and context == "everyday")
    outer = ""
    if weather == "rain":
        outer = "hooded waterproof shell sized over the middle layer" if trail else "hooded waterproof raincoat sized over the middle layer"
        if not has_middle:
            outer = "hooded waterproof raincoat"
    elif weather == "cold":
        outer = "insulated wind-resistant hiking jacket sized over the middle layer" if trail else "insulated coat sized over the middle layer"
    accessories = {
        "warm": "sun hat, lightweight hiking socks, and a small daypack" if trail else "sunglasses and a lightweight bag",
        "mild": "wool hiking socks and a small daypack" if trail else "plain socks and a simple bag",
        "rain": "wool hiking socks and a waterproof pack cover" if trail else "compact umbrella and a water-resistant bag",
        "cold": "warm beanie, gloves, and wool hiking socks" if trail else "wool scarf, gloves, and warm socks",
    }[weather]
    palettes = ",\n".join("    (" + ", ".join(json.dumps(x) for x in p) + ")" for p in PALETTES)
    layer_code = 'lines.push(format!("Middle layer: {{accent}} {{}}.", choose(rng, MIDDLE)));' if has_middle else ""
    outer_code = 'lines.push(format!("Outer layer: {{dark}} {outer}."));' if outer else ""
    # Doubled braces in these fragments belong to Rust, not this outer f-string.
    layer_code = layer_code.replace("{{", "{").replace("}}", "}")
    outer_code = outer_code.replace("{{", "{").replace("}}", "}").replace("{outer}", outer)
    accent = "sun hat" if trail and weather == "warm" else "pack cover" if trail and weather == "rain" else "daypack" if trail else "bag"
    tests = []
    if weather == "warm":
        tests += ['assert!(!text.contains("Middle layer:") && !text.contains("Outer layer:"));']
    if weather == "rain":
        tests += ['assert!(text.contains("Outer layer:") && text.contains("waterproof"));', 'assert!(text.lines().find(|l| l.starts_with("Footwear:")).unwrap().contains("resistant") || text.lines().find(|l| l.starts_with("Footwear:")).unwrap().contains("waterproof"));']
    if weather == "cold":
        tests += ['assert!(text.contains("Middle layer:") && text.contains("insulated"));', 'assert!(text.contains("gloves") && !text.contains("shorts"));']
    if trail:
        tests += ['assert!(text.contains("base layer") && (text.contains("hiking") || text.contains("trail")));']
    if context in ("office", "dinner"):
        tests += ['assert!(text.contains("trousers") || text.contains("skirt"));', 'assert!(text.lines().find(|l| l.starts_with("Footwear:")).unwrap().contains("leather"));']
    return f'''// Generated by ../../import_fabrics.py; outfit constraints are original.
use rand::{{rngs::StdRng, Rng}};

const PALETTES: [(&str, &str, &str, &str); {len(PALETTES)}] = [
{palettes}
];
const TOPS: &[&str] = {rust_strings(c['warm_top'] if weather == 'warm' else c['top'])};
const BOTTOMS: &[&str] = {rust_strings(c['warm_bottom'] if weather == 'warm' else c['bottom'])};
const SHOES: &[&str] = {rust_strings(c['wet_shoes'] if weather in ('rain', 'cold') else c['dry_shoes'])};
''' + (f"const MIDDLE: &[&str] = {rust_strings(middle)};\n" if has_middle else "") + f'''
fn choose<'a>(rng: &mut StdRng, options: &[&'a str]) -> &'a str {{
    options[rng.random_range(0..options.len())]
}}

pub fn generate(rng: &mut StdRng) -> String {{
    let (light, dark, accent, leather) = PALETTES[rng.random_range(0..PALETTES.len())];
    let temperature = rng.random_range({low}..={high});
    let shoe_color = {'dark' if trail else 'leather'};
    {'let _ = leather;' if trail else ''}
    let mut lines = vec![
        format!("{c['title']}; {label}, {{temperature}} C."),
        format!("Top: {{light}} {{}}.", choose(rng, TOPS)),
        format!("Bottom: {{dark}} {{}}.", choose(rng, BOTTOMS)),
    ];
    {layer_code}
    {outer_code}
    lines.push(format!("Footwear: {{shoe_color}} {{}}.", choose(rng, SHOES)));
    lines.push("Accessories: {accessories}.".to_owned());
    {'' if has_middle else f'lines.push(format!("Accent: {{accent}} {accent}."));'}
    lines.join("\\n")
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn scenario_constraints_and_actual_palette_items() {{
        let mut rng = StdRng::seed_from_u64(31);
        let mut outputs = HashSet::new();
        for _ in 0..500 {{
            let text = generate(&mut rng);
            {' '.join(tests)}
            let matched = PALETTES.iter().any(|(light, dark, accent, leather)| {{
                {'let _ = leather;' if trail else ''}
                text.contains(&format!("Top: {{light}} "))
                    && text.contains(&format!("Bottom: {{dark}} "))
                    && text.contains(&format!("{{}}: {{accent}} ", {'"Middle layer"' if has_middle else '"Accent"'}))
                    && text.contains(&format!("Footwear: {{}} ", {'dark' if trail else 'leather'}))
            }});
            assert!(matched, "{{text}}");
            assert!(!text.ends_with('\\n'));
            outputs.insert(text);
        }}
        assert!(outputs.len() > 150);
    }}
}}
'''


def main():
    cache = ROOT / "target/imports/corpora/data/materials/fabrics.json"
    if not cache.exists():
        cache.parent.mkdir(parents=True, exist_ok=True)
        with urllib.request.urlopen(URL) as response:
            cache.write_bytes(response.read())
    raw = cache.read_bytes()
    if hashlib.sha256(raw).hexdigest() != SHA256:
        raise ValueError("Pinned fabrics checksum mismatch")
    vocabulary = set(json.loads(raw)["fabrics"])
    if not ACCEPTED <= vocabulary:
        raise ValueError(f"Absent reviewed fabric names: {ACCEPTED - vocabulary}")
    base = Path(__file__).resolve().parent
    for context in CONTEXTS:
        for weather in WEATHER:
            leaf = base / context / weather
            leaf.mkdir(parents=True, exist_ok=True)
            (leaf / "generator.rs").write_text(source(context, weather), encoding="utf-8")
    print("Generated 16 constrained wardrobe leaves; verified 19 Corpora fabric names.")


if __name__ == "__main__":
    main()
