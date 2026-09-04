# Capture Modes & CLI Reference

`shotdock` provides a modular CLI interface for all capture workflows. Commands execute sub-150ms without needing to launch a GUI toolbar.

---

## 1. Clean Area Snip (with Window Snapping)

Interactive selection via `slurp`. Drag any arbitrary rectangular box, or **single-click on any window** to snap to its exact geometry.

```sh
shotdock -a
```

- **Output**: Clean, unadorned PNG saved directly to disk and copied to your Wayland clipboard.
- **Workflow**: Ideal daily driver replacement for `grim+slurp` and `grimblast`.

---

## 2. Frozen Screen Selection

Freezes display buffers and moving video/animations during area selection via `hyprpicker`:

```sh
# Freeze screen during snip
shotdock -a --freeze

# Or shorthand
shotdock -z
```

---

## 3. Studio Framed Window / Area

Captures a window or custom region and formats it into a presentation card with 16px anti-aliased rounded corners, multi-pass Gaussian drop shadows, mock macOS window titlebar, and your chosen canvas theme:

```sh
shotdock -w
```

Click any window or drag any region to capture it with studio styling.

---

## 4. Focused Monitor

Captures the entire display of the currently focused monitor:

```sh
shotdock -f
```

---

## 5. All Connected Monitors

Captures the full spanning desktop across all connected monitors (ideal for the `Print` key):

```sh
shotdock -p

# Or long-form
shotdock --all
```

---

## 6. Optical Character Recognition (OCR)

Selects a screen region, runs Tesseract OCR engine, and copies plain text directly to the Wayland clipboard:

```sh
shotdock -t
```

---

## Modifier Flags

Combine these flags with any capture mode:

| Flag | Description |
| :--- | :--- |
| `-e`, `--edit` | Open the captured image directly in your annotation editor (`satty` or `swappy`) |
| `--no-edit` | Bypass the editor even if `"open_in_editor": true` in `config.json` |
| `--no-shadow` | Disable Gaussian drop shadow rendering |
| `--no-titlebar` | Disable mock macOS window titlebar |
| `-z`, `--freeze` | Freeze screen during region selection |

### Examples

```sh
# Snip an area and immediately open in annotation editor
shotdock -a -e

# Capture studio window without shadow
shotdock -w --no-shadow

# Capture studio window without macOS titlebar
shotdock -w --no-titlebar
```

---

## Optional: Interactive Floating Dock

Whenever a visual toolbar is preferred, launch `shotdock` with no arguments:

```sh
shotdock
```

- `Escape`: Close dock
- `Enter`: Trigger capture using currently selected mode
- Click any button to trigger the corresponding capture mode
- Click the gear icon to open the settings popover
