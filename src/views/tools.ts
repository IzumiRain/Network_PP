import { renderCalc } from "./calc";
import { renderPackets } from "./packets";

type Tool = "cidr" | "ports" | "packets";

export function renderTools(el: HTMLElement, params: URLSearchParams): void {
  const tool: Tool = (["cidr", "ports", "packets"] as const).includes(
    params.get("tool") as Tool
  )
    ? (params.get("tool") as Tool)
    : "cidr";

  el.innerHTML = `
    <div class="view-head">
      <h1>Tools</h1>
      <p>Calculators and packet lab — everything computes locally, nothing leaves the machine.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:24px">
      <button role="tab" data-tool="cidr" aria-selected="${tool === "cidr"}">CIDR &amp; IP range</button>
      <button role="tab" data-tool="ports" aria-selected="${tool === "ports"}">Ports</button>
      <button role="tab" data-tool="packets" aria-selected="${tool === "packets"}">Packet lab</button>
    </div>
    <div id="tools-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#tools-body")!;
  el.querySelectorAll<HTMLButtonElement>(".segmented button[data-tool]").forEach((b) => {
    b.addEventListener("click", () => {
      location.hash = `/tools?tool=${b.dataset.tool}`;
    });
  });

  if (tool === "packets") {
    renderPackets(body, new URLSearchParams(params));
  } else {
    renderCalc(body, new URLSearchParams(`tab=${tool}`));
  }
}
