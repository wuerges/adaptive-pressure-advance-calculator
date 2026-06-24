# Adaptive Pressure Advance Table Generator

Single HTML page to generate an adaptive pressure advance measurements table for Klipper.

Only HTML, CSS, and JavaScript in a single web page. No dependencies.

## Inputs

- **Speeds**: comma-separated list (default: `50, 100, 150, 200`)
- **Accelerations**: comma-separated list (default: `1000, 2000, 4000`)
- **Generate Table** button to build/rebuild the table

## Table

| Speed | Flow | Acceleration | PA | Model values |
|-------|------|-------------|-----|-------------|

- **Speed** and **Acceleration** are read-only (from the inputs).
- **Flow** and **PA** are editable `<input>` fields, blank by default.
- **Model values** auto-compute as `PA, Flow, Acceleration` and update live as the user types.
- The table contains all combinations of speeds × accelerations, sorted by acceleration then speed.

## Keyboard navigation

- Tab jumps within the same row: **Flow → PA**.
- Tab on the last column jumps to **Flow in the next row**.
- Shift+Tab reverses the direction.
- No manual tabindex — all controlled by a single keydown handler.

## Config output

- A text area below the table showing one line per row: `PA, Flow, Acceleration`
- Updates live as cells are edited.
- A **Copy** button next to the text area copies the config to the clipboard.

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
