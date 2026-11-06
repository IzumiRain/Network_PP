// Captures screenshots from the running Network++ window via WebView2 CDP.
// Usage: node scripts/cdp_capture.mjs <outdir> name1=#/route1 [name2=#/route2] ...
// Requires the app launched with WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=9223"
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const OUT = process.argv[2];
const targets = process.argv.slice(3);
if (!OUT || targets.length === 0) {
  console.error("usage: cdp_capture.mjs <outdir> name=#/route ...");
  process.exit(1);
}
mkdirSync(OUT, { recursive: true });

const list = await (await fetch("http://127.0.0.1:9223/json/list")).json();
const page = list.find((t) => t.type === "page" && /tauri\.localhost|Network/.test(t.url + t.title));
if (!page) {
  console.error("Network++ page target not found:", list.map((t) => t.title));
  process.exit(1);
}

const ws = new WebSocket(page.webSocketDebuggerUrl);
await new Promise((res, rej) => {
  ws.onopen = res;
  ws.onerror = rej;
});

let seq = 0;
const pending = new Map();
ws.onmessage = (ev) => {
  const msg = JSON.parse(ev.data);
  if (msg.id && pending.has(msg.id)) {
    pending.get(msg.id)(msg);
    pending.delete(msg.id);
  }
};
const send = (method, params = {}) =>
  new Promise((res) => {
    const id = ++seq;
    pending.set(id, res);
    ws.send(JSON.stringify({ id, method, params }));
  });

const evaluate = async (expression) => {
  const r = await send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
  return r.result?.result?.value;
};

await send("Page.enable");
await send("Emulation.setDeviceMetricsOverride", {
  width: 1200, height: 760, deviceScaleFactor: 2, mobile: false,
});

for (const spec of targets) {
  const eq = spec.indexOf("=");
  const name = spec.slice(0, eq);
  let route = spec.slice(eq + 1);
  let preEval = null;
  const pipe = route.indexOf("|");
  if (pipe !== -1) {
    preEval = route.slice(pipe + 1);
    route = route.slice(0, pipe);
  }
  // Git Bash MSYS path conversion can mangle "/home" → "C:/Program Files/Git/home";
  // rebuild the hash route from whatever arrived.
  route = "#/" + route.replace(/^#\/?/, "").replace(/^([A-Za-z]:\/)?(Program%20Files\/Git\/)?/, "").replace(/^\/+/, "");
  await send("Page.navigate", { url: `http://tauri.localhost/${route}` });
  await new Promise((r) => setTimeout(r, 1100));
  console.log(`  [${name}] hash=${await evaluate("location.hash")} h1=${await evaluate("document.querySelector('.view-head h1')?.textContent")}`);
  if (preEval) {
    await evaluate(preEval);
    await new Promise((r) => setTimeout(r, 700));
  }
  const shot = await send("Page.captureScreenshot", { format: "png" });
  const buf = Buffer.from(shot.result.data, "base64");
  const file = join(OUT, `${name}.png`);
  writeFileSync(file, buf);
  console.log(`saved ${file} (${Math.round(buf.length / 1024)} KB)`);
}

ws.close();
process.exit(0);
