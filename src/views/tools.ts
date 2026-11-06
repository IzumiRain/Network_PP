import { renderCalc } from "./calc";
import { renderPackets } from "./packets";
import {
  renderIpv6Panel,
  renderPerfPanel,
  renderConvertPanel,
  renderGeneratePanel,
  renderTextPanel,
} from "./toolpanels";

type Group = "calc" | "convert" | "generate" | "text";
type CalcTool = "cidr" | "ports" | "ipv6" | "perf" | "packets";

export function renderTools(el: HTMLElement, params: URLSearchParams): void {
  const group: Group = (["calc", "convert", "generate", "text"] as const).includes(
    params.get("group") as Group
  )
    ? (params.get("group") as Group)
    : "calc";
  const calcTool: CalcTool = (["cidr", "ports", "ipv6", "perf", "packets"] as const).includes(
    params.get("tool") as CalcTool
  )
    ? (params.get("tool") as CalcTool)
    : "cidr";

  const CALC_LABEL: Record<CalcTool, string> = {
    cidr: "CIDR & IP range",
    ports: "Ports",
    ipv6: "IPv6",
    perf: "Performance",
    packets: "Packet lab",
  };

  el.innerHTML = `
    <div class="view-head">
      <h1>Tools</h1>
      <p>Calculators, converters, generators and text utilities — everything computes locally.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:16px">
      <button role="tab" data-group="calc" aria-selected="${group === "calc"}">Calculators</button>
      <button role="tab" data-group="convert" aria-selected="${group === "convert"}">Converters</button>
      <button role="tab" data-group="generate" aria-selected="${group === "generate"}">Generators</button>
      <button role="tab" data-group="text" aria-selected="${group === "text"}">Text &amp; data</button>
    </div>
    ${
      group === "calc"
        ? `<div class="chips" style="margin-bottom:20px" id="tool-calc-tabs">
            ${(Object.keys(CALC_LABEL) as CalcTool[])
              .map(
                (t) =>
                  `<button class="chip" data-tool="${t}" aria-pressed="${calcTool === t}">${CALC_LABEL[t]}</button>`
              )
              .join("")}
          </div>`
        : ""
    }
    <div id="tools-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#tools-body")!;
  el.querySelectorAll<HTMLButtonElement>(".segmented button[data-group]").forEach((b) => {
    b.addEventListener("click", () => {
      location.hash = `/tools?group=${b.dataset.group}`;
    });
  });
  el.querySelectorAll<HTMLButtonElement>("#tool-calc-tabs .chip").forEach((c) => {
    c.addEventListener("click", () => {
      location.hash = `/tools?group=calc&tool=${c.dataset.tool}`;
    });
  });

  if (group === "calc") {
    if (calcTool === "packets") renderPackets(body, new URLSearchParams());
    else if (calcTool === "ipv6") renderIpv6Panel(body);
    else if (calcTool === "perf") renderPerfPanel(body);
    else renderCalc(body, new URLSearchParams(`tab=${calcTool}`));
  } else if (group === "convert") renderConvertPanel(body);
  else if (group === "generate") renderGeneratePanel(body);
  else renderTextPanel(body);
}
