import "./styles/tokens.css";
import "./styles/base.css";
import "./styles/components.css";
import "./styles/layout.css";

import { icons, type IconName } from "./lib/icons";
import { invoke, inTauri } from "./lib/tauri";
import type { SearchHit } from "./lib/types";
import { renderHome } from "./views/home";
import { renderReference } from "./views/reference";
import { renderArticle } from "./views/article";
import { renderTools } from "./views/tools";
import { renderDiagnostics } from "./views/diagnostics";
import { renderRunbooks } from "./views/runbooks";
import { renderSettings } from "./views/settings";

type ViewFn = (el: HTMLElement, params: URLSearchParams) => void;

interface NavItem {
  path: string;
  label: string;
  short?: string;
  icon: IconName;
}

// Five destinations in the mobile bar; Settings lives in the desktop rail
// and the mobile top-bar gear.
const NAV: NavItem[] = [
  { path: "/home", label: "Home", icon: "home" },
  { path: "/reference", label: "Reference", short: "Ref", icon: "book" },
  { path: "/tools", label: "Tools", icon: "calc" },
  { path: "/diagnostics", label: "Diagnostics", short: "Diag", icon: "radar" },
  { path: "/runbooks", label: "Runbooks", short: "Books", icon: "layers" },
];

const SETTINGS_NAV: NavItem = { path: "/settings", label: "Settings", icon: "home" };

const VIEWS: Record<string, ViewFn> = {
  "/home": renderHome,
  "/reference": renderReference,
  "/article": renderArticle,
  "/tools": renderTools,
  "/diagnostics": renderDiagnostics,
  "/runbooks": renderRunbooks,
  "/settings": renderSettings,
};

function navLink(item: NavItem, short = false): string {
  return `<a href="#${item.path}" data-path="${item.path}">
    ${icons[item.icon]}<span>${short ? (item.short ?? item.label) : item.label}</span>
  </a>`;
}

const gearIcon = `<svg class="icon-svg" viewBox="0 0 24 24" aria-hidden="true" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.01a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.01a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1Z"/></svg>`;
const searchIcon = icons.search;

function shell(): string {
  return `
  <div class="app">
    <aside class="sidebar">
      <div class="brand">
        <img src="/icon.png" alt="Network++ logo" />
        <div class="name">Network<span>++</span></div>
      </div>
      <button class="btn btn--ghost search-trigger" id="search-trigger" type="button">
        ${searchIcon}<span>Search</span><kbd>Ctrl K</kbd>
      </button>
      <nav class="nav" aria-label="Primary">
        ${NAV.map((n) => navLink(n)).join("")}
        ${navLink(SETTINGS_NAV)}
      </nav>
      <div class="sidebar-footer">
        <div id="offline-indicator" class="offline-chip"><span class="dot"></span>Offline-ready</div>
        <div id="version-line" class="mono-data">v—</div>
      </div>
    </aside>
    <div style="flex:1;min-width:0;display:flex;flex-direction:column;min-height:100vh">
      <header class="topbar">
        <img src="/icon.png" alt="" />
        <div class="name">Network<span style="color:var(--accent-strong)">++</span></div>
        <div style="flex:1"></div>
        <button class="icon-btn" id="search-trigger-m" aria-label="Search" type="button">${searchIcon}</button>
        <a class="icon-btn" href="#/settings" aria-label="Settings">${gearIcon}</a>
      </header>
      <main class="main"><div id="view" class="view"></div></main>
      <nav class="bottomnav" aria-label="Primary mobile">${NAV.map((n) => navLink(n, true)).join("")}</nav>
    </div>
    <div id="search-overlay" class="search-overlay" hidden>
      <div class="search-panel" role="dialog" aria-label="Global search">
        <div class="search-bar">
          ${searchIcon}
          <input id="search-input" class="search-input" type="search"
            placeholder="Search commands, protocols, errors, runbooks…" autocomplete="off" />
          <button class="icon-btn" id="search-close" aria-label="Close search" type="button">✕</button>
        </div>
        <div id="search-results" class="search-results"></div>
        <div class="search-hint field-hint">Type at least two characters · Enter opens the first hit</div>
      </div>
    </div>
  </div>`;
}

