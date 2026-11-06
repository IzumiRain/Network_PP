import { invoke } from "../lib/tauri";
import type { Packet, TcpFlags, NetworkLayer, TransportLayer, AppLayer } from "../lib/types";

// Sample: Ethernet + IPv4 + TCP + HTTP GET (built by the Rust test suite).
const SAMPLE_HEX =
  "ffffffffffff001a2b3c4d5e08004500006e1c4640004006d347c0a801325db8d822c73801bbb1122334400000000" +
  "50021000" + "0000" + "0000" + "474554202f696e6465782e68746d6c20485454502f312e310d0a486f73743a206578616d706c652e636f6d0d0a557365722d4167656e743a206e657470700d0a0d0a";

type Mode = "build" | "parse";

export function renderPackets(el: HTMLElement, params: URLSearchParams): void {
  const mode: Mode = params.get("mode") === "parse" ? "parse" : "build";

  el.innerHTML = `
    <div class="view-head">
      <h1>Packet visualizer</h1>
      <p>Edit a field, watch the bytes change — or paste a hex dump and read every layer.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:24px">
      <button role="tab" id="pk-build" aria-selected="${mode === "build"}">Build (live)</button>
      <button role="tab" id="pk-parse" aria-selected="${mode === "parse"}">Parse hex</button>
    </div>
    <div id="pk-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#pk-body")!;
  el.querySelector("#pk-build")!.addEventListener("click", () => (location.hash = "/packets?mode=build"));
  el.querySelector("#pk-parse")!.addEventListener("click", () => (location.hash = "/packets?mode=parse"));

  if (mode === "build") wireBuild(body);
  else wireParse(body);
}

// ---------- build mode ----------

function wireBuild(body: HTMLElement): void {
  body.innerHTML = `
    <div class="split-2" style="align-items:start">
      <section class="card">
        <h2>Packet fields</h2>
        <div class="panel-grid">
          <div class="split-2" style="gap:12px">
            <div class="field">
              <label for="b-src">Source IP (L3)</label>
              <input id="b-src" class="input input--mono" value="192.168.1.50" />
            </div>
            <div class="field">
              <label for="b-dst">Destination IP (L3)</label>
              <input id="b-dst" class="input input--mono" value="93.184.216.34" />
            </div>
            <div class="field">
              <label for="b-sport">Source port (L4)</label>
              <input id="b-sport" class="input input--mono" type="number" min="1" max="65535" value="51000" />
            </div>
            <div class="field">
              <label for="b-dport">Destination port (L4)</label>
              <input id="b-dport" class="input input--mono" type="number" min="1" max="65535" value="443" />
            </div>
          </div>
          <div class="field">
            <label>TCP flags</label>
            <div class="flag-group">
              ${["fin", "syn", "rst", "psh", "ack", "urg"]
                .map(
                  (f) => `<label class="check"><input type="checkbox" data-flag="${f}" ${
                    f === "syn" ? "checked" : ""
                  } /><span class="box"></span>${f.toUpperCase()}</label>`
                )
                .join("")}
            </div>
          </div>
          <div class="field">
            <label for="b-payload">L7 payload (plain text — sent as bytes)</label>
            <textarea id="b-payload" class="input input--mono" rows="4" spellcheck="false">GET /index.html HTTP/1.1
Host: example.com
User-Agent: netpp

</textarea>
            <p class="field-hint">A payload starting with an HTTP method renders as an HTTP request layer.</p>
          </div>
        </div>
      </section>

      <section class="card" style="min-width:0">
        <h2>Live output</h2>
        <div id="b-out" class="packet-layers"></div>
        <div class="field" style="margin-top:16px">
          <label>Hex dump (L3 datagram)</label>
          <pre id="b-hex" class="hexdump"></pre>
        </div>
      </section>
    </div>
  `;

  const out = body.querySelector<HTMLElement>("#b-out")!;
  const hex = body.querySelector<HTMLElement>("#b-hex")!;

  const readFlags = (): TcpFlags => {
    const g = (n: string) =>
      body.querySelector<HTMLInputElement>(`input[data-flag="${n}"]`)?.checked ?? false;
    return {
      fin: g("fin"),
      syn: g("syn"),
      rst: g("rst"),
      psh: g("psh"),
      ack: g("ack"),
      urg: g("urg"),
      ece: false,
      cwr: false,
    };
  };

  const update = async () => {
    const args = {
      src: (body.querySelector("#b-src") as HTMLInputElement).value,
      dst: (body.querySelector("#b-dst") as HTMLInputElement).value,
      sport: Number((body.querySelector("#b-sport") as HTMLInputElement).value),
      dport: Number((body.querySelector("#b-dport") as HTMLInputElement).value),
      ...readFlags(),
      payload: (body.querySelector("#b-payload") as HTMLTextAreaElement).value,
    };
    try {
      const hexStr = await invoke<string>("packet_build_tcp_hex", args);
      hex.innerHTML = payloadHexdump(hexStr);
      const packet = await invoke<Packet>("packet_parse", { bytesHex: hexStr });
      out.innerHTML = renderLayers(packet, false);
    } catch (e) {
      out.innerHTML = `<p class="field-error">${String(e).replace("Error: ", "")}</p>`;
      hex.textContent = "";
    }
  };

  body.querySelectorAll("input, textarea").forEach((i) => {
    i.addEventListener("input", () => void update());
  });
  void update();
}

// ---------- parse mode ----------

function wireParse(body: HTMLElement): void {
  body.innerHTML = `
    <section class="card" style="margin-bottom:16px">
      <div class="field">
        <label for="p-hex">Hex bytes — with or without Ethernet header, spaces optional</label>
        <textarea id="p-hex" class="input input--mono" rows="5" spellcheck="false" placeholder="45 00 00 3c …">${SAMPLE_HEX}</textarea>
        <div class="chips">
          <button class="chip" id="p-load-sample">Load HTTP sample</button>
        </div>
      </div>
    </section>
    <div id="p-out" class="packet-layers"></div>
  `;

  const input = body.querySelector<HTMLTextAreaElement>("#p-hex")!;
  const out = body.querySelector<HTMLElement>("#p-out")!;

  const update = async () => {
    try {
      const packet = await invoke<Packet>("packet_parse", { bytesHex: input.value });
      out.innerHTML = renderLayers(packet, true);
    } catch (e) {
      out.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    }
  };

  let timer: ReturnType<typeof setTimeout> | undefined;
  input.addEventListener("input", () => {
    clearTimeout(timer);
    timer = setTimeout(() => void update(), 150);
  });
  body.querySelector("#p-load-sample")!.addEventListener("click", () => {
    input.value = SAMPLE_HEX;
    void update();
  });
  void update();
}

// ---------- layer rendering ----------

function rows(pairs: [string, string | number | boolean][], copyValues = true): string {
  return `<dl class="kv">${pairs
    .map(([k, v]) => {
      const s = String(v);
      return `<dt>${k}</dt><dd class="${copyValues ? "copyable" : ""}" ${
        copyValues ? `data-copy="${s}"` : ""
      }>${s}</dd>`;
    })
    .join("")}</dl>`;
}

function renderLayers(p: Packet, showHexdump: boolean): string {
  const layers: string[] = [];

  if (p.link) {
    layers.push(
      `<details class="layer" open><summary><span class="badge badge--l2">L2</span> Ethernet <span class="mono-data" style="color:var(--text-muted)">${p.link.ethertype_name}</span></summary>
       <div class="layer-body">${rows([
         ["Destination MAC", p.link.destination],
         ["Source MAC", p.link.source],
         ["EtherType", `0x${p.link.ethertype.toString(16).padStart(4, "0")} (${p.link.ethertype_name})`],
       ])}</div></details>`
    );
  }

  if (p.network) {
    layers.push(networkHtml(p.network));
  }

  if (p.transport) {
    layers.push(transportHtml(p.transport));
  }

  if (p.application) {
    layers.push(appHtml(p.application));
  }

  if (p.payload_hex && !p.application) {
    layers.push(
      `<div class="layer"><summary style="display:flex;padding:9px 12px;gap:8px"><span class="badge badge--l4">DATA</span> ${p.payload_ascii.length} bytes payload</summary></div>`
    );
  }

  if (showHexdump && p.payload_hex) {
    layers.push(
      `<div class="field"><label>Application payload</label><pre class="hexdump">${payloadHexdump(
        p.payload_hex
      )}</pre></div>`
    );
  }

  return layers.join("") || `<div class="empty"><p>Paste bytes to decode, or switch to Build.</p></div>`;
}

function payloadHexdump(hexStr: string): string {
  const bytes = hexStr.match(/.{2}/g) ?? [];
  let out = "";
  for (let i = 0; i < bytes.length; i += 16) {
    const chunk = bytes.slice(i, i + 16);
    const hex = chunk.join(" ").padEnd(47, " ");
    const ascii = chunk
      .map((b) => {
        const v = parseInt(b, 16);
        return v >= 0x20 && v < 0x7f ? String.fromCharCode(v) : ".";
      })
      .join("");
    out += `<span class="offset">${(i).toString(16).padStart(8, "0")}</span>  ${hex}  ${ascii}\n`;
  }
  return out;
}

function networkHtml(n: NetworkLayer): string {
  if (n.kind === "ipv4") {
    const h = n.data;
    return `<details class="layer" open><summary><span class="badge">L3</span> IPv4 <span class="mono-data" style="color:var(--text-muted)">${h.source} → ${h.destination}</span></summary>
      <div class="layer-body">${rows([
        ["Version", h.version],
        ["IHL", `${h.ihl} words (${h.ihl * 4} bytes)`],
        ["DSCP / ECN", `${h.dscp} / ${h.ecn}`],
        ["Total length", h.total_length],
        ["Identification", `0x${h.identification.toString(16).padStart(4, "0")}`],
        ["Flags", `${h.dont_fragment ? "DF " : ""}${h.more_fragments ? "MF" : ""}`.trim() || "—"],
        ["Fragment offset", h.fragment_offset],
        ["TTL", h.ttl],
        ["Protocol", `${h.protocol} (${h.protocol_name})`],
        ["Header checksum", `0x${h.header_checksum.toString(16).padStart(4, "0")}`],
        ["Source", h.source],
        ["Destination", h.destination],
        ...(h.options.length ? ([["Options", h.options.join(" ")]] as [string, string][]) : []),
      ])}</div></details>`;
  }
  const h = n.data;
  return `<details class="layer" open><summary><span class="badge">L3</span> IPv6 <span class="mono-data" style="color:var(--text-muted)">${h.source} → ${h.destination}</span></summary>
    <div class="layer-body">${rows([
      ["Traffic class", h.traffic_class],
      ["Flow label", `0x${h.flow_label.toString(16)}`],
      ["Payload length", h.payload_length],
      ["Next header", `${h.next_header} (${h.next_header_name})`],
      ["Hop limit", h.hop_limit],
      ["Source", h.source],
      ["Destination", h.destination],
    ])}</div></details>`;
}

function transportHtml(t: TransportLayer): string {
  if (t.kind === "tcp") {
    const h = t.data;
    return `<details class="layer" open><summary><span class="badge badge--l4">L4</span> TCP <span class="mono-data" style="color:var(--text-muted)">port ${h.source_port} → ${h.destination_port} · ${h.flag_names.join("+") || "no flags"}</span></summary>
      <div class="layer-body">${rows([
        ["Source port", h.source_port],
        ["Destination port", h.destination_port],
        ["Sequence", h.sequence],
        ["Acknowledgment", h.acknowledgment],
        ["Data offset", `${h.data_offset} words (${h.data_offset * 4} bytes)`],
        ["Flags", h.flag_names.join(", ") || "—"],
        ["Window size", h.window_size],
        ["Checksum", `0x${h.checksum.toString(16).padStart(4, "0")}`],
        ["Urgent pointer", h.urgent_pointer],
        ...(h.options.length ? ([["Options", h.options.join(" ")]] as [string, string][]) : []),
      ])}</div></details>`;
  }
  if (t.kind === "udp") {
    const h = t.data;
    return `<details class="layer" open><summary><span class="badge badge--l4">L4</span> UDP <span class="mono-data" style="color:var(--text-muted)">port ${h.source_port} → ${h.destination_port}</span></summary>
      <div class="layer-body">${rows([
        ["Source port", h.source_port],
        ["Destination port", h.destination_port],
        ["Length", h.length],
        ["Checksum", h.checksum === 0 ? "0x0000 (none)" : `0x${h.checksum.toString(16).padStart(4, "0")}`],
      ])}</div></details>`;
  }
  const h = t.data;
  return `<details class="layer" open><summary><span class="badge badge--l4">L4</span> ICMP <span class="mono-data" style="color:var(--text-muted)">${h.type_name}</span></summary>
    <div class="layer-body">${rows([
      ["Type", h.message_type],
      ["Code", h.code],
      ["Meaning", h.type_name],
      ["Rest of header", h.rest_of_header],
    ])}</div></details>`;
}

function appHtml(a: AppLayer): string {
  if (a.kind === "http-request") {
    const h = a.data;
    return `<details class="layer" open><summary><span class="badge badge--l7">L7</span> HTTP request <span class="mono-data" style="color:var(--text-muted)">${h.method} ${h.target}</span></summary>
      <div class="layer-body">${rows([
        ["Method", h.method],
        ["Target", h.target],
        ["Version", h.version],
        ...h.headers.map(([k, v]) => [k, v] as [string, string]),
        ...(h.body ? ([["Body", h.body]] as [string, string][]) : []),
      ])}</div></details>`;
  }
  if (a.kind === "http-response") {
    const h = a.data;
    return `<details class="layer" open><summary><span class="badge badge--l7">L7</span> HTTP response <span class="mono-data" style="color:var(--text-muted)">${h.status_code} ${h.reason}</span></summary>
      <div class="layer-body">${rows([
        ["Version", h.version],
        ["Status", `${h.status_code} ${h.reason}`],
        ...h.headers.map(([k, v]) => [k, v] as [string, string]),
        ...(h.body ? ([["Body", h.body]] as [string, string][]) : []),
      ])}</div></details>`;
  }
  return "";
}
