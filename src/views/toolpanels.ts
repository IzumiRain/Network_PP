import { invoke } from "../lib/tauri";
import type {
  DiffLine,
  Generated,
  Ipv6Info,
  JsonVars,
  MacForms,
  MtuResult,
  NumberResult,
  OverheadResult,
  RegexResult,
  TransferResult,
  TimestampResult,
  Ipv4Forms,
} from "../lib/types2";

// ---------- shared helpers ----------

function debounce(fn: () => void, ms: number): () => void {
  let t: ReturnType<typeof setTimeout> | undefined;
  return () => {
    clearTimeout(t);
    t = setTimeout(fn, ms);
  };
}

function kv(label: string, value: string, hl = false): string {
  return `<dt>${label}</dt><dd class="${hl ? "hl" : ""} copyable" data-copy="${value}">${value}</dd>`;
}

function bindCopy(root: HTMLElement): void {
  root.querySelectorAll<HTMLElement>(".copyable").forEach((dd) => {
    dd.addEventListener("click", () => {
      navigator.clipboard.writeText(dd.dataset.copy ?? "");
      const old = dd.textContent;
      dd.textContent = "Copied ✓";
      setTimeout(() => (dd.textContent = old), 700);
    });
  });
}

function errBox(e: unknown): string {
  return `<p class="field-error">${String(e).replace("Error: ", "")}</p>`;
}

// ---------- IPv6 ----------

export function renderIpv6Panel(el: HTMLElement): void {
  el.innerHTML = `
    <section class="card">
      <div class="field">
        <label for="i6-input">Address — with or without /prefix</label>
        <input id="i6-input" class="input input--mono" placeholder="2001:db8::1/64" spellcheck="false" autofocus />
        <div class="chips">
          ${["2001:db8::1/64", "fe80::1", "::ffff:192.168.1.1", "fd00::1"]
            .map((x) => `<button class="chip" data-fill="${x}">${x}</button>`)
            .join("")}
        </div>
      </div>
      <div id="i6-out" style="margin-top:16px"></div>
    </section>
    <section class="card" style="margin-top:16px">
      <h2>EUI-64 interface identifier</h2>
      <div class="toolbar">
        <div class="field">
          <label for="i6-mac">MAC address</label>
          <input id="i6-mac" class="input input--mono" placeholder="00:1a:2b:3c:4d:5e" spellcheck="false" />
        </div>
        <span id="i6-eui-out" class="mono-data" style="align-self:center"></span>
      </div>
    </section>
  `;

  const input = el.querySelector<HTMLInputElement>("#i6-input")!;
  const out = el.querySelector<HTMLElement>("#i6-out")!;

  const live = debounce(async () => {
    if (!input.value.trim()) {
      out.innerHTML = "";
      return;
    }
    try {
      const info = await invoke<Ipv6Info>("ipv6_calculate", { input: input.value });
      out.innerHTML = `<dl class="kv">
        ${kv("Compressed (RFC 5952)", info.compressed, true)}
        ${kv("Expanded", info.expanded)}
        ${kv("Prefix", `/${info.prefix}`)}
        ${kv("Network", info.network, true)}
        ${kv("First address", info.first_address)}
        ${kv("Last address", info.last_address)}
        ${kv("Total addresses", info.total_addresses)}
        ${kv("Type", info.address_type)}
        ${kv("Binary", info.binary)}
      </dl>`;
      bindCopy(out);
    } catch (e) {
      out.innerHTML = errBox(e);
    }
  }, 120);
  input.addEventListener("input", live);
  el.querySelectorAll<HTMLButtonElement>(".chip[data-fill]").forEach((c) =>
    c.addEventListener("click", () => {
      input.value = c.dataset.fill!;
      input.dispatchEvent(new Event("input"));
    })
  );
  void live();

  const mac = el.querySelector<HTMLInputElement>("#i6-mac")!;
  const euiOut = el.querySelector<HTMLElement>("#i6-eui-out")!;
  const euiLive = debounce(async () => {
    if (!mac.value.trim()) {
      euiOut.textContent = "";
      return;
    }
    try {
      euiOut.innerHTML = `<span class="pill pill--open">EUI-64</span> ${await invoke<string>("ipv6_eui64", { mac: mac.value })}`;
    } catch (e) {
      euiOut.textContent = String(e).replace("Error: ", "");
    }
  }, 120);
  mac.addEventListener("input", euiLive);
}

