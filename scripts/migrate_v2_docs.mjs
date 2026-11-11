// Migrates addons-src packs from the v1 content model (one .md per command)
// to the v2 document model (docs/PACK-SPEC.md): ONE Markdown document per
// Model/id, whose `##` sections are the old topic folders, each carrying its
// own tags via a `<!-- section-tags: … -->` marker. Source files become
// `### <title>` entries inside their topic section; their own headings are
// demoted by two levels (fenced code is never touched) and their Source /
// attribution lines are preserved verbatim.
//
// The merge is mechanical and deterministic: same input → same output.
// Re-running after a pack is migrated is a no-op (sources are gone and the
// generated docs are recognised by their `doc/` id prefix and skipped).
//
// Usage:
//   node scripts/migrate_v2_docs.mjs            # dry-run: print the plan
//   node scripts/migrate_v2_docs.mjs --write    # write merged docs + report
//   node scripts/migrate_v2_docs.mjs --write --delete-old
//                                               # also delete migrated sources
//   node scripts/migrate_v2_docs.mjs --verify   # title-coverage check only
//
// Packs handled here: every addons-src pack except vendor-f5-bigip, whose
// pilot document (content/bigip-4000.md) was hand-built in Slice 1 — this
// script only *verifies* it (see --verify) before its sources are removed
// by the same --delete-old flow.
import {
  existsSync,
  readdirSync,
  readFileSync,
  writeFileSync,
  rmSync,
  statSync,
} from "node:fs";
import { join, relative, sep } from "node:path";

const ROOT = process.cwd();
const SRC = join(ROOT, "addons-src");
const WRITE = process.argv.includes("--write");
const DELETE_OLD = process.argv.includes("--delete-old");
const VERIFY = process.argv.includes("--verify");

