import { invoke, inTauri } from "../lib/tauri";
import { icons, type IconName } from "../lib/icons";
import type { AppInfo, HistoryEntry, StoreStats } from "../lib/types";

const QUICK_TOOLS: { path: string; title: string; icon: IconName }[] = [
  { path: "/tools?tool=cidr", title: "CIDR calculator", icon: "calc" },
  { path: "/tools?tool=ports", title: "Port reference", icon: "plug" },
  { path: "/tools?tool=packets", title: "Packet visualizer", icon: "layers" },
  { path: "/diagnostics?tab=target", title: "Target scan", icon: "radar" },
  { path: "/diagnostics?tab=local", title: "Local devices", icon: "wifi" },
  { path: "/diagnostics?tab=conn", title: "Connections", icon: "activity" },
];

function statusValue(v: number): string {
  return v.toLocaleString("en-US");
}

export function renderHome(el: HTMLElement): void {
  el.innerHTML = `
    <div class="view-head">
      <h1>Home</h1>
      <p>Your offline IT field toolkit — everything on this screen is stored locally.</p>
    </div>

    <section class="card" style="margin-bottom:16px">
      <h2>Quick tools</h2>
      <div class="chip-row">
        ${QUICK_TOOLS.map(
          (t) =>
            `<a class="quick-chip" href="#${t.path}"><span class="quick-ico">${icons[t.icon]}</span>${t.title}</a>`
        ).join("")}
      </div>
    </section>

    <div class="split-2" style="margin-bottom:16px">
      <section class="card">
        <h2>Pinned</h2>
        <div id="home-pins" class="home-list" aria-live="polite"></div>
      </section>
      <section class="card">
        <h2>Recent activity</h2>
        <div id="home-recent" class="home-list" aria-live="polite"></div>
      </section>
    </div>

    <section class="card" style="margin-bottom:16px">
      <h2>Favorites</h2>
      <div id="home-favs" class="home-list" aria-live="polite"></div>
    </section>

    <div class="split-2">
      <section class="card">
        <h2>Library status</h2>
        <dl class="kv" id="home-stats">
          <dt>Loading…</dt><dd></dd>
        </dl>
      </section>
      <section class="card">
        <h2>Platform</h2>
        <dl class="kv" id="home-platform">
          <dt>Loading…</dt><dd></dd>
        </dl>
      </section>
    </div>
  `;

  const linkItem = (itemId: string, title: string, kind: string) =>
    `<a class="home-item" href="#/article?id=${encodeURIComponent(itemId)}">
       <span class="hit-kind">${kind}</span><span class="hit-title">${title}</span>
     </a>`;

  void (async () => {
    // pinned
    try {
      const pins = await invoke<[string, string, string, string][]>("pin_list");
      el.querySelector<HTMLElement>("#home-pins")!.innerHTML = pins.length
        ? pins
            .map(
              ([, title, kind, route]) =>
                `<a class="home-item" href="#${route}">
                 <span class="hit-kind">${kind}</span><span class="hit-title">${title}</span>
               </a>`
            )
            .join("")
        : `<p class="field-hint">Nothing pinned yet — pin content from any article or tool header.</p>`;
    } catch (e) {
      el.querySelector<HTMLElement>("#home-pins")!.innerHTML = `<p class="field-hint">${String(e).replace("Error: ", "")}</p>`;
    }

    // recent
    try {
      const recent = await invoke<HistoryEntry[]>("history_list", { limit: 8 });
      el.querySelector<HTMLElement>("#home-recent")!.innerHTML = recent.length
        ? recent.map((h) => linkItem(h.id, h.title, h.kind)).join("")
        : `<p class="field-hint">Open any article to start your history.</p>`;
    } catch (e) {
      el.querySelector<HTMLElement>("#home-recent")!.innerHTML = `<p class="field-hint">${String(e).replace("Error: ", "")}</p>`;
    }

    // favorites
    try {
      const favs = await invoke<[string, string, string][]>("fav_list");
      el.querySelector<HTMLElement>("#home-favs")!.innerHTML = favs.length
        ? favs.map(([id, title, kind]) => linkItem(id, title, kind)).join("")
        : `<p class="field-hint">Star anything worth keeping — it lands here.</p>`;
    } catch (e) {
      el.querySelector<HTMLElement>("#home-favs")!.innerHTML = `<p class="field-hint">${String(e).replace("Error: ", "")}</p>`;
    }

    // stats + platform
    try {
      const [stats, info] = await Promise.all([
        invoke<StoreStats>("store_stats"),
        invoke<AppInfo>("app_info"),
      ]);
      el.querySelector<HTMLElement>("#home-stats")!.innerHTML = `
        <dt>Content entities</dt><dd class="hl">${statusValue(stats.entities)}</dd>
        <dt>Installed packs</dt><dd>${statusValue(stats.packs)}</dd>
        <dt>Favorites</dt><dd>${statusValue(stats.favorites)}</dd>
        <dt>History entries</dt><dd>${statusValue(stats.history)}</dd>`;
      el.querySelector<HTMLElement>("#home-platform")!.innerHTML = `
        <dt>Version</dt><dd>v${info.version}</dd>
        <dt>Platform</dt><dd>${info.os} / ${info.arch}</dd>
        <dt>Live tools</dt><dd>${inTauri() ? "Available" : "Browser preview only"}</dd>
        <dt>Backend</dt><dd>${inTauri() ? "Tauri (local)" : "Not connected"}</dd>`;
    } catch (e) {
      const msg = String(e).replace("Error: ", "");
      el.querySelector<HTMLElement>("#home-stats")!.innerHTML = `<dt>Status</dt><dd>${msg}</dd>`;
      el.querySelector<HTMLElement>("#home-platform")!.innerHTML = "";
    }
  })();
}
