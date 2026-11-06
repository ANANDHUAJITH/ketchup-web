# ketchup-web

The landing page for [Ketchup](https://github.com/ANANDHUAJITH/ketchup), written in Rust and compiled to
WebAssembly with [Yew](https://yew.rs), built with [Trunk](https://trunkrs.dev), deployed to GitHub Pages
by `.github/workflows/deploy.yml`.

It reuses the OS's own mascot/logo SVGs and the same `ketchup-compact.txt` ascii-art banner that
`fastfetch` shows on the OS (see `assets/`), so the web banner and the terminal banner are the same art,
just rendered by two different pieces of Rust-adjacent tooling.

## Run it locally

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve --open
```

## Deploy

1. Push this to **a new repo** at `github.com/ANANDHUAJITH/ketchup-web` (or push it as a `web/`
   subfolder of the `ketchup` repo and point the workflow at that subfolder - either works, just
   keep the `--public-url` in `deploy.yml` in sync with where Pages will actually serve it from).
2. In the repo's **Settings -> Pages**, set "Build and deployment" source to **GitHub Actions**.
3. Push to `main`. The workflow builds with `trunk build --release` and publishes `dist/` via
   `actions/deploy-pages`.
4. The site will be at `https://anandhuajith.github.io/ketchup-web/` (or `/ketchup/` if you keep
   it in the OS repo) - update `--public-url` in `deploy.yml` if you rename the repo.

## Why Rust/WASM here (and not just plain HTML)

- The whole page, including the ascii-art banner's color parsing, is one `.rs` file - no JS build step,
  no npm, consistent with the rest of the OS's Rust-first tooling.
- `wasm-bindgen`/`web-sys` are used for the one bit of real interactivity (copy-the-download-command
  via the Clipboard API) so the page has a genuine, if small, reason to ship WASM rather than wasm for
  its own sake.
- `opt-level = "z"`, `lto = true` and `codegen-units = 1` in `Cargo.toml`, plus `data-wasm-opt="z"` in
  `index.html`, keep the shipped `.wasm` small so first paint stays fast on GitHub Pages' plain static
  hosting (no server-side compression tuning available there).

## Updating the content

- Feature list: the `FEATURES` array in `src/main.rs`.
- Ascii banner: `assets/ketchup-compact.txt` - copy a newer version from
  `ketchup/archiso/airootfs/usr/share/ketchup/branding/ascii/` if the OS's branding changes.
- Colors: `parse_ascii`'s `palette` array in `src/main.rs`, and the CSS custom properties at the top of
  `styles.css` - keep both in sync with the OS's brand colors (`tools/make-assets.py` in the main repo).
