import { renderArticle } from "../views/article";

/**
 * Article slide-in: the article view mounted in a sheet that slides in
 * from the right — full-screen on mobile (covering the bottom nav), a
 * wide right panel over a scrim on desktop. The underlying view (tree,
 * home, …) is never unmounted, so its state (tree expansion, scroll)
 * survives the round trip.
 *
 * Closing runs the slide-out transition before the node is removed.
 * The router (main.ts) owns the URL: opening is driven by the
 * `#/article?id=…` hash, and `onClose` navigates back to the base hash;
 * this module only manages the DOM + motion.
 */

let overlay: HTMLElement | null = null;
let sheetBody: HTMLElement | null = null;
let idLabel: HTMLElement | null = null;
let currentId = "";
let closeTimer: ReturnType<typeof setTimeout> | undefined;
let lastFocus: HTMLElement | null = null;
let onClose: (() => void) | null = null;

const reducedMotion = () =>
  window.matchMedia("(prefers-reduced-motion: reduce)").matches;

function build(): void {
  overlay = document.createElement("div");
  overlay.className = "article-overlay";
  overlay.innerHTML = `
    <div class="article-scrim" data-close></div>
    <section class="article-sheet" role="dialog" aria-modal="true" aria-label="Article">
      <header class="article-sheet-bar">
        <button class="icon-btn article-back" type="button" data-close aria-label="Back">
          <svg class="icon-svg" viewBox="0 0 24 24" aria-hidden="true"><path d="M19 12H5"/><path d="m11 18-6-6 6-6"/></svg>
        </button>
        <span class="article-sheet-id mono-data"></span>
      </header>
      <div class="article-sheet-body"></div>
    </section>`;
  sheetBody = overlay.querySelector<HTMLElement>(".article-sheet-body")!;
  idLabel = overlay.querySelector<HTMLElement>(".article-sheet-id")!;
  overlay.querySelectorAll<HTMLElement>("[data-close]").forEach((b) =>
    b.addEventListener("click", requestClose)
  );
  overlay.addEventListener("keydown", (e) => {
    if (e.key === "Escape") requestClose();
  });
  document.body.appendChild(overlay);
  document.body.classList.add("no-scroll");
}

function requestClose(): void {
  // Slide out immediately for responsiveness; the callback changes the
  // hash, and the router's closeArticleOverlay() call then no-ops.
  closeArticleOverlay();
  onClose?.();
}

/** Open the slide-in for `id` (or swap its content if already open). */
export function openArticleOverlay(id: string, close: () => void): void {
  onClose = close;
  if (!overlay) {
    // Drop any previous sheet still animating out (rapid close → open).
    document.querySelectorAll(".article-overlay").forEach((n) => n.remove());
    lastFocus = document.activeElement as HTMLElement | null;
    build();
  }
  clearTimeout(closeTimer);
  if (currentId !== id) {
    currentId = id;
    if (idLabel) idLabel.textContent = id;
    renderArticle(sheetBody!, new URLSearchParams({ id }));
    sheetBody!.scrollTop = 0;
  }
  // Next frame so the transform transition runs from the off-canvas state.
  requestAnimationFrame(() => overlay?.classList.add("open"));
  overlay?.querySelector<HTMLElement>(".article-back")?.focus({ preventScroll: true });
}

/** Slide out and unmount. Safe to call when nothing is open. */
export function closeArticleOverlay(): void {
  if (!overlay) return;
  const node = overlay;
  overlay = null;
  sheetBody = null;
  idLabel = null;
  currentId = "";
  node.classList.remove("open");
  document.body.classList.remove("no-scroll");
  clearTimeout(closeTimer);
  closeTimer = setTimeout(() => node.remove(), reducedMotion() ? 0 : 260);
  if (lastFocus && document.contains(lastFocus)) {
    lastFocus.focus({ preventScroll: true });
  }
  lastFocus = null;
}
