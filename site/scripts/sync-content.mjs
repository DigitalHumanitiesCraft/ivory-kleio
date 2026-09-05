import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { parse } from 'csv-parse/sync';
import { base, documents, repository } from './content-map.mjs';
import { renderDocument } from './content.mjs';

const site = fileURLToPath(new URL('../', import.meta.url));
const root = path.resolve(site, '..');
const destination = path.join(site, 'src/content/docs/generated');
await mkdir(destination, { recursive: true });

for (const document of documents) {
  const text = await readFile(path.join(root, document.source), 'utf8');
  await writeFile(path.join(destination, document.slug + '.md'), renderDocument(text, document));
}
const rows = parse(await readFile(path.join(root, 'research-artefacts/corpus.csv'), 'utf8'), { columns: true, skip_empty_lines: true });
const statuses = { identified: 'Identified', partial: 'Partial reading', secondary_only: 'Secondary account only', fully_analyzed: 'Fully analysed' };
const lines = [
  '---',
  'title: Sources and reading',
  'description: The primary corpus, consulted representations and recorded reading states.',
  'editUrl: ' + repository + '/edit/main/research-artefacts/corpus.csv',
  '---',
  '',
  'The source inventory is a preliminary bibliography. Reading states describe the evidence recorded by the project. Complete reconstruction of the relevant corpus remains research work.',
  '',
  '[View the canonical inventory](' + repository + '/blob/main/research-artefacts/corpus.csv).',
  '',
  '## Reading states',
  '',
  '**Identified** means a work has been located. **Partial reading** means selected passages have been consulted. **Secondary account only** means direct analysis remains pending. **Fully analysed** requires complete relevant reading and an integrated account of the implications.',
  '',
  '## Corpus',
  '',
];
for (const row of rows) {
  if (!(row.reading_status in statuses)) throw new Error('Unknown reading status for ' + row.source_id);
  const note = documents.find((doc) => doc.source === 'research-artefacts/' + row.reading_note);
  if (!note) throw new Error('Unmapped reading note for ' + row.source_id);
  lines.push(
    '### ' + row.title,
    '',
    row.authors + ' · ' + row.publication_year,
    '',
    '**' + statuses[row.reading_status] + '**',
    '',
    row.representation + '. ' + row.locators + '.',
    '',
    row.relevance + '.',
    '',
    '[Publication](' + row.original_url + ')' +
      (row.consulted_url && row.consulted_url !== row.original_url ? ' · [Consulted representation](' + row.consulted_url + ')' : '') +
      ' · [Reading record](' + base + '/generated/' + note.slug + '/)',
    '',
  );
}
lines.push('## Method and reuse', '', '[Research methodology](' + base + '/generated/methodology/) defines the reading procedure. [Source acquisition](' + base + '/generated/source-material/) records representation and reuse conventions. Public access does not itself establish permission to redistribute a source file.', '');
await writeFile(path.join(destination, 'sources.md'), lines.join('\n'));
console.log('Canonical project documents and corpus synchronised.');