// ---------- performance ----------

export function renderPerfPanel(el: HTMLElement): void {
  el.innerHTML = `
    <div class="split-2" style="align-items:start">
      <section class="card">
        <h2>Transfer time</h2>
        <div class="toolbar">
          <div class="field">
            <label for="pf-size">Size</label>
            <input id="pf-size" class="input input--mono" type="number" min="1" value="1073741824" />
          </div>
          <div class="field" style="max-width:150px">
            <label for="pf-unit">Unit</label>
            <select id="pf-unit" class="input">
              <option value="1048576">MiB</option>
              <option value="1000000">MB</option>
              <option value="1073741824" selected>GiB</option>
              <option value="1000000000">GB</option>
              <option value="1">byte</option>
            </select>
          </div>
          <div class="field" style="max-width:150px">
            <label for="pf-bw">Link (Mbps)</label>
            <input id="pf-bw" class="input input--mono" type="number" min="0.1" step="0.1" value="100" />
          </div>
        </div>
        <div id="pf-transfer"></div>
      </section>
      <section class="card">
        <h2>MTU &amp; MSS</h2>
        <div class="toolbar">
          <div class="field" style="max-width:160px">
            <label for="pf-mtu">MTU</label>
            <input id="pf-mtu" class="input input--mono" type="number" value="1500" min="576" max="65535" />
          </div>
        </div>
        <div id="pf-mtu-out"></div>
      </section>
    </div>
    <section class="card" style="margin-top:16px">
      <h2>Per-packet overhead (Ethernet + IPv4 + TCP)</h2>
      <div class="toolbar">
        <div class="field" style="max-width:220px">
          <label for="pf-payload">Application payload (bytes)</label>
          <input id="pf-payload" class="input input--mono" type="number" min="1" value="1000000" />
        </div>
      </div>
      <div id="pf-overhead"></div>
    </section>
  `;

  const transferOut = el.querySelector<HTMLElement>("#pf-transfer")!;
  const mtuOut = el.querySelector<HTMLElement>("#pf-mtu-out")!;
  const ohOut = el.querySelector<HTMLElement>("#pf-overhead")!;

  const live = debounce(async () => {
    const size = Number((el.querySelector("#pf-size") as HTMLInputElement).value) *
      Number((el.querySelector("#pf-unit") as HTMLSelectElement).value);
    const bw = Number((el.querySelector("#pf-bw") as HTMLInputElement).value);
    try {
      const r = await invoke<TransferResult>("calc_transfer", { sizeBytes: Math.round(size), bandwidthMbps: bw });
      transferOut.innerHTML = `<dl class="kv">
        ${kv("Time", r.human_time, true)}
        ${kv("Seconds", r.seconds.toFixed(3))}
        ${kv("Bytes", r.size_bytes.toLocaleString("en-US"))}
      </dl>`;
    } catch (e) {
      transferOut.innerHTML = errBox(e);
    }
  }, 150);

  const mtuLive = debounce(async () => {
    const mtu = Number((el.querySelector("#pf-mtu") as HTMLInputElement).value);
    try {
      const r = await invoke<MtuResult>("calc_mtu", { mtu });
      mtuOut.innerHTML = `<dl class="kv">
        ${kv("TCP MSS", String(r.tcp_mss), true)}
        ${kv("UDP payload max", String(r.udp_payload_max))}
        ${kv("IPv4 minimum MTU", String(r.ipv4_min_mtu))}
        ${kv("Ping test size", r.pings_bytes)}
      </dl>`;
    } catch (e) {
      mtuOut.innerHTML = errBox(e);
    }
  }, 150);

  const ohLive = debounce(async () => {
    const payload = Number((el.querySelector("#pf-payload") as HTMLInputElement).value);
    try {
      const r = await invoke<OverheadResult>("calc_overhead", { payloadBytes: payload });
      ohOut.innerHTML = `<dl class="kv">
        ${kv("Packets", r.packets.toLocaleString("en-US"), true)}
        ${kv("On the wire", r.total_wire_bytes.toLocaleString("en-US") + " bytes")}
        ${kv("Efficiency", r.efficiency_percent.toFixed(2) + "%")}
      </dl>`;
    } catch (e) {
      ohOut.innerHTML = errBox(e);
    }
  }, 150);

  el.querySelectorAll("input, select").forEach((i) => {
    i.addEventListener("input", () => {
      live();
      mtuLive();
      ohLive();
    });
  });
  live();
  mtuLive();
  ohLive();
}

