# vendor-f5-bigip — v2 pilot note

`content/bigip-4000.md` is the **v2 pilot document** for this pack
(see `docs/PACK-SPEC.md` → "Document model (v2 draft)"):

- ONE Markdown document per Model/id — `doc/f5/bigip/4000`,
  category `reference/vendors/f5/bigip/4000` — instead of one file
  per command.
- Each `##` section carries its own tags via a
  `<!-- section-tags: … -->` marker directly under the heading.
- Its content is reorganised, unchanged, from the existing per-command
  files in this pack (source attributions kept per topic).

Migration status: the original per-command files **remain in place** for
now so the currently published pack zips/registry are unaffected. They are
removed in a later slice, when the registry is rebuilt from the v2 layout.
