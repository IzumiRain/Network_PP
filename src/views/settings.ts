import { invoke, pickZipFile, inTauri } from "../lib/tauri";
import { esc } from "../lib/safe";
import type { AppInfo, InstallReport, PackRow, RegistryEntry, StoreStats } from "../lib/types";

type Tab = "packs" | "data" | "about";

export function renderSettings(el: HTMLElement, params: URLSearchParams): void {
  const tab: Tab = (["packs", "data", "about"] as const).includes(params.get("tab") as Tab)
    ? (params.get("tab") as Tab)
    : "packs";

  el.innerHTML = `
    <div class="view-head">
      <h1>Settings</h1>
      <p>Content packs, local data and application information.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:24px">
      <button role="tab" data-tab="packs" aria-selected="${tab === "packs"}">Content packs</button>
      <button role="tab" data-tab="data" aria-selected="${tab === "data"}">Data &amp; storage</button>
      <button role="tab" data-tab="about" aria-selected="${tab === "about"}">About</button>
    </div>
    <div id="settings-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#settings-body")!;
  el.querySelectorAll<HTMLButtonElement>(".segmented button[data-tab]").forEach((b) => {
    b.addEventListener("click", () => {
      location.hash = `/settings?tab=${b.dataset.tab}`;
    });
  });

  if (tab === "packs") void renderPacks(body);
  else if (tab === "data") void renderData(body);
  else void renderAbout(body);
}

// ---------- content packs ----------

async function renderPacks(body: HTMLElement): Promise<void> {
  body.innerHTML = `
    <section class="card" style="margin-bottom:16px">
      <div class="toolbar">
        <button class="btn" id="pk-install-file" ${inTauri() ? "" : "disabled"}>Install from file…</button>
        <button class="btn btn--ghost" id="pk-refresh">Check registry</button>
        <span id="pk-status" class="field-hint" style="align-self:center"></span>
      </div>
      <div class="split-2">
        <div class="field">
          <label for="pk-registry">Registry URL (index.json)</label>
          <input id="pk-registry" class="input input--mono" placeholder="https://cdn.jsdelivr.net/gh/networkpp/addons@main/index.json" />
        </div>
        <div class="field">
          <label for="pk-proxy">Network proxy — socks5://host:port or http://host:port</label>
          <input id="pk-proxy" class="input input--mono" placeholder="socks5://127.0.0.1:10808" />
        </div>
      </div>
      <p class="field-hint">Packs are plain data (Markdown) — verified with SHA-256 before install. Nothing in a pack can execute.</p>
    </section>
    <section class="card" style="margin-bottom:16px">
      <h2>Installed</h2>
      <div id="pk-installed"></div>
    </section>
    <section class="card">
      <h2>Available in registry</h2>
      <div id="pk-available"><p class="field-hint">Press “Check registry” to fetch the index.</p></div>
    </section>
  `;

  const installed = body.querySelector<HTMLElement>("#pk-installed")!;
  const available = body.querySelector<HTMLElement>("#pk-available")!;
  const status = body.querySelector<HTMLElement>("#pk-status")!;

  const registryInput = body.querySelector<HTMLInputElement>("#pk-registry")!;
  const proxyInput = body.querySelector<HTMLInputElement>("#pk-proxy")!;

  try {
    registryInput.value = (await invoke<string | null>("settings_get", { key: "packs.registryUrl" })) ?? "";
  } catch { /* preview mode */ }
  try {
    proxyInput.value = (await invoke<string | null>("settings_get", { key: "network.proxy" })) ?? "";
  } catch { /* preview mode */ }

  const saveSettings = async () => {
    try {
      await invoke("settings_set", { key: "packs.registryUrl", value: registryInput.value.trim() });
      await invoke("settings_set", { key: "network.proxy", value: proxyInput.value.trim() });
    } catch { /* preview mode */ }
  };
  registryInput.addEventListener("change", saveSettings);
  proxyInput.addEventListener("change", saveSettings);

  const paintInstalled = async () => {
    try {
      const packs = await invoke<PackRow[]>("pack_list");
      installed.innerHTML = packs.length
        ? `<div class="table-wrap"><table class="data">
            <thead><tr><th>Pack</th><th>Version</th><th>Entities</th><th>Source</th><th></th></tr></thead>
            <tbody>${packs
              .map(
                (p) => `<tr>
                  <td>${esc(p.title)}<span class="mono-data" style="color:var(--text-muted)"> ${esc(p.id)}</span></td>
                  <td>${esc(p.version)}</td>
                  <td>${p.entity_count.toLocaleString("en-US")}</td>
                  <td class="wrap">${esc(p.source)}</td>
                  <td>${p.id === "core" ? `<span class="pill pill--open">core</span>` : `<button class="btn btn--danger" data-remove="${p.id}" style="padding:4px 10px;min-height:28px">Remove</button>`}</td>
                </tr>`
              )
              .join("")}</tbody></table></div>`
        : `<div class="empty"><p>No packs installed.</p></div>`;
      installed.querySelectorAll<HTMLButtonElement>("[data-remove]").forEach((btn) => {
        btn.addEventListener("click", async () => {
          btn.classList.add("is-loading");
          try {
            await invoke("pack_remove", { id: btn.dataset.remove });
            await paintInstalled();
          } catch (e) {
            status.textContent = String(e).replace("Error: ", "");
          }
        });
      });
    } catch (e) {
      installed.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    }
  };
  await paintInstalled();

  const paintAvailable = async () => {
    status.textContent = "Fetching registry…";
    try {
      const entries = await invoke<RegistryEntry[]>("pack_available");
      status.textContent = `${entries.length} packs in registry.`;
      const installedNow = new Set(
        ((await invoke<PackRow[]>("pack_list")) ?? []).map((p) => p.id)
      );
      available.innerHTML = entries.length
        ? entries
            .map(
              (p) => `
          <div class="pack-offer">
            <div>
              <strong>${esc(p.title)}</strong> <span class="mono-data" style="color:var(--text-muted)">${esc(p.id)} v${esc(p.version)}</span>
              <p class="field-hint">${esc(p.description)}</p>
            </div>
            <div style="text-align:right;flex:none">
              <div class="mono-data" style="margin-bottom:6px">${p.size ? formatSize(p.size) : "—"}</div>
              <button class="btn" data-install="${p.id}" ${
                installedNow.has(p.id) ? "disabled" : ""
              }>${installedNow.has(p.id) ? "Installed" : "Download"}</button>
            </div>
          </div>`
            )
            .join("")
        : `<div class="empty"><p>Registry is reachable but lists no packs.</p></div>`;

      available.querySelectorAll<HTMLButtonElement>("[data-install]").forEach((btn) => {
        btn.addEventListener("click", async () => {
          btn.classList.add("is-loading");
          btn.textContent = "Downloading…";
          try {
            const report = await invoke<InstallReport>("pack_install_registry", {
              id: btn.dataset.install,
            });
            status.textContent = `Installed ${report.title} — ${report.entity_count} entities.`;
            await paintInstalled();
            btn.textContent = "Installed";
            btn.disabled = true;
          } catch (e) {
            btn.classList.remove("is-loading");
            btn.textContent = "Retry";
            status.textContent = String(e).replace("Error: ", "");
          }
        });
      });
    } catch (e) {
      status.textContent = "";
      available.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p><p class="field-hint">Set the registry URL or a proxy above, then retry.</p></div>`;
    }
  };

  body.querySelector("#pk-refresh")!.addEventListener("click", () => void paintAvailable());

  body.querySelector<HTMLButtonElement>("#pk-install-file")!.addEventListener("click", async () => {
    const path = await pickZipFile();
    if (!path) return;
    status.textContent = "Installing…";
    try {
      const report = await invoke<InstallReport>("pack_install_file", { path });
      status.textContent = `Installed ${report.title} — ${report.entity_count} entities.` +
        (report.warnings.length ? ` ${report.warnings.length} files skipped.` : "");
      await paintInstalled();
    } catch (e) {
      status.textContent = String(e).replace("Error: ", "");
    }
  });
}

