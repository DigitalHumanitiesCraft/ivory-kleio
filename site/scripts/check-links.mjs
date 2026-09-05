import { readFile, readdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { parse } from 'parse5';

const dist = fileURLToPath(new URL('../dist/', import.meta.url));
const base = '/ivory-kleio/';
const origin = 'https://digitalhumanitiescraft.github.io';
const errors = [];
async function walk(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const nested = await Promise.all(entries.map((entry) => entry.isDirectory()
    ? walk(path.join(directory, entry.name)) : [path.join(directory, entry.name)]));
  return nested.flat();
}
const files = await walk(dist);
const inventory = new Set(files);
const pages = new Map();
for (const file of files.filter((name) => name.endsWith('.html'))) {
  const html = await readFile(file, 'utf8');
  const page = { ids: new Set(), links: [], h1: 0, title: false, main: false };
  function visit(node) {
    const attrs = Object.fromEntries((node.attrs || []).map(({ name, value }) => [name, value]));
    if (attrs.id) page.ids.add(attrs.id);
    if (node.tagName === 'h1') page.h1++;
    if (node.tagName === 'title') page.title = true;
    if (node.tagName === 'main') page.main = true;
    for (const attr of ['href', 'src']) if (attrs[attr]) page.links.push(attrs[attr]);
    for (const child of node.childNodes || []) visit(child);
  }
  visit(parse(html));
  if (page.h1 !== 1 || !page.title || !page.main) errors.push(file + ': missing or duplicate page landmarks');
  pages.set(file, page);
}
let checked = 0;
for (const [file, page] of pages) {
  const relative = path.relative(dist, file).split(path.sep).join('/');
  const pageURL = new URL(base + relative.replace(/index\.html$/, ''), origin);
  for (const href of page.links) {
    const url = new URL(href, pageURL);
    if (url.origin !== origin) continue;
    if (!url.pathname.startsWith(base)) { errors.push(relative + ': missing repository base in ' + href); continue; }
    const local = decodeURIComponent(url.pathname.slice(base.length));
    const target = path.join(dist, local.endsWith('/') || !local ? local + 'index.html' : local);
    checked++;
    if (!inventory.has(target)) errors.push(relative + ': missing ' + href);
    else if (url.hash && pages.has(target) && !pages.get(target).ids.has(decodeURIComponent(url.hash.slice(1)))) {
      errors.push(relative + ': missing anchor in ' + href);
    }
  }
}
if (errors.length) {
  console.error(errors.join('\n'));
  process.exitCode = 1;
} else {
  console.log('Verified ' + pages.size + ' built pages and ' + checked + ' local links and assets.');
}
