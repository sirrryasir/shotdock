# shotdock

**Modern, CLI-first Wayland screen capture, studio framing, and recording suite**  
*(Hyprland, Sway, Niri, River, Wayfire)*

*A fast, modular replacement for `grimblast` and capture scripts with an integrated CleanShot X aesthetic pipeline and optional floating toolbar.*

![shotdock demo](images/demo.gif)

`shotdock` is built to be **CLI-first**. It allows you to trigger instantaneous raw snips, frozen screen selections, OCR text extraction, or studio-grade presentation cards directly from your terminal, scripts, or compositor keybindings. When you want visual controls, launch the optional GTK4 LayerShell dock.

---

## Showcase

| Presentation Canvas | Framed Window | Floating Dock |
|:---:|:---:|:---:|
| ![Canvas Theme](images/canvas_presentation.png) | ![Framed Window](images/framed_window.png) | ![Floating Dock](images/toolbar.png) |

---

## Key Highlights

- **CLI-First Architecture**: Sub-150ms instant execution. Invoke captures headlessly without GUI startup latency.
- **Your Binds, Your Rules**: `shotdock` never enforces keybindings. You choose which shortcuts in your compositor map to which commands.
- **Raw Snips & Studio Cards in One Tool**:
  - Run `shotdock -a` for clean, unadorned snips piped directly to clipboard or your annotation editor (`satty`/`swappy`).
  - Run `shotdock -w` to produce polished cards with 16px rounded corners, multi-pass Gaussian drop shadows, mock titlebars, and wallpaper gradients.
- **Screen Freeze Selection**: Freeze screen animations and video playback during area selection via `--freeze` (`hyprpicker`).
- **Interactive Screen Recording**: 60 FPS visually lossless MP4 recording with an interactive target selector (choose monitor, window, or region) and clipboard file path copying.
- **Offline Headless Framing**: Frame existing image files from terminal or CI/CD via `shotdock frame <file>`.
- **OCR Text Extraction**: Snip any text on screen to extract plain text directly to your Wayland clipboard via Tesseract.
- **Optional Floating Dock**: Launch `shotdock` without flags whenever you prefer an anchored, glassmorphism floating toolbar.

---

## Quick Start

### Installation

```sh
# On Arch Linux (AUR)
yay -S shotdock
```

### Direct CLI Capture

```sh
# Clean area selection (click window or drag region)
shotdock -a

# Studio framed window or area (shadows + titlebar + canvas)
shotdock -w

# Fullscreen capture
shotdock -f

# Extract text to clipboard (OCR)
shotdock -t
```
