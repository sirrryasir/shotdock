# shotdock

Modern floating screenshot and screen recording toolbar for Wayland compositors (Hyprland, Sway, Wayfire).

`shotdock` brings window framing, presentation canvas backgrounds, and native Linux Wayland performance together into an anchored, responsive floating pill toolbar.

---

## Key Highlights

- **Window Framing**: Automatic 16px anti-aliased rounded corners, omnidirectional Gaussian drop shadows, and dark mock window titlebars with traffic lights (`🔴 🟡 🟢`).
- **Presentation Canvas**: Wrap any window or region capture into aesthetic backgrounds:
  - `Transparent` (alpha PNG)
  - `Follow System (Wallbash)` (auto-matches your desktop color scheme)
  - `Real Wallpaper (Blurred)` (centers your capture over your blurred wallpaper)
  - `Gradients` (Sunset, Candy, Breeze, Raindrop, Midnight, Forest)
  - `Solid Studio` (Minimalist White and Black)
- **Interactive Floating Dock**: Responsive GTK4 LayerShell dock anchored seamlessly to your screen with backdrop blur.
- **Rich Notification Preview Cards**: Instant visual image thumbnail cards in SwayNC with interactive `[ Annotate ]` and `[ Delete ]` actions.
- **Hardware-Accelerated Screen Recording**: H.264/MP4 recording with isolated PID tracking and one-click playback.
- **OCR Text Extraction**: Snip any text on screen to extract plain text directly to your Wayland clipboard via Tesseract.

---

## Quick Start

```sh
# On Arch Linux (AUR)
yay -S shotdock

# Launch the dock
shotdock
```
