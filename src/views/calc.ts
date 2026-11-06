import { invoke } from "../lib/tauri";
import type { CidrInfo, PortReport, PortInfo } from "../lib/types";

interface CalcSections {
  input: HTMLInputElement;
  error: HTMLElement;
  result: HTMLElement;
}

function kvRow(label: string, value: string, hl = false, copy = true): string {
  const cls = hl ? "hl" : "";
  return copy
    ? `<dt>${label}</dt><dd class="${cls} copyable" data-copy="${value}" title="Click to copy">${value}</dd>`
    : `<dt>${label}</dt><dd class="${cls}">${value}</dd>`;
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

function cidrResultHtml(c: CidrInfo): string {
  return `
    <dl class="kv">
      ${kvRow("Network", c.network, true)}
      ${kvRow("Broadcast", c.broadcast)}
      ${kvRow("First host", c.first_host)}
      ${kvRow("Last host", c.last_host)}
      ${kvRow("Netmask", c.netmask)}
      ${kvRow("Wildcard", c.wildcard)}
      ${kvRow("Prefix", `/${c.prefix}`)}
      ${kvRow("Total addresses", c.total_addresses.toLocaleString("en-US"))}
      ${kvRow("Usable hosts", c.usable_hosts.toLocaleString("en-US"), true)}
      ${kvRow("Class", c.ip_class, false, false)}
      ${kvRow("Type", c.address_type, false, false)}
      ${kvRow("Mask binary", c.netmask_binary, false, false)}
      ${kvRow("IP binary", c.ip_binary, false, false)}
    </dl>`;
}

async function calcCidrLive(sections: CalcSections): Promise<void> {
  const value = sections.input.value;
  if (!value.trim()) {
    sections.error.textContent = "";
    sections.result.innerHTML = "";
    return;
  }
  try {
    const info = await invoke<CidrInfo>("cidr_calculate", { input: value });
    sections.error.textContent = "";
    sections.input.setAttribute("aria-invalid", "false");
    sections.result.innerHTML = cidrResultHtml(info);
    bindCopy(sections.result);
  } catch (e) {
    sections.input.setAttribute("aria-invalid", "true");
    sections.error.textContent = String(e).replace("Error: ", "");
    sections.result.innerHTML = "";
  }
}

function wireCidrTab(host: HTMLElement): void {
  const input = host.querySelector<HTMLInputElement>("#cidr-input")!;
  const error = host.querySelector<HTMLElement>("#cidr-error")!;
  const result = host.querySelector<HTMLElement>("#cidr-result")!;

  let timer: ReturnType<typeof setTimeout> | undefined;
  const live = () => {
    clearTimeout(timer);
    timer = setTimeout(() => void calcCidrLive({ input, error, result }), 120);
  };
  input.addEventListener("input", live);
  input.addEventListener("change", () => void calcCidrLive({ input, error, result }));
  void calcCidrLive({ input, error, result });

  // subnet split
  const splitBtn = host.querySelector<HTMLButtonElement>("#cidr-split-btn")!;
  const splitPrefix = host.querySelector<HTMLInputElement>("#cidr-split-prefix")!;
  const splitOut = host.querySelector<HTMLElement>("#cidr-split-out")!;
  splitBtn.addEventListener("click", async () => {
    splitBtn.classList.add("is-loading");
    try {
      const subs = await invoke<string[]>("cidr_split", {
        input: input.value,
        newPrefix: Number(splitPrefix.value),
        limit: 256,
      });
      splitOut.innerHTML = `
        <div class="table-wrap"><table class="data">
          <thead><tr><th>#</th><th>Subnet</th><th>Range</th></tr></thead>
          <tbody>${subs
            .map((s, i) => {
              const [ip, p] = s.split("/");
              const size = Math.pow(2, 32 - Number(p));
              const first = i === 0 ? ip : nextIp(ip, 1);
              const last = nextIp(ip, size - 2);
              return `<tr><td>${i + 1}</td><td class="copyable" data-copy="${s}">${s}</td><td>${first} – ${last}</td></tr>`;
            })
            .join("")}</tbody></table></div>`;
      bindCopy(splitOut);
    } catch (e) {
      splitOut.innerHTML = `<p class="field-error">${String(e).replace("Error: ", "")}</p>`;
    } finally {
      splitBtn.classList.remove("is-loading");
    }
  });

  // range -> cidrs
  const rangeBtn = host.querySelector<HTMLButtonElement>("#cidr-range-btn")!;
  const rangeFrom = host.querySelector<HTMLInputElement>("#cidr-range-from")!;
  const rangeTo = host.querySelector<HTMLInputElement>("#cidr-range-to")!;
  const rangeOut = host.querySelector<HTMLElement>("#cidr-range-out")!;
  rangeBtn.addEventListener("click", async () => {
    rangeBtn.classList.add("is-loading");
    try {
      const list = await invoke<string[]>("cidr_range_to_cidrs", {
        start: rangeFrom.value,
        end: rangeTo.value,
      });
      rangeOut.innerHTML = `<div class="chips">${list
        .map((c) => `<span class="chip" aria-pressed="true">${c}</span>`)
        .join("")}</div>`;
    } catch (e) {
      rangeOut.innerHTML = `<p class="field-error">${String(e).replace("Error: ", "")}</p>`;
    } finally {
      rangeBtn.classList.remove("is-loading");
    }
  });

  // contains
  const contBtn = host.querySelector<HTMLButtonElement>("#cidr-contains-btn")!;
  const contIp = host.querySelector<HTMLInputElement>("#cidr-contains-ip")!;
  const contOut = host.querySelector<HTMLElement>("#cidr-contains-out")!;
  contBtn.addEventListener("click", async () => {
    try {
      const inside = await invoke<boolean>("cidr_contains", {
        cidrInput: input.value,
        ip: contIp.value,
      });
      contOut.innerHTML = inside
        ? `<span class="pill pill--open">In range</span> <span class="mono-data">${contIp.value} belongs to ${input.value}</span>`
        : `<span class="pill pill--filtered">Not in range</span> <span class="mono-data">${contIp.value} is outside ${input.value}</span>`;
    } catch (e) {
      contOut.innerHTML = `<p class="field-error">${String(e).replace("Error: ", "")}</p>`;
    }
  });
}

function nextIp(ip: string, add: number): string {
  const o = ip.split(".").map(Number);
  let n = ((o[0] << 24) | (o[1] << 16) | (o[2] << 8) | o[3]) >>> 0;
  n = (n + add) >>> 0;
  return [n >>> 24, (n >>> 16) & 255, (n >>> 8) & 255, n & 255].join(".");
}

// ---------- ports tab ----------

function portRowHtml(a: PortInfo): string {
  return `<tr><td>${a.port}</td><td>${a.protocol}</td><td>${a.service}</td><td class="wrap">${a.description}</td></tr>`;
}

function wirePortsTab(host: HTMLElement): void {
  const input = host.querySelector<HTMLInputElement>("#port-input")!;
  const out = host.querySelector<HTMLElement>("#port-out")!;

  let timer: ReturnType<typeof setTimeout> | undefined;
  const live = () => {
    clearTimeout(timer);
    timer = setTimeout(async () => {
      if (!input.value.trim()) {
        out.innerHTML = "";
        return;
      }
      try {
        const r = await invoke<PortReport>("port_lookup", { port: Number(input.value) });
        out.innerHTML = `
          <dl class="kv">
            ${kvRow("Range", r.range, false, false)}
          </dl>
          ${
            r.assignments.length
              ? `<div class="table-wrap"><table class="data">
                  <thead><tr><th>Port</th><th>Proto</th><th>Service</th><th>Description</th></tr></thead>
                  <tbody>${r.assignments.map(portRowHtml).join("")}</tbody></table></div>`
              : ""
          }
          <p class="field-hint">${r.notes.join(" · ")}</p>`;
      } catch (e) {
        out.innerHTML = `<p class="field-error">${String(e).replace("Error: ", "")}</p>`;
      }
    }, 120);
  };
  input.addEventListener("input", live);
  live();

  // service search
  const search = host.querySelector<HTMLInputElement>("#port-search")!;
  const searchOut = host.querySelector<HTMLElement>("#port-search-out")!;
  let sTimer: ReturnType<typeof setTimeout> | undefined;
  search.addEventListener("input", () => {
    clearTimeout(sTimer);
    sTimer = setTimeout(async () => {
      if (search.value.trim().length < 2) {
        searchOut.innerHTML = "";
        return;
      }
      const hits = await invoke<PortInfo[]>("port_search", { query: search.value });
      searchOut.innerHTML = hits.length
        ? `<div class="table-wrap"><table class="data">
            <thead><tr><th>Port</th><th>Proto</th><th>Service</th><th>Description</th></tr></thead>
            <tbody>${hits.map(portRowHtml).join("")}</tbody></table></div>`
        : `<p class="field-hint">No assignment matches “${search.value}”.</p>`;
    }, 150);
  });

  // range describe
  const from = host.querySelector<HTMLInputElement>("#port-range-from")!;
  const to = host.querySelector<HTMLInputElement>("#port-range-to")!;
  const rangeOut = host.querySelector<HTMLElement>("#port-range-out")!;
  [from, to].forEach((el) =>
    el.addEventListener("input", async () => {
      if (!from.value || !to.value) {
        rangeOut.textContent = "";
        return;
      }
      try {
        rangeOut.textContent = await invoke<string>("port_describe_range", {
          from: Number(from.value),
          to: Number(to.value),
        });
      } catch (e) {
        rangeOut.textContent = String(e).replace("Error: ", "");
      }
    })
  );
}

// ---------- shell ----------

export function renderCalc(el: HTMLElement, params: URLSearchParams): void {
  const tab = params.get("tab") === "ports" ? "ports" : "cidr";
  el.innerHTML = `
    <div class="view-head">
      <h1>Calculators</h1>
      <p>IPv4 subnet math and port reference — results update as you type.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:24px">
      <button role="tab" id="tab-cidr" aria-selected="${tab === "cidr"}">CIDR &amp; IP range</button>
      <button role="tab" id="tab-ports" aria-selected="${tab === "ports"}">Ports</button>
    </div>
    <div id="calc-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#calc-body")!;
  const showTab = (t: string) => {
    location.hash = `/calc?tab=${t}`;
  };
  el.querySelector("#tab-cidr")!.addEventListener("click", () => showTab("cidr"));
  el.querySelector("#tab-ports")!.addEventListener("click", () => showTab("ports"));

  if (tab === "cidr") {
    body.innerHTML = `
      <div class="panel-grid">
        <section class="card">
          <div class="field">
            <label for="cidr-input">Address — with prefix, mask or bare IP</label>
            <input id="cidr-input" class="input input--mono" placeholder="192.168.1.10/24" autocomplete="off" spellcheck="false" autofocus />
            <div class="chips" aria-label="Examples">
              ${["192.168.1.10/24", "10.0.0.0 255.0.0.0", "172.16.4.9/20", "8.8.8.8/32"]
                .map((x) => `<button class="chip" data-fill="${x}">${x}</button>`)
                .join("")}
            </div>
            <p id="cidr-error" class="field-error" role="alert"></p>
          </div>
          <div id="cidr-result" style="margin-top:16px"></div>
        </section>

        <div class="split-2">
          <section class="card">
            <h2>Split subnet</h2>
            <div class="toolbar">
              <div class="field">
                <label for="cidr-split-prefix">New prefix</label>
                <input id="cidr-split-prefix" class="input input--mono" type="number" min="0" max="32" value="26" style="max-width:110px" />
              </div>
              <button class="btn" id="cidr-split-btn">Split</button>
            </div>
            <div id="cidr-split-out" class="mono-data"></div>
          </section>
          <section class="card">
            <h2>Range to CIDR</h2>
            <div class="toolbar">
              <div class="field">
                <label for="cidr-range-from">Start IP</label>
                <input id="cidr-range-from" class="input input--mono" placeholder="192.168.1.1" />
              </div>
              <div class="field">
                <label for="cidr-range-to">End IP</label>
                <input id="cidr-range-to" class="input input--mono" placeholder="192.168.1.6" />
              </div>
              <button class="btn" id="cidr-range-btn">Convert</button>
            </div>
            <div id="cidr-range-out"></div>
          </section>
        </div>

        <section class="card">
          <h2>Address in range</h2>
          <div class="toolbar">
            <div class="field">
              <label for="cidr-contains-ip">IP to test against the block above</label>
              <input id="cidr-contains-ip" class="input input--mono" placeholder="192.168.1.200" />
            </div>
            <button class="btn btn--ghost" id="cidr-contains-btn">Check</button>
          </div>
          <div id="cidr-contains-out"></div>
        </section>
      </div>
    `;

    body.querySelectorAll<HTMLButtonElement>(".chip[data-fill]").forEach((chip) => {
      chip.addEventListener("click", () => {
        const input = body.querySelector<HTMLInputElement>("#cidr-input")!;
        input.value = chip.dataset.fill ?? "";
        input.dispatchEvent(new Event("change"));
      });
    });
    wireCidrTab(el);
  } else {
    body.innerHTML = `
      <div class="panel-grid">
        <section class="card">
          <div class="field">
            <label for="port-input">Port number (1–65535)</label>
            <input id="port-input" class="input input--mono" type="number" min="1" max="65535" placeholder="443" autofocus />
          </div>
          <div id="port-out" style="margin-top:16px"></div>
        </section>
        <div class="split-2">
          <section class="card">
            <h2>Search by service</h2>
            <div class="field">
              <label for="port-search">Service name or keyword</label>
              <input id="port-search" class="input input--mono" placeholder="dns, vnc, sql…" />
            </div>
            <div id="port-search-out" style="margin-top:12px"></div>
          </section>
          <section class="card">
            <h2>Port range</h2>
            <div class="toolbar">
              <div class="field">
                <label for="port-range-from">From</label>
                <input id="port-range-from" class="input input--mono" type="number" min="1" max="65535" placeholder="80" />
              </div>
              <div class="field">
                <label for="port-range-to">To</label>
                <input id="port-range-to" class="input input--mono" type="number" min="1" max="65535" placeholder="100" />
              </div>
            </div>
            <p id="port-range-out" class="mono-data"></p>
          </section>
        </div>
      </div>
    `;
    wirePortsTab(el);
  }
}
