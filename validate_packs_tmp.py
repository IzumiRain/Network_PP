import json, re, sys, pathlib

ROOT = pathlib.Path(r"C:\CODING\codespace\Network++\addons-src")
PACKS = ["vendor-juniper", "vendor-mikrotik", "vendor-palo-alto",
         "vendor-pfsense-opnsense", "vendor-checkpoint-sophos", "virtualization-cloud"]

ALLOWED_KINDS = {"command","protocol","vendor","tool","runbook","error","port","configuration","standard"}
ALLOWED_RISK = {"info","low","medium","high"}
ID_RE = re.compile(r"^[a-z0-9][a-z0-9/_-]*$")
PACK_ID_RE = re.compile(r"^[a-z0-9-]+$")
SEMVER_RE = re.compile(r"^\d+\.\d+\.\d+$")

errors = []
all_ids = []
per_pack = {}

for pack in PACKS:
    pdir = ROOT / pack
    man_path = pdir / "manifest.json"
    if not man_path.exists():
        errors.append(f"{pack}: missing manifest.json")
        continue
    try:
        man = json.loads(man_path.read_text(encoding="utf-8"))
    except Exception as e:
        errors.append(f"{pack}: manifest.json invalid JSON: {e}")
        continue
    for k in ("schema","id","title","version","min_app_version","description","author"):
        if k not in man:
            errors.append(f"{pack}: manifest missing key {k}")
    if man.get("schema") != 1:
        errors.append(f"{pack}: schema != 1")
    if not PACK_ID_RE.match(man.get("id","")):
        errors.append(f"{pack}: bad manifest id {man.get('id')!r}")
    if man.get("id") != pack:
        errors.append(f"{pack}: manifest id {man.get('id')!r} != folder name")
    if not SEMVER_RE.match(man.get("version","")):
        errors.append(f"{pack}: bad version {man.get('version')!r}")

    files = sorted(pdir.glob("content/**/*.md"))
    per_pack[pack] = len(files)
    for f in files:
        text = f.read_text(encoding="utf-8")
        rel = f.relative_to(ROOT)
        if not text.startswith("---\n"):
            errors.append(f"{rel}: does not start with --- frontmatter")
            continue
        m = re.match(r"^---\n(.*?)\n---\n(.*)$", text, re.S)
        if not m:
            errors.append(f"{rel}: frontmatter not terminated")
            continue
        fm, body = m.group(1), m.group(2)
        fields = dict(re.findall(r'^(\w+)\s*=\s*"([^"]*)"$', fm, re.M))
        for req in ("id","kind","title","category","tags","version"):
            if req not in fields:
                errors.append(f"{rel}: frontmatter missing {req}")
        if "risk" not in fields:
            errors.append(f"{rel}: frontmatter missing risk")
        if fields.get("kind") not in ALLOWED_KINDS:
            errors.append(f"{rel}: bad kind {fields.get('kind')!r}")
        if fields.get("kind") == "command" and fields.get("risk") not in ALLOWED_RISK:
            errors.append(f"{rel}: command with bad/missing risk {fields.get('risk')!r}")
        if fields.get("risk") not in (ALLOWED_RISK | {""}):
            errors.append(f"{rel}: bad risk {fields.get('risk')!r}")
        eid = fields.get("id","")
        if not ID_RE.match(eid):
            errors.append(f"{rel}: bad entity id {eid!r}")
        all_ids.append((eid, rel))
        if not eid.startswith("cmd/"):
            errors.append(f"{rel}: entity id not prefixed cmd/: {eid!r}")
        if not re.match(r"^reference/", fields.get("category","")):
            errors.append(f"{rel}: category not under reference/: {fields.get('category')!r}")
        # tags parse check (simple list)
        if not re.search(r'^tags\s*=\s*\[.*\]$', fm, re.M):
            errors.append(f"{rel}: tags not a simple TOML list")
        # body: must end with Source line
        body_strip = body.rstrip("\n")
        if not re.search(r"^Source: .+ — https?://\S+$", body_strip.splitlines()[-1] if body_strip else ""):
            errors.append(f"{rel}: last line is not a valid Source citation: {body_strip.splitlines()[-1:]!r}")
        if "<" in body and "&lt;" not in body:
            pass  # angle brackets allowed in code blocks, HTML check is manual
        if re.search(r"<[a-zA-Z][a-zA-Z0-9]*[\s>/]", body):
            errors.append(f"{rel}: possible HTML tag in body")

# duplicate ids
seen = {}
for eid, rel in all_ids:
    if eid in seen:
        errors.append(f"DUPLICATE id {eid!r}: {seen[eid]} and {rel}")
    else:
        seen[eid] = rel

print("PACKS:", per_pack)
print("TOTAL entities:", sum(per_pack.values()))
if errors:
    print("\nERRORS (%d):" % len(errors))
    for e in errors:
        print(" -", e)
    sys.exit(1)
print("\nALL CHECKS PASSED")
