import { invoke } from "../lib/tauri";
import type { DnsResult, HttpTestResult, ProcessInfo, DiskInfo, SystemInfo, SessionRow } from "../lib/types2";

function errBox(e: unknown): string {
  return `<p class="field-error">${String(e).replace("Error: ", "")}</p>`;
}

// ---------- DNS ----------

export function renderDnsPanel(el: HTMLElement): void {
  el.innerHTML = `
    <section class="card" style="margin-bottom:16px">
      <h2>DNS lookup</h2>
      <div class="toolbar">
        <div class="field" style="flex:2">
          <label for="dns-name">Name</label>
          <input id="dns-name" class="input input--mono" placeholder="example.com" spellcheck="false" />
        </div>
        <div class="field" style="max-width:130px">
          <label for="dns-type">Type</label>
          <select id="dns-type" class="input">
            <option>A</option><option>AAAA</option><option>CNAME</option><option>MX</option>
            <option>NS</option><option>TXT</option><option>SOA</option><option>SRV</option>
          </select>
        </div>
        <div class="field" style="max-width:170px">
          <label for="dns-server">Server (empty = system)</label>
          <input id="dns-server" class="input input--mono" placeholder="1.1.1.1" spellcheck="false" />
        </div>
        <button class="btn" id="dns-go">Resolve</button>
      </div>
      <p class="field-hint">Queries go straight to the resolver you pick — UDP port 53, built and parsed by the app itself.</p>
      <div id="dns-out" style="margin-top:12px"></div>
    </section>
    <section class="card">
      <h2>Compare resolvers</h2>
      <div class="toolbar">
        <div class="field" style="flex:2">
          <label for="dns-cmp-name">Name</label>
          <input id="dns-cmp-name" class="input input--mono" placeholder="example.com" spellcheck="false" />
        </div>
        <div class="field" style="flex:2">
          <label for="dns-cmp-servers">Servers (comma separated)</label>
          <input id="dns-cmp-servers" class="input input--mono" value="1.1.1.1, 8.8.8.8, 9.9.9.9" spellcheck="false" />
        </div>
        <button class="btn" id="dns-cmp-go">Compare</button>
      </div>
      <div id="dns-cmp-out" style="margin-top:12px"></div>
    </section>
  `;

  const out = el.querySelector<HTMLElement>("#dns-out")!;
  el.querySelector("#dns-go")!.addEventListener("click", async () => {
    const btn = el.querySelector<HTMLButtonElement>("#dns-go")!;
    btn.classList.add("is-loading");
    try {
      const r = await invoke<DnsResult>("dns_lookup", {
        name: (el.querySelector("#dns-name") as HTMLInputElement).value.trim(),
        qtype: (el.querySelector("#dns-type") as HTMLSelectElement).value,
        server: (el.querySelector("#dns-server") as HTMLInputElement).value.trim() || "system",
        timeoutMs: 2500,
      });
      out.innerHTML = dnsResultHtml(r);
    } catch (e) {
      out.innerHTML = errBox(e);
    } finally {
      btn.classList.remove("is-loading");
    }
  });

  const cmpOut = el.querySelector<HTMLElement>("#dns-cmp-out")!;
  el.querySelector("#dns-cmp-go")!.addEventListener("click", async () => {
    const btn = el.querySelector<HTMLButtonElement>("#dns-cmp-go")!;
    btn.classList.add("is-loading");
    try {
      const results = await invoke<DnsResult[]>("dns_compare", {
        name: (el.querySelector("#dns-cmp-name") as HTMLInputElement).value.trim(),
        qtype: "A",
        servers: (el.querySelector("#dns-cmp-servers") as HTMLInputElement).value,
        timeoutMs: 2000,
      });
      cmpOut.innerHTML = `<div class="table-wrap"><table class="data">
        <thead><tr><th>Server</th><th>Time</th><th>RCODE</th><th>Answers</th></tr></thead>
        <tbody>${results
          .map(
            (r) => `<tr>
              <td>${r.server}</td>
              <td>${r.error ? "—" : `${r.response_ms} ms`}</td>
              <td>${r.error ? `<span class="pill pill--filtered">error</span>` : r.ok ? `<span class="pill pill--open">${r.rcode}</span>` : `<span class="pill pill--filtered">${r.rcode}</span>`}</td>
              <td class="wrap">${r.error ?? (r.answers.map((a) => a.value).join(", ") || "—")}</td>
            </tr>`
          )
          .join("")}</tbody></table></div>`;
    } catch (e) {
      cmpOut.innerHTML = errBox(e);
    } finally {
      btn.classList.remove("is-loading");
    }
  });
}

