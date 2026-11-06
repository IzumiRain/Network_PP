import { invoke } from "../lib/tauri";
import type {
  PortScanResult,
  HostResult,
  PingResult,
  ArpEntry,
  LocalNetworkInfo,
  Connection,
  NetstatSnapshot,
  PortState,
} from "../lib/types";

type ScanTab = "target" | "local" | "conn";

export function renderScan(el: HTMLElement, params: URLSearchParams): void {
  const tab: ScanTab = (["target", "local", "conn"] as const).includes(
    params.get("tab") as ScanTab
  )
    ? (params.get("tab") as ScanTab)
    : "target";

  el.innerHTML = `
    <div class="view-head">
      <h1>Scanner</h1>
      <p>TCP connect scanning, host discovery and live connection tables. Desktop only — Android builds ship without this module.</p>
    </div>
    <div class="segmented" role="tablist" style="margin-bottom:24px">
      <button role="tab" id="st-target" aria-selected="${tab === "target"}">Target scan</button>
      <button role="tab" id="st-local" aria-selected="${tab === "local"}">Local devices</button>
      <button role="tab" id="st-conn" aria-selected="${tab === "conn"}">Connections</button>
    </div>
    <div id="scan-body"></div>
  `;

  const body = el.querySelector<HTMLElement>("#scan-body")!;
  const go = (t: string) => (location.hash = `/scan?tab=${t}`);
  el.querySelector("#st-target")!.addEventListener("click", () => go("target"));
  el.querySelector("#st-local")!.addEventListener("click", () => go("local"));
  el.querySelector("#st-conn")!.addEventListener("click", () => go("conn"));

  if (tab === "target") wireTarget(body);
  else if (tab === "local") wireLocal(body);
  else wireConnections(body);
}

function statePill(state: PortState): string {
  return `<span class="pill pill--${state}">${state}</span>`;
}

// ---------- target scan ----------

function wireTarget(body: HTMLElement): void {
  body.innerHTML = `
    <section class="card" style="margin-bottom:16px">
      <div class="toolbar">
        <div class="field" style="flex:2">
          <label for="t-host">Host or IP</label>
          <input id="t-host" class="input input--mono" placeholder="192.168.1.1" spellcheck="false" />
        </div>
        <div class="field" style="flex:2">
          <label for="t-ports">Ports</label>
          <input id="t-ports" class="input input--mono" value="22,80,443,445,3389,8080" spellcheck="false" />
        </div>
        <div class="field">
          <label for="t-timeout">Timeout (ms)</label>
          <input id="t-timeout" class="input input--mono" type="number" value="800" min="50" max="10000" style="max-width:110px" />
        </div>
        <label class="check"><input id="t-banner" type="checkbox" checked /><span class="box"></span>Grab banners</label>
        <button class="btn" id="t-go">Scan</button>
      </div>
      <p id="t-status" class="field-hint"></p>
    </section>
    <section class="card" style="margin-bottom:16px">
      <div class="toolbar">
        <div class="field" style="flex:1">
          <label for="t-ping-ip">Ping a single host</label>
          <input id="t-ping-ip" class="input input--mono" placeholder="8.8.8.8" spellcheck="false" />
        </div>
        <button class="btn btn--ghost" id="t-ping">Ping</button>
        <span id="t-ping-out" class="mono-data"></span>
      </div>
    </section>
    <div id="t-out"></div>
  `;

  const out = body.querySelector<HTMLElement>("#t-out")!;
  const status = body.querySelector<HTMLElement>("#t-status")!;

  body.querySelector("#t-go")!.addEventListener("click", async () => {
    const btn = body.querySelector<HTMLButtonElement>("#t-go")!;
    const host = (body.querySelector("#t-host") as HTMLInputElement).value.trim();
    if (!host) {
      status.textContent = "Enter a host first.";
      return;
    }
    btn.classList.add("is-loading");
    status.textContent = `Scanning ${host}…`;
    const t0 = performance.now();
    try {
      const results = await invoke<PortScanResult[]>("scan_tcp", {
        host,
        portSpec: (body.querySelector("#t-ports") as HTMLInputElement).value,
        timeoutMs: Number((body.querySelector("#t-timeout") as HTMLInputElement).value),
        withBanner: (body.querySelector("#t-banner") as HTMLInputElement).checked,
        maxThreads: 128,
      });
      const open = results.filter((r) => r.state === "open").length;
      status.textContent = `${results.length} ports scanned in ${Math.round(
        performance.now() - t0
      )} ms — ${open} open.`;
      out.innerHTML = results.length
        ? `<div class="table-wrap"><table class="data">
            <thead><tr><th>Port</th><th>State</th><th>Service</th><th>Banner</th></tr></thead>
            <tbody>${results
              .map(
                (r) => `<tr>
                  <td>${r.port}</td>
                  <td>${statePill(r.state)}</td>
                  <td>${r.service}</td>
                  <td class="wrap">${r.banner ?? "—"}</td>
                </tr>`
              )
              .join("")}</tbody></table></div>`
        : `<div class="empty"><p>No results — check the host address.</p></div>`;
    } catch (e) {
      status.textContent = "";
      out.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    } finally {
      btn.classList.remove("is-loading");
    }
  });

  body.querySelector("#t-ping")!.addEventListener("click", async () => {
    const ip = (body.querySelector("#t-ping-ip") as HTMLInputElement).value.trim();
    const outEl = body.querySelector<HTMLElement>("#t-ping-out")!;
    if (!ip) return;
    outEl.textContent = "Pinging…";
    try {
      const r = await invoke<PingResult>("scan_ping_one", { ip, timeoutMs: 1500 });
      outEl.innerHTML = r.up
        ? `<span class="pill pill--open">reply</span> ${r.latency_ms} ms`
        : `<span class="pill pill--filtered">no reply</span>`;
    } catch (e) {
      outEl.textContent = String(e).replace("Error: ", "");
    }
  });
}

