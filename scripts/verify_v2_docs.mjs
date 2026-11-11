// Verifies every v2 model document in addons-src and packs-src/core
// (core coverage includes the generated command documents, the migrated
// reference and runbook documents, and the error documents — the errors
// keep their stable v1 `err/…` ids on purpose, for favorites/history):
//  - TOML frontmatter present with id / kind / title / category
//  - ids unique across ALL packs
//  - at least one `<!-- section-tags: … -->` marker
//  - category matches one of the v2 grammar shapes (docs/PACK-SPEC.md):
//      reference/vendors/<company>/<product>[/<model>[/<topic>…]]
//      reference/<area>[/<group>[/<topic>…]]
//      runbooks/<area>[/…]
// Usage: node scripts/verify_v2_docs.mjs
import { readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";

const ROOT = process.cwd();
const TREES = [join(ROOT, "addons-src"), join(ROOT, "packs-src", "core")];

function walk(dir) {
  const out = [];
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) out.push(...walk(p));
    else if (e.name.endsWith(".md")) out.push(p);
  }
  return out;
}

function parseFm(text) {
  const m = /^---\n([\s\S]*?)\n---\n?/.exec(text);
  if (!m) return null;
  const fm = {};
  for (const line of m[1].split("\n")) {
    const kv = /^(\w+)\s*=\s*(.+)$/.exec(line.trim());
    if (kv) fm[kv[1]] = kv[2].trim().replace(/^"|"$/g, "");
  }
  return fm;
}

function categoryOk(cat) {
  const seg = cat.split("/").filter(Boolean);
  if (seg[0] === "runbooks") return seg.length >= 2;
  if (seg[0] !== "reference") return false;
  if (seg[1] === "vendors") return seg.length >= 3; // company… (+product/model)
  return seg.length >= 2; // reference/<area>…
}

const ids = new Map();
let docs = 0;
let sections = 0;
const problems = [];
for (const tree of TREES) {
  for (const file of walk(tree)) {
    const text = readFileSync(file, "utf8");
    const fm = parseFm(text);
    if (!fm || !/^(doc|err)\//.test(fm.id ?? "")) continue;
    docs++;
    const rel = relative(ROOT, file);
    for (const field of ["id", "kind", "title", "category"])
      if (!fm[field]) problems.push(`${rel}: missing frontmatter field ${field}`);
    if (ids.has(fm.id)) problems.push(`${rel}: duplicate id ${fm.id} (also ${ids.get(fm.id)})`);
    ids.set(fm.id, rel);
    const markers = (text.match(/<!-- section-tags:/g) ?? []).length;
    if (markers < 1) problems.push(`${rel}: no section-tags marker`);
    sections += markers;
    if (fm.category && !categoryOk(fm.category))
      problems.push(`${rel}: category "${fm.category}" does not match v2 grammar`);
  }
}
console.log(`v2 documents: ${docs}, section-tags markers: ${sections}, unique ids: ${ids.size}`);
if (problems.length) {
  console.log("PROBLEMS:");
  for (const p of problems) console.log(" -", p);
  process.exit(1);
}
console.log("all checks passed");
