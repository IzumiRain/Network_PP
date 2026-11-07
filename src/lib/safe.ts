// Escape untrusted strings before HTML interpolation. Content packs, DNS
// answers, banners, headers and registry metadata are all external data —
// the webview holds Tauri invoke, so an XSS here is full backend access.

export function esc(s: string | number | null | undefined): string {
  return String(s ?? "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}