function dnsResultHtml(r: DnsResult): string {
  const state = r.error
    ? `<span class="pill pill--filtered">error</span>`
    : r.ok
      ? `<span class="pill pill--open">${r.rcode}</span>`
      : `<span class="pill pill--filtered">${r.rcode}</span>`;
  return `
    <dl class="kv">
      <dt>Status</dt><dd>${state} ${r.error ?? ""}</dd>
      <dt>Server</dt><dd>${r.server}</dd>
      <dt>Response time</dt><dd>${r.response_ms} ms</dd>
    </dl>
    ${
      r.answers.length
        ? `<div class="table-wrap" style="margin-top:12px"><table class="data">
            <thead><tr><th>Name</th><th>Type</th><th>TTL</th><th>Value</th></tr></thead>
            <tbody>${r.answers
              .map(
                (a) => `<tr><td>${a.name}</td><td>${a.rtype}</td><td>${a.ttl}</td><td class="wrap">${a.value}</td></tr>`
              )
              .join("")}</tbody></table></div>`
        : ""
    }`;
}

// ---------- HTTP ----------

export function renderHttpPanel(el: HTMLElement): void {
  el.innerHTML = `
    <section class="card">
      <h2>HTTP test</h2>
      <div class="toolbar">
        <div class="field" style="flex:2">
          <label for="http-url">URL</label>
          <input id="http-url" class="input input--mono" placeholder="https://example.com" spellcheck="false" />
        </div>
        <div class="field" style="max-width:120px">
          <label for="http-method">Method</label>
          <select id="http-method" class="input"><option>GET</option><option>HEAD</option></select>
        </div>
        <div class="field" style="max-width:130px">
          <label for="http-timeout">Timeout (ms)</label>
          <input id="http-timeout" class="input input--mono" type="number" value="8000" />
        </div>
        <button class="btn" id="http-go">Test</button>
      </div>
      <div id="http-out" style="margin-top:12px"></div>
    </section>
  `;

  const out = el.querySelector<HTMLElement>("#http-out")!;
  el.querySelector("#http-go")!.addEventListener("click", async () => {
    const btn = el.querySelector<HTMLButtonElement>("#http-go")!;
    btn.classList.add("is-loading");
    out.innerHTML = `<div class="dots"><i></i><i></i><i></i></div>`;
    try {
      const r = await invoke<HttpTestResult>("http_test_cmd", {
        url: (el.querySelector("#http-url") as HTMLInputElement).value.trim(),
        method: (el.querySelector("#http-method") as HTMLSelectElement).value,
        timeoutMs: Number((el.querySelector("#http-timeout") as HTMLInputElement).value),
        proxy: null,
      });
      const statePill = r.error
        ? `<span class="pill pill--filtered">failed</span>`
        : r.ok
          ? `<span class="pill pill--open">${r.status}</span>`
          : `<span class="pill pill--time-wait">${r.status}</span>`;
      out.innerHTML = `
        <dl class="kv">
          <dt>Result</dt><dd>${statePill} ${r.error ?? r.status_text}</dd>
          <dt>Total time</dt><dd class="hl">${r.total_ms} ms</dd>
          <dt>Body size</dt><dd>${r.body_bytes.toLocaleString("en-US")} bytes</dd>
          ${r.server ? `<dt>Server</dt><dd>${r.server}</dd>` : ""}
          ${r.content_type ? `<dt>Content type</dt><dd>${r.content_type}</dd>` : ""}
        </dl>
        ${
          r.headers.length
            ? `<div class="table-wrap" style="margin-top:12px"><table class="data">
                <thead><tr><th>Header</th><th>Value</th></tr></thead>
                <tbody>${r.headers
                  .map((h) => `<tr><td>${h.name}</td><td class="wrap">${h.value}</td></tr>`)
                  .join("")}</tbody></table></div>`
            : ""
        }`;
    } catch (e) {
      out.innerHTML = errBox(e);
    } finally {
      btn.classList.remove("is-loading");
    }
  });
}

