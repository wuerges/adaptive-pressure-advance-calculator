# Adaptive Pressure Advance Calculator

A single-page HTML tool to generate Klipper Adaptive Pressure Advance measurement tables.

All combinations of speeds × accelerations are generated as editable rows. Flow and PA values are entered manually. The config output is copyable with one click.

## Usage

1. Enter **Speeds** and **Accelerations** as comma-separated values
2. Click **Generate Table** to create the grid
3. Fill in **Flow** and **PA** values for each row
4. The **Model values** column and **Config** textarea update live as you type
5. Click **Copy** to copy the config to your clipboard

### Keyboard navigation

Press **Tab** to move forward: Flow → PA → (next row) Flow → PA → ...
Press **Shift+Tab** to reverse direction.

## Run locally

Just open `index.html` in any browser. No server or build step required.

## Deploy

Deploy to any static host (Vercel, Cloudflare Pages, GitHub Pages, Netlify) with zero configuration.

## Built with

HTML, CSS, JavaScript — zero dependencies.