// ---------- local devices ----------

function wireLocal(body: HTMLElement): void {
  body.innerHTML = `
    <section class="card" style="margin-bottom:16px">
      <div class="toolbar">
        <div class="field">
          <label for="l-cidr">Network range</label>
          <input id="l-cidr" class="input input--mono" placeholder="192.168.1.0/24" spellcheck="false" />
        </div>
        <div class="field">
          <label for="l-mode">Method</label>
          <div class="segmented" id="l-mode">
            <button data-mode="arp" aria-selected="true">ARP table</button>
            <button data-mode="tcp">TCP discover</button>
            <button data-mode="ping">Ping sweep</button>
          </div>
        </div>
        <button class="btn" id="l-go">Scan</button>
      </div>
      <p class="field-hint">ARP reads the local neighbor table instantly (same broadcast domain only). TCP discovery tests common ports per host; ping sweep uses the system ping.</p>
      <p id="l-status" class="field-hint"></p>
    </section>
    <div id="l-out"></div>
  `;

  let mode = "arp";
  body.querySelectorAll<HTMLButtonElement>("#l-mode button").forEach((b) => {
    b.addEventListener("click", () => {
      mode = b.dataset.mode!;
      body.querySelectorAll("#l-mode button").forEach((x) => x.setAttribute("aria-selected", String(x === b)));
    });
  });

  const out = body.querySelector<HTMLElement>("#l-out")!;
  const status = body.querySelector<HTMLElement>("#l-status")!;

  // Autodetect local network once.
  void invoke<LocalNetworkInfo>("scan_local_network_info")
    .then((info) => {
      (body.querySelector("#l-cidr") as HTMLInputElement).value = info.suggested_cidr;
      status.innerHTML = `Your address: <span class="mono-data">${info.local_ip}</span>`;
    })
    .catch(() => undefined);

  body.querySelector("#l-go")!.addEventListener("click", async () => {
    const btn = body.querySelector<HTMLButtonElement>("#l-go")!;
    btn.classList.add("is-loading");
    const cidrInput = (body.querySelector("#l-cidr") as HTMLInputElement).value.trim();
    const t0 = performance.now();
    try {
      if (mode === "arp") {
        const entries = await invoke<ArpEntry[]>("scan_arp_table");
        status.textContent = `${entries.length} neighbors in ${Math.round(performance.now() - t0)} ms (whole table, not only the range above).`;
        out.innerHTML = entries.length
          ? `<div class="table-wrap"><table class="data">
              <thead><tr><th>IP</th><th>MAC</th><th>Vendor</th><th>State</th></tr></thead>
              <tbody>${entries
                .map(
                  (e) => `<tr><td>${e.ip}</td><td>${e.mac}</td><td class="wrap">${e.vendor}</td><td class="wrap">${e.state}</td></tr>`
                )
                .join("")}</tbody></table></div>`
          : `<div class="empty"><p>ARP table is empty — talk to some devices first or use TCP discover.</p></div>`;
      } else if (mode === "tcp") {
        status.textContent = `Discovering hosts in ${cidrInput}…`;
        const hosts = await invoke<HostResult[]>("scan_discover_hosts", {
          cidrInput,
          portSpec: "22,80,443,445,3389,8080",
          timeoutMs: 500,
          maxThreads: 128,
        });
        status.textContent = `${hosts.length} hosts up in ${Math.round(performance.now() - t0)} ms.`;
        out.innerHTML = hosts.length
          ? `<div class="table-wrap"><table class="data">
              <thead><tr><th>IP</th><th>Hostname</th><th>Latency</th><th>Open ports</th></tr></thead>
              <tbody>${hosts
                .map(
                  (h) => `<tr>
                    <td>${h.ip}</td>
                    <td class="wrap">${h.hostname ?? "—"}</td>
                    <td>${h.latency_ms ?? "—"} ms</td>
                    <td>${h.open_ports.join(", ")}</td>
                  </tr>`
                )
                .join("")}</tbody></table></div>`
          : `<div class="empty"><p>No hosts answered on the common ports. Try ping sweep or the ARP table.</p></div>`;
      } else {
        status.textContent = `Pinging ${cidrInput}…`;
        const pings = await invoke<PingResult[]>("scan_ping_sweep", {
          cidrInput,
          timeoutMs: 800,
          maxThreads: 64,
        });
        const up = pings.filter((p) => p.up);
        status.textContent = `${up.length}/${pings.length} hosts replied in ${Math.round(performance.now() - t0)} ms.`;
        out.innerHTML = up.length
          ? `<div class="table-wrap"><table class="data">
              <thead><tr><th>IP</th><th>Latency</th></tr></thead>
              <tbody>${up
                .map((p) => `<tr><td>${p.ip}</td><td>${p.latency_ms ?? "—"} ms</td></tr>`)
                .join("")}</tbody></table></div>`
          : `<div class="empty"><p>No replies. Hosts often block ICMP — try TCP discover.</p></div>`;
      }
    } catch (e) {
      status.textContent = "";
      out.innerHTML = `<div class="empty"><p class="field-error" style="margin:0">${String(e).replace("Error: ", "")}</p></div>`;
    } finally {
      btn.classList.remove("is-loading");
    }
  });
}