// ---------- system ----------

export function renderSystemPanel(el: HTMLElement): void {
  el.innerHTML = `
    <div class="split-2" style="align-items:start">
      <section class="card">
        <h2>This machine</h2>
        <dl class="kv" id="sys-info"><dt>Loading…</dt><dd></dd></dl>
      </section>
      <section class="card">
        <h2>Disks</h2>
        <div id="sys-disks"><p class="field-hint">Loading…</p></div>
      </section>
    </div>
    <section class="card" style="margin-top:16px">
      <h2>Top processes by memory</h2>
      <div id="sys-procs"><p class="field-hint">Loading…</p></div>
    </section>
  `;

  void (async () => {
    try {
      const info = await invoke<SystemInfo>("sys_info_cmd");
      el.querySelector<HTMLElement>("#sys-info")!.innerHTML = `
        <dt>Hostname</dt><dd class="hl">${info.hostname}</dd>
        <dt>User</dt><dd>${info.username}</dd>
        <dt>OS</dt><dd>${info.os} / ${info.arch}</dd>
        <dt>CPU cores</dt><dd>${info.cpu_cores}</dd>
        <dt>App version</dt><dd>v${info.app_version}</dd>`;
    } catch (e) {
      el.querySelector<HTMLElement>("#sys-info")!.innerHTML = `<dt>Error</dt><dd>${String(e).replace("Error: ", "")}</dd>`;
    }
    try {
      const disks = await invoke<DiskInfo[]>("sys_disks");
      el.querySelector<HTMLElement>("#sys-disks")!.innerHTML = disks.length
        ? `<div class="table-wrap"><table class="data">
            <thead><tr><th>Volume</th><th>Total</th><th>Free</th><th>Used</th></tr></thead>
            <tbody>${disks
              .map((d) => {
                const usedPct = d.total_gb > 0 ? Math.round(((d.total_gb - d.free_gb) / d.total_gb) * 100) : 0;
                return `<tr><td>${d.name}</td><td>${d.total_gb.toFixed(1)} GB</td><td>${d.free_gb.toFixed(1)} GB</td><td>${usedPct}%</td></tr>`;
              })
              .join("")}</tbody></table></div>`
        : `<p class="field-hint">No volumes reported.</p>`;
    } catch (e) {
      el.querySelector<HTMLElement>("#sys-disks")!.innerHTML = errBox(e);
    }
    try {
      const procs = await invoke<ProcessInfo[]>("sys_processes");
      el.querySelector<HTMLElement>("#sys-procs")!.innerHTML = procs.length
        ? `<div class="table-wrap"><table class="data">
            <thead><tr><th>PID</th><th>Name</th><th>Memory</th></tr></thead>
            <tbody>${procs
              .map((p) => `<tr><td>${p.pid}</td><td class="wrap">${p.name}</td><td>${p.mem_mb != null ? `${p.mem_mb.toFixed(1)} MB` : "—"}</td></tr>`)
              .join("")}</tbody></table></div>`
        : `<p class="field-hint">Could not read the process table.</p>`;
    } catch (e) {
      el.querySelector<HTMLElement>("#sys-procs")!.innerHTML = errBox(e);
    }
  })();
}

// ---------- sessions ----------

