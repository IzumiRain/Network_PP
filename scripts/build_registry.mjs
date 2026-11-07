// Builds the addons registry: zips each pack in addons-src/, computes
// SHA-256, and writes index.json for the Network_PP_addons repo.
// Usage: node scripts/build_registry.mjs
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { join, basename } from "node:path";

const ROOT = process.cwd();
const SRC = join(ROOT, "addons-src");
const OUT = join(ROOT, "addons-registry", "packs");

if (!existsSync(SRC)) {
  console.error("addons-src/ not found — run from the Network++ root");
  process.exit(1);
}
mkdirSync(OUT, { recursive: true });

const packs = [];
for (const dir of readdirSync(SRC, { withFileTypes: true })) {
  if (!dir.isDirectory() || dir.name === "SPEC.md") continue;
  const packDir = join(SRC, dir.name);
  const manifestPath = join(packDir, "manifest.json");
  if (!existsSync(manifestPath)) continue;

  const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
  const zipName = `${manifest.id}-${manifest.version}.zip`;
  const zipPath = join(OUT, zipName);

  // Re-zip cleanly (bsdtar on Windows supports --format zip; entries use
  // forward slashes with the pack folder as root so manifest.json sits at
  // the archive root — matching the app's expectations).
  rmSync(zipPath, { force: true });
  const { execSync } = await import("node:child_process");
  // Windows ships bsdtar at System32\tar.exe, which supports --format zip
  // (Git Bash's GNU tar does not). Fall back to PATH tar elsewhere.
  const tarBin = process.platform === "win32" ? "C:/Windows/System32/tar.exe" : "tar";
  execSync(
    `"${tarBin}" --format zip -cf "${zipPath}" -C "${packDir}" manifest.json content`,
    { stdio: "pipe", shell: "cmd.exe" },
  );

  const zipBytes = readFileSync(zipPath);
  const sha256 = createHash("sha256").update(zipBytes).digest("hex");

  // Count entities.
  let entities = 0;
  const walk = (d) => {
    for (const e of readdirSync(d, { withFileTypes: true })) {
      if (e.isDirectory()) walk(join(d, e.name));
      else if (e.name.endsWith(".md")) entities++;
    }
  };
  walk(join(packDir, "content"));

  packs.push({
    id: manifest.id,
    title: manifest.title,
    version: manifest.version,
    description: manifest.description,
    size: zipBytes.length,
    sha256,
    url: `https://raw.githubusercontent.com/IzumiRain/Network_PP_addons/main/packs/${zipName}`,
    min_app_version: manifest.min_app_version,
    categories: manifest.categories ?? [],
    entities,
  });
  console.log(`packed ${manifest.id}: ${entities} entities, ${Math.round(zipBytes.length / 1024)} KB`);
}

packs.sort((a, b) => a.id.localeCompare(b.id));
const index = {
  schema: 1,
  updated: new Date().toISOString().slice(0, 10),
  packs: packs.map(({ entities, ...p }) => p),
};
writeFileSync(join(ROOT, "addons-registry", "index.json"), JSON.stringify(index, null, 2) + "\n");
console.log(`\nindex.json: ${packs.length} packs, ${packs.reduce((s, p) => s + p.entities, 0)} entities total`);
