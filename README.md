# Le Gopal

Website of Le Gopal, a vegetarian restaurant in Tours — [legopal.fr](https://legopal.fr).

The pages are written as [Leptos](https://github.com/leptos-rs/leptos) components and
prerendered to plain HTML at build time, so the site is fully static: no server and no
WebAssembly bundle. The little interactivity it has (mobile menu, announcement modal,
gallery zoom) lives in `public/site.js`.

## Requirements

- Rust nightly (picked up automatically from `rust-toolchain.toml`)
- Node.js 22, for the Tailwind CLI: `npm ci`

## Building and previewing

```bash
npm run dev
```

This renders every route into `dist/` (`/menu` → `dist/menu.html`, plus `dist/404.html`),
copies `public/` next to it, builds the stylesheet into `dist/pkg/gopal.css`, and serves
the result on http://127.0.0.1:3000 the same way GitHub Pages does.

`npm run build` and `npm run serve` run the two halves separately.

Set `IF_SHOW_ANNOUNCEMENT_BUTTON=true` when building to show the "Annonce" button and
modal.

## Deploying

Every push to `main` builds the site and publishes it to GitHub Pages
(`.github/workflows/pages.yml`). The announcement is controlled by the
`IF_SHOW_ANNOUNCEMENT_BUTTON` repository variable.

## Testing

```bash
cargo test
```

End-to-end tests live in `end2end/` and run with Playwright against
http://localhost:3000 (start it with `npm run dev`):

```bash
cd end2end && npx playwright test
```
