import { invoke } from "../lib/tauri";
import { icons } from "../lib/icons";

export function renderRunbooks(el: HTMLElement): void {
  el.innerHTML = `
    <div class="view-head">
      <h1>Runbooks</h1>
      <p>Step-by-step operational guides: what to check, in what order, and how to validate.</p>
    </div>
    <section class="card" id="rb-list">
      <div class="dots" aria-label="loading"><i></i><i></i><i></i></div>
    </section>
  `;

  const box = el.querySelector<HTMLElement>("#rb-list")!;

  void (async () => {
    try {
      const categories = await invoke<string[]>("content_categories");
      const rbCats = categories.filter((c) => c.startsWith("runbooks"));
      const groups: { cat: string; rows: [string, string, string][] }[] = [];
      for (const cat of rbCats) {
        const rows = await invoke<[string, string, string][]>("content_list", {
          categoryPrefix: cat,
          kind: "runbook",
        });
        if (rows.length) groups.push({ cat, rows });
      }
      if (!groups.length) {
        box.innerHTML = `<div class="empty"><p>No runbooks installed yet — they ship in content packs.</p></div>`;
        return;
      }
      box.innerHTML = groups
        .map(
          (g) => `
        <h2 class="ref-list-title">${g.cat.replace("runbooks/", "").replace(/-/g, " ")}</h2>
        <ul class="list" style="margin-bottom:20px">${g.rows
          .map(
            ([id, title]) =>
              `<li data-id="${id}">
                <span class="hit-kind">runbook</span>
                <span class="hit-title">${title}</span>
                <span class="arrow">${icons.arrow}</span>
              </li>`
          )
          .join("")}</ul>`
        )
        .join("");
      box.querySelectorAll<HTMLLIElement>(".list > li").forEach((li) => {
        li.addEventListener("click", () => {
          location.hash = `/article?id=${encodeURIComponent(li.dataset.id!)}`;
        });
      });
    } catch (e) {
      box.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    }
  })();
}
