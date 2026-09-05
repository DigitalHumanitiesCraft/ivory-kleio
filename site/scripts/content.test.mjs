import test from 'node:test';
import assert from 'node:assert/strict';
import { renderDocument, rewriteTarget } from './content.mjs';

test('canonical cross-references remain within the published specification', () => {
  assert.equal(rewriteTarget('specification.md#minimum-prototype', 'knowledge/project.md'),
    '/ivory-kleio/generated/specification/#minimum-prototype');
  assert.equal(rewriteTarget('../research-artefacts/corpus.csv', 'knowledge/project.md'),
    '/ivory-kleio/generated/sources/');
  assert.equal(rewriteTarget('#terms', 'knowledge/INDEX.md'), '#terms');
  assert.equal(rewriteTarget('https://example.org/paper#section', 'knowledge/project.md'),
    'https://example.org/paper#section');
});

test('unpublished material links back to the repository and cannot escape it', () => {
  assert.equal(rewriteTarget('plan.md', 'knowledge/project.md'),
    'https://github.com/DigitalHumanitiesCraft/ivory-kleio/blob/main/knowledge/plan.md');
  assert.throws(() => rewriteTarget('../../private.md', 'knowledge/project.md'));
});

test('the public rendering retains the source text, with one title and traceable links', () => {
  const rendered = renderDocument('---\nstatus: draft\n---\n\n# Internal title\n\nEvidence remains uncertain. [Case](specification.md#requirements)\n', {
    title: 'Public title', description: 'A description.', source: 'knowledge/project.md',
  });
  assert.match(rendered, /title: "Public title"/);
  assert.match(rendered, /Evidence remains uncertain\./);
  assert.doesNotMatch(rendered, /# Internal title/);
  assert.match(rendered, /\/generated\/specification\/#requirements/);
  assert.match(rendered, /View canonical source/);
  assert.match(rendered, /edit\/main\/knowledge\/project.md/);
});