export function renderSessionsPanel(el: HTMLElement): void {
  el.innerHTML = `
    <section class="card" style="margin-bottom:16px">
      <h2>Diagnostic sessions</h2>
      <div class="toolbar">
        <div class="field" style="flex:2">
          <label for="sess-title">New session title</label>
          <input id="sess-title" class="input" placeholder="Branch office check — Nov 7" />
        </div>
        <button class="btn" id="sess-create">Create</button>
        <button class="btn btn--ghost" id="sess-refresh">Refresh</button>
      </div>
      <p class="field-hint">A session groups the tests you run against one problem, so you can export one clean report afterwards.</p>
      <div id="sess-list" style="margin-top:12px"></div>
    </section>
    <section class="card" id="sess-detail" hidden>
      <div class="toolbar">
        <h2 id="sess-detail-title" style="margin:0;flex:1"></h2>
        <button class="btn btn--ghost" id="sess-export">Export Markdown…</button>
        <button class="btn btn--danger" id="sess-delete">Delete</button>
      </div>
      <div class="toolbar">
        <div class="field" style="flex:1">
          <label for="sess-notes">Notes</label>
          <input id="sess-notes" class="input" placeholder="Context, environment, what changed…" />
        </div>
        <button class="btn btn--ghost" id="sess-notes-save">Save notes</button>
      </div>
      <div id="sess-results" style="margin-top:8px"></div>
      <h3 style="margin:16px 0 8px">Run a test into this session</h3>
      <div class="toolbar">
        <div class="field" style="max-width:130px">
          <label for="sess-kind">Test</label>
          <select id="sess-kind" class="input">
            <option value="ping">Ping</option>
            <option value="tcp">TCP port</option>
            <option value="dns">DNS A</option>
            <option value="http">HTTP GET</option>
          </select>
        </div>
        <div class="field" style="flex:1">
          <label for="sess-target" id="sess-target-label">Target</label>
          <input id="sess-target" class="input input--mono" placeholder="8.8.8.8" spellcheck="false" />
        </div>
        <button class="btn" id="sess-run">Run &amp; record</button>
      </div>
      <p class="field-hint" id="sess-status"></p>
    </section>
  `;

  const list = el.querySelector<HTMLElement>("#sess-list")!;
  const detail = el.querySelector<HTMLElement>("#sess-detail")!;
  let current = 0;

  const paintList = async () => {
    try {
      const sessions = await invoke<SessionRow[]>("session_list");
      list.innerHTML = sessions.length
        ? `<div class="table-wrap"><table class="data">
            <thead><tr><th>Title</th><th>Results</th><th>Created</th><th></th></tr></thead>
            <tbody>${sessions
              .map(
                (s) => `<tr>
                  <td class="wrap"><a href="#/diagnostics?tab=sessions&open=${s.id}">${s.title}</a></td>
                  <td>${s.result_count}</td>
                  <td>${new Date(s.created_at * 1000).toLocaleString("en-GB")}</td>
                  <td><button class="btn btn--ghost" data-open="${s.id}" style="padding:4px 10px;min-height:28px">Open</button></td>
                </tr>`
              )
              .join("")}</tbody></table></div>`
        : `<div class="empty"><p>No sessions yet — create one above.</p></div>`;
      list.querySelectorAll<HTMLButtonElement>("[data-open]").forEach((b) =>
        b.addEventListener("click", () => void openSession(Number(b.dataset.open)))
      );
      const openParam = new URLSearchParams(location.hash.split("?")[1] ?? "").get("open");
      if (openParam) void openSession(Number(openParam));
    } catch (e) {
      list.innerHTML = errBox(e);
    }
  };

  const paintResults = async () => {
    if (!current) return;
    try {
      const results = await invoke<{ id: number; ts: number; kind: string; target: string; ok: boolean; summary: string }[]>(
        "session_results",
        { sessionId: current }
      );
      el.querySelector<HTMLElement>("#sess-results")!.innerHTML = results.length
        ? `<div class="table-wrap"><table class="data">
            <thead><tr><th>Time</th><th>Test</th><th>Target</th><th>Result</th><th>Detail</th></tr></thead>
            <tbody>${results
              .map(
                (r) => `<tr>
                  <td>${new Date(r.ts * 1000).toLocaleTimeString("en-GB")}</td>
                  <td>${r.kind}</td>
                  <td class="wrap">${r.target}</td>
                  <td>${r.ok ? `<span class="pill pill--open">OK</span>` : `<span class="pill pill--filtered">FAIL</span>`}</td>
                  <td class="wrap">${r.summary}</td>
                </tr>`
              )
              .join("")}</tbody></table></div>`
        : `<p class="field-hint">No results yet — run a test below.</p>`;
    } catch (e) {
      el.querySelector<HTMLElement>("#sess-results")!.innerHTML = errBox(e);
    }
  };

  const openSession = async (id: number) => {
    try {
      const sessions = await invoke<SessionRow[]>("session_list");
      const s = sessions.find((x) => x.id === id);
      if (!s) return;
      current = id;
      detail.hidden = false;
      (el.querySelector("#sess-detail-title") as HTMLElement).textContent = s.title;
      (el.querySelector("#sess-notes") as HTMLInputElement).value = s.notes;
      detail.scrollIntoView({ behavior: "smooth", block: "nearest" });
      await paintResults();
    } catch (e) {
      el.querySelector<HTMLElement>("#sess-status")!.textContent = String(e).replace("Error: ", "");
    }
  };

  el.querySelector("#sess-create")!.addEventListener("click", async () => {
    const title = (el.querySelector("#sess-title") as HTMLInputElement).value.trim();
    if (!title) return;
    await invoke("session_create", { title });
    (el.querySelector("#sess-title") as HTMLInputElement).value = "";
    await paintList();
  });
  el.querySelector("#sess-refresh")!.addEventListener("click", () => void paintList());
  el.querySelector("#sess-notes-save")!.addEventListener("click", async () => {
    if (!current) return;
    await invoke("session_set_notes", {
      sessionId: current,
      notes: (el.querySelector("#sess-notes") as HTMLInputElement).value,
    });
    el.querySelector<HTMLElement>("#sess-status")!.textContent = "Notes saved ✓";
    setTimeout(() => (el.querySelector<HTMLElement>("#sess-status")!.textContent = ""), 1200);
  });
  el.querySelector("#sess-delete")!.addEventListener("click", async () => {
    if (!current) return;
    await invoke("session_delete", { sessionId: current });
    current = 0;
    detail.hidden = true;
    await paintList();
  });

  // run a test into the session
  el.querySelector("#sess-run")!.addEventListener("click", async () => {
    if (!current) return;
    const kind = (el.querySelector("#sess-kind") as HTMLSelectElement).value;
    const target = (el.querySelector("#sess-target") as HTMLInputElement).value.trim();
    const status = el.querySelector<HTMLElement>("#sess-status")!;
    if (!target) return;
    status.textContent = "Running…";
    try {
      const { ok, summary } = await runQuickTest(kind, target);
      await invoke("session_add_result", { sessionId: current, kind, target, ok, summary });
      status.textContent = `Recorded: ${summary}`;
      (el.querySelector("#sess-target") as HTMLInputElement).value = "";
      await paintResults();
      await paintList();
    } catch (e) {
      status.textContent = String(e).replace("Error: ", "");
    }
  });

  // target placeholder follows the kind
  const kindSel = el.querySelector<HTMLSelectElement>("#sess-kind")!;
  kindSel.addEventListener("change", () => {
    const ph: Record<string, string> = {
      ping: "8.8.8.8",
      tcp: "example.com:443",
      dns: "example.com",
      http: "https://example.com",
    };
    (el.querySelector("#sess-target") as HTMLInputElement).placeholder = ph[kindSel.value] ?? "";
  });

  void paintList();
}

