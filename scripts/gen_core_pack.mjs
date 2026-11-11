// Generates the core pack's command DOCUMENTS from src/cheatsheets/*.md
// (v2 document model — docs/PACK-SPEC.md, "Document model (v2 draft)").
//
// One document per cheatsheet source: every `##` section of the cheatsheet
// becomes a `##` section of the document (carrying its own tags via a
// section-tags marker), and every table row becomes a `### <command>` entry
// inside its section — description, the command in a fenced block, and its
// computed risk. Prose-only sections (no table rows — e.g. the Splunk
// pattern sections and the Windows "Notes") have no handwritten core
// counterpart, so their text is included verbatim as a section too.
//
// Usage: node scripts/gen_core_pack.mjs
// Idempotent: wipes packs-src/core/content/commands/ and regenerates.
import { readFileSync, writeFileSync, mkdirSync, rmSync } from "node:fs";
import { join } from "node:path";

const ROOT = process.cwd();
const OUT = join(ROOT, "packs-src", "core", "content", "commands");

const SOURCES = [
  {
    file: "src/cheatsheets/linux.md",
    os: "linux",
    id: "doc/os/linux/linux",
    title: "Linux Commands",
    category: "reference/os/linux",
    out: "linux.md",
    fence: "bash",
  },
  {
    file: "src/cheatsheets/windows.md",
    os: "windows",
    id: "doc/os/windows/windows",
    title: "Windows Commands",
    category: "reference/os/windows",
    out: "windows.md",
    fence: "powershell",
  },
  {
    file: "src/cheatsheets/splunk.md",
    os: "splunk",
    id: "doc/monitoring/splunk/splunk",
    title: "Splunk SPL",
    category: "reference/monitoring/splunk",
    out: "splunk.md",
    fence: "spl",
  },
];

const HIGH_RISK = [
  /\brm\s+-rf?\b/, /\bdd\b/, /mkfs/, /shutdown/, /\breboot\b/, /kill\s+-9/,
  /taskkill\s+\/f/i, /stop-process\s+-force/i, /iptables\s+-f/i, /\bflush\b/i,
  /ipconfig\s+\/release/i, /remove-dir/i, /reset\b/i, /\btruncate\b/i,
  /delete\s+from/i, /drop\s+table/i, /:\(\)\{/, /\bformat\b/i,
];

const slugify = (s) =>
  s
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 56) || "x";

const tomlStr = (s) => `"${s.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"`;
const tomlList = (items) => `[${items.map(tomlStr).join(", ")}]`;

function riskFor(cmd, os) {
  if (HIGH_RISK.some((re) => re.test(cmd))) return "medium";
  if (os === "splunk") return "info";
  return "low";
}

const RISK_RANK = { info: 0, low: 1, medium: 2, high: 3 };

// Split a Markdown table row on unescaped pipes only (commands contain
// escaped `\|`, e.g. PowerShell pipelines `Get-NetAdapter \| ft …`), then
// unescape `\|` → `|` in each cell.
function splitRow(line) {
  const cells = [];
  let cur = "";
  for (let i = 0; i < line.length; i++) {
    const ch = line[i];
    if (ch === "\\" && line[i + 1] === "|") {
      cur += "|";
      i++;
    } else if (ch === "|") {
      cells.push(cur);
      cur = "";
    } else {
      cur += ch;
    }
  }
  cells.push(cur);
  // Drop the empty artefacts of the leading/trailing pipe.
  if (cells.length && cells[0].trim() === "") cells.shift();
  if (cells.length && cells[cells.length - 1].trim() === "") cells.pop();
  return cells.map((c) => c.trim());
}

