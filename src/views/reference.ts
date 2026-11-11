import { loadDocTree, mountDocTree } from "../lib/doctree";

/**
 * Reference: one nested tree over every content root except `runbooks`
 * (that root has its own view). Branch rows only expand/collapse;
 * document leaves open the article slide-in (via the /article route, so
 * search hits, pins and deep links share the same path). Expansion state
 * lives at module level so it survives the slide-in and view switches.
 */
const expanded = new Set<string>();

export function renderReference(el: HTMLElement): void {
  el.innerHTML = `
    <div class="view-head">
      <h1>Reference</h1>
      <p>Commands, protocols, errors and standards — from your installed content packs.</p>
    </div>
    <section class="card doctree-card">
      <nav id="ref-tree" aria-label="Reference categories">
        <div class="dots" aria-label="loading"><i></i><i></i><i></i></div>
      </nav>
    </section>
  `;

  const treeBox = el.querySelector<HTMLElement>("#ref-tree")!;

  void (async () => {
    try {
      const roots = await loadDocTree({
        includeRoot: (root) => root !== "runbooks",
        kind: null,
      });
      // First visit: open the top level so the shape of the library shows.
      if (!expanded.size) {
        for (const n of roots) expanded.add(n.path);
      }
      mountDocTree(treeBox, roots, {
        expanded,
        onOpenDoc: (id) => {
          location.hash = `/article?id=${encodeURIComponent(id)}`;
        },
        emptyText: "No content installed. Open Settings → Content packs.",
      });
    } catch (e) {
      treeBox.innerHTML = `<div class="empty" style="padding:24px"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    }
  })();
}
