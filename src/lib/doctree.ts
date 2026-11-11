import { invoke } from "./tauri";
import { icons } from "./icons";
import { esc } from "./safe";

/**
 * Shared nested document tree (v2 data-driven model, docs/PACK-SPEC.md).
 *
 * One component drives both Reference and Runbooks: branch nodes are the
 * segments of each entity's category path (Category → Company → Device →
 * Model …), document leaves are the entities themselves. The tree is
 * navigation only — no content is rendered inside it; tapping a document
 * leaf is left to the caller (the article slide-in).
 *
 * Everything is derived from `content_categories` + `content_list`; no
 * vendor, product or section name is ever hardcoded here.
 */

export interface DocLeaf {
  id: string;
  title: string;
  kind: string;
}

export interface DocTreeNode {
  /** Raw slug segment, e.g. `fortinet`. */
  name: string;
  /** Cumulative category path, e.g. `reference/vendors/fortinet`. */
  path: string;
  children: DocTreeNode[];
  /** Documents whose category is exactly this node's path. */
  docs: DocLeaf[];
  /** Total documents in this subtree (own docs + descendants). */
  docCount: number;
}

/**
 * Display names are derived generically from the slug — prettified, never
 * looked up in a hardcoded table. Any category a pack invents tomorrow
 * renders correctly with no code change. Well-known all-caps tokens keep
 * their casing (e.g. `os` would prettify to "Os" without this; casing here
 * is typography, not a category list).
 */
const KEEP_CAPS = new Set(["os", "ip", "dns", "ha", "ssl", "tls", "vpn", "sso", "api", "cli", "id"]);

export function prettySlug(name: string): string {
  return name
    .replace(/-/g, " ")
    .replace(/\b\w+/g, (w) =>
      KEEP_CAPS.has(w.toLowerCase())
        ? w.toUpperCase()
        : w.charAt(0).toUpperCase() + w.slice(1)
    );
}

type Row = [string, string, string];

export interface DocTreeFilter {
  /** Which category roots to include, decided by the first path segment
   *  (`reference`, `runbooks`, … — the schema roots are structural, see
   *  src-tauri/src/commands/content.rs). */
  includeRoot: (root: string) => boolean;
  /** Optional backend kind filter (`runbook` for the Runbooks view). */
  kind: string | null;
}

/**
 * Load the full document tree for a view.
 *
 * `content_list` matches by category *prefix* and does not return each
 * row's category, so a document's exact home node is recovered by set
 * difference: the rows listed for a category minus the rows listed for
 * every longer category that starts with it. That is exact because every
 * entity's category is itself in the `content_categories` set.
 */
export async function loadDocTree(filter: DocTreeFilter): Promise<DocTreeNode[]> {
  const categories = await invoke<string[]>("content_categories");
  const shown = categories.filter((c) => filter.includeRoot(c.split("/")[0] ?? ""));

  const rowsByCat = new Map<string, DocLeaf[]>();
  await Promise.all(
    shown.map(async (cat) => {
      const rows = await invoke<Row[]>("content_list", {
        categoryPrefix: cat,
        kind: filter.kind,
      });
      rowsByCat.set(
        cat,
        rows.map(([id, title, kind]) => ({ id, title, kind }))
      );
    })
  );

  const exactDocs = (cat: string): DocLeaf[] => {
    const mine = rowsByCat.get(cat) ?? [];
    if (!mine.length) return mine;
    const deeper = new Set<string>();
    for (const [other, rows] of rowsByCat) {
      if (other !== cat && other.startsWith(cat)) {
        for (const r of rows) deeper.add(r.id);
      }
    }
    return mine.filter((r) => !deeper.has(r.id));
  };

  const root: DocTreeNode = { name: "", path: "", children: [], docs: [], docCount: 0 };
  const byPath = new Map<string, DocTreeNode>([["", root]]);
  for (const cat of shown) {
    let node = root;
    let path = "";
    for (const part of cat.split("/").filter(Boolean)) {
      path = path ? `${path}/${part}` : part;
      let child = byPath.get(path);
      if (!child) {
        child = { name: part, path, children: [], docs: [], docCount: 0 };
        byPath.set(path, child);
        node.children.push(child);
      }
      node = child;
    }
    node.docs = exactDocs(cat);
  }

  const finish = (n: DocTreeNode): number => {
    n.children.sort((a, b) => a.name.localeCompare(b.name));
    n.docs.sort((a, b) => a.title.localeCompare(b.title));
    n.docCount =
      n.docs.length + n.children.reduce((sum, c) => sum + finish(c), 0);
    return n.docCount;
  };
  root.children.forEach(finish);
  // Nodes that carry nothing (category prefixes with no documents under
  // the active kind filter) are pruned — the tree shows documents only.
  const prune = (n: DocTreeNode) => {
    n.children = n.children.filter((c) => c.docCount > 0);
    n.children.forEach(prune);
  };
  prune(root);
  return root.children;
}

