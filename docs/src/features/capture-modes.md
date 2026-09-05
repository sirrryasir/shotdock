# Capture Modes

`shotdock` provides CLI commands for all capture operations.

---

## 1. Area Snip (with Window Snapping)

Interactive selection via `slurp`. Drag a rectangular box or click on any window to select it:

```sh
shotdock -a
```

Saves the capture to disk and copies it to the Wayland clipboard.

---

## 2. Screen Freeze Selection

Freezes screen content during selection using `hyprpicker`:

```sh
shotdock -a --freeze
# or
shotdock -z
```

---

## 3. Window & Region Framing

Captures a window or region and applies 16px rounded corners, Gaussian drop shadow, window titlebar, and canvas background:

```sh
shotdock -w
```

---

## 4. Focused Monitor

Captures the currently focused monitor:

```sh
shotdock -f
```

---

## 5. All Connected Monitors

Captures a full desktop span across all monitors:

```sh
shotdock -p
# or
shotdock --all
```

---

## 6. OCR Text Extraction

Extracts text from a selected area to the clipboard using Tesseract:

```sh
shotdock -t
```

---

## Modifier Flags

| Flag | Description |
|:---|:---|
| `-e`, `--edit` | Open capture in editor (`satty` or `swappy`) |
| `--no-edit` | Bypass editor |
| `--no-shadow` | Disable drop shadow |
| `--no-titlebar` | Disable window titlebar |
| `-z`, `--freeze` | Freeze screen during selection |

### Examples

```sh
# Snip and open in editor
shotdock -a -e

# Framed capture without shadow
shotdock -w --no-shadow

# Framed capture without titlebar
shotdock -w --no-titlebar
```

---

## Floating Dock

Run `shotdock` without arguments to open the GTK4 LayerShell dock:

```sh
shotdock
```

- `Escape`: Close dock
- `Enter`: Trigger capture for active mode
- Click buttons to trigger actions or click gear icon for settings popover
