# Adaptive Pressure Advance Calculator

A Leptos (Rust → WASM) single-page app to generate Klipper Adaptive Pressure Advance measurement tables.

All combinations of speeds × accelerations are generated as editable rows. Flow and PA values are entered manually. The config output is copyable with one click.

## Usage

1. Enter **Speeds** and **Accelerations** as comma-separated values
2. Click **Generate Table** to create the grid
3. Fill in **Flow** and **PA** values for each row
4. The **Model values** column and **Config** textarea update live as you type
5. Click **Copy** to copy the config to your clipboard

### Saved configs

Profile configurations can be saved to localStorage, named, and recalled at any time. Selecting a profile from the dropdown automatically loads it.

### Keyboard navigation

Press **Tab** to move forward: Flow → PA → (next row) Flow → PA → ...
Press **Shift+Tab** to reverse direction.

## Build

```bash
# install trunk
cargo install trunk --locked

# add wasm target
rustup target add wasm32-unknown-unknown

# build
trunk build --release
```

Output goes to `dist/`.

## Deploy

The repo includes a `vercel.json` — just import on Vercel and it builds and deploys automatically. Also works on any static host (Cloudflare Pages, Netlify, GitHub Pages) by pointing to the `dist/` directory.

## Built with

- [Leptos](https://leptos.dev) — reactive Rust/WASM framework (CSR)
- [Trunk](https://trunkrs.dev) — WASM bundler
- Deployed via Vercel