async function runQuickTest(kind: string, target: string): Promise<{ ok: boolean; summary: string }> {
  if (kind === "ping") {
    const r = await invoke<{ up: boolean; latency_ms: number | null }>("scan_ping_one", {
      ip: target,
      timeoutMs: 1500,
    });
    return { ok: r.up, summary: r.up ? `${r.latency_ms ?? "?"} ms` : "no reply" };
  }
  if (kind === "tcp") {
    const [host, portStr] = target.split(":");
    const port = Number(portStr ?? "80");
    const r = await invoke<{ port: number; state: string }[]>("scan_tcp", {
      host,
      portSpec: String(port),
      timeoutMs: 1200,
      withBanner: false,
      maxThreads: 8,
    });
    const state = r[0]?.state ?? "filtered";
    return { ok: state === "open", summary: `port ${port} ${state}` };
  }
  if (kind === "dns") {
    const r = await invoke<DnsResult>("dns_lookup", {
      name: target,
      qtype: "A",
      server: "system",
      timeoutMs: 2000,
    });
    const ips = r.answers.map((a) => a.value).join(", ");
    return { ok: r.ok, summary: r.error ?? (ips || r.rcode) };
  }
  // http
  const r = await invoke<HttpTestResult>("http_test_cmd", {
    url: target,
    method: "GET",
    timeoutMs: 6000,
    proxy: null,
  });
  return {
    ok: r.ok,
    summary: r.error ?? `${r.status} in ${r.total_ms} ms`,
  };
}
