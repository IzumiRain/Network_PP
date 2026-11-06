// Fetch uiverse.io components and extract embedded JSON-LD source code.
// Usage: node scripts/parse_uiverse.mjs <pagesDir> <outDir>
// (pages are downloaded separately via curl; direct node fetch is blocked by CDN)
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const PAGES = process.argv[2] ?? "/tmp/uiverse_pages";
const OUT = process.argv[3] ?? "docs/research/uiverse";
mkdirSync(OUT, { recursive: true });

const COMPONENTS = [
  ["yellow-rattlesnake-26", "https://uiverse.io/mymiamo/yellow-rattlesnake-26", "button-section"],
  ["soft-octopus-29", "https://uiverse.io/vinodjangid07/soft-octopus-29", "toggle-1"],
  ["tall-starfish-3", "https://uiverse.io/chicogale/tall-starfish-3", "toggle-2"],
  ["thin-sloth-31", "https://uiverse.io/Praashoo7/thin-sloth-31", "card-box-2"],
  ["itchy-bobcat-50", "https://uiverse.io/EmmaxPlay/itchy-bobcat-50", "box-1"],
  ["fluffy-quail-74", "https://uiverse.io/catraco/fluffy-quail-74", "button-1"],
  ["horrible-fish-20", "https://uiverse.io/themrsami/horrible-fish-20", "button-2"],
  ["shaggy-quail-31", "https://uiverse.io/icochran10/shaggy-quail-31", "button-3"],
  ["odd-squid-52", "https://uiverse.io/andrew-demchenk0/odd-squid-52", "button-4"],
  ["fresh-tiger-82", "https://uiverse.io/vinodjangid07/fresh-tiger-82", "button-5"],
  ["hot-fox-27", "https://uiverse.io/mRcOol7/hot-fox-27", "button-6"],
  ["chilly-swan-51", "https://uiverse.io/milley69/chilly-swan-51", "loader"],
  ["warm-eel-62", "https://uiverse.io/liyaxu123/warm-eel-62", "input"],
  ["bad-cheetah-74", "https://uiverse.io/JohnnyCSilva/bad-cheetah-74", "form"],
  ["quiet-snail-9", "https://uiverse.io/SelfMadeSystem/quiet-snail-9", "pattern"],
  ["spicy-kangaroo-36", "https://uiverse.io/kelvin_9865/spicy-kangaroo-36", "tooltip"],
  ["tall-swan-6", "https://uiverse.io/JulanDeAlb/tall-swan-6", "list"],
  ["green-bobcat-29", "https://uiverse.io/SelfMadeSystem/green-bobcat-29", "checkbox"],
];

const decodeEntities = (s) =>
  s
    .replace(/&quot;/g, '"')
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&#39;/g, "'")
    .replace(/&#x27;/g, "'")
    .replace(/&amp;/g, "&");

// Extract every application/ld+json payload (script tag OR meta script= attr);
// keep SoftwareSourceCode entries.
function extractCode(html) {
  const out = [];
  const re = /<script type="application\/ld\+json"[^>]*>([\s\S]*?)<\/script>|<meta[^>]*type="application\/ld\+json"[^>]*script="([\s\S]*?)"\s*\/?>/gi;
  let m;
  while ((m = re.exec(html)) !== null) {
    const payload = m[1] ?? m[2];
    if (!payload) continue;
    let raw = decodeEntities(payload.trim());
    try {
      const data = JSON.parse(raw);
      const items = Array.isArray(data) ? data : [data];
      for (const it of items) {
        const node = it?.["@graph"] ? it["@graph"] : it;
        const list = Array.isArray(node) ? node : [node];
        for (const n of list) {
          if (n?.["@type"] === "SoftwareSourceCode" && typeof n.text === "string") {
            out.push({ text: n.text, name: n.name ?? "", keywords: n.keywords ?? "" });
          }
        }
      }
    } catch {
      // Some pages embed several concatenated JSON objects; retry by splitting.
      const parts = raw.split(/(?<=\})\s*(?=\{)/);
      for (const p of parts) {
        try {
          const n = JSON.parse(p);
          if (n?.["@type"] === "SoftwareSourceCode" && typeof n.text === "string") {
            out.push({ text: n.text, name: n.name ?? "", keywords: n.keywords ?? "" });
          }
        } catch {
          /* ignore unparsable fragment */
        }
      }
    }
  }
  return out;
}

function splitSource(text) {
  const marker = text.indexOf("/* CSS */");
  if (marker === -1) return { html: text.trim(), css: "" };
  return {
    html: text.slice(0, marker).trim(),
    css: text.slice(marker + 9).trim(),
  };
}

let ok = 0;
for (const [slug, url, role] of COMPONENTS) {
  try {
    const html = readFileSync(join(PAGES, `${slug}.html`), "utf8");
    const found = extractCode(html);
    if (found.length === 0) throw new Error("no ld+json SoftwareSourceCode found");
    const src = found[0];
    const { html: codeHtml, css } = splitSource(src.text);
    const front = `---\nslug: ${slug}\nrole: ${role}\nurl: ${url}\nkeywords: ${src.keywords}\nlicense: MIT (uiverse.io)\n---\n\n`;
    writeFileSync(join(OUT, `${role}.md`), front + "# HTML\n\n```html\n" + codeHtml + "\n```\n\n# CSS\n\n```css\n" + css + "\n```\n");
    ok++;
    console.log(`ok  ${role}  (${codeHtml.length}b html / ${css.length}b css)`);
  } catch (e) {
    console.log(`FAIL ${role} ${slug}: ${e.message}`);
  }
}
console.log(`\n${ok}/${COMPONENTS.length} components extracted -> ${OUT}`);