// ---------- converters ----------

type ConvMode = "num" | "ip" | "mac" | "b64" | "url" | "hex" | "ts";
const CONV_LABEL: Record<ConvMode, string> = {
  num: "Number systems",
  ip: "IPv4 forms",
  mac: "MAC formats",
  b64: "Base64",
  url: "URL encoding",
  hex: "Text ↔ Hex",
  ts: "Timestamp",
};

export function renderConvertPanel(el: HTMLElement): void {
  el.innerHTML = `
    <section class="card">
      <div class="chips" id="cv-modes" style="margin-bottom:16px">
        ${(Object.keys(CONV_LABEL) as ConvMode[]).map((m) => `<button class="chip" data-mode="${m}" aria-pressed="${m === "num"}">${CONV_LABEL[m]}</button>`).join("")}
      </div>
      <div id="cv-body"></div>
    </section>
  `;

  const body = el.querySelector<HTMLElement>("#cv-body")!;
  const paint = (mode: ConvMode) => {
    el.querySelectorAll<HTMLButtonElement>("#cv-modes .chip").forEach((c) =>
      c.setAttribute("aria-pressed", String(c.dataset.mode === mode))
    );
    body.innerHTML = "";

    const monoIn = (id: string, label: string, placeholder: string) => `
      <div class="field">
        <label for="${id}">${label}</label>
        <input id="${id}" class="input input--mono" placeholder="${placeholder}" spellcheck="false" autofocus />
      </div>`;

    if (mode === "num") {
      body.innerHTML = `
        <div class="toolbar">
          ${monoIn("cv-input", "Value (with 0x / 0b prefix optional)", "255 or 0xff or 0b11111111")}
          <div class="field" style="max-width:130px">
            <label for="cv-base">Base</label>
            <select id="cv-base" class="input"><option>10</option><option>2</option><option>16</option><option>8</option></select>
          </div>
        </div>
        <div id="cv-out"></div>`;
      const input = body.querySelector<HTMLInputElement>("#cv-input")!;
      const out = body.querySelector<HTMLElement>("#cv-out")!;
      const live = debounce(async () => {
        if (!input.value.trim()) return (out.innerHTML = "");
        try {
          const r = await invoke<NumberResult>("num_convert", {
            input: input.value,
            base: Number((body.querySelector("#cv-base") as HTMLSelectElement).value),
          });
          out.innerHTML = `<dl class="kv">
            ${kv("Decimal", r.decimal, true)}${kv("Binary", r.binary)}${kv("Hex", r.hex)}${kv("Octal", r.octal)}${kv("Significant bits", String(r.bits))}
          </dl>`;
          bindCopy(out);
        } catch (e) {
          out.innerHTML = errBox(e);
        }
      }, 120);
      input.addEventListener("input", live);
      (body.querySelector("#cv-base") as HTMLSelectElement).addEventListener("change", live);
    } else if (mode === "ip") {
      body.innerHTML = `${monoIn("cv-input", "Dotted quad or 32-bit integer", "192.168.1.1")}<div id="cv-out"></div>`;
      const input = body.querySelector<HTMLInputElement>("#cv-input")!;
      const out = body.querySelector<HTMLElement>("#cv-out")!;
      const live = debounce(async () => {
        if (!input.value.trim()) return (out.innerHTML = "");
        try {
          const r = await invoke<Ipv4Forms>("ipv4_forms", { input: input.value });
          out.innerHTML = `<dl class="kv">${kv("Dotted", r.dotted, true)}${kv("Integer", r.integer)}${kv("Binary", r.binary)}${kv("Hex", r.hex)}</dl>`;
          bindCopy(out);
        } catch (e) {
          out.innerHTML = errBox(e);
        }
      }, 120);
      input.addEventListener("input", live);
    } else if (mode === "mac") {
      body.innerHTML = `${monoIn("cv-input", "Any MAC format", "00:1a:2b:3c:4d:5e")}<div id="cv-out"></div>`;
      const input = body.querySelector<HTMLInputElement>("#cv-input")!;
      const out = body.querySelector<HTMLElement>("#cv-out")!;
      const live = debounce(async () => {
        if (!input.value.trim()) return (out.innerHTML = "");
        try {
          const r = await invoke<MacForms>("mac_forms", { input: input.value });
          out.innerHTML = `<dl class="kv">${kv("Normalized", r.normalized, true)}${kv("Windows", r.windows)}${kv("Cisco", r.cisco)}${kv("Bare", r.bare)}${kv("OUI (vendor half)", r.oui)}${kv("NIC half", r.nic)}</dl>`;
          bindCopy(out);
        } catch (e) {
          out.innerHTML = errBox(e);
        }
      }, 120);
      input.addEventListener("input", live);
    } else if (mode === "b64" || mode === "url" || mode === "hex") {
      const cmd = mode === "b64" ? "base64_convert" : mode === "url" ? "url_convert" : "hex_convert";
      body.innerHTML = `
        <div class="field">
          <label for="cv-input">Input</label>
          <textarea id="cv-input" class="input input--mono" rows="3" spellcheck="false"></textarea>
        </div>
        <div class="toolbar" style="margin-top:12px">
          <button class="btn" id="cv-enc">Encode →</button>
          <button class="btn btn--ghost" id="cv-dec">← Decode</button>
        </div>
        <div class="field">
          <label for="cv-out2">Output</label>
          <textarea id="cv-out2" class="input input--mono" rows="3" readonly></textarea>
        </div>`;
      const input = body.querySelector<HTMLTextAreaElement>("#cv-input")!;
      const out = body.querySelector<HTMLTextAreaElement>("#cv-out2")!;
      const run = async (decode: boolean) => {
        try {
          out.value = await invoke<string>(cmd, { text: input.value, decode });
        } catch (e) {
          out.value = String(e).replace("Error: ", "");
        }
      };
      body.querySelector("#cv-enc")!.addEventListener("click", () => void run(false));
      body.querySelector("#cv-dec")!.addEventListener("click", () => void run(true));
    } else {
      // timestamp
      body.innerHTML = `
        <div class="toolbar">
          ${monoIn("cv-unix", "Unix seconds", "1700000000")}
          <button class="btn btn--ghost" id="cv-now">Now</button>
        </div>
        ${monoIn("cv-iso", "…or ISO 8601 (UTC)", "2023-11-14T22:13:20Z")}
        <div id="cv-out" style="margin-top:16px"></div>`;
      const unixIn = body.querySelector<HTMLInputElement>("#cv-unix")!;
      const isoIn = body.querySelector<HTMLInputElement>("#cv-iso")!;
      const out = body.querySelector<HTMLElement>("#cv-out")!;
      const paint = (r: TimestampResult) => {
        out.innerHTML = `<dl class="kv">${kv("Unix", String(r.unix), true)}${kv("ISO 8601 (UTC)", r.iso_utc)}</dl>`;
        bindCopy(out);
      };
      unixIn.addEventListener("input", debounce(async () => {
        if (!unixIn.value.trim()) return (out.innerHTML = "");
        try {
          paint(await invoke<TimestampResult>("timestamp_from_unix_cmd", { unix: Number(unixIn.value) }));
        } catch (e) {
          out.innerHTML = errBox(e);
        }
      }, 150));
      isoIn.addEventListener("input", debounce(async () => {
        if (!isoIn.value.trim()) return (out.innerHTML = "");
        try {
          paint(await invoke<TimestampResult>("timestamp_parse_cmd", { iso: isoIn.value }));
        } catch (e) {
          out.innerHTML = errBox(e);
        }
      }, 150));
      body.querySelector("#cv-now")!.addEventListener("click", () => {
        unixIn.value = String(Math.floor(Date.now() / 1000));
        unixIn.dispatchEvent(new Event("input"));
      });
    }
  };

  el.querySelectorAll<HTMLButtonElement>("#cv-modes .chip").forEach((c) =>
    c.addEventListener("click", () => paint(c.dataset.mode as ConvMode))
  );
  paint("num");
}

