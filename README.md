# shotdock

Automated window framing, soft drop shadows, and studio screen capture for Wayland (**Hyprland**, **Sway**, **Niri**, **Wayfire**).

*The CleanShot X aesthetic pipeline for Linux Wayland.*

![shotdock demo](assets/demo.gif)

`shotdock` transforms raw screenshots into presentation-ready cards with 16px rounded corners, multi-pass Gaussian drop shadows, mock titlebars, and wallpaper gradients. Use it as an anchored floating dock or invoke it completely headlessly from your compositor keybindings.

---

## Showcase

| Floating Dock | Presentation Canvas |
|:---:|:---:|
| ![Floating Dock](assets/toolbar.png) | ![Canvas Theme](assets/canvas_presentation.png) |

---

## Features

- **CleanShot X Aesthetic Pipeline**: Automatic 16px anti-aliased rounded corners, omnidirectional Gaussian drop shadows, and dark mock window titlebars.
- **Presentation Canvas Backgrounds**: Wrap any capture in aesthetic backdrops:
  - `Transparent` (clean alpha PNG with soft drop shadow)
  - `FollowSystem (Wallbash)` (dynamically matches active wallpaper palette)
  - `RealWallpaper (Blurred)` (centers screenshot over your blurred wallpaper)
  - `Gradients` (Sunset, Candy, Breeze, Raindrop, Midnight, Forest)
  - `Solid Studio` (Minimalist White & Black)
- **Keybind-First or Floating Dock**: Run headlessly with instant CLI shortcuts or trigger an interactive GTK4 LayerShell dock.
- **Screen Freeze**: Freeze moving content and video playback during area selection via `--freeze` (`hyprpicker`).
- **Click-to-Snap Window**: In area mode (`-a`), drag any custom rectangle or single-click any window to capture its exact geometry.
- **Offline Headless Framing**: Frame any existing image file from scripts or CI/CD via `shotdock frame <file>`.
- **60 FPS Studio Recording**: Visually lossless H.264/MP4 recording (CRF 18) with isolated PID tracking and one-click playback.
- **Multi-Compositor IPC**: Direct coordinate queries for Hyprland (`hyprctl`), Sway (`swaymsg`), and Niri (`niri msg`).
- **Rich Notification Actions**: Interactive cards in your notification daemon with `[ Annotate ]` (auto-detects `satty` / `swappy`) and `[ Delete ]`.

---

## Dependencies

Runtime requirements:

- `gtk4` & `gtk4-layer-shell`
- `grim` & `slurp`
- `imagemagick` (ImageMagick 7 for shadow and canvas pipelines)
- `wl-clipboard`
- `libnotify` (`notify-send`)
- `hyprpicker` (optional, for screen freezing during selection)
- `tesseract` (optional, for OCR text extraction)
- `wf-recorder` (optional, for video screen recording)
- `satty` or `swappy` (optional, for screenshot annotation)

### Arch Linux

```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify hyprpicker tesseract tesseract-data-eng wf-recorder swappy
```

### Fedora

```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel grim slurp ImageMagick wl-clipboard libnotify tesseract wf-recorder swappy
```

---

## Build and Installation

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

## Usage

### 1. Scriptable Keybindings & CLI

Bypass the GUI toolbar for instantaneous scriptable captures:

```sh
shotdock -a            # Snip area or click any window to capture
shotdock -a --freeze   # Freeze screen during area selection (or shotdock -z)
shotdock -w            # Capture active focused window
shotdock -f            # Capture focused monitor
shotdock -p            # Capture all connected monitors (Print key)
shotdock -t            # Optical Character Recognition (extract text to clipboard)
shotdock -r            # Toggle fullscreen 60 FPS recording
shotdock --record-area # Toggle selected region recording
```

### 2. Frame Existing Images

Apply shadows, titlebars, and canvas backgrounds to any existing image on disk:

```sh
# Frame with default theme
shotdock frame input.png -o framed.png

# Frame with Sunset gradient canvas and copy to clipboard
shotdock frame screenshot.png -o card.png --theme Sunset -c

# Available themes: Transparent, FollowSystem, RealWallpaper, Sunset, Candy, Breeze, Raindrop, Midnight, Forest, White, Black
```

### 3. Interactive Floating Dock

Launch the dock:

```sh
shotdock
```

- `Escape`: Close dock
- `Enter`: Trigger capture using currently selected mode

---

## Configuration

Configuration is located at `~/.config/shotdock/config.json`:

```json
{
  "show_cursor": false,
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
  "record_fps": 60,
  "freeze": false
}
```

### Canvas Themes

- `"Transparent"`
- `"FollowSystem"`
- `"RealWallpaper"`
- `"White"`
- `"Black"`
- `"Sunset"`
- `"Candy"`
- `"Breeze"`
- `"Raindrop"`
- `"Midnight"`
- `"Forest"`

---

## Compositor Setup

### Hyprland

Add the following to `~/.config/hypr/hyprland.conf`:

```ini
# Trigger floating dock
bind = SUPER SHIFT, D, exec, shotdock

# Direct shortcuts
bind = SUPER, P, exec, shotdock -a                  # Area selection or click window
bind = SUPER CTRL, P, exec, shotdock -a --freeze    # Frozen screen area selection
bind = SUPER ALT, P, exec, shotdock -f              # Focused monitor
bind = , Print, exec, shotdock --all                # All connected monitors

# Layer rules for backdrop blur
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

### Sway

Add to `~/.config/sway/config`:

```ini
bindsym $mod+Shift+d exec shotdock
bindsym $mod+p exec shotdock -a
bindsym $mod+Ctrl+p exec shotdock -a --freeze
bindsym $mod+Alt+p exec shotdock -f
bindsym Print exec shotdock --all
```

### Niri

Add to `~/.config/niri/config.kdl`:

```kdl
binds {
    Mod+Shift+D { spawn "shotdock"; }
    Mod+P { spawn "shotdock" "-a"; }
    Mod+Ctrl+P { spawn "shotdock" "-a" "--freeze"; }
    Mod+Alt+P { spawn "shotdock" "-f"; }
    Print { spawn "shotdock" "--all"; }
}
```

---

## Contributing & Roadmap

- [ROADMAP.md](ROADMAP.md) - Active milestones and features.
- [CONTRIBUTING.md](CONTRIBUTING.md) - Environment setup, engineering standards, and PR workflows.

---

## License

This project is licensed under the [MIT License](LICENSE). Copyright (c) 2026 Yasir.
