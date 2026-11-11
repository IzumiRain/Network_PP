import { invoke } from "../lib/tauri";
import { icons } from "../lib/icons";
import { esc } from "../lib/safe";

interface CategoryNode {
  name: string;
  path: string;
  children: CategoryNode[];
}

function buildTree(categories: string[]): CategoryNode[] {
  const root: CategoryNode = { name: "", path: "", children: [] };
  for (const cat of categories) {
    const parts = cat.split("/").filter(Boolean);
    let node = root;
    let path = "";
    for (const part of parts) {
      path = path ? `${path}/${part}` : part;
      let child = node.children.find((c) => c.name === part);
      if (!child) {
        child = { name: part, path, children: [] };
        node.children.push(child);
      }
      node = child;
    }
  }
  const sortRec = (n: CategoryNode) => {
    n.children.sort((a, b) => a.name.localeCompare(b.name));
    n.children.forEach(sortRec);
  };
  root.children.sort((a, b) => a.name.localeCompare(b.name));
  root.children.forEach(sortRec);
  return root.children;
}

/**
 * Display names are derived generically from the slug — prettified, never
 * looked up in a hardcoded table (v2 data-driven model, docs/PACK-SPEC.md).
 * Any category a pack invents tomorrow renders correctly with no code change.
 * Well-known all-caps tokens keep their casing (e.g. `os` would prettify to
 * "Os" without this; casing here is typography, not a category list).
 */
const KEEP_CAPS = new Set(["os", "ip", "dns", "ha", "ssl", "tls", "vpn", "sso", "api", "cli", "id"]);

function pretty(name: string): string {
  return name
    .replace(/-/g, " ")
    .replace(/\b\w+/g, (w) =>
      KEEP_CAPS.has(w.toLowerCase())
        ? w.toUpperCase()
        : w.charAt(0).toUpperCase() + w.slice(1)
    );
}

function nodeHtml(node: CategoryNode, depth: number, active: string): string {
  const hasKids = node.children.length > 0;
  const isActive = active === node.path;
  const kidHtml = node.children
    .filter((c) => c.children.length > 0) // leaves are shown as the list pane
    .map((c) => nodeHtml(c, depth + 1, active))
    .join("");
  return `
    <details class="ref-branch" ${isActive || (active && active.startsWith(node.path + "/") && hasKids) ? "open" : ""} ${depth === 0 ? "open" : ""}>
      <summary data-cat="${esc(node.path)}">${esc(pretty(node.name))}${hasKids ? "" : `<span class="ref-count mono-data" data-count="${node.path}"></span>`}</summary>
      ${kidHtml}
    </details>`;
}

export function renderReference(el: HTMLElement, params: URLSearchParams): void {
  const active = params.get("cat") ?? "";

  el.innerHTML = `
    <div class="view-head">
      <h1>Reference</h1>
      <p>Commands, protocols, errors and standards — from your installed content packs.</p>
    </div>
    <div class="ref-layout">
      <nav class="card ref-tree" aria-label="Reference categories" id="ref-tree">
        <div class="dots" aria-label="loading"><i></i><i></i><i></i></div>
      </nav>
      <section class="card ref-list" id="ref-list">
        <p class="field-hint">Pick a category on the left.</p>
      </section>
    </div>
  `;

  const tree = el.querySelector<HTMLElement>("#ref-tree")!;
  const list = el.querySelector<HTMLElement>("#ref-list")!;

  const loadList = async (cat: string) => {
    list.innerHTML = `<div class="dots" aria-label="loading"><i></i><i></i><i></i></div>`;
    try {
      const rows = await invoke<[string, string, string][]>("content_list", {
        categoryPrefix: cat,
        kind: null,
      });
      if (!rows.length) {
        list.innerHTML = `<div class="empty"><p>No entries here yet.</p><p class="mono-data">${cat}</p></div>`;
        return;
      }
      list.innerHTML = `
        <h2 class="ref-list-title">${esc(pretty(cat.split("/").pop() ?? cat))}</h2>
        <p class="field-hint mono-data" style="margin-bottom:12px">${esc(cat)} · ${rows.length} entries</p>
        <ul class="list">${rows
          .map(
            ([id, title, kind]) =>
              `<li data-id="${esc(id)}">
                <span class="hit-kind">${esc(kind)}</span>
                <span class="hit-title">${esc(title)}</span>
                <span class="arrow">${icons.arrow}</span>
              </li>`
          )
          .join("")}</ul>`;
      list.querySelectorAll<HTMLLIElement>(".list > li").forEach((li) => {
        li.addEventListener("click", () => {
          location.hash = `/article?id=${encodeURIComponent(li.dataset.id!)}`;
        });
      });
      // record navigation in history
      void invoke("history_add", {
        id: `cat:${cat}`,
        title: pretty(cat.split("/").pop() ?? cat),
        kind: "category",
      }).catch(() => undefined);
    } catch (e) {
      list.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    }
  };

  const paintTree = (categories: string[]) => {
    const sections = buildTree(categories);
    if (!sections.length) {
      tree.innerHTML = `<div class="empty" style="padding:24px"><p>No content installed. Open Settings → Content packs.</p></div>`;
      return;
    }
    tree.innerHTML = sections.map((s) => nodeHtml(s, 0, active)).join("");
    tree.querySelectorAll<HTMLElement>("summary[data-cat]").forEach((s) => {
      s.addEventListener("click", (ev) => {
        // navigating on label click; toggling still works via the marker
        ev.preventDefault();
        location.hash = `/reference?cat=${s.dataset.cat}`;
      });
    });
  };

  void (async () => {
    try {
      const categories = await invoke<string[]>("content_categories");
      paintTree(categories);
      if (active) {
        await loadList(active);
      } else {
        list.innerHTML = `<div class="empty">
          <p>Select a category to browse its entries.</p>
          <p class="mono-data">Tip: Ctrl+K searches everything at once</p>
        </div>`;
      }
    } catch (e) {
      tree.innerHTML = `<div class="empty" style="padding:24px"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
      list.innerHTML = "";
    }
  })();
}