// ---------------------------------------------------------------------------
// Target documents per pack. `match(rel, fm)` receives the content-relative
// path (forward slashes) and parsed frontmatter, and decides membership.
// Vendor packs follow doc/<company>/<product>/<model>; non-vendor packs use
// doc/<area>/<group>/<topic>. Model = product-level document unless a real
// model exists (F5's 4000 pilot).
// ---------------------------------------------------------------------------
const PACK_DOCS = {
  "vendor-fortigate": [
    {
      id: "doc/fortinet/fortigate/fortigate",
      title: "Fortinet FortiGate",
      category: "reference/vendors/fortinet/fortigate",
      file: "fortigate.md",
      kind: "vendor",
      // runbooks/policy-not-matching.md already lives on as the Slice-1
      // pilot runbook document — it is verified, not merged again.
      match: (rel) => rel !== "runbooks/policy-not-matching.md",
    },
  ],
  "vendor-cisco-asa": [
    {
      id: "doc/cisco/asa/asa",
      title: "Cisco ASA",
      category: "reference/vendors/cisco/asa",
      file: "asa.md",
      kind: "vendor",
      match: () => true,
    },
  ],
  "vendor-fortiweb": [
    {
      id: "doc/fortinet/fortiweb/fortiweb",
      title: "Fortinet FortiWeb",
      category: "reference/vendors/fortinet/fortiweb",
      file: "fortiweb.md",
      kind: "vendor",
      match: () => true,
    },
  ],
  "vendor-fortiauthenticator": [
    {
      id: "doc/fortinet/fortiauthenticator/fortiauthenticator",
      title: "Fortinet FortiAuthenticator",
      category: "reference/vendors/fortinet/fortiauthenticator",
      file: "fortiauthenticator.md",
      kind: "vendor",
      match: () => true,
    },
  ],
  "vendor-palo-alto": [
    {
      id: "doc/palo-alto/palo-alto/palo-alto",
      title: "Palo Alto Networks NGFW",
      category: "reference/vendors/palo-alto",
      file: "palo-alto.md",
      kind: "vendor",
      match: () => true,
    },
  ],
  "vendor-juniper": [
    {
      id: "doc/juniper/juniper/juniper",
      title: "Juniper Junos OS",
      category: "reference/vendors/juniper",
      file: "juniper.md",
      kind: "vendor",
      match: () => true,
    },
  ],
  "vendor-mikrotik": [
    {
      id: "doc/mikrotik/mikrotik/mikrotik",
      title: "MikroTik RouterOS",
      category: "reference/vendors/mikrotik",
      file: "mikrotik.md",
      kind: "vendor",
      match: () => true,
    },
  ],
  "vendor-checkpoint-sophos": [
    {
      id: "doc/check-point/check-point/check-point",
      title: "Check Point Gaia",
      category: "reference/vendors/check-point",
      file: "check-point.md",
      kind: "vendor",
      match: (rel) => rel.startsWith("checkpoint/"),
    },
    {
      id: "doc/sophos/sophos/sophos",
      title: "Sophos Firewall",
      category: "reference/vendors/sophos",
      file: "sophos.md",
      kind: "vendor",
      match: (rel) => rel.startsWith("sophos/"),
    },
  ],
  "vendor-pfsense-opnsense": [
    {
      id: "doc/pfsense-opnsense/pfsense/pfsense",
      title: "pfSense",
      category: "reference/vendors/pfsense-opnsense/pfsense",
      file: "pfsense.md",
      kind: "vendor",
      match: (rel) => !rel.startsWith("opnsense/"),
    },
    {
      id: "doc/pfsense-opnsense/opnsense/opnsense",
      title: "OPNsense",
      category: "reference/vendors/pfsense-opnsense/opnsense",
      file: "opnsense.md",
      kind: "vendor",
      match: (rel) => rel.startsWith("opnsense/"),
    },
  ],
  "vendor-waf": [
    {
      id: "doc/security/waf/modsecurity",
      title: "ModSecurity",
      category: "reference/security/waf/modsecurity",
      file: "modsecurity.md",
      match: (rel) => !rel.startsWith("crs/"),
    },
    {
      id: "doc/security/waf/crs",
      title: "OWASP ModSecurity Core Rule Set (CRS)",
      category: "reference/security/waf/crs",
      file: "crs.md",
      match: (rel) => rel.startsWith("crs/"),
    },
  ],
  "splunk-operations": [
    {
      id: "doc/monitoring/splunk-ops/splunk-ops",
      title: "Splunk Operations",
      category: "reference/monitoring/splunk-ops",
      file: "splunk-ops.md",
      match: () => true,
    },
  ],
  "iso-standards": [
    {
      id: "doc/security/iso-standards/iso-standards",
      title: "ISO/IEC Standards",
      category: "reference/security/iso-standards",
      file: "iso-standards.md",
      match: () => true,
    },
  ],
  "security-plus": [
    {
      id: "doc/security/security-plus/security-plus",
      title: "CompTIA Security+",
      category: "reference/security/security-plus",
      file: "security-plus.md",
      match: () => true,
    },
  ],
  "virtualization-cloud": [
    ...["aws", "azure", "docker", "gcp", "kubernetes", "proxmox"].map((p) => ({
      id: `doc/vendors/${p}/${p}`,
      title: {
        aws: "Amazon Web Services (AWS)",
        azure: "Microsoft Azure",
        docker: "Docker",
        gcp: "Google Cloud (GCP)",
        kubernetes: "Kubernetes",
        proxmox: "Proxmox VE",
      }[p],
      category: `reference/vendors/${p}`,
      file: `${p}.md`,
      kind: "vendor",
      match: (rel) => rel.startsWith(`${p}/`),
    })),
  ],
};

// Packs whose v2 documents already exist (Slice-1 pilots): verify coverage
// of the listed pre-built docs against their sources, then (with
// --delete-old) remove the sources. `sources` = every non-doc .md in the
// pack; each source title must appear in one of the pilot docs.
const PILOT_PACKS = {
  "vendor-f5-bigip": ["bigip-4000.md"],
  "vendor-fortigate": ["fortigate-runbook-policy.md"],
};

// ---------------------------------------------------------------------------
function walk(dir) {
  const out = [];
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) out.push(...walk(p));
    else if (e.name.endsWith(".md")) out.push(p);
  }
  return out.sort();
}

function parseFrontmatter(text) {
  const m = /^---\n([\s\S]*?)\n---\n?/.exec(text);
  if (!m) return { fm: {}, body: text };
  const fm = {};
  for (const line of m[1].split("\n")) {
    const kv = /^(\w+)\s*=\s*(.+)$/.exec(line.trim());
    if (!kv) continue;
    let v = kv[2].trim();
    if (v.startsWith("[")) {
      fm[kv[1]] = [...v.matchAll(/"([^"]*)"/g)].map((x) => x[1]);
    } else {
      fm[kv[1]] = v.replace(/^"|"$/g, "");
    }
  }
  return { fm, body: text.slice(m[0].length) };
}

