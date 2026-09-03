# GTM Distribution & Copy Pack: shotdock

Target Channels:
- Reddit: r/unixporn, r/hyprland, r/linux, r/rust, r/archlinux
- X / Twitter: Dev / Rice Community
- Hacker News: Show HN
- Product Hunt / AlternativeTo

---

## 1. Reddit Playbook

### A. r/unixporn & r/hyprland
**Title**: [OC] shotdock: Modern floating screenshot and recording dock for Wayland (written in Rust)

**Post Body**:
I got tired of stringing together grim, slurp, and bash scripts to take decent screenshots on Hyprland, so I wrote `shotdock` in Rust with GTK4 layer-shell.

Key features:
- Floating pill dock anchored to the screen edge with backdrop blur.
- Window framing: 16px anti-aliased rounded corners and omnidirectional drop shadows.
- Presentation canvas: Wrap window or area captures onto your active wallpaper (blurred), system Wallbash palette, or solid colors.
- Fullscreen, active window, area snip, OCR (tesseract to clipboard), and MP4 screen recording (wf-recorder with PID tracking).
- Rich notifications: Generates a visual thumbnail card in SwayNC with instant Annotate (swappy) and Delete actions.
- Pure Rust: Zero Node.js. Built with GTK4-rs and layer-shell.

AUR package:
```sh
yay -S shotdock
```

GitHub: https://github.com/sirrryasir/shotdock
Docs: https://sirrryasir.github.io/shotdock/

Feedback on performance, compositor compatibility, and features is welcome.

---

### B. r/rust
**Title**: shotdock: Floating screenshot & screen recording dock for Wayland built with GTK4-rs

**Post Body**:
I wanted to share `shotdock`, an open-source Wayland utility built with Rust, GTK4, and gtk4-layer-shell.

Architecture details:
- Geometry and Compositor queries: Interfaces with Hyprland IPC to calculate window coordinates.
- PNG header decoding: Dimensions are parsed directly from byte offsets 16..24 of PNG chunks in memory to eliminate subprocess overhead.
- ImageMagick pipelines: Composes multi-pass Gaussian drop shadows and canvas backdrops via piped stdio.
- Documentation: 100% Rust using mdBook, deployed directly to GitHub Pages with zero JavaScript dependencies.

Source: https://github.com/sirrryasir/shotdock
Docs: https://sirrryasir.github.io/shotdock/

---

### C. r/archlinux
**Title**: shotdock is now available in the AUR (floating screenshot dock for Wayland)

**Post Body**:
Packaged `shotdock` for Arch Linux users on Hyprland and Sway.

Installation:
```sh
yay -S shotdock
```

Includes runtime hooks for grim, slurp, imagemagick, and optional optdepends for tesseract (OCR) and wf-recorder (screen recording).

AUR Package: https://aur.archlinux.org/packages/shotdock
GitHub: https://github.com/sirrryasir/shotdock

---

## 2. X / Twitter Playbook

**Main Tweet**:
Introducing shotdock: a modern floating screenshot and recording dock for Wayland compositors (Hyprland, Sway).

- Written in Rust (GTK4 layer-shell)
- Automatic window framing, rounded corners, and soft drop shadows
- Canvas backdrops (matches your active wallpaper / Wallbash colors)
- OCR text extraction & MP4 screen recording
- Rich SwayNC notification cards

Available now on the AUR: yay -S shotdock
GitHub: https://github.com/sirrryasir/shotdock
Docs: https://sirrryasir.github.io/shotdock/

[Attach 30-second silent video demo or GIF]

**Follow-up Tweet (Technical Stack)**:
Technical notes:
- Native Wayland layer-shell protocol with backdrop blur
- Zero-latency PNG dimension parsing directly from header bytes
- Pure Rust ecosystem: documentation powered by mdBook (<40ms build time)
- Process-isolated PID tracking for video recordings

---

## 3. Hacker News (Show HN)

**Title**: Show HN: Shotdock – Floating screenshot and recording dock for Wayland (Rust)

**Post Body**:
I built shotdock, an open-source on-screen screenshot and screen recording utility for Wayland compositors.

Most Wayland setups rely on custom shell scripts tying grim, slurp, and swappy together. While functional, they lack visual feedback, presentation-ready formatting (drop shadows, rounded window borders), and easy access to multiple modes like OCR or screen recording.

Shotdock runs as an anchored GTK4 layer-shell dock. It provides:
1. Mode selection: Fullscreen, active window, region snip, OCR to clipboard, and screen recording.
2. Built-in image post-processing: Generates 16px anti-aliased rounded corners, omnidirectional Gaussian drop shadows, and canvas backgrounds.
3. System integration: Reads Hyprland active window geometries and color themes from local dotfile configs.
4. Rich notifications: Hands off thumbnail images to SwayNC with interactive action callbacks.

The project is written in Rust. Documentation is built with mdBook and hosted on GitHub Pages.

Repository: https://github.com/sirrryasir/shotdock
Documentation: https://sirrryasir.github.io/shotdock/

I would appreciate thoughts on Wayland protocol handling, multi-monitor edge cases, and feature additions.
