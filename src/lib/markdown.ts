// Minimal markdown renderer for the bundled cheatsheets.
// Supports: h1-h3, fenced code blocks (with copy button), tables, bullet lists,
// numbered lists, bold, inline code, links. ~120 lines, zero dependencies.

const esc = (s: string): string =>
  s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");

function inline(text: string): string {
  return esc(text)
    .replace(/`([^`]+)`/g, "<code>$1</code>")
    .replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>")
    .replace(
      /\[([^\]]+)\]\(([^)]+)\)/g,
      '<a href="$2" target="_blank" rel="noopener">$1</a>'
    );
}

export function renderMarkdown(src: string): string {
  const out: string[] = [];
  const lines = src.split("\n");
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];

    // fenced code
    if (line.startsWith("```")) {
      const lang = line.slice(3).trim();
      const buf: string[] = [];
      i++;
      while (i < lines.length && !lines[i].startsWith("```")) {
        buf.push(lines[i]);
        i++;
      }
      i++; // closing fence
      out.push(
        `<div class="codeblock"><button class="copy-btn" type="button">Copy</button><pre data-lang="${esc(
          lang
        )}">${esc(buf.join("\n"))}</pre></div>`
      );
      continue;
    }

    // table
    if (line.trim().startsWith("|") && i + 1 < lines.length && /^\s*\|[-| :]+\|\s*$/.test(lines[i + 1])) {
      const header = line
        .trim()
        .slice(1, -1)
        .split("|")
        .map((c) => c.trim());
      i += 2;
      const rows: string[][] = [];
      while (i < lines.length && lines[i].trim().startsWith("|")) {
        rows.push(
          lines[i]
            .trim()
            .replace(/^\|/, "")
            .replace(/\|$/, "")
            .split("|")
            .map((c) => c.trim())
        );
        i++;
      }
      out.push(
        `<div class="table-wrap"><table class="data"><thead><tr>${header
          .map((h) => `<th>${inline(h)}</th>`)
          .join("")}</tr></thead><tbody>${rows
          .map(
            (r) =>
              `<tr>${r
                .map((c) => `<td class="wrap">${inline(c)}</td>`)
                .join("")}</tr>`
          )
          .join("")}</tbody></table></div>`
      );
      continue;
    }

    // headings
    const h = /^(#{1,3})\s+(.*)$/.exec(line);
    if (h) {
      out.push(`<h${h[1].length + 1}>${inline(h[2])}</h${h[1].length + 1}>`);
      i++;
      continue;
    }

    // lists (incl. task lists "- [ ] step")
    if (/^\s*[-*]\s+/.test(line) || /^\s*\d+\.\s+/.test(line)) {
      const ordered = /^\s*\d+\./.test(line);
      const isTasks = /^\s*[-*]\s+\[[ xX]\]\s+/.test(line);
      const items: string[] = [];
      while (i < lines.length && (/^\s*[-*]\s+/.test(lines[i]) || /^\s*\d+\.\s+/.test(lines[i]))) {
        if (isTasks) {
          const check = /^\s*[-*]\s+\[([ xX])\]\s+(.*)$/.exec(lines[i]);
          if (check) {
            const done = check[1].toLowerCase() === "x";
            items.push(
              `<li class="task"><input type="checkbox" ${done ? "checked" : ""} tabindex="-1" /><span>${inline(
                check[2]
              )}</span></li>`
            );
          } else {
            items.push(lines[i]);
          }
        } else {
          items.push(lines[i].replace(/^\s*(?:[-*]|\d+\.)\s+/, ""));
        }
        i++;
      }
      if (isTasks) {
        out.push(
          `<ul class="tasks" style="margin:8px 0;padding-left:6px;list-style:none">${
            items.join("")
          }</ul>`
        );
      } else {
        const tag = ordered ? "ol" : "ul";
        out.push(
          `<${tag} style="margin:8px 0;padding-left:22px">${
            items.map((it) => `<li>${inline(it)}</li>`).join("")
          }</${tag}>`
        );
      }
      continue;
    }

    // paragraph
    if (line.trim() !== "") {
      out.push(`<p style="margin:8px 0">${inline(line)}</p>`);
    }
    i++;
  }
  return out.join("\n");
}

/** Wire copy buttons on every .codeblock inside root. Call after injecting HTML. */
export function bindCopyButtons(root: HTMLElement): void {
  root.querySelectorAll(".copy-btn").forEach((btn) => {
    btn.addEventListener("click", () => {
      const pre = btn.parentElement?.querySelector("pre");
      if (!pre) return;
      navigator.clipboard.writeText(pre.textContent ?? "").then(() => {
        btn.textContent = "Copied";
        setTimeout(() => (btn.textContent = "Copy"), 1200);
      });
    });
  });
}