function parseCheatsheet(md) {
  // Returns [{title, rows: [[command, description]], prose: [lines]}].
  // Row detection matches the v1 generator: header rows are skipped when
  // the first cell is "command"/"what it does"; a one-cell row whose cell
  // starts with a backticked span is a command fragment + description
  // (the `| eval-forward` note in the Splunk sheet).
  const lines = md.split("\n");
  const sections = [];
  let section = null;
  let inFence = false;
  for (const raw of lines) {
    const line = raw.trimEnd();
    const t = line.trim();
    if (t.startsWith("```")) {
      inFence = !inFence;
      if (section) section.prose.push(line);
      continue;
    }
    if (!inFence) {
      const h = /^##\s+(?!#)(.+)$/.exec(t);
      if (h) {
        section = { title: h[1].trim(), rows: [], prose: [] };
        sections.push(section);
        continue;
      }
    }
    if (!section) continue;
    if (!inFence && t.startsWith("|")) {
      if (/^\|[-| :]+\|$/.test(t)) continue; // separator
      const cells = splitRow(t);
      if (cells.length >= 2) {
        if (/^(command|what it does)/i.test(cells[0])) continue; // header
        section.rows.push([cells[0], cells.slice(1).join(" | ")]);
      } else if (cells.length === 1) {
        const m = /^(`[^`]*`)\s+(.*)$/.exec(cells[0]);
        if (m) section.rows.push([m[1], m[2]]);
        else section.prose.push(line);
      }
      continue;
    }
    if (t !== "") section.prose.push(line);
  }
  for (const s of sections) {
    // Trim blank padding captured around fences/prose blocks.
    while (s.prose.length && s.prose[0].trim() === "") s.prose.shift();
    while (s.prose.length && s.prose[s.prose.length - 1].trim() === "") s.prose.pop();
  }
  return sections;
}

rmSync(OUT, { recursive: true, force: true });
mkdirSync(OUT, { recursive: true });

let totalEntries = 0;
for (const src of SOURCES) {
  const md = readFileSync(join(ROOT, src.file), "utf8");
  const sections = parseCheatsheet(md).filter((s) => s.rows.length || s.prose.length);

  const tags = [src.os];
  for (const s of sections) {
    const slug = slugify(s.title);
    if (!tags.includes(slug)) tags.push(slug);
  }
  tags.push("model-doc");

  let entries = 0;
  let docRisk = "info";
  const rendered = [];
  for (const sec of sections) {
    const secTags = [src.os, slugify(sec.title)];
    rendered.push(`## ${sec.title}\n<!-- section-tags: ${secTags.join(", ")} -->`);
    for (const [command, description] of sec.rows) {
      const plain = command.replace(/`/g, "").replace(/\s+/g, " ").trim();
      const risk = riskFor(plain, src.os);
      if (RISK_RANK[risk] > RISK_RANK[docRisk]) docRisk = risk;
      entries++;
      rendered.push(
        [
          `### ${plain}`,
          "",
          description,
          "",
          "```" + src.fence,
          plain,
          "```",
          "",
          `Risk: ${risk}`,
        ].join("\n"),
      );
    }
    if (sec.prose.length) rendered.push(sec.prose.join("\n"));
  }

  const fm = [
    `id = ${tomlStr(src.id)}`,
    `kind = "command"`,
    `title = ${tomlStr(src.title)}`,
    `category = ${tomlStr(src.category)}`,
    `tags = ${tomlList(tags)}`,
    `risk = ${tomlStr(docRisk)}`,
    `version = "core-0.3.0"`,
  ].join("\n");

  const intro =
    `The complete ${src.title} document: every command for this area in one\n` +
    `file (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").\n` +
    `Content is reorganised, unchanged, from ${src.file}; each topic is a\n` +
    `section carrying its own tags via a section-tags marker.`;

  writeFileSync(join(OUT, src.out), `---\n${fm}\n---\n${intro}\n\n\n${rendered.join("\n\n")}\n`);
  totalEntries += entries;
  console.log(
    `${src.out}: ${sections.length} sections, ${entries} command entries`,
  );
}
console.log(`generated 3 command documents (${totalEntries} entries) -> packs-src/core/content/commands/`);
