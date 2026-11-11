// Migrates packs-src/core handwritten reference files to the v2 document
// model (docs/PACK-SPEC.md): ONE document per category group under
// reference/networking/ — files are grouped by their frontmatter `category`
// (reference/networking/<group>), NOT by folder (the fundamentals/ folder
// feeds the fundamentals, security, services, switching and transport
// groups). Each source file becomes a `##` section (its title) carrying its
// own tags via a section-tags marker; its body is preserved with headings
// demoted by two levels (fenced code untouched) and its `version` kept as a
// line under the marker.
//
// Usage:
//   node scripts/migrate_core_reference.mjs             # dry-run plan
//   node scripts/migrate_core_reference.mjs --write     # write merged docs
//   node scripts/migrate_core_reference.mjs --write --delete-old
//                                                       # + delete sources
// Re-running after migration is a no-op: sources are gone and the merged
// docs (doc/ ids) are recognised and skipped.
import { existsSync, readdirSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";

const ROOT = process.cwd();
const REF = join(ROOT, "packs-src", "core", "content", "reference");
const WRITE = process.argv.includes("--write");
const DELETE_OLD = process.argv.includes("--delete-old");

function parseFrontmatter(text) {
  const m = /^---\n([\s\S]*?)\n---\n?/.exec(text);
  if (!m) return { fm: {}, body: text };
  const fm = {};
  for (const line of m[1].split("\n")) {
    const kv = /^(\w+)\s*=\s*(.+)$/.exec(line.trim());
    if (!kv) continue;
    const v = kv[2].trim();
    fm[kv[1]] = v.startsWith("[")
      ? [...v.matchAll(/"([^"]*)"/g)].map((x) => x[1])
      : v.replace(/^"|"$/g, "");
  }
  return { fm, body: text.slice(m[0].length) };
}

function demote(body) {
  let fence = false;
  return body
    .split("\n")
    .map((line) => {
      if (line.startsWith("```")) fence = !fence;
      if (fence) return line;
      const h = /^(#{1,6})(\s+.*)$/.exec(line);
      return h ? "#".repeat(Math.min(6, h[1].length + 2)) + h[2] : line;
    })
    .join("\n");
}

const RISK_RANK = { info: 0, low: 1, medium: 2, high: 3 };
const cap = (s) => s[0].toUpperCase() + s.slice(1);

// Collect sources: every .md under reference/<folder>/ that is not already
// a merged v2 document (merged docs sit directly in reference/ and have
// doc/ ids).
const sources = [];
for (const e of readdirSync(REF, { withFileTypes: true })) {
  if (!e.isDirectory()) continue;
  const dir = join(REF, e.name);
  for (const n of readdirSync(dir).filter((x) => x.endsWith(".md")).sort()) {
    const path = join(dir, n);
    const { fm, body } = parseFrontmatter(readFileSync(path, "utf8"));
    if ((fm.id ?? "").startsWith("doc/")) continue;
    sources.push({ name: n, path, fm, body });
  }
}

const groups = new Map();
for (const s of sources) {
  const group = (s.fm.category ?? "").split("/").pop();
  if (!group) {
    console.log(`SKIP ${s.name}: no category`);
    continue;
  }
  if (!groups.has(group)) groups.set(group, []);
  groups.get(group).push(s);
}

let failures = 0;
for (const [group, files] of [...groups.entries()].sort()) {
  files.sort((a, b) => (a.fm.title ?? "").localeCompare(b.fm.title ?? ""));
  const tags = [];
  for (const f of files) for (const t of f.fm.tags ?? []) if (!tags.includes(t)) tags.push(t);
  if (!tags.includes(group)) tags.push(group);
  if (!tags.includes("model-doc")) tags.push("model-doc");
  // Dominant kind: most frequent; ties keep first-seen order.
  const kindCount = new Map();
  for (const f of files) kindCount.set(f.fm.kind, (kindCount.get(f.fm.kind) ?? 0) + 1);
  const kind = [...kindCount.entries()].sort((a, b) => b[1] - a[1])[0][0];
  const risks = files.map((f) => f.fm.risk).filter(Boolean);
  const risk = risks.length
    ? risks.sort((a, b) => RISK_RANK[b] - RISK_RANK[a])[0]
    : null;

  const title = `Networking: ${cap(group)}`;
  let out =
    `---\nid = "doc/networking/${group}/${group}"\nkind = "${kind}"\ntitle = "${title}"\n` +
    `category = "reference/networking/${group}"\n` +
    `tags = [${tags.map((t) => `"${t}"`).join(", ")}]\n` +
    (risk ? `risk = "${risk}"\n` : "") +
    `---\nThe complete ${title} document: every topic for this area in one\n` +
    `file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").\n` +
    `Content is reorganised, unchanged, from this pack's original per-topic\n` +
    `files; each topic is a section carrying its own tags via a section-tags\n` +
    `marker.`;

  for (const f of files) {
    out += `\n\n## ${f.fm.title}\n<!-- section-tags: ${(f.fm.tags ?? []).join(", ")} -->\n`;
    if (f.fm.version) out += `\nVersion: ${f.fm.version}\n`;
    out += `\n${demote(f.body.trim())}\n`;
    if (!out.includes(f.fm.title)) failures++;
  }
  out += "\n";

  const target = join(REF, `${group}.md`);
  console.log(`${group}: ${files.length} files → reference/${group}.md (${kind})`);
  if (WRITE) {
    if (existsSync(target)) {
      // Already generated by a previous --write run of this script: only
      // proceed (e.g. to --delete-old) when the content is byte-identical.
      if (readFileSync(target, "utf8") !== out) {
        console.log(`  REFUSING to overwrite differing existing ${group}.md`);
        failures++;
        continue;
      }
    } else {
      writeFileSync(target, out);
    }
    if (DELETE_OLD) for (const f of files) rmSync(f.path);
  }
}
console.log(`failures: ${failures}`);
process.exit(failures ? 1 : 0);