// ---------- rendering ----------

const chevronSvg = `<svg class="icon-svg dtree-chev" viewBox="0 0 24 24" aria-hidden="true"><path d="m9 6 6 6-6 6"/></svg>`;

function nodeHtml(node: DocTreeNode, depth: number, expanded: Set<string>): string {
  const open = expanded.has(node.path);
  const kids = [
    ...node.children.map((c) => nodeHtml(c, depth + 1, expanded)),
    ...node.docs.map(
      (d) => `
      <button class="dtree-doc" type="button" data-doc="${esc(d.id)}" style="--d:${depth + 1}">
        <span class="hit-kind">${esc(d.kind)}</span>
        <span class="dtree-doc-main">
          <span class="dtree-doc-title">${esc(d.title)}</span>
          <span class="dtree-doc-id mono-data">${esc(d.id)}</span>
        </span>
        <span class="dtree-arrow">${icons.arrow}</span>
      </button>`
    ),
  ].join("");
  return `
    <div class="dtree-node">
      <button class="dtree-row" type="button" data-node="${esc(node.path)}"
        aria-expanded="${open}" style="--d:${depth}">
        ${chevronSvg}
        <span class="dtree-label">${esc(prettySlug(node.name))}</span>
        <span class="dtree-count mono-data">${node.docCount}</span>
      </button>
      <div class="dtree-children${open ? " open" : ""}"><div class="dtree-inner">${kids}</div></div>
    </div>`;
}

export interface DocTreeMount {
  /** Expanded node paths — owned by the caller so expansion survives the
   *  article slide-in and view re-renders. */
  expanded: Set<string>;
  onOpenDoc: (id: string) => void;
  emptyText: string;
}

/** Render the tree into `container` and wire expand/collapse + doc taps. */
export function mountDocTree(
  container: HTMLElement,
  roots: DocTreeNode[],
  opts: DocTreeMount
): void {
  if (!roots.length) {
    container.innerHTML = `<div class="empty" style="padding:24px"><p>${esc(opts.emptyText)}</p></div>`;
    return;
  }
  container.innerHTML = roots.map((n) => nodeHtml(n, 0, opts.expanded)).join("");

  container.addEventListener("click", (e) => {
    const target = e.target as HTMLElement;
    const doc = target.closest<HTMLElement>("[data-doc]");
    if (doc) {
      opts.onOpenDoc(doc.dataset.doc!);
      return;
    }
    const row = target.closest<HTMLElement>("[data-node]");
    if (!row) return;
    const path = row.dataset.node!;
    const children = row.parentElement?.querySelector<HTMLElement>(":scope > .dtree-children");
    const open = row.getAttribute("aria-expanded") !== "true";
    row.setAttribute("aria-expanded", String(open));
    children?.classList.toggle("open", open);
    if (open) opts.expanded.add(path);
    else opts.expanded.delete(path);
  });
}
