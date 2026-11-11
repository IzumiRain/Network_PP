import { loadDocTree, mountDocTree } from "../lib/doctree";

/**
 * Runbooks: the same nested document tree as Reference, filtered to the
 * `runbooks` category root and the runbook kind. Document leaves open the
 * article slide-in, where step checkboxes and progress live.
 */
const expanded = new Set<string>();

export function renderRunbooks(el: HTMLElement): void {
  el.innerHTML = `
    <div class="view-head">
      <h1>Runbooks</h1>
      <p>Step-by-step operational guides: what to check, in what order, and how to validate.</p>
    </div>
    <section class="card doctree-card">
      <nav id="rb-tree" aria-label="Runbook categories">
        <div class="dots" aria-label="loading"><i></i><i></i><i></i></div>
      </nav>
    </section>
  `;

  const treeBox = el.querySelector<HTMLElement>("#rb-tree")!;

  void (async () => {
    try {
      const roots = await loadDocTree({
        includeRoot: (root) => root === "runbooks",
        kind: "runbook",
      });
      if (!expanded.size) {
        for (const n of roots) expanded.add(n.path);
      }
      mountDocTree(treeBox, roots, {
        expanded,
        onOpenDoc: (id) => {
          location.hash = `/article?id=${encodeURIComponent(id)}`;
        },
        emptyText: "No runbooks installed yet — they ship in content packs.",
      });
    } catch (e) {
      treeBox.innerHTML = `<div class="empty" style="padding:24px"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    }
  })();
}
