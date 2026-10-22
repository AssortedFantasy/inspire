"""Source replay and literary-unit checks; run with python -B."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from bs4 import BeautifulSoup
import import_data as importer


class LiteratureTests(unittest.TestCase):
    def test_layout_cleanup_keeps_inline_words_and_verse_lines(self):
        p = BeautifulSoup('<p><span class="dropcap">A</span>lice <i>was</i> here<span class="pagenum">[7]</span>.</p>', 'html.parser').p
        self.assertEqual(importer.clean(p), 'Alice was here.')
        self.assertEqual(importer.clean(BeautifulSoup('<p>One<br/>Two</p>', 'html.parser').p, True), 'One\nTwo')

    def test_source_tampering_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            (Path(directory) / 'pg21.html').write_text('changed source', encoding='utf-8')
            with self.assertRaisesRegex(ValueError, 'checksum mismatch: 21'):
                importer.build(Path(directory))

    def test_gulistan_keeps_whole_numbered_units_after_introduction(self):
        soup = BeautifulSoup(
            '<h2>INTRODUCTION</h2><p>Not the author.</p><h4>I</h4><p>Editor.</p>'
            '<h2>CHAPTER IV</h2><h4 id="tale-1">I</h4><p>' + 'First paragraph. ' * 8 +
            '</p><p>A separate concluding verse.</p><h4 id="tale-2">II</h4><p>' +
            'Too long. ' * 250 + '</p><h4 id="tale-3">III</h4><p>' +
            'Missing text. ' * 10 + ' * * *</p>', 'html.parser')
        source = {'id': 1, 'path': 'test', 'author': 'Writer', 'title': 'Book',
                  'edition': '1900', 'url': 'source', 'sha256': 'checksum', 'rights': 'Public domain.'}
        rows = list(importer.gulistan(source, soup))
        self.assertEqual(len(rows), 1)
        body = importer.body_text(rows[0][1]['text'])
        self.assertEqual(body, ('First paragraph. ' * 8).strip() + '\n\nA separate concluding verse.')

    def test_offline_replay_and_complete_units(self):
        outputs = importer.build(Path('target/imports/literature'))
        for path, rows in outputs.items():
            self.assertEqual(rows, json.loads((importer.ROOT / path / 'records.json').read_text(encoding='utf-8')))
        quatrains = outputs['verse/translations/persian/khayyam/fitzgerald']
        self.assertEqual(len(quatrains), 75)
        self.assertTrue(all(len(importer.body_text(row['text']).splitlines()) == 4 for row in quatrains))
        self.assertEqual(len(outputs['verse/translations/indic/tagore/gitanjali']), 103)
        self.assertEqual(len(outputs['narrative/anecdotes/persian/sadi/gulistan']), 191)
        for rows in outputs.values():
            for row in rows:
                body = importer.body_text(row['text'])
                self.assertNotIn('Project Gutenberg', body)
                self.assertNotIn('\x00', body)
                self.assertNotIn('Illustration:', body)

    def test_schema_unique_ids_and_baseline_retention(self):
        all_rows = []
        for path in importer.ROOT.rglob('records.json'):
            rows = json.loads(path.read_text(encoding='utf-8'))
            self.assertTrue(rows, path)
            self.assertEqual(len({r['id'] for r in rows}), len(rows), path)
            for row in rows:
                self.assertEqual(set(row), {'id', 'text', 'provenance'})
                self.assertTrue(all(isinstance(v, str) and v.strip() for v in row.values()))
                self.assertFalse(row['text'].endswith('\n'))
            all_rows.extend(rows)
        baseline = json.loads((importer.ROOT / 'baseline_manifest.json').read_text(encoding='utf-8'))
        self.assertEqual(len({importer.body_key(r['text']) for r in all_rows}), len(all_rows))
        current = {r['id']: r for r in all_rows}
        for old in baseline:
            self.assertIn(old['id'], current)
            canonical = json.dumps(current[old['id']], ensure_ascii=False, sort_keys=True).encode()
            self.assertEqual(hashlib.sha256(canonical).hexdigest(), old['sha256'])


if __name__ == '__main__':
    unittest.main()
