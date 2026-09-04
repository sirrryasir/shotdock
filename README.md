# shotdock

**Modern, CLI-first Wayland screen capture, studio framing, and recording suite**  
*(Hyprland, Sway, Niri, River, Wayfire)*

*A fast, modular replacement for `grimblast` and ad-hoc capture scripts with an integrated CleanShot X aesthetic pipeline and optional floating toolbar.*

![shotdock demo](assets/demo.gif)

`shotdock` is designed from the ground up to be **CLI-first**. Trigger instantaneous raw snips, frozen screen selections, OCR text extraction, or studio-grade presentation cards directly from your terminal, scripts, or compositor keybindings. When you want visual controls, launch the optional GTK4 LayerShell dock.

---

## Showcase

| Presentation Canvas | Framed Window | Floating Dock |
|:---:|:---:|:---:|
| ![Canvas Theme](assets/canvas_presentation.png) | ![Framed Window](assets/framed_window.png) | ![Floating Dock](assets/toolbar.png) |

---

## Why shotdock?

- **CLI-First Architecture**: Sub-150ms instant execution. No slow GUI startup overhead for daily captures.
- **Your Binds, Your Rules**: `shotdock` never enforces keybindings. Map whichever shortcuts you prefer to its clean, modular CLI flags.
- **Raw Snips & Studio Cards in One Tool**:
  - Run `shotdock -a` for clean, unadorned snips piped straight to clipboard or your annotation editor (`satty`/`swappy`).
  - Run `shotdock -w` to produce polished marketing/docs cards with 16px rounded corners, multi-pass Gaussian drop shadows, mock titlebars, and wallpaper gradients.
- **Interactive Screen Recording**: 60 FPS visually lossless MP4 recording with an interactive target selector (choose monitor, window, or region) and clipboard file path copying.
- **Offline Headless Framing**: Turn existing images into styled presentation cards via `shotdock frame <file>`.
- **Optional Floating Dock**: Launch `shotdock` without flags whenever you want an anchored, glassmorphism floating toolbar.

---

## CLI Reference & Usage

### Capture Commands

| Command | Description |
| :--- | :--- |
| `shotdock -a` | **Clean Area Snip**: Drag any custom region or single-click any window to capture |
| `shotdock -a --freeze` *(or `-z`)* | **Screen Freeze Snip**: Freeze animations and video playback during selection (`hyprpicker`) |
| `shotdock -w` | **Studio Framed Window / Area**: Apply 16px radius, Gaussian shadow, mock titlebar & canvas |
| `shotdock -f` | **Focused Monitor**: Capture the entire active display |
| `shotdock -p` *(or `--all`)* | **All Connected Monitors**: Clean desktop span across all screens (ideal for `Print` key) |
| `shotdock -t` | **OCR Text Snip**: Extract text from screen directly into Wayland clipboard (`tesseract`) |
| `shotdock -r` | **Screen Recording**: Interactive 60 FPS recording (prompts to choose display, window, or region) |
| `shotdock --record-area` | **Direct Region Recording**: Immediate area video capture via `wf-recorder` |
| `shotdock frame <file>` | **Headless Framing**: Frame any existing image on disk |
| `shotdock` | **Floating Dock**: Launch the interactive GTK4 LayerShell dock |

### Modifier Flags

Combine these flags with any capture mode:

```sh
# Open capture immediately in your annotation editor (satty or swappy)
shotdock -a -e

# Force bypass editor even if "open_in_editor": true in config
shotdock -w --no-edit

# Studio capture without drop shadow
shotdock -w --no-shadow

# Studio capture without macOS window titlebar
shotdock -w --no-titlebar

# Freeze moving screen content during area selection
shotdock -a --freeze
```

---

## Compositor Setup & Keybindings

> [!NOTE]
> **Keybindings are completely up to you!**  
> `shotdock` does not force any specific bindings. You are free to assign any key combinations in your compositor config to the `shotdock` CLI commands above.

Below are **suggested examples** illustrating how you can integrate `shotdock` into your setup:

### Hyprland (`~/.config/hypr/hyprland.conf`)

