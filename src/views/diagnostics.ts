import { renderScan } from "./scan";
import { renderDnsPanel, renderHttpPanel, renderSystemPanel, renderSessionsPanel } from "./diagpanels";

type Tab = "target" | "local" | "conn" | "net" | "system" | "sessions";

const TAB_TITLE: Record<Tab, string> = {
  target: "Host discovery & port scan",
  local: "Local network inspection",
  conn: "Live connections",
  net: "DNS & HTTP probes",
  system: "System diagnostics",
  sessions: "Diagnostic sessions",
};

export function renderDiagnostics(el: HTMLElement, params: URLSearchParams): void {
  const tab: Tab = (["target", "local", "conn", "net", "system", "sessions"] as const).includes(
    params.get("tab") as Tab
  )
    ? (params.get("tab") as Tab)
    : "target";

  el.innerHTML = `
    <div class="view-head">
      <h1>Diagnostics</h1>
      <p>Connectivity, name resolution, HTTP probes, system state and repeatable test sessions.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:24px">
      <button role="tab" data-tab="target" aria-selected="${tab === "target"}">Scan a target</button>
      <button role="tab" data-tab="local" aria-selected="${tab === "local"}">Local devices</button>
      <button role="tab" data-tab="conn" aria-selected="${tab === "conn"}">Connections</button>
      <button role="tab" data-tab="net" aria-selected="${tab === "net"}">DNS &amp; HTTP</button>
      <button role="tab" data-tab="system" aria-selected="${tab === "system"}">System</button>
      <button role="tab" data-tab="sessions" aria-selected="${tab === "sessions"}">Sessions</button>
    </div>
    <p class="field-hint" style="margin:-8px 0 16px">${TAB_TITLE[tab]} · ${tab === "net" || tab === "system" || tab === "sessions" ? "" : "desktop module — "}${tab === "target" || tab === "local" || tab === "conn" ? "Android builds ship without scan tabs." : ""}</p>
    <div id="diag-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#diag-body")!;
  el.querySelectorAll<HTMLButtonElement>(".segmented button[data-tab]").forEach((b) => {
    b.addEventListener("click", () => {
      location.hash = `/diagnostics?tab=${b.dataset.tab}`;
    });
  });

  if (tab === "target") renderScan(body, new URLSearchParams("tab=target"));
  else if (tab === "local") renderScan(body, new URLSearchParams("tab=local"));
  else if (tab === "conn") renderScan(body, new URLSearchParams("tab=conn"));
  else if (tab === "net") {
    const wrap = document.createElement("div");
    body.appendChild(wrap);
    const dnsCard = document.createElement("div");
    const httpCard = document.createElement("div");
    body.appendChild(dnsCard);
    body.appendChild(httpCard);
    renderDnsPanel(dnsCard);
    renderHttpPanel(httpCard);
    wrap.remove();
  } else if (tab === "system") renderSystemPanel(body);
  else renderSessionsPanel(body);
}