function formatSize(bytes: number): string {
  if (bytes > 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  return `${Math.max(1, Math.round(bytes / 1024))} KB`;
}

// ---------- data & storage ----------

async function renderData(body: HTMLElement): Promise<void> {
  body.innerHTML = `
    <div class="split-2">
      <section class="card">
        <h2>Local database</h2>
        <dl class="kv" id="data-stats"><dt>Loading…</dt><dd></dd></dl>
      </section>
      <section class="card">
        <h2>Maintenance</h2>
        <div class="panel-grid">
          <button class="btn btn--ghost" id="data-clear-history" type="button">Clear history</button>
          <button class="btn btn--ghost" id="data-recore" type="button">Reinstall core pack</button>
        </div>
        <p class="field-hint" style="margin-top:12px">The database lives in the OS app-data folder. Favorites, pins and history are yours; pack content can always be re-downloaded.</p>
      </section>
    </div>
  `;

  try {
    const stats = await invoke<StoreStats>("store_stats");
    body.querySelector<HTMLElement>("#data-stats")!.innerHTML = `
      <dt>Content entities</dt><dd class="hl">${stats.entities.toLocaleString("en-US")}</dd>
      <dt>Installed packs</dt><dd>${stats.packs}</dd>
      <dt>Favorites</dt><dd>${stats.favorites}</dd>
      <dt>History entries</dt><dd>${stats.history}</dd>`;
  } catch (e) {
    body.querySelector<HTMLElement>("#data-stats")!.innerHTML =
      `<dt>Status</dt><dd>${String(e).replace("Error: ", "")}</dd>`;
  }

  body.querySelector("#data-clear-history")!.addEventListener("click", async (ev) => {
    (ev.target as HTMLButtonElement).textContent = "Cleared ✓";
    try {
      await invoke("history_clear");
    } catch { /* preview mode */ }
    setTimeout(() => void renderData(body), 900);
  });

  body.querySelector("#data-recore")!.addEventListener("click", async (ev) => {
    const btn = ev.target as HTMLButtonElement;
    btn.textContent = "Reinstalling…";
    try {
      await invoke("pack_ensure_core");
      btn.textContent = "Done ✓";
    } catch (e) {
      btn.textContent = String(e).replace("Error: ", "");
    }
    setTimeout(() => (btn.textContent = "Reinstall core pack"), 1400);
  });
}

// ---------- about ----------

async function renderAbout(body: HTMLElement): Promise<void> {
  let info: AppInfo = { version: "0.2.0", os: "—", arch: "—" };
  try {
    info = await invoke<AppInfo>("app_info");
  } catch { /* preview mode */ }

  body.innerHTML = `
    <section class="card" style="max-width:75ch;margin-bottom:16px">
      <h2>Network++</h2>
      <dl class="kv">
        <dt>Version</dt><dd class="hl">v${info.version}</dd>
        <dt>Platform</dt><dd>${info.os} / ${info.arch}</dd>
        <dt>Engine</dt><dd>Rust + Tauri 2 — offline-first</dd>
        <dt>License</dt><dd>PolyForm Noncommercial 1.0.0 + additional terms</dd>
        <dt>Source code</dt><dd><a href="https://github.com/IzumiRain/Network_PP" target="_blank" rel="noopener">github.com/IzumiRain/Network_PP</a></dd>
        <dt>Fonts</dt><dd>Inter &amp; JetBrains Mono (SIL OFL 1.1)</dd>
        <dt>Design references</dt><dd>uiverse.io components (MIT)</dd>
      </dl>
    </section>
    <section class="card" style="max-width:75ch;margin-bottom:16px">
      <h2>Acceptable use</h2>
      <p>
        Network++ is a field toolkit for <strong>defensive and administrative</strong>
        work on networks and systems <strong>you own or are explicitly authorized to
        manage</strong>. Using its scanning, packet or probing features against
        systems without that authorization is illegal in most jurisdictions
        (e.g. the CFAA in the United States, the Computer Misuse Act in the
        United Kingdom, and equivalent laws elsewhere) and is prohibited by
        the license.
      </p>
      <p style="margin-top:8px">
        The authors accept no liability for misuse. Diagnostic and security
        knowledge in content packs is provided for education and lawful
        operations — not for offensive activity.
      </p>
    </section>
    <section class="card" style="max-width:75ch">
      <h2>License</h2>
      <p>
        PolyForm Noncommercial 1.0.0 — personal study, research, education and
        noncommercial organizational use are permitted. Commercial use,
        distribution for profit, and use by for-profit businesses require a
        separate license from the copyright holder.
      </p>
      <p class="field-hint" style="margin-top:8px">
        Full text including additional terms: the LICENSE file in the
        repository root.
      </p>
    </section>
  `;
}