function parseHash(): { path: string; params: URLSearchParams } {
  const raw = location.hash.replace(/^#/, "") || "/home";
  const [path, query] = raw.split("?");
  const p = path === "/" ? "/home" : path;
  return { path: p, params: new URLSearchParams(query ?? "") };
}

function render(): void {
  const { path, params } = parseHash();
  const el = document.getElementById("view");
  if (!el) return;

  const known = VIEWS[path] ? path : "/home";
  el.innerHTML = "";
  VIEWS[known](el, params);
  el.scrollTop = 0;
  window.scrollTo(0, 0);

  document.querySelectorAll(".nav a, .bottomnav a").forEach((a) => {
    const target = (a as HTMLAnchorElement).dataset.path;
    if (target === known) a.setAttribute("aria-current", "page");
    else a.removeAttribute("aria-current");
  });
}

// ---------- global search (Ctrl+K) ----------

function wireSearch(): void {
  const overlay = document.getElementById("search-overlay")!;
  const input = document.getElementById("search-input") as HTMLInputElement;
  const results = document.getElementById("search-results")!;
  let timer: ReturnType<typeof setTimeout> | undefined;

  const close = () => {
    overlay.hidden = true;
    input.value = "";
    results.innerHTML = "";
  };

  const open = () => {
    overlay.hidden = false;
    input.focus();
  };

  const hitHtml = (h: SearchHit): string => `
    <a class="search-hit" href="#/article?id=${encodeURIComponent(h.id)}" data-id="${h.id}">
      <span class="hit-kind">${h.kind}</span>
      <span class="hit-main">
        <span class="hit-title">${h.title}</span>
        <span class="hit-snip mono-data">${h.snippet}</span>
      </span>
      <span class="hit-cat mono-data">${h.category}</span>
    </a>`;

  const run = async () => {
    const q = input.value.trim();
    if (q.length < 2) {
      results.innerHTML = "";
      return;
    }
    try {
      const hits = await invoke<SearchHit[]>("search_query", { q, limit: 25 });
      results.innerHTML = hits.length
        ? hits.map(hitHtml).join("")
        : `<div class="empty" style="margin-top:8px"><p>No results for “${q}”.</p></div>`;
    } catch (e) {
      results.innerHTML = `<p class="field-error" style="padding:12px">${String(e).replace("Error: ", "")}</p>`;
    }
  };

  document.getElementById("search-trigger")?.addEventListener("click", open);
  document.getElementById("search-trigger-m")?.addEventListener("click", open);
  document.getElementById("search-close")?.addEventListener("click", close);
  overlay.addEventListener("click", (e) => {
    if (e.target === overlay) close();
  });
  input.addEventListener("input", () => {
    clearTimeout(timer);
    timer = setTimeout(() => void run(), 140);
  });
  input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      const first = results.querySelector<HTMLAnchorElement>(".search-hit");
      if (first) {
        close();
        location.hash = first.getAttribute("href")!.slice(1);
      }
    }
  });
  results.addEventListener("click", () => close());
  document.addEventListener("keydown", (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
      e.preventDefault();
      overlay.hidden ? open() : close();
    }
    if (e.key === "Escape" && !overlay.hidden) close();
  });
}

async function paintChrome(): Promise<void> {
  const version = document.getElementById("version-line");
  try {
    const info = await invoke<{ version: string; os: string; arch: string }>("app_info");
    if (version) version.textContent = `v${info.version} — ${info.os}/${info.arch}`;
  } catch {
    if (version) version.textContent = "v0.2.0 — browser preview";
  }
}

function boot(): void {
  const root = document.getElementById("app");
  if (!root) return;
  root.innerHTML = shell();

  window.addEventListener("hashchange", render);
  wireSearch();
  render();
  void paintChrome();

  if (!inTauri()) {
    const note = document.createElement("div");
    note.className = "card";
    note.style.marginBottom = "16px";
    note.innerHTML =
      "<strong>Preview mode.</strong> The Tauri backend is not running, so content, search and live tools respond with errors. Launch the desktop shell with <code>npm run tauri dev</code> for full function.";
    document.getElementById("view")?.prepend(note);
  }
}

boot();
