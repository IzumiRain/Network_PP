// Generates Network++ app icons (PNG + ICO) with zero dependencies.
// Dark rounded square, emerald network motif: 4 nodes linked to a center node.
import { writeFileSync, mkdirSync } from "node:fs";
import { deflateSync } from "node:zlib";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = join(dirname(fileURLToPath(import.meta.url)), "..");
const OUT = join(ROOT, "src-tauri", "icons");
mkdirSync(OUT, { recursive: true });

// ---- minimal PNG encoder (RGBA, 8-bit, no filtering) ----
const CRC_TABLE = new Int32Array(256).map((_, n) => {
  let c = n;
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
  return c;
});
function crc32(buf) {
  let c = -1;
  for (const b of buf) c = CRC_TABLE[(c ^ b) & 0xff] ^ (c >>> 8);
  return (c ^ -1) >>> 0;
}
function chunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const body = Buffer.concat([Buffer.from(type, "ascii"), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(body));
  return Buffer.concat([len, body, crc]);
}
function encodePng(w, h, rgba) {
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(w, 0);
  ihdr.writeUInt32BE(h, 4);
  ihdr[8] = 8; // bit depth
  ihdr[9] = 6; // color type RGBA
  const raw = Buffer.alloc((w * 4 + 1) * h);
  for (let y = 0; y < h; y++) {
    raw[y * (w * 4 + 1)] = 0; // filter none
    rgba.copy(raw, y * (w * 4 + 1) + 1, y * w * 4, (y + 1) * w * 4);
  }
  return Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    chunk("IHDR", ihdr),
    chunk("IDAT", deflateSync(raw, { level: 9 })),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

// ---- canvas helpers ----
function canvas(size) {
  return { size, px: Buffer.alloc(size * size * 4) };
}
function put(cv, x, y, r, g, b, a) {
  if (x < 0 || y < 0 || x >= cv.size || y >= cv.size) return;
  const i = (y * cv.size + x) * 4;
  const na = a / 255;
  cv.px[i] = Math.round(r * na + cv.px[i] * (1 - na));
  cv.px[i + 1] = Math.round(g * na + cv.px[i + 1] * (1 - na));
  cv.px[i + 2] = Math.round(b * na + cv.px[i + 2] * (1 - na));
  cv.px[i + 3] = Math.max(cv.px[i + 3], a);
}
function fillCircle(cv, cx, cy, rad, [r, g, b], a = 255) {
  const x0 = Math.floor(cx - rad), x1 = Math.ceil(cx + rad);
  for (let y = Math.floor(cy - rad); y <= Math.ceil(cy + rad); y++)
    for (let x = x0; x <= x1; x++) {
      const d = Math.hypot(x - cx, y - cy);
      if (d <= rad - 0.7) put(cv, x, y, r, g, b, a);
      else if (d <= rad + 0.7) put(cv, x, y, r, g, b, a * (1 - (d - (rad - 0.7)) / 1.4));
    }
}
function line(cv, x0, y0, x1, y1, w, [r, g, b], a = 255) {
  const steps = Math.ceil(Math.hypot(x1 - x0, y1 - y0)) * 2;
  for (let i = 0; i <= steps; i++) {
    const t = i / steps;
    fillCircle(cv, x0 + (x1 - x0) * t, y0 + (y1 - y0) * t, w / 2, [r, g, b], a);
  }
}

// ---- icon design ----
const COLORS = {
  bg: [12, 13, 14],
  surface: [26, 29, 32],
  accent: [16, 185, 129],   // #10b981
  mint: [54, 249, 199],     // #36f9c7
  node: [230, 237, 243],
};
const S = 512;
const cv = canvas(S);
// rounded-square background
const RAD = 96;
for (let y = 0; y < S; y++)
  for (let x = 0; x < S; x++) {
    const dx = Math.max(RAD - x, x - (S - 1 - RAD), 0);
    const dy = Math.max(RAD - y, y - (S - 1 - RAD), 0);
    const d = Math.hypot(dx, dy);
    if (d <= RAD) put(cv, x, y, ...COLORS.bg, 255);
    else if (d <= RAD + 1) put(cv, x, y, ...COLORS.bg, Math.round(255 * (1 - (d - RAD))));
  }
// subtle inner panel glow ring
fillCircle(cv, S / 2, S / 2, 190, COLORS.surface, 110);

// network motif: center node + 4 satellites
const cx = S / 2, cy = S / 2;
const sats = [
  [cx, cy - 118],
  [cx - 118, cy + 78],
  [cx + 118, cy + 78],
  [cx + 0, cy + 118],
];
for (const [sx, sy] of sats) line(cv, cx, cy, sx, sy, 14, COLORS.accent, 220);
line(cv, sats[1][0], sats[1][1], sats[2][0], sats[2][1], 10, COLORS.accent, 140);
for (const [sx, sy] of sats) fillCircle(cv, sx, sy, 26, COLORS.mint);
fillCircle(cv, cx, cy, 40, COLORS.node);
fillCircle(cv, cx, cy, 40, COLORS.accent, 60);

// "++" mark bottom-right: two plus signs
const plus = (px, py, s, col) => {
  line(cv, px - s, py, px + s, py, 12, col);
  line(cv, px, py - s, px, py + s, 12, col);
};
plus(372, 396, 22, COLORS.mint);
plus(424, 396, 22, COLORS.accent);

const png = encodePng(S, S, cv.px);
writeFileSync(join(OUT, "icon.png"), png);
writeFileSync(join(OUT, "128x128.png"), png); // placeholder sizes written below properly
writeFileSync(join(OUT, "32x32.png"), png);

// real resized versions (nearest-neighbour) + ICO (PNG-encoded, valid at all sizes)
function resize(srcSize, srcPx, outSize) {
  const out = Buffer.alloc(outSize * outSize * 4);
  for (let y = 0; y < outSize; y++)
    for (let x = 0; x < outSize; x++) {
      const sx = Math.min(srcSize - 1, Math.floor((x / outSize) * srcSize));
      const sy = Math.min(srcSize - 1, Math.floor((y / outSize) * srcSize));
      srcPx.copy(out, (y * outSize + x) * 4, (sy * srcSize + sx) * 4, (sy * srcSize + sx) * 4 + 4);
    }
  return out;
}
writeFileSync(join(OUT, "128x128.png"), encodePng(128, 128, resize(S, cv.px, 128)));
writeFileSync(join(OUT, "32x32.png"), encodePng(32, 32, resize(S, cv.px, 32)));

function makeIco(pngBuffers) {
  const count = pngBuffers.length;
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0);
  header.writeUInt16LE(1, 2);
  header.writeUInt16LE(count, 4);
  const entries = [];
  let offset = 6 + 16 * count;
  for (const { size, data } of pngBuffers) {
    const e = Buffer.alloc(16);
    e[0] = size === 256 ? 0 : size; // width
    e[1] = size === 256 ? 0 : size; // height
    e[4] = 1; e[6] = 32; // planes, bpp
    e.writeUInt32LE(data.length, 8);
    e.writeUInt32LE(offset, 12);
    entries.push(e);
    offset += data.length;
  }
  return Buffer.concat([header, ...entries, ...pngBuffers.map((p) => p.data)]);
}
const ico = makeIco([
  { size: 256, data: encodePng(256, 256, resize(S, cv.px, 256)) },
  { size: 128, data: encodePng(128, 128, resize(S, cv.px, 128)) },
  { size: 64, data: encodePng(64, 64, resize(S, cv.px, 64)) },
  { size: 32, data: encodePng(32, 32, resize(S, cv.px, 32)) },
  { size: 16, data: encodePng(16, 16, resize(S, cv.px, 16)) },
]);
writeFileSync(join(OUT, "icon.ico"), ico);
console.log("icons written:", OUT);