// ---------- connections ----------

function wireConnections(body: HTMLElement): void {
  body.innerHTML = `
    <section class="card" style="margin-bottom:16px">
      <div class="toolbar">
        <div class="field" style="flex:1">
          <label for="c-filter">Filter (address, port, state, pid)</label>
          <input id="c-filter" class="input input--mono" placeholder=":443 ESTABLISHED" spellcheck="false" />
        </div>
        <label class="switch">
          <input id="c-live" type="checkbox" />
          <span class="track"></span>
          <span class="switch-label">Live refresh</span>
        </label>
        <button class="btn btn--ghost" id="c-refresh">Refresh now</button>
      </div>
      <p id="c-status" class="field-hint"></p>
    </section>
    <div id="c-out"></div>
  `;

  const out = body.querySelector<HTMLElement>("#c-out")!;
  const status = body.querySelector<HTMLElement>("#c-status")!;
  const filter = body.querySelector<HTMLInputElement>("#c-filter")!;
  const live = body.querySelector<HTMLInputElement>("#c-live")!;
  let timer: ReturnType<typeof setInterval> | undefined;
  let last: NetstatSnapshot | null = null;

  const render = (snap: NetstatSnapshot) => {
    last = snap;
    status.innerHTML = `${snap.total} sockets — ${snap.by_state
      .map(([s, n]) => `${s}: <span class="mono-data">${n}</span>`)
      .join(" · ")}`;
    const q = filter.value.trim().toLowerCase();
    const rows = snap.connections.filter((c) => {
      if (!q) return true;
      return [
        c.proto,
        c.local_addr,
        String(c.local_port),
        c.remote_addr,
        c.remote_port ? String(c.remote_port) : "",
        c.state,
        c.pid ? String(c.pid) : "",
      ]
        .join(" ")
        .toLowerCase()
        .includes(q);
    });
    out.innerHTML = `<p class="field-hint" style="margin-bottom:8px">Showing ${rows.length} of ${snap.total}</p>
      <div class="table-wrap"><table class="data">
        <thead><tr><th>Proto</th><th>Local</th><th>Remote</th><th>State</th><th>PID</th></tr></thead>
        <tbody>${
          rows.length
            ? rows
                .map((c) => connectionRow(c))
                .join("")
            : `<tr><td colspan="5" class="wrap" style="color:var(--text-muted)">No connections match.</td></tr>`
        }</tbody></table></div>`;
  };

  const refresh = async () => {
    try {
      render(await invoke<NetstatSnapshot>("netstat_snapshot"));
    } catch (e) {
      status.textContent = String(e).replace("Error: ", "");
    }
  };

  body.querySelector("#c-refresh")!.addEventListener("click", () => void refresh());
  filter.addEventListener("input", () => last && render(last));
  live.addEventListener("change", () => {
    if (live.checked) {
      timer = setInterval(() => void refresh(), 2000);
    } else if (timer) {
      clearInterval(timer);
      timer = undefined;
    }
  });
  void refresh();
}

function connectionRow(c: Connection): string {
  const stateClass =
    c.proto === "udp"
      ? "pill--closed"
      : /ESTAB/i.test(c.state)
        ? "pill--estab"
        : /LISTEN/i.test(c.state)
          ? "pill--listen"
          : /TIME_WAIT|CLOSE_WAIT/i.test(c.state)
            ? "pill--time-wait"
            : "pill--closed";
  return `<tr>
    <td>${c.proto}</td>
    <td>${c.local_addr}:${c.local_port}</td>
    <td>${c.remote_addr}${c.remote_port != null ? `:${c.remote_port}` : ""}</td>
    <td><span class="pill ${stateClass}">${c.state}</span></td>
    <td>${c.pid ?? "—"}</td>
  </tr>`;
}
