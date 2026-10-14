"""Import neutral unmodified aldoses from pinned PubChem responses."""
import argparse
import hashlib
import json
import re
import time
import urllib.parse
import urllib.request
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent
REPO = next(parent for parent in ROOT.parents if (parent / 'Cargo.toml').exists())
CACHE = REPO / 'target' / 'chemistry-sources'
BASE = 'https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/'
PROPS = 'MolecularFormula,MolecularWeight,IUPACName,SMILES,Charge'
GROUPS = {
    'trioses': (3, ['glyceraldehyde']),
    'tetroses': (4, ['erythrose', 'threose']),
    'pentoses': (5, ['ribose', 'arabinose', 'xylose', 'lyxose']),
    'hexoses': (6, ['allose', 'altrose', 'glucose', 'mannose', 'gulose', 'idose', 'galactose', 'talose']),
}
LINEAR_D = {
    'glyceraldehyde': 'R', 'erythrose': 'RR', 'threose': 'SR',
    'ribose': 'RRR', 'arabinose': 'SRR', 'xylose': 'RSR', 'lyxose': 'SSR',
    'allose': 'RRRR', 'altrose': 'SRRR', 'glucose': 'RSRR', 'mannose': 'SSRR',
    'gulose': 'RRSR', 'idose': 'SRSR', 'galactose': 'RSSR', 'talose': 'SSSR',
}
CYCLIC_D = {
    'ribose': 'RRR', 'arabinose': 'SRR', 'xylose': 'RSR', 'lyxose': 'SSR',
    'allose': 'RRSR', 'altrose': 'SRSR', 'glucose': 'RSSR', 'mannose': 'SSSR',
    'gulose': 'RRRR', 'idose': 'SRRR', 'galactose': 'RSRR', 'talose': 'SSRR',
}


def structure_inventory(smiles):
    """Independent valence inventory for this restricted neutral C/O SMILES set.

    This is deliberately not a general SMILES toolkit. Reject unsupported tokens.
    Rings and branches contribute bonds; bare atoms receive valence hydrogens.
    """
    tokens = re.findall(r'\[C@@?H\]|[CO()]|=|[1-9]', smiles)
    assert ''.join(tokens) == smiles, ('Unsupported SMILES', smiles)
    atoms, bonds, branches, rings = [], [], [], {}
    current, order, closures = None, 1, 0

    def bond(a, b, count):
        bonds[a] += count
        bonds[b] += count

    for token in tokens:
        if token == '(':
            branches.append(current)
        elif token == ')':
            current = branches.pop()
        elif token == '=':
            order = 2
        elif token.isdigit():
            if token in rings:
                bond(current, rings.pop(token), order)
                closures += 1
            else:
                rings[token] = current
            order = 1
        else:
            atoms.append(token)
            bonds.append(0)
            next_atom = len(atoms) - 1
            if current is not None:
                bond(current, next_atom, order)
            current, order = next_atom, 1
    assert not rings and not branches and order == 1
    inventory = Counter()
    for token, valence in zip(atoms, bonds, strict=True):
        element = 'O' if token == 'O' else 'C'
        hydrogens = (2 if element == 'O' else 4) - valence
        assert hydrogens >= 0
        if token.startswith('['):
            assert hydrogens == 1, token
        inventory[element] += 1
        inventory['H'] += hydrogens
    return dict(inventory), closures


def validate(entry):
    data, carbon, label = entry['data'], entry['carbon'], entry['name']
    assert data['MolecularFormula'] == f'C{carbon}H{2 * carbon}O{carbon}'
    assert data['Charge'] == 0
    assert abs(float(data['MolecularWeight']) - carbon * 30.026) < 0.015
    inventory, closures = structure_inventory(data['SMILES'])
    assert inventory == {'C': carbon, 'H': 2 * carbon, 'O': carbon}
    assert data['SMILES'].count('=') == (1 if closures == 0 else 0)
    assert closures in [0, 1]
    cyclic = closures == 1
    assert ('oxane' in data['IUPACName']) == cyclic
    config, name = label.split('-', 1)
    positions = re.findall(r'(\d+)([RS])', data['IUPACName'].split(')')[0])
    expected = (CYCLIC_D if cyclic else LINEAR_D)[name]
    if config == 'L':
        expected = expected.translate(str.maketrans('RS', 'SR'))
    start = 3 if cyclic else 2
    assert positions == [(str(start + i), stereo) for i, stereo in enumerate(expected)], (label, positions, expected)
    assert len(re.findall(r'\[C@@?H\]', data['SMILES'])) == carbon - 2
    return cyclic


