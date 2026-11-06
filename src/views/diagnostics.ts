import { renderScan } from "./scan";

type Tab = "target" | "local" | "conn";

const TAB_TITLE: Record<Tab, string> = {
  target: "Host discovery & port scan",
  local: "Local network inspection",
  conn: "Live connections",
};

export function renderDiagnostics(el: HTMLElement, params: URLSearchParams): void {
  const tab: Tab = (["target", "local", "conn"] as const).includes(
    params.get("tab") as Tab
  )
    ? (params.get("tab") as Tab)
    : "target";

  el.innerHTML = `
    <div class="view-head">
      <h1>Diagnostics</h1>
      <p>Connectivity, discovery and connection tables for this machine and the network around it.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:24px">
      <button role="tab" data-tab="target" aria-selected="${tab === "target"}">Scan a target</button>
      <button role="tab" data-tab="local" aria-selected="${tab === "local"}">Local devices</button>
      <button role="tab" data-tab="conn" aria-selected="${tab === "conn"}">Connections</button>
    </div>
    <p class="field-hint" style="margin:-8px 0 16px">${TAB_TITLE[tab]} · desktop module — Android builds ship without this section.</p>
    <div id="diag-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#diag-body")!;
  el.querySelectorAll<HTMLButtonElement>(".segmented button[data-tab]").forEach((b) => {
    b.addEventListener("click", () => {
      location.hash = `/diagnostics?tab=${b.dataset.tab}`;
    });
  });

  renderScan(body, new URLSearchParams(`tab=${tab}`));
}