// ---------- generators ----------

type GenMode = "uuid" | "mac" | "ip" | "password" | "hash";
const GEN_LABEL: Record<GenMode, string> = {
  uuid: "UUID v4",
  mac: "MAC",
  ip: "IPv4",
  password: "Password",
  hash: "Hash (SHA-256/512)",
};

export function renderGeneratePanel(el: HTMLElement): void {
  el.innerHTML = `
    <section class="card">
      <div class="chips" id="gn-modes" style="margin-bottom:16px">
        ${(Object.keys(GEN_LABEL) as GenMode[]).map((m) => `<button class="chip" data-mode="${m}" aria-pressed="${m === "uuid"}">${GEN_LABEL[m]}</button>`).join("")}
      </div>
      <div id="gn-body"></div>
    </section>
  `;

  const body = el.querySelector<HTMLElement>("#gn-body")!;
  const paint = async (mode: GenMode) => {
    el.querySelectorAll<HTMLButtonElement>("#gn-modes .chip").forEach((c) =>
      c.setAttribute("aria-pressed", String(c.dataset.mode === mode))
    );

    if (mode === "hash") {
      body.innerHTML = `
        <div class="field">
          <label for="gn-text">Text</label>
          <textarea id="gn-text" class="input input--mono" rows="3" spellcheck="false"></textarea>
        </div>
        <div id="gn-out" style="margin-top:12px"></div>`;
      const input = body.querySelector<HTMLTextAreaElement>("#gn-text")!;
      const out = body.querySelector<HTMLElement>("#gn-out")!;
      const live = debounce(async () => {
        if (!input.value) return (out.innerHTML = "");
        const r = await invoke<Generated>("hash_text_cmd", { text: input.value });
        out.innerHTML = `<dl class="kv">${kv("SHA-256", r.value, true)}${kv("SHA-512", r.extra ?? "")}</dl>`;
        bindCopy(out);
      }, 150);
      input.addEventListener("input", live);
      return;
    }

    const isCount = mode === "uuid" || mode === "mac" || mode === "ip";
    body.innerHTML = `
      <div class="toolbar">
        <div class="field" style="max-width:150px">
          <label for="gn-count">Count</label>
          <input id="gn-count" class="input input--mono" type="number" min="1" max="50" value="${mode === "password" ? 1 : 5}" />
        </div>
        ${
          mode === "ip"
            ? `<label class="check"><input id="gn-priv" type="checkbox" checked /><span class="box"></span>Private ranges only</label>`
            : ""
        }
        ${
          mode === "password"
            ? `<div class="field" style="max-width:150px"><label for="gn-len">Length</label>
               <input id="gn-len" class="input input--mono" type="number" min="8" max="128" value="20" /></div>
               <label class="check"><input id="gn-sym" type="checkbox" checked /><span class="box"></span>Symbols</label>`
            : ""
        }
        <button class="btn" id="gn-go">Generate</button>
      </div>
      <div class="field">
        <label for="gn-out">Output</label>
        <textarea id="gn-out" class="input input--mono" rows="${mode === "password" ? 2 : 6}" readonly></textarea>
      </div>
      <p class="field-hint" id="gn-extra"></p>
    `;

    const go = async () => {
      const out = body.querySelector<HTMLTextAreaElement>("#gn-out")!;
      const extra = body.querySelector<HTMLElement>("#gn-extra")!;
      const count = Number((body.querySelector("#gn-count") as HTMLInputElement).value);
      try {
        const r = await invoke<Generated>(
          mode === "uuid" ? "uuid_gen" : mode === "mac" ? "gen_mac" : mode === "ip" ? "gen_ipv4" : "gen_password",
          mode === "uuid" ? { count }
            : mode === "mac" ? { count }
            : mode === "ip" ? { count, privateOnly: (body.querySelector("#gn-priv") as HTMLInputElement)?.checked ?? false }
            : { length: Number((body.querySelector("#gn-len") as HTMLInputElement).value), symbols: (body.querySelector("#gn-sym") as HTMLInputElement)?.checked ?? true }
        );
        out.value = r.value;
        extra.textContent = r.extra ?? "";
      } catch (e) {
        out.value = String(e).replace("Error: ", "");
      }
    };
    body.querySelector("#gn-go")!.addEventListener("click", () => void go());
    if (!isCount) void go();
  };

  el.querySelectorAll<HTMLButtonElement>("#gn-modes .chip").forEach((c) =>
    c.addEventListener("click", () => void paint(c.dataset.mode as GenMode))
  );
  void paint("uuid");
}