const ACRONYMS = {
  acl: "ACL", ha: "HA", vpn: "VPN", nat: "NAT", crs: "CRS", sso: "SSO",
  ldap: "LDAP", api: "API", ip: "IP", dns: "DNS", os: "OS", ssl: "SSL",
  ike: "IKE", spl: "SPL", waf: "WAF", vm: "VM",
};
const titleCase = (slug) =>
  slug
    .split("-")
    .map(
      (w) =>
        ACRONYMS[w] ??
        (w.length <= 2 ? w.toUpperCase() : w[0].toUpperCase() + w.slice(1))
    )
    .join(" ");

const RISK_ORDER = ["info", "low", "medium", "high"];

/** Demote headings by two levels (## → #### …), never inside code fences. */
function demoteHeadings(body) {
  let inFence = false;
  return body
    .split("\n")
    .map((line) => {
      if (line.startsWith("```")) inFence = !inFence;
      if (inFence) return line;
      const h = /^(#{1,6})(\s+.*)$/.exec(line);
      if (!h) return line;
      const level = Math.min(6, Math.max(4, h[1].length + 2));
      return "#".repeat(level) + h[2];
    })
    .join("\n");
}

function buildDoc(packId, spec, files) {
  // Group by topic: first content-relative folder, else the category's
  // last segment, else "Overview".
  const byTopic = new Map();
  for (const f of files) {
    const parts = f.rel.split("/");
    let topic;
    if (parts.length > 1) topic = parts[0];
    else topic = (f.fm.category ?? "").split("/").pop() || "overview";
    // For per-product packs the first folder IS the product, not a topic:
    // fall back to the category topic segment when available.
    if (
      (packId === "vendor-checkpoint-sophos" || packId === "virtualization-cloud") &&
      parts.length > 1
    ) {
      const catTopic = (f.fm.category ?? "").split("/").pop();
      topic = catTopic && catTopic !== parts[0] ? catTopic : "commands";
    }
    if (!byTopic.has(topic)) byTopic.set(topic, []);
    byTopic.get(topic).push(f);
  }

  const allTags = [];
  const versions = new Map();
  let risk = "info";
  const kinds = new Map();
  for (const f of files) {
    for (const t of f.fm.tags ?? []) if (!allTags.includes(t)) allTags.push(t);
    if (f.fm.version) versions.set(f.fm.version, (versions.get(f.fm.version) ?? 0) + 1);
    if (f.fm.risk && RISK_ORDER.indexOf(f.fm.risk) > RISK_ORDER.indexOf(risk)) risk = f.fm.risk;
    if (f.fm.kind) kinds.set(f.fm.kind, (kinds.get(f.fm.kind) ?? 0) + 1);
  }
  const versionList = [...versions.entries()].sort((a, b) => b[1] - a[1]).map(([v]) => v);
  const version = versionList.length <= 2 ? versionList.join(" / ") : versionList[0];
  const dominantKind = [...kinds.entries()].sort((a, b) => b[1] - a[1])[0]?.[0] ?? "protocol";
  const kind = spec.kind ?? dominantKind;
  if (!allTags.includes("model-doc")) allTags.push("model-doc");

  const tomlList = (xs) => `[${xs.map((x) => `"${x}"`).join(", ")}]`;
  let out = `---\nid = "${spec.id}"\nkind = "${kind}"\ntitle = "${spec.title}"\ncategory = "${spec.category}"\ntags = ${tomlList(allTags)}\nrisk = "${risk}"\n`;
  if (version) out += `version = "${version}"\n`;
  out += `---\nThe complete ${spec.title} document: every topic for this model in one\nfile (v2 document model — see docs/PACK-SPEC.md, "Document model (v2 draft)").\nContent is reorganised, unchanged, from this pack's original per-command\nfiles; each topic is a section carrying its own tags via a section-tags\nmarker. Source attributions are kept per topic.\n`;

  let sections = 0;
  for (const topic of [...byTopic.keys()].sort()) {
    const group = byTopic.get(topic).sort((a, b) => a.rel.localeCompare(b.rel));
    const sectionTags = [];
    for (const f of group)
      for (const t of f.fm.tags ?? []) if (!sectionTags.includes(t)) sectionTags.push(t);
    out += `\n\n## ${titleCase(topic)}\n<!-- section-tags: ${sectionTags.join(", ")} -->\n`;
    sections++;
    for (const f of group) {
      out += `\n\n### ${f.fm.title ?? f.rel}\n\n${demoteHeadings(f.body.trim())}\n`;
    }
  }
  return { text: out + "\n", sections, version, kind };
}

// ---------------------------------------------------------------------------
const report = [];
let failures = 0;

for (const packId of Object.keys(PACK_DOCS)) {
  const packDir = join(SRC, packId);
  const contentDir = join(packDir, "content");
  if (!existsSync(contentDir)) continue;
  const all = walk(contentDir).map((p) => {
    const { fm, body } = parseFrontmatter(readFileSync(p, "utf8"));
    return { path: p, rel: relative(contentDir, p).split(sep).join("/"), fm, body };
  });
  const sources = all.filter((f) => !(f.fm.id ?? "").startsWith("doc/"));
  const kept = all.filter((f) => (f.fm.id ?? "").startsWith("doc/"));

  for (const spec of PACK_DOCS[packId]) {
    const files = sources.filter((f) => spec.match(f.rel, f.fm));
    if (!files.length) {
      report.push(`${packId} :: ${spec.id} — no sources (already migrated?)`);
      continue;
    }
    const doc = buildDoc(packId, spec, files);
    const target = join(contentDir, spec.file);
    report.push(
      `${packId} :: ${files.length} files → ${spec.file} (${doc.sections} sections, kind=${doc.kind}, version=${doc.version ?? "—"})`
    );
    // Coverage: every source title must appear as a ### entry.
    for (const f of files) {
      if (f.fm.title && !doc.text.includes(`### ${f.fm.title}\n`)) {
        console.error(`COVERAGE FAIL ${packId}/${f.rel}: title not in ${spec.file}`);
        failures++;
      }
    }
    if (WRITE) writeFileSync(target, doc.text);
    if (WRITE && DELETE_OLD) {
      for (const f of files) rmSync(f.path);
    }
  }
  if (kept.length)
    report.push(`${packId} :: kept ${kept.length} pre-existing v2 doc(s): ${kept.map((k) => k.rel).join(", ")}`);
}

// Pilot packs: verify the hand-built docs cover their sources.
for (const [packId, pilotFiles] of Object.entries(PILOT_PACKS)) {
  const packDir = join(SRC, packId);
  const contentDir = join(packDir, "content");
  if (!existsSync(contentDir)) continue;
  const all = walk(contentDir).map((p) => {
    const { fm, body } = parseFrontmatter(readFileSync(p, "utf8"));
    return { path: p, rel: relative(contentDir, p).split(sep).join("/"), fm, body, raw: readFileSync(p, "utf8") };
  });
  const pilots = all.filter((f) => pilotFiles.includes(f.rel));
  // Only sources NOT claimed by a generated document (PACK_DOCS above) are
  // the pilot's responsibility — e.g. for vendor-fortigate that is just
  // runbooks/policy-not-matching.md; the generated fortigate.md covers
  // everything else and is coverage-checked in its own block.
  const claimed = new Set();
  for (const spec of PACK_DOCS[packId] ?? [])
    for (const f of all) if (spec.match(f.rel, f.fm)) claimed.add(f.rel);
  const sources = all.filter(
    (f) =>
      !(f.fm.id ?? "").startsWith("doc/") &&
      !pilotFiles.includes(f.rel) &&
      !claimed.has(f.rel)
  );
  const extraDocs = [];
  const haystack = [...pilots.map((p) => p.raw), ...extraDocs].join("\n");
  let covered = 0;
  for (const f of sources) {
    // Title match, or (pilot titles were lightly reworded in Slice 1) any
    // distinctive body line carried over unchanged.
    const bodyHit = f.body
      .split("\n")
      .map((l) => l.trim())
      .find((l) => l.length > 30 && haystack.includes(l));
    if (f.fm.title && (haystack.includes(f.fm.title) || bodyHit)) covered++;
    else {
      console.error(`PILOT COVERAGE FAIL ${packId}/${f.rel}: "${f.fm.title}" not found in pilot doc(s)`);
      failures++;
    }
  }
  report.push(`${packId} :: pilot doc(s) [${pilotFiles.join(", ")}] cover ${covered}/${sources.length} source titles`);
  if (WRITE && DELETE_OLD && failures === 0) {
    for (const f of sources) rmSync(f.path);
  }
}

console.log(report.join("\n"));
console.log(
  `\nmode: ${VERIFY ? "verify" : WRITE ? "write" : "dry-run"}${DELETE_OLD ? " +delete-old" : ""} — failures: ${failures}`
);
process.exit(failures ? 1 : 0);
