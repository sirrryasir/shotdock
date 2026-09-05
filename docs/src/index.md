# shotdock

Wayland screenshot and screen recording tool with window framing and an optional floating dock.

Supports Hyprland, Sway, River, Niri, and other Wayland compositors.

![shotdock demo](images/demo.gif)

---

## Showcase

| Canvas Background | Window Framing | Floating Dock |
|:---:|:---:|:---:|
| ![Canvas Theme](images/canvas_presentation.png) | ![Framed Window](images/framed_window.png) | ![Floating Dock](images/toolbar.png) |

---

## Features

- **Area & Window Snip**: Interactive region selection with window snapping (`slurp`).
- **Window Framing**: Rounded corners, Gaussian drop shadow, macOS-style titlebar, and gradient canvas backdrops.
- **Offline Framing**: Frame existing image files via `shotdock frame <file>`.
- **Screen Freeze**: Freeze screen content during area selection (`hyprpicker`).
- **Screen Recording**: 60 FPS H.264 video recording with monitor, window, or region selector (`wf-recorder`).
- **OCR Text Extraction**: Extract text from screen directly to Wayland clipboard (`tesseract`).
- **Optional Floating Dock**: GTK4 LayerShell toolbar for quick visual access.

---

## Quick Start

### Installation

```sh
yay -S shotdock
```

### CLI Capture

```sh
# Snip region or click window
shotdock -a

# Capture window or region with framing
shotdock -w

# Capture focused monitor
shotdock -f

# Extract text to clipboard (OCR)
shotdock -t

# Toggle screen recording
shotdock -r
```