```ini
# --- Suggested shotdock Keybindings (Customize to your preference) ---

# Clean raw snip (drag region or click window) -> clipboard & notification
bind = SUPER, P, exec, shotdock -a

# Frozen screen snip (freezes video playback during selection)
bind = SUPER CTRL, P, exec, shotdock -a --freeze

# Studio presentation capture (applies rounded corners, shadow & canvas)
bind = SUPER ALT, P, exec, shotdock -w

# Full desktop capture across all monitors
bind = , Print, exec, shotdock -p

# OCR text extraction to clipboard
bind = SUPER CTRL, T, exec, shotdock -t

# Toggle 60 FPS screen recording (prompts for display, window, or area)
bind = SUPER, R, exec, shotdock -r

# Optional: Launch floating visual dock
bind = SUPER SHIFT, D, exec, shotdock

# Layer rules for dock backdrop blur
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

### Sway (`~/.config/sway/config`)

```ini
# --- Suggested shotdock Keybindings ---
bindsym $mod+p exec shotdock -a
bindsym $mod+Ctrl+p exec shotdock -a --freeze
bindsym $mod+Alt+p exec shotdock -w
bindsym Print exec shotdock -p
bindsym $mod+Ctrl+t exec shotdock -t
bindsym $mod+r exec shotdock -r
bindsym $mod+Shift+d exec shotdock
```

### Niri (`~/.config/niri/config.kdl`)

```kdl
// --- Suggested shotdock Keybindings ---
binds {
    Mod+P { spawn "shotdock" "-a"; }
    Mod+Ctrl+P { spawn "shotdock" "-a" "--freeze"; }
    Mod+Alt+P { spawn "shotdock" "-w"; }
    Print { spawn "shotdock" "-p"; }
    Mod+Ctrl+T { spawn "shotdock" "-t"; }
    Mod+R { spawn "shotdock" "-r"; }
    Mod+Shift+D { spawn "shotdock"; }
}
```

---

## Offline Headless Framing (`shotdock frame`)

Transform existing images, diagrams, or screenshots into styled presentation cards without opening a GUI:

```sh
# Basic framing with default theme
shotdock frame input.png -o framed.png

# Frame with Sunset gradient canvas and copy directly to clipboard
shotdock frame screenshot.png -o card.png --theme Sunset -c

# Frame with Breeze gradient and custom decorations
shotdock frame terminal.png --theme Breeze --no-titlebar -o output.png

# Available themes:
# Transparent, FollowSystem, RealWallpaper, Sunset, Candy, Breeze, Raindrop, Midnight, Forest, White, Black
```

---

## Configuration

Configuration is stored in `~/.config/shotdock/config.json`:

```json
{
  "show_cursor": false,
  "freeze": false,
  "window_shadow": true,
  "macos_titlebar": true,
  "canvas_theme": "Transparent",
  "timer_seconds": 0,
  "save_to_disk": true,
  "copy_to_clipboard": true,
  "open_in_editor": false,
  "save_dir": "~/Pictures/Screenshots",
  "editor": null,
  "ocr_lang": "eng",
  "studio_quality": true,
  "record_fps": 60
}
```

### Canvas Themes

- `"Transparent"`: Clean alpha PNG with soft drop shadow.
- `"FollowSystem"`: Dynamic gradient matching your current wallpaper palette (Wallbash).
- `"RealWallpaper"`: Screenshot centered over your blurred desktop wallpaper.
- `"Sunset"`: Rose to Violet gradient (`#f43f5e` to `#8b5cf6`).
- `"Candy"`: Vibrant Violet gradient (`#ec4899` to `#a855f7`).
- `"Breeze"`: Cyan to Blue gradient (`#06b6d4` to `#3b82f6`).
- `"Raindrop"`: Ocean to Indigo gradient (`#3b82f6` to `#6366f1`).
- `"Midnight"`: Deep dark slate gradient (`#1e1b4b` to `#0f172a`).
- `"Forest"`: Emerald to Mint gradient (`#059669` to `#10b981`).
- `"White"` / `"Black"`: Minimalist studio backdrops.

---

## Dependencies

Runtime requirements:

- `gtk4` & `gtk4-layer-shell`
- `grim` & `slurp`
- `imagemagick` (ImageMagick 7 for shadow and canvas pipelines)
- `wl-clipboard`
- `libnotify` (`notify-send`)
- `hyprpicker` *(optional, for screen freezing during selection)*
- `tesseract` & `tesseract-data-eng` *(optional, for OCR text extraction)*
- `wf-recorder` *(optional, for video screen recording)*
- `satty` or `swappy` *(optional, for annotation)*
- `rofi` *(optional, for interactive recording target menu)*

### Arch Linux

```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify hyprpicker tesseract tesseract-data-eng wf-recorder swappy rofi
```

### Fedora

```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel grim slurp ImageMagick wl-clipboard libnotify tesseract wf-recorder swappy rofi
```

---

## Installation

### Arch Linux (AUR)

```sh
# Using yay
yay -S shotdock

# Using paru
paru -S shotdock
```

### From Source

```sh
git clone https://github.com/sirrryasir/shotdock.git
cd shotdock
cargo build --release
sudo install -Dm755 target/release/shotdock /usr/local/bin/shotdock
```

---

## Contributing & Roadmap

- [ROADMAP.md](ROADMAP.md) - Active milestones and features.
- [CONTRIBUTING.md](CONTRIBUTING.md) - Development setup, code standards, and PR workflows.

---

## License

This project is licensed under the [MIT License](LICENSE). Copyright (c) 2026 Yasir.
