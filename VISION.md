# Adaptive Pressure Advance Table Generator

Single HTML page to generate an adaptive pressure advance measurements table for Klipper.

Built with Leptos (Rust → WASM), bundled with Trunk, deployable on Vercel.

## Inputs

- **Speeds**: comma-separated list (default: `50, 100, 150, 200`)
- **Accelerations**: comma-separated list (default: `1000, 2000, 4000`)
- **Generate Table** button to build/rebuild the table

## Table

| Speed | Flow | Acceleration | PA | Model values |
|-------|------|-------------|-----|-------------|

- **Speed** and **Acceleration** are read-only (from the inputs).
- **Flow** and **PA** are editable `<input>` fields, blank by default.
- **Model values** auto-compute as `PA , Flow , Acceleration` and update live as the user types.
- The table contains all combinations of speeds × accelerations, sorted ascending by acceleration then speed.

## Keyboard navigation

- Tab jumps within the same row: **Flow → PA**.
- Tab on the last column jumps to **Flow in the next row**.
- Shift+Tab reverses the direction.
- No manual tabindex — all controlled by a single keydown handler.

## Config output

- A text area below the table showing one line per row: `PA , Flow , Acceleration`
- Updates live as cells are edited.
- A **Copy** button next to the text area copies the config to the clipboard.
- A **good/bad status indicator** shows whether all cells are filled. The copy button is disabled until all Flow and PA values are entered.

## Saved configs (localStorage)

- The Saved Configs section sits at the top of the page, above the inputs.
- Configs are stored in `localStorage` keyed by a user-defined name.
- **Select** dropdown lists all saved configs (sorted alphabetically). Selecting a profile automatically loads its speeds, accelerations, and cell values. No separate Load button.
- **Name** field — type a new name or select an existing profile to auto-fill it.
- **Save** — saves the current speeds, accelerations, and all cell values. If the name already exists, a confirmation dialog asks before overwriting.
- **Delete** — removes the selected profile after confirmation.

## Example filled table

After entering values for Flow and PA, the table looks like this:

| Speed | Flow | Acceleration | PA | Model values |
|-------|------|-------------|-----|-------------|
| 50 | 3.84 | 1000 | 0.036 | 0.036 , 3.84 , 1000 |
| 100 | 7.68 | 1000 | 0.036 | 0.036 , 7.68 , 1000 |
| 150 | 11.51 | 1000 | 0.036 | 0.036 , 11.51 , 1000 |
| 200 | 15.35 | 1000 | 0.036 | 0.036 , 15.35 , 1000 |
| 50 | 3.84 | 2000 | 0.036 | 0.036 , 3.84 , 2000 |
| 100 | 7.68 | 2000 | 0.03 | 0.03 , 7.68 , 2000 |
| 150 | 11.51 | 2000 | 0.029 | 0.029 , 11.51 , 2000 |
| 200 | 15.35 | 2000 | 0.028 | 0.028 , 15.35 , 2000 |
| 50 | 3.84 | 4000 | 0.032 | 0.032 , 3.84 , 4000 |
| 100 | 7.68 | 4000 | 0.028 | 0.028 , 7.68 , 4000 |
| 150 | 11.51 | 4000 | 0.026 | 0.026 , 11.51 , 4000 |
| 200 | 15.35 | 4000 | 0.024 | 0.024 , 15.35 , 4000 |

## Example generated config

The config text area shows one line per row:

```
0.036 , 3.84 , 1000
0.036 , 7.68 , 1000
0.036 , 11.51 , 1000
0.036 , 15.35 , 1000
0.036 , 3.84 , 2000
0.03 , 7.68 , 2000
0.029 , 11.51 , 2000
0.028 , 15.35 , 2000
0.032 , 3.84 , 4000
0.028 , 7.68 , 4000
0.026 , 11.51 , 4000
0.024 , 15.35 , 4000
```

## Edge cases

- **Empty table**: before Generate is clicked, a placeholder message appears ("Enter valid speeds and accelerations, then click Generate").
- **Re-generate**: editing speeds or accelerations after the table is built does not auto-update the table — click Generate again. If cells have been filled, a confirmation dialog asks before clearing them.
- **Name field vs dropdown**: typing a new name while a profile is selected in the dropdown does not change the dropdown selection. Save uses the name field, Delete uses the dropdown selection.
- **Invalid inputs**: non-numeric, zero, negative, Infinity, and NaN values in the comma-separated lists are silently ignored.

## Build & deploy

- Build: `trunk build --release` → outputs `dist/`
- Deploy: import the repo on Vercel — it auto-detects the config
- The app is fully client-side (CSR); no server or backend required
