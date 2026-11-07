import { invoke } from "../lib/tauri";
import { renderMarkdown, bindCopyButtons } from "../lib/markdown";
import { esc } from "../lib/safe";
import type { Entity } from "../lib/types";

let el: HTMLElement;

const RISK_LABEL: Record<string, string> = {
  info: "Read-only",
  low: "Low risk",
  medium: "Caution",
  high: "Danger",
};

export function riskPill(risk: string | null): string {
  if (!risk) return "";
  const cls =
    risk === "high" ? "pill--filtered" : risk === "medium" ? "pill--time-wait" : "pill--open";
  return `<span class="pill ${cls}" title="Risk classification">${RISK_LABEL[risk] ?? risk}</span>`;
}

export function renderArticle(viewEl: HTMLElement, params: URLSearchParams): void {
  el = viewEl;
  const id = params.get("id") ?? "";
  el.innerHTML = `<div class="empty"><div class="dots" aria-label="loading"><i></i><i></i><i></i></div><p class="mono-data">loading ${id}</p></div>`;

  void (async () => {
    try {
      const entity = await invoke<Entity | null>("content_entity", { id });
      if (!entity) {
        el.innerHTML = `<div class="empty"><p>Content “${id}” was not found. It may live in a pack that is not installed — check Settings → Content packs.</p></div>`;
        return;
      }
      paint(entity);
      void invoke("history_add", { id: entity.id, title: entity.title, kind: entity.kind }).catch(
        () => undefined
      );
    } catch (e) {
      el.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    }
  })();
}

function paint(entity: Entity): void {
  const rendered = renderMarkdown(entity.body); // renderer escapes everything
  el.innerHTML = `
    <div class="view-head article-head">
      <div class="article-meta">
        <span class="badge article-badge">${esc(entity.kind)}</span>
        ${riskPill(entity.risk)}
        ${entity.version ? `<span class="mono-data" title="Content version">v${esc(entity.version)}</span>` : ""}
        ${entity.pack_id !== "core" ? `<span class="mono-data">pack: ${esc(entity.pack_id)}</span>` : ""}
      </div>
      <h1>${esc(entity.title)}</h1>
      <p class="mono-data">${esc(entity.category)}</p>
      <div class="article-actions">
        <button class="btn btn--ghost" id="art-fav" aria-pressed="false" type="button">☆ Favorite</button>
        <button class="btn btn--ghost" id="art-pin" aria-pressed="false" type="button">Pin to Home</button>
      </div>
    </div>
    <article class="card article-body" style="max-width:75ch">${rendered}</article>
    ${
      entity.tags.length
        ? `<div class="chips" style="margin-top:16px">${entity.tags
            .map((t) => `<span class="chip" aria-pressed="false">${esc(t)}</span>`)
            .join("")}</div>`
        : ""
    }
  `;
  bindCopyButtons(el.querySelector<HTMLElement>(".article-body")!);
  wireActions(entity);
  if (entity.kind === "runbook") void wireRunbookProgress(entity);
}

// runbook checkboxes persist per entity
async function wireRunbookProgress(entity: Entity): Promise<void> {
  const boxes = el.querySelectorAll<HTMLInputElement>(".article-body ul.tasks input[type=checkbox]");
  if (!boxes.length) return;
  const actions = el.querySelector<HTMLElement>(".article-actions")!;
  const reset = document.createElement("button");
  reset.className = "btn btn--ghost";
  reset.type = "button";
  reset.textContent = "Reset progress";
  actions.appendChild(reset);

  let t: ReturnType<typeof setTimeout> | undefined;
  const save = () => {
    clearTimeout(t);
    t = setTimeout(() => {
      const checked = Array.from(boxes).flatMap((b, i) => (b.checked ? [i] : []));
      void invoke("rb_progress_set", { entityId: entity.id, checked }).catch(() => undefined);
    }, 350);
  };

  try {
    const saved = await invoke<number[]>("rb_progress_get", { entityId: entity.id });
    saved.forEach((i) => {
      const b = boxes[i];
      if (b) b.checked = true;
    });
  } catch {
    // preview mode
  }
  boxes.forEach((b) => b.addEventListener("change", save));
  reset.addEventListener("click", async () => {
    boxes.forEach((b) => (b.checked = false));
    await invoke("rb_progress_clear", { entityId: entity.id }).catch(() => undefined);
    reset.textContent = "Cleared ✓";
    setTimeout(() => (reset.textContent = "Reset progress"), 1200);
  });
}

function wireActions(entity: Entity): void {
  const favBtn = el.querySelector<HTMLButtonElement>("#art-fav")!;
  const pinBtn = el.querySelector<HTMLButtonElement>("#art-pin")!;

  void invoke<boolean>("fav_is", { id: entity.id }).then((fav) => {
    favBtn.setAttribute("aria-pressed", String(fav));
    favBtn.textContent = fav ? "★ Favorite" : "☆ Favorite";
  }).catch(() => undefined);

  favBtn.addEventListener("click", async () => {
    const pressed = favBtn.getAttribute("aria-pressed") === "true";
    if (pressed) {
      await invoke("fav_remove", { id: entity.id });
      favBtn.setAttribute("aria-pressed", "false");
      favBtn.textContent = "☆ Favorite";
    } else {
      await invoke("fav_add", { id: entity.id, title: entity.title, kind: entity.kind });
      favBtn.setAttribute("aria-pressed", "true");
      favBtn.textContent = "★ Favorite";
    }
  });

  pinBtn.addEventListener("click", async () => {
    const route = `/article?id=${encodeURIComponent(entity.id)}`;
    await invoke("pin_add", {
      id: entity.id,
      title: entity.title,
      kind: entity.kind,
      route,
    });
    pinBtn.textContent = "Pinned ✓";
    setTimeout(() => (pinBtn.textContent = "Pin to Home"), 1200);
  });
}
