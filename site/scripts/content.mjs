import path from 'node:path';
import { base, documents, repository } from './content-map.mjs';

export function rewriteTarget(target, source) {
  if (/^(?:[a-z][a-z\d+.-]*:|\/\/|#)/i.test(target)) return target;
  const [file, fragment] = target.split('#');
  const resolved = path.posix.normalize(path.posix.join(path.posix.dirname(source), file));
  if (resolved.startsWith('../') || resolved.startsWith('/')) {
    throw new Error('A content link leaves the repository: ' + target);
  }
  const suffix = fragment ? '#' + fragment : '';
  if (resolved === 'research-artefacts/corpus.csv') return base + '/generated/sources/' + suffix;
  const doc = documents.find((entry) => entry.source === resolved);
  return doc
    ? base + '/generated/' + doc.slug + '/' + suffix
    : repository + '/blob/main/' + resolved + suffix;
}

export function renderDocument(text, document) {
  const body = text.replace(/^---\r?\n[\s\S]*?\r?\n---\r?\n/, '')
    .replace(/^\s*# [^\r\n]+\r?\n/, '')
    .replace(/(\[[^\]\n]*\]\()([^\s)]+)(\))/g,
      (_, start, target, end) => start + rewriteTarget(target, document.source) + end)
    .trim();
  return [
    '---',
    'title: ' + JSON.stringify(document.title),
    'description: ' + JSON.stringify(document.description),
    'editUrl: ' + repository + '/edit/main/' + document.source,
    '---',
    '',
    '> Maintained in the project repository. [View canonical source](' + repository + '/blob/main/' + document.source + ').',
    '',
    body,
    '',
  ].join('\n');
}
