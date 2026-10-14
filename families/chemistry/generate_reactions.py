"""Reproduce bounded reaction leaves; no network or inferred reactions.

Species and coefficients are explicit, chemically reviewed catalog entries.
The Rust support beside every leaf independently tests atom and charge balance.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
CATALOG = {}


def add(path, title, left, right, conditions, equilibrium=False):
    def terms(text):
        result = []
        for term in text.split(" + "):
            coefficient, sep, species = term.partition(" ")
            result.append((int(coefficient), species) if sep else (1, coefficient))
        return result
    CATALOG.setdefault(path, []).append((title, terms(left), terms(right), conditions, equilibrium))


# Dilute aqueous strong acids and completely dissolved hydroxides; spectator
# ions stay soluble for all selected combinations.
for acid, anion in [("HCl", "Cl"), ("HBr", "Br"), ("HI", "I"), ("HNO3", "NO3")]:
    for metal in ["Li", "Na", "K"]:
        add("aqueous/acid-base/neutralization", f"{acid} and {metal}OH neutralization",
            f"{acid}(aq) + {metal}OH(aq)", f"{metal}{anion}(aq) + H2O(l)",
            "Dilute aqueous solutions. Net ionic reaction: H3O^+(aq) + OH^-(aq) -> 2 H2O(l).")
add("aqueous/acid-base/neutralization", "Barium hydroxide and nitric acid",
    "Ba(OH)2(aq) + 2 HNO3(aq)", "Ba(NO3)2(aq) + 2 H2O(l)", "Dilute aqueous solutions; soluble nitrate salt.")
for name, acid, base in [("acetic acid", "CH3COOH", "CH3COO^-"), ("formic acid", "HCOOH", "HCOO^-"),
    ("hydrofluoric acid", "HF", "F^-"), ("hypochlorous acid", "HClO", "ClO^-"),
    ("ammonium", "NH4^+", "NH3"), ("bicarbonate", "HCO3^-", "CO3^2-")]:
    add("aqueous/acid-base/weak-equilibria", f"Proton donation by {name}", f"{acid}(aq) + H2O(l)",
        f"{base}(aq) + H3O^+(aq)", "Aqueous acid dissociation equilibrium; ionization fraction depends on concentration and Ka.", True)
for name, base, acid in [("ammonia", "NH3", "NH4^+"), ("methylamine", "CH3NH2", "CH3NH3^+"),
    ("acetate", "CH3COO^-", "CH3COOH")]:
    add("aqueous/acid-base/weak-equilibria", f"Proton acceptance by {name}", f"{base}(aq) + H2O(l)",
        f"{acid}(aq) + OH^-(aq)", "Aqueous base hydrolysis; equilibrium amounts depend on concentration and the base constant.", True)
for title, left, right in [
    ("Silver chloride", "Ag^+(aq) + Cl^-(aq)", "AgCl(s)"),
    ("Silver bromide", "Ag^+(aq) + Br^-(aq)", "AgBr(s)"),
    ("Silver iodide", "Ag^+(aq) + I^-(aq)", "AgI(s)"),
    ("Barium sulfate", "Ba^2+(aq) + SO4^2-(aq)", "BaSO4(s)"),
    ("Strontium sulfate", "Sr^2+(aq) + SO4^2-(aq)", "SrSO4(s)"),
    ("Lead(II) sulfate", "Pb^2+(aq) + SO4^2-(aq)", "PbSO4(s)"),
    ("Lead(II) iodide", "Pb^2+(aq) + 2 I^-(aq)", "PbI2(s)"),
    ("Calcium carbonate", "Ca^2+(aq) + CO3^2-(aq)", "CaCO3(s)"),
    ("Lead(II) carbonate", "Pb^2+(aq) + CO3^2-(aq)", "PbCO3(s)")]:
    add("aqueous/precipitation/ionic-salts", title + " precipitation", left, right,
        "Net ionic precipitation equation when the ion activity product exceeds Ksp; no excess complexing ligand.")
for metal in ["Ca", "Mg", "Zn"]:
    add("aqueous/gas-evolution/carbonates", f"Acid dissolution of {metal}CO3",
        f"{metal}CO3(s) + 2 H3O^+(aq)", f"{metal}^2+(aq) + CO2(g) + 3 H2O(l)",
        "Excess dilute nonoxidizing aqueous acid; carbon dioxide leaves the solution.")
add("aqueous/gas-evolution/carbonates", "Bicarbonate acidification", "HCO3^-(aq) + H3O^+(aq)",
    "CO2(g) + 2 H2O(l)", "Excess aqueous acid; carbon dioxide escapes. Bicarbonate is protonated and the carbonic acid product decomposes.")
for title, left, right in [
    ("Zinc displaces copper", "Zn(s) + Cu^2+(aq)", "Zn^2+(aq) + Cu(s)"),
    ("Iron displaces copper", "Fe(s) + Cu^2+(aq)", "Fe^2+(aq) + Cu(s)"),
    ("Copper displaces silver", "Cu(s) + 2 Ag^+(aq)", "Cu^2+(aq) + 2 Ag(s)"),
    ("Zinc displaces silver", "Zn(s) + 2 Ag^+(aq)", "Zn^2+(aq) + 2 Ag(s)"),
    ("Zinc dissolves in acid", "Zn(s) + 2 H3O^+(aq)", "Zn^2+(aq) + H2(g) + 2 H2O(l)"),
    ("Magnesium dissolves in acid", "Mg(s) + 2 H3O^+(aq)", "Mg^2+(aq) + H2(g) + 2 H2O(l)"),
    ("Sodium reacts with water", "2 Na(s) + 2 H2O(l)", "2 Na^+(aq) + 2 OH^-(aq) + H2(g)")]:
    add("redox/metal-displacement", title, left, right,
        "Aqueous net ionic equation; exposed metal surface, no passivating layer or competing oxidant. Acid cases assume dilute nonoxidizing acid.")
for title, left, right, condition in [
    ("Permanganate oxidizes iron(II)", "MnO4^-(aq) + 5 Fe^2+(aq) + 8 H^+(aq)", "Mn^2+(aq) + 5 Fe^3+(aq) + 4 H2O(l)", "Strongly acidic aqueous solution; H^+ denotes solvated proton convention."),
    ("Dichromate oxidizes iron(II)", "Cr2O7^2-(aq) + 6 Fe^2+(aq) + 14 H^+(aq)", "2 Cr^3+(aq) + 6 Fe^3+(aq) + 7 H2O(l)", "Strongly acidic aqueous solution; H^+ denotes solvated proton convention."),
    ("Cerium(IV) oxidizes iron(II)", "Ce^4+(aq) + Fe^2+(aq)", "Ce^3+(aq) + Fe^3+(aq)", "Acidic aqueous solution maintaining cerium(IV) in solution; no competing reductant."),
    ("Chlorine oxidizes bromide", "Cl2(aq) + 2 Br^-(aq)", "2 Cl^-(aq) + Br2(aq)", "Aqueous halogen displacement; bromide is available in excess and strongly alkaline conditions are excluded."),
    ("Iodine oxidizes thiosulfate", "I2(aq) + 2 S2O3^2-(aq)", "2 I^-(aq) + S4O6^2-(aq)", "Near-neutral aqueous iodometric conditions; tetrathionate is the oxidation product."),
    ("Permanganate oxidizes oxalate", "2 MnO4^-(aq) + 5 C2O4^2-(aq) + 16 H^+(aq)", "2 Mn^2+(aq) + 10 CO2(g) + 8 H2O(l)", "Warm strongly acidic aqueous solution; carbon dioxide escapes. Reaction can be slow initially.")]:
    add("redox/aqueous-electron-transfer", title, left, right, condition)
for title, left, right, condition in [
    ("Calcium carbonate decomposition", "CaCO3(s)", "CaO(s) + CO2(g)", "Strong heating with low carbon-dioxide partial pressure; CO2 removal favors decomposition."),
    ("Magnesium carbonate decomposition", "MgCO3(s)", "MgO(s) + CO2(g)", "Heating with carbon-dioxide removal; anhydrous carbonate stoichiometry."),
    ("Sodium bicarbonate decomposition", "2 NaHCO3(s)", "Na2CO3(s) + CO2(g) + H2O(g)", "Heating; water and carbon dioxide leave as gases."),
    ("Potassium chlorate decomposition", "2 KClO3(s)", "2 KCl(s) + 3 O2(g)", "Heating, commonly with manganese dioxide catalyst; ideal overall decomposition equation."),
    ("Silver(I) oxide decomposition", "2 Ag2O(s)", "4 Ag(s) + O2(g)", "Heating above the decomposition temperature; oxygen escapes.")]:
    add("inorganic/thermal-decomposition", title, left, right, condition)
for title, left, right, condition in [
    ("Ammonia synthesis", "N2(g) + 3 H2(g)", "2 NH3(g)", "Haber equilibrium at elevated pressure and temperature with an iron-based catalyst."),
    ("Sulfur trioxide formation", "2 SO2(g) + O2(g)", "2 SO3(g)", "Contact-process gas equilibrium with a vanadium oxide catalyst; it accelerates approach to equilibrium."),
    ("Dinitrogen tetroxide dissociation", "N2O4(g)", "2 NO2(g)", "Gas phase; increasing temperature favors dissociation to nitrogen dioxide."),
    ("Hydrogen iodide formation", "H2(g) + I2(g)", "2 HI(g)", "Heated gas mixture, above iodine's condensation temperature for the stated gas phases."),
    ("Water-gas shift", "CO(g) + H2O(g)", "CO2(g) + H2(g)", "Steam and carbon monoxide over an appropriate shift catalyst; gas-phase equilibrium."),
    ("Methane steam reforming", "CH4(g) + H2O(g)", "CO(g) + 3 H2(g)", "High-temperature nickel-catalyzed gas equilibrium; this is the reforming reaction, excluding downstream shift.")]:
    add("inorganic/gas-equilibria", title, left, right, condition, True)


def literal(value):
    return json.dumps(value, ensure_ascii=False)


FUELS = [
    ("methane", "CH4"), ("ethane", "C2H6"), ("propane", "C3H8"),
    ("butane", "C4H10"), ("pentane", "C5H12"), ("hexane", "C6H14"),
    ("heptane", "C7H16"), ("octane", "C8H18"), ("ethene", "C2H4"),
    ("propene", "C3H6"), ("ethyne", "C2H2"), ("propyne", "C3H4"),
    ("cyclopropane", "C3H6"), ("cyclohexane", "C6H12"), ("benzene", "C6H6"),
    ("toluene", "C7H8"), ("methanol", "CH4O"), ("ethanol", "C2H6O"),
    ("1-propanol", "C3H8O"), ("1-butanol", "C4H10O"),
    ("1,2-ethanediol", "C2H6O2"), ("propanone", "C3H6O"),
    ("dimethyl ether", "C2H6O"), ("diethyl ether", "C4H10O"),
    ("methanal", "CH2O"), ("ethanal", "C2H4O"),
    ("methanoic acid", "CH2O2"), ("ethanoic acid", "C2H4O2"),
]

# This catalog was retained from the original combustion generator. Balance
# follows C/H/O conservation, but applicability requires these real fuel names.
import math
import re
GAS_FUELS = {"methane", "ethane", "propane", "butane", "ethene", "propene",
    "ethyne", "propyne", "cyclopropane", "dimethyl ether", "methanal", "ethanal"}
for name, formula in FUELS:
    atoms = {e: int(n or 1) for e, n in re.findall(r"([A-Z][a-z]?)(\d*)", formula)}
    c, h, o = atoms["C"], atoms["H"], atoms.get("O", 0)
    coefficients = [4, 4*c+h-2*o, 4*c, 2*h]
    divisor = math.gcd(*coefficients)
    fuel, oxygen, carbon, water = [n // divisor for n in coefficients]
    def term(n, species):
        return (str(n) + " " if n != 1 else "") + species
    phase = "g" if name in GAS_FUELS else "l"
    add("combustion/" + ("oxygenated-fuels" if o else "hydrocarbons"),
        "Complete combustion of " + name,
        term(fuel, formula + "(" + phase + ")") + " + " + term(oxygen, "O2(g)"),
        term(carbon, "CO2(g)") + " + " + term(water, "H2O(g)"),
        "Ideal complete combustion with sufficient oxygen and ignition; high-temperature products are CO2(g) and H2O(g). Fuel inlet phase is stated; condensed water is excluded.")


def main():
    support = (ROOT / "reaction_support.rs").read_text(encoding="utf-8")
    for category, rows in CATALOG.items():
        leaf = ROOT / "reactions" / category
        leaf.mkdir(parents=True, exist_ok=True)
        constants = []
        for title, left, right, conditions, equilibrium in rows:
            def terms(data):
                return "&[" + ", ".join(f"({n}, {literal(s)})" for n, s in data) + "]"
            constants.append(f"Reaction {{ title: {literal(title)}, left: {terms(left)}, right: {terms(right)}, conditions: {literal(conditions)}, equilibrium: {str(equilibrium).lower()} }}")
        source = "// Generated by chemistry/generate_reactions.py; edit the source catalog.\nmod support;\nuse rand::rngs::StdRng;\nuse support::Reaction;\n\nconst REACTIONS: &[Reaction] = &[\n" + ",\n".join(constants) + "\n];\n\npub fn generate(rng: &mut StdRng) -> String { support::generate(rng, REACTIONS) }\n"
        (leaf / "generator.rs").write_text(source, encoding="utf-8")
        (leaf / "support.rs").write_text(support, encoding="utf-8")
    print(f"Generated {sum(map(len, CATALOG.values()))} established reactions in {len(CATALOG)} leaves.")


if __name__ == "__main__":
    main()