// ---------- text & data ----------

type TextMode = "json" | "regex" | "diff" | "case";
const TEXT_LABEL: Record<TextMode, string> = {
  json: "JSON formatter",
  regex: "Regex tester",
  diff: "Line diff",
  case: "Case & whitespace",
};

export function renderTextPanel(el: HTMLElement): void {
  el.innerHTML = `
    <section class="card">
      <div class="chips" id="tx-modes" style="margin-bottom:16px">
        ${(Object.keys(TEXT_LABEL) as TextMode[]).map((m) => `<button class="chip" data-mode="${m}" aria-pressed="${m === "json"}">${TEXT_LABEL[m]}</button>`).join("")}
      </div>
      <div id="tx-body"></div>
    </section>
  `;

  const body = el.querySelector<HTMLElement>("#tx-body")!;
  const paint = (mode: TextMode) => {
    el.querySelectorAll<HTMLButtonElement>("#tx-modes .chip").forEach((c) =>
      c.setAttribute("aria-pressed", String(c.dataset.mode === mode))
    );
    body.innerHTML = "";

    if (mode === "json") {
      body.innerHTML = `
        <div class="field">
          <label for="tx-input">JSON</label>
          <textarea id="tx-input" class="input input--mono" rows="6" spellcheck="false">{"app":"Network++","offline":true,"tools":["cidr","packets"]}</textarea>
        </div>
        <div class="toolbar" style="margin-top:12px">
          <button class="btn" id="tx-fmt">Format</button>
          <span id="tx-status" class="field-hint" style="align-self:center"></span>
        </div>
        <div class="field">
          <label for="tx-out">Formatted</label>
          <textarea id="tx-out" class="input input--mono" rows="8" readonly></textarea>
        </div>`;
      const input = body.querySelector<HTMLTextAreaElement>("#tx-input")!;
      const out = body.querySelector<HTMLTextAreaElement>("#tx-out")!;
      const status = body.querySelector<HTMLElement>("#tx-status")!;
      const run = debounce(async () => {
        const r = await invoke<JsonVars>("json_format_cmd", { text: input.value, indent: 2 });
        status.textContent = r.valid ? "Valid ✓" : `Invalid — ${r.error}`;
        out.value = r.formatted;
      }, 200);
      input.addEventListener("input", run);
      body.querySelector("#tx-fmt")!.addEventListener("click", run);
      void run();
    } else if (mode === "regex") {
      body.innerHTML = `
        <div class="toolbar">
          <div class="field" style="flex:1">
            <label for="tx-pattern">Pattern</label>
            <input id="tx-pattern" class="input input--mono" placeholder="(\\d+)\\.(\\d+)\\.(\\d+)" spellcheck="false" />
          </div>
        </div>
        <div class="field">
          <label for="tx-text">Text</label>
          <textarea id="tx-text" class="input input--mono" rows="4" spellcheck="false">version v1.2.3 released, then v10.20.30</textarea>
        </div>
        <div id="tx-out" style="margin-top:12px"></div>`;
      const pattern = body.querySelector<HTMLInputElement>("#tx-pattern")!;
      const text = body.querySelector<HTMLTextAreaElement>("#tx-text")!;
      const out = body.querySelector<HTMLElement>("#tx-out")!;
      const run = debounce(async () => {
        if (!pattern.value) return (out.innerHTML = "");
        const r = await invoke<RegexResult>("regex_test_cmd", { pattern: pattern.value, text: text.value });
        if (!r.valid) {
          out.innerHTML = errBox(r.error);
          return;
        }
        out.innerHTML = `<p class="field-hint" style="margin-bottom:8px">${r.match_count} match${r.match_count === 1 ? "" : "es"}</p>
          <div class="table-wrap"><table class="data">
            <thead><tr><th>Match</th><th>Position</th><th>Groups</th></tr></thead>
            <tbody>${r.matches.map((m) => `<tr><td>${m.text}</td><td>${m.start}–${m.end}</td><td class="wrap">${m.groups.join(", ") || "—"}</td></tr>`).join("")}</tbody>
          </table></div>`;
      }, 200);
      pattern.addEventListener("input", run);
      text.addEventListener("input", run);
      void run();
    } else if (mode === "diff") {
      body.innerHTML = `
        <div class="split-2">
          <div class="field"><label for="tx-a">Original</label><textarea id="tx-a" class="input input--mono" rows="7" spellcheck="false">interface GigabitEthernet0/1
 switchport mode access
 switchport access vlan 10</textarea></div>
          <div class="field"><label for="tx-b">Changed</label><textarea id="tx-b" class="input input--mono" rows="7" spellcheck="false">interface GigabitEthernet0/1
 switchport mode trunk
 switchport trunk allowed vlan 10,20</textarea></div>
        </div>
        <div id="tx-out" style="margin-top:12px"></div>`;
      const a = body.querySelector<HTMLTextAreaElement>("#tx-a")!;
      const b = body.querySelector<HTMLTextAreaElement>("#tx-b")!;
      const out = body.querySelector<HTMLElement>("#tx-out")!;
      const run = debounce(async () => {
        const lines = await invoke<DiffLine[]>("diff_lines_cmd", { a: a.value, b: b.value });
        out.innerHTML = `<div class="diff-box">${lines
          .map((l) => {
            const cls = l.op === "add" ? "diff-add" : l.op === "del" ? "diff-del" : "diff-same";
            const mark = l.op === "add" ? "+" : l.op === "del" ? "−" : " ";
            return `<div class="${cls}"><span class="mono-data">${mark}</span> ${l.text || " "}</div>`;
          })
          .join("")}</div>`;
      }, 200);
      a.addEventListener("input", run);
      b.addEventListener("input", run);
      void run();
    } else {
      body.innerHTML = `
        <div class="field">
          <label for="tx-input">Text</label>
          <textarea id="tx-input" class="input input--mono" rows="4" spellcheck="false">hello world-example</textarea>
        </div>
        <div class="chips" style="margin:12px 0">
          ${["camel", "pascal", "snake", "kebab", "title", "upper", "lower"].map((s) => `<button class="chip" data-style="${s}">${s}</button>`).join("")}
        </div>
        <div class="field"><label for="tx-out">Result</label><textarea id="tx-out" class="input input--mono" rows="2" readonly></textarea></div>
        <div class="toolbar">
          <label class="check"><input id="tx-collapse" type="checkbox" checked /><span class="box"></span>Collapse whitespace</label>
          <label class="check"><input id="tx-trim" type="checkbox" checked /><span class="box"></span>Trim lines</label>
          <button class="btn btn--ghost" id="tx-clean">Clean</button>
        </div>
        <div class="field"><label for="tx-out2">Cleaned</label><textarea id="tx-out2" class="input input--mono" rows="2" readonly></textarea></div>`;
      const input = body.querySelector<HTMLTextAreaElement>("#tx-input")!;
      const out = body.querySelector<HTMLTextAreaElement>("#tx-out")!;
      body.querySelectorAll<HTMLButtonElement>(".chip[data-style]").forEach((c) =>
        c.addEventListener("click", async () => {
          try {
            out.value = await invoke<string>("case_convert_cmd", { text: input.value, style: c.dataset.style });
          } catch (e) {
            out.value = String(e).replace("Error: ", "");
          }
        })
      );
      const cleanBtn = body.querySelector<HTMLButtonElement>("#tx-clean")!;
      const out2 = body.querySelector<HTMLTextAreaElement>("#tx-out2")!;
      cleanBtn.addEventListener("click", async () => {
        out2.value = await invoke<string>("clean_whitespace_cmd", {
          text: input.value,
          collapse: (body.querySelector("#tx-collapse") as HTMLInputElement).checked,
          trimLines: (body.querySelector("#tx-trim") as HTMLInputElement).checked,
        });
      });
    }
  };

  el.querySelectorAll<HTMLButtonElement>("#tx-modes .chip").forEach((c) =>
    c.addEventListener("click", () => paint(c.dataset.mode as TextMode))
  );
  paint("json");
}
