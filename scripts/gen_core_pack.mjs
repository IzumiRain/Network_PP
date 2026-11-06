// Migrates src/cheatsheets/*.md tables into core-pack command entities.
// Usage: node scripts/gen_core_pack.mjs
// Idempotent: wipes packs-src/core/content/commands/ and regenerates.
import { readFileSync, writeFileSync, mkdirSync, rmSync, readdirSync } from "node:fs";
import { join } from "node:path";

const ROOT = process.cwd();
const OUT = join(ROOT, "packs-src", "core", "content", "commands");

const SOURCES = [
  {
    file: "src/cheatsheets/linux.md",
    os: "linux",
    categoryBase: "reference/os/linux",
    prefix: "cmd/linux",
    domain: "reference/os/linux",
  },
  {
    file: "src/cheatsheets/windows.md",
    os: "windows",
    categoryBase: "reference/os/windows",
    prefix: "cmd/windows",
    domain: "reference/os/windows",
  },
  {
    file: "src/cheatsheets/splunk.md",
    os: "splunk",
    categoryBase: "reference/monitoring/splunk",
    prefix: "cmd/splunk",
    domain: "reference/monitoring/splunk",
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

function riskFor(cmd, os) {
  if (HIGH_RISK.some((re) => re.test(cmd))) return "medium";
  if (os === "splunk") return "info";
  return "low";
}

function parseTables(md) {
  // Returns [{section, rows: [[command, description]]}]
  const lines = md.split("\n");
  const sections = [];
  let section = null;
  let inTable = false;
  for (const line of lines) {
    const h = /^##\s+(?!#)(.+)$/.exec(line.trim());
    if (h) {
      section = { title: h[1].trim(), rows: [] };
      sections.push(section);
      inTable = false;
      continue;
    }
    if (!section) continue;
    const t = line.trim();
    if (t.startsWith("|")) {
      if (/^\|[-| :]+\|$/.test(t)) continue; // separator
      const cells = t
        .replace(/^\|/, "")
        .replace(/\|$/, "")
        .split("|")
        .map((c) => c.trim());
      if (cells.length >= 2 && !/^(command|what it does)/i.test(cells[0])) {
        section.rows.push([cells[0], cells[1]]);
        inTable = true;
      }
      continue;
    }
    if (inTable && t === "") inTable = false;
  }
  return sections;
}

const used = new Set();
let n = 0;

for (const src of SOURCES) {
  const md = readFileSync(join(ROOT, src.file), "utf8");
  const sections = parseTables(md);
  for (const sec of sections) {
    if (sec.rows.length === 0) continue; // prose sections are handwritten
    const category = `${src.categoryBase}/${slugify(sec.title)}`;
    for (const [command, description] of sec.rows) {
      const plain = command.replace(/`/g, "").replace(/\s+/g, " ").trim();
      const base = slugify(plain.replace(/[^\w\s-]/g, "").slice(0, 40) || sec.title);
      let id = `${src.prefix}/${base}`;
      let k = 2;
      while (used.has(id)) id = `${src.prefix}/${base}-${k++}`;
      used.add(id);

      const risk = riskFor(plain, src.os);
      const body = [
        `**Command:** \`${plain}\``,
        "",
        description,
        "",
        "```" + (src.os === "splunk" ? "spl" : src.os === "windows" ? "powershell" : "bash"),
        command.replace(/`/g, ""),
        "```",
      ].join("\n");

      const fm = [
        `id = ${tomlStr(id)}`,
        `kind = "command"`,
        `title = ${tomlStr(plain.slice(0, 90))}`,
        `category = ${tomlStr(category)}`,
        `tags = [${tomlStr(src.os)}, ${tomlStr(slugify(sec.title))}]`,
        `risk = ${tomlStr(risk)}`,
        `version = "core-0.2.0"`,
      ].join("\n");

      writeFileSync(join(OUT, `${id.replace(/\//g, "__")}.md`), `---\n${fm}\n---\n\n${body}\n`);
      n++;
    }
  }
}
console.log(`generated ${n} command entities -> packs-src/core/content/commands/`);
