# shotdock

Modern floating screenshot and 4K screen recording dock for Wayland compositors (**Hyprland**, **Sway**, **Niri**, **Wayfire**).

![shotdock demo](assets/demo.gif)

`shotdock` provides an anchored floating pill dock, macOS window framing, soft drop shadows, 60 FPS studio recording, and presentation canvas themes.

---

## Showcase

| Floating Dock | Presentation Canvas |
|:---:|:---:|
| ![Floating Dock](assets/toolbar.png) | ![Canvas Theme](assets/canvas_presentation.png) |

---

## Features

- Floating GTK4 LayerShell dock for Wayland.
- Window framing: 16px rounded corners, multi-pass Gaussian drop shadow, dark mock titlebars.
- 60 FPS H.264 screen recording (CRF 18) via wf-recorder with isolated PID tracking.
- Multi-compositor query integration: Hyprland (`hyprctl`), Sway (`swaymsg`), Niri (`niri msg`).
- Canvas backgrounds: transparent PNG, dynamic Wallbash/Pywal palette matching, blurred wallpaper, solid colors, and gradients.
- Capture modes: Fullscreen (focused monitor), Active Window, Area selection, OCR text extraction via Tesseract, Screen Recording.
- Notification preview actions: annotate (`satty` / `swappy` auto-detected), delete file.
- Direct POSIX syscalls (`getuid`, `kill`) and isolated runtime directory (`$XDG_RUNTIME_DIR/shotdock`, mode 0700).
- Roadmap: see [ROADMAP.md](ROADMAP.md).

---

## Dependencies

Runtime requirements:

- `gtk4` & `gtk4-layer-shell`
- `grim` & `slurp`
- `imagemagick` (ImageMagick 7 for shadow and canvas pipelines)
- `wl-clipboard`
- `libnotify` (`notify-send`)
- `tesseract` (optional, for OCR text extraction)
- `wf-recorder` (optional, for video screen recording)
- `satty` or `swappy` (optional, for screenshot annotation)

### Arch Linux

```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify tesseract tesseract-data-eng wf-recorder swappy
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

### Cargo

```sh
cargo install --path .
```

---

## Usage

### Interactive Dock

Launch without arguments:

```sh
shotdock
```

**Keybindings when dock is visible:**
- `Escape`: Close dock
- `Enter`: Trigger capture using the currently active mode

### CLI Shortcuts

Bypass the GUI for instant scriptable hotkeys:

```sh
shotdock -f            # Capture focused screen
shotdock -a            # Interactively snip selected area
shotdock -w            # Capture active window
shotdock -t            # Extract text from area (OCR)
shotdock -r            # Toggle fullscreen 60 FPS video recording
shotdock --record-area # Toggle area video recording
```

---

## Configuration

Configuration is stored at `~/.config/shotdock/config.json`. Options are saved automatically through the Options popover or can be edited directly:

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
  "record_fps": 60
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
# Keybinding
bind = SUPER SHIFT, D, exec, shotdock

# Direct shortcuts
bind = SUPER, P, exec, shotdock -a
bind = SUPER CTRL, P, exec, shotdock -w
bind = SUPER ALT, P, exec, shotdock -f

# Layer rules for backdrop blur
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

### Sway

Add to `~/.config/sway/config`:

```ini
bindsym $mod+Shift+d exec shotdock
bindsym $mod+p exec shotdock -a
```

### Niri

Add to `~/.config/niri/config.kdl`:

```kdl
binds {
    Mod+Shift+D { spawn "shotdock"; }
    Mod+P { spawn "shotdock" "-a"; }
}
```

---

## Contributing & Roadmap

- [ROADMAP.md](ROADMAP.md) - Active milestones and features.
- [CONTRIBUTING.md](CONTRIBUTING.md) - Environment setup, engineering standards, and PR workflows.

---

## License

MIT License. Copyright (c) 2026 Yasir.
