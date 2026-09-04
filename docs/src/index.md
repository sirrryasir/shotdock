# shotdock

Automated window framing, soft drop shadows, and studio screen capture for Wayland (**Hyprland**, **Sway**, **Niri**, **Wayfire**).

*The CleanShot X aesthetic pipeline for Linux Wayland.*

![shotdock demo](images/demo.gif)

`shotdock` transforms raw screenshots into presentation-ready cards with 16px rounded corners, multi-pass Gaussian drop shadows, mock titlebars, and wallpaper gradients. Use it as an anchored floating dock or invoke it completely headlessly from your compositor keybindings.

---

## Showcase

| Floating Dock | Presentation Canvas |
|:---:|:---:|
| ![Floating Dock](images/toolbar.png) | ![Canvas Theme](images/canvas_presentation.png) |

---

## Key Highlights

- **CleanShot X Aesthetic Pipeline**: Automatic 16px rounded corners, omnidirectional Gaussian drop shadows, and dark mock window titlebars.
- **Presentation Canvas Backgrounds**: Wrap any window or region capture into aesthetic backdrops:
  - `Transparent` (clean alpha PNG with soft drop shadow)
  - `Follow System (Wallbash)` (dynamically matches active wallpaper palette)
  - `Real Wallpaper (Blurred)` (centers your capture over your blurred wallpaper)
  - `Gradients` (Sunset, Candy, Breeze, Raindrop, Midnight, Forest)
  - `Solid Studio` (Minimalist White and Black)
- **Keybind-First Architecture**: Trigger instant captures, screen freeze, and OCR directly from compositor shortcuts without ever opening a GUI toolbar.
- **Offline Headless Framing**: Frame existing image files from terminal or CI/CD via `shotdock frame <file>`.
- **Screen Freeze**: Freeze animations and video playback during area selection via `--freeze` (`hyprpicker`).
- **Interactive Floating Dock**: Responsive GTK4 LayerShell dock anchored to screen with backdrop blur when a visual UI is desired.
- **Rich Notification Preview Cards**: Instant visual image thumbnail cards in notification center with `[ Annotate ]` and `[ Delete ]` actions.
- **Studio Screen Recording**: 60 FPS H.264/MP4 recording (CRF 18) with isolated PID tracking and one-click playback.
- **OCR Text Extraction**: Snip any text on screen to extract plain text directly to your Wayland clipboard via Tesseract.

---

## Quick Start

```sh
# On Arch Linux (AUR)
yay -S shotdock

# Launch the dock
shotdock
```
