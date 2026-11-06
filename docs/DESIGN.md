# Network++ — Design Contract

Product: Network++ — a lightweight network engineering toolkit (calculators,
cheatsheets, packet inspection, local scanning) for desktop and Android.

Primary user: network engineers, security students, and sysadmins. They type
addresses, ports, and commands; they read hex, masks, and packet fields. The UI
must feel like a fast terminal-adjacent tool, not a marketing site.

This file is the single source of visual truth. Components are adapted from
MIT-licensed uiverse.io references (see `docs/research/uiverse/`) and unified
into one token system — components are never copy-pasted with mismatched
colors; every component consumes the tokens below.

## 1. Archetype

**Technical Dense Terminal** (anti-slop Archetype B): dark canvas, hairline
borders, glass surfaces, emerald accent, tabular monospace for all data.

Banned on sight (from the anti-slop matrix): purple/indigo gradient blobs, raw
emoji as icons, vanity stats ("10x faster!"), tracked-out ALL-CAPS eyebrow
labels, single-word italic accents, duplicated metric cards.

## 2. Tokens

### Color

```css
--bg-canvas: #0c0d0e;            /* app background */
--bg-surface: #141618;           /* panels */
--bg-surface-elevated: #1a1d20;  /* cards, modals */
--glass: rgba(20, 20, 20, 0.80); /* glass surface (toggle-2/tooltip ref) */
--glass-border: rgba(255, 255, 255, 0.10);

--border-subtle: #24282c;
--border-hover: #383e44;

--accent: #10b981;               /* primary action — Precision Emerald */
--accent-strong: #36f9c7;        /* active/glow — neon mint (toggle-2 ref) */
--accent-dim: rgba(54, 249, 199, 0.30);

--text-primary: #e6edf3;
--text-secondary: #8b949e;
--text-muted: #565f66;

--danger: #ff4d4f;               /* loader/error ref */
--warn: #f5a623;
--ok: #00ca4e;
/* window-chrome dots (box-1 ref): --danger / --warn / --ok */

--input-bg: #1e2226;
--input-border: #2d3238;
```

Contrast floor: body text ≥ 4.5:1 on surfaces; accent text only on dark canvas.

### Typography

Max two families, both bundled locally (no CDN):

- UI: **Inter** — headings, labels, body.
- Data: **JetBrains Mono** — every IP, port, mask, hex dump, packet field,
  command. `font-variant-numeric: tabular-nums` on all numeric output.

Scale: H1 1.5rem/600 · H2 1.125rem/600 · body 0.875rem/400 · meta 0.75rem/500.
Sentence case everywhere. Line length ≤ 75ch for prose (cheatsheet descriptions).

### Space & radius

8-pt rhythm: 4 / 8 / 12 / 16 / 24 / 32. Card padding ≥ 16px.
Radii: sm 4px · md 8px · lg 14px · pill 999px. Nested rule:
`radius_inner = max(0, radius_outer − padding)`.

### Motion

- High-frequency actions (input updates, calculator recompute): **0 ms**.
- Dropdown/tooltip/popover: 120–160 ms `ease-out`, scale from trigger origin.
- Modals: 200 ms.
- `:active` on buttons: `scale(0.97)` 60 ms.
- Never `ease-in`; never animate the calculators' live output.

## 3. Layout

- **Desktop (≥ 900 px):** fixed 232 px sidebar (icon + label, collapsible to
  64 px icon rail) + main scroll area (`max-width 1100px`, 24 px gutter).
- **Mobile / Android (< 900 px):** sidebar collapses to top app bar +
  bottom navigation (max 5 items, 48 px minimum targets). No horizontal
  overflow anywhere; calculators stack vertically in input order.
- Page-level controls live at page top — never duplicated per card.
- Each tool page follows 1-to-3: one primary action (e.g. "Calculate"), max 3
  secondary controls, results as a single dense output block.

## 4. Components (adapted from uiverse refs)

| Component | Base reference | Adaptation |
|---|---|---|
| `.btn` (primary/ghost) | fluffy-quail-74, horrible-fish-20 | zinc→token colors, emerald primary, all 8 states |
| `.toggle` | tall-starfish-3, soft-octopus-29 | glass track, mint glow knob, 44 px hit area |
| `.card` / `.panel` | thin-sloth-31, itchy-bobcat-50 | dark surfaces, hairline border, no glow spam |
| `.input` | warm-eel-62, bad-cheetah-74 | mono font for data inputs, focus ring mint |
| `.tooltip` | spicy-kangaroo-36 | glass surface, 120 ms fade from trigger |
| `.list-item` | tall-swan-6 | devices, ports, cheatsheet entries |
| `.checkbox` | green-bobcat-29 | scan options |
| `.loader` | chilly-swan-51 | capture/scan in-progress states |
| `.tabbar` / nav rail | yellow-rattlesnake-26 | icon+label, active = mint tint |
| empty-state bg | quiet-snail-9 | faint CSS grid pattern |

Every interactive element ships all 8 states: default, hover, focus-visible,
active, disabled, loading, error, success.

## 5. Accessibility

- Visible `:focus-visible` ring (2px mint) on everything; never `outline:none`
  without a replacement.
- Icon-only controls get `aria-label`; semantic HTML for nav/main/buttons.
- Scan results and calculators are tables/lists with proper scope, not divs.
- Contrast tested against the token table above.

## 6. Copy

Short, imperative, technical. Labels name the field ("Subnet mask", not "Enter
your subnet mask here"). No exclamation marks, no fake numbers, no
"in today's fast-paced world". Errors say what to fix: "/33 is invalid — prefix
must be 0–32".

## 7. Acceptance gates

Before any UI lands: zero banned tropes; one primary action per section; all
8 states on interactive primitives; mono font on all data; no horizontal
scroll ≤ 360 px; focus rings visible; rendered result inspected once before
handoff (unslop US-003).