def records(selected, manifest):
    grouped = defaultdict(list)
    for entry in selected:
        cyclic = validate(entry)
        data, carbon, label = entry['data'], entry['carbon'], entry['name']
        category = entry['group']
        if carbon >= 5:
            category += '/pyranoses' if cyclic else '/open-chain'
        form = 'Pyranose (six-membered cyclic hemiacetal); anomeric configuration unspecified.' if cyclic else 'Open-chain aldose; terminal aldehyde.'
        text = (
            f"{label}\nRepresentation: {form}\n"
            f"Molecular formula: {data['MolecularFormula']}; empirical formula: CH2O.\n"
            f"Molar mass (calculated): {data['MolecularWeight']} g/mol; formal charge: 0.\n"
            f"Atoms per molecule: C: {carbon}, H: {2 * carbon}, O: {carbon}.\n"
            f"Systematic name: {data['IUPACName']}\n"
            f"Stereochemical SMILES: {data['SMILES']}\n"
            'D/L labels specify configuration; optical-rotation sign is a separate property. '
            + ('Aqueous aldoses also form cyclic hemiacetals.' if not cyclic else 'Open-chain and other cyclic forms also occur in aqueous solution.')
        )
        provenance = (
            f"Selected PubChem factual identity/computed properties; {manifest[entry['key']]['url']}; CID {data['CID']}; "
            'original formatting and aldose classification. NCBI policy https://www.ncbi.nlm.nih.gov/home/about/policies/; '
            'NCBI/NLM credit; contributed material may have separate rights. No third-party descriptive prose imported. '
            'See chemistry/LICENSE.md and aldoses/source_manifest.json.'
        )
        grouped[category].append({'id': f"pubchem-cid-{data['CID']}", 'text': text, 'provenance': provenance})
    rows = [row for group in grouped.values() for row in group]
    assert len(rows) == len({row['id'] for row in rows}) == len({row['text'] for row in rows}) == 30
    assert all(set(row) == {'id', 'text', 'provenance'} and all(row.values()) and '\0' not in row['text'] for row in rows)
    for category, rows in grouped.items():
        write(ROOT / category / 'records.json', sorted(rows, key=lambda row: row['text']))
    return grouped


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def fetch(key, url, download, previous, manifest):
    path = CACHE / (key + '.json')
    if download and not path.exists():
        time.sleep(0.5)
        with urllib.request.urlopen(url, timeout=60) as response:
            path.write_bytes(response.read())
    payload = path.read_bytes()
    digest = hashlib.sha256(payload).hexdigest()
    if key in previous:
        assert previous[key] == {'url': url, 'sha256': digest}, ('Source changed', key)
    elif not download:
        raise ValueError('Unpinned offline source: ' + key)
    manifest[key] = {'url': url, 'sha256': digest}
    props = json.loads(payload)['PropertyTable']['Properties']
    assert len(props) == 1, (key, 'ambiguous identity')
    return props[0]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--download', action='store_true')
    args = parser.parse_args()
    manifest_path = ROOT / 'source_manifest.json'
    previous = json.loads(manifest_path.read_text()) if manifest_path.exists() else {}
    manifest = {}
    selected = []
    for group, (carbon, names) in GROUPS.items():
        for name in names:
            for config in ['D', 'L']:
                label = config + '-' + name
                if label in ['D-glucose', 'D-ribose']:
                    continue
                query = label
                key = 'monosaccharides-aldoses-' + query.lower()
                url = BASE + 'name/' + urllib.parse.quote(query, safe='') + '/property/' + PROPS + '/JSON'
                data = fetch(key, url, args.download, previous, manifest)
                selected.append({'group': group, 'carbon': carbon, 'name': label, 'key': key, 'data': data})
    for group, carbon, label, key in [('hexoses', 6, 'D-glucose', 'compound-d-glucose'), ('pentoses', 5, 'D-ribose', 'compound-d-ribose')]:
        url = previous[key]['url']
        data = fetch(key, url, args.download, previous, manifest)
        selected.append({'group': group, 'carbon': carbon, 'name': label, 'key': key, 'data': data})
    write(CACHE / 'monosaccharides-aldoses-selected.json', selected)
    write(manifest_path, manifest)
    grouped = records(selected, manifest)
    (ROOT / 'records.json').unlink(missing_ok=True)
    print(f'Imported {len(selected)} aldoses in {len(grouped)} leaves; formula, valence, mass and stereoconfiguration checks passed.')


if __name__ == '__main__':
    main()
