# shotdock

Modern floating screenshot and screen recording toolbar for Wayland compositors (Hyprland, Sway, Wayfire).

`shotdock` provides an anchored, responsive floating pill toolbar with deep compositor integration, window framing, soft drop shadows, and canvas backgrounds.

---

## Features

- **Interactive Floating Dock**: Responsive GTK4 LayerShell dock anchored seamlessly with backdrop blur and smooth transitions.
- **macOS Window Framing**: Automatic 16px anti-aliased rounded corners, omnidirectional Gaussian drop shadows, and dark mock window titlebars with traffic lights (`🔴 🟡 🟢`).
- **Canvas Background Palettes**:
  - `Transparent`: Clean 32-bit alpha PNG with natural drop shadow.
  - `Follow System (Wallbash)`: Generates dynamic gradient backgrounds matching your active Hyprland wallpaper palette.
  - `Real Wallpaper (Blurred)`: Centers your window over your active desktop wallpaper with Gaussian blur.
  - `Aesthetic Gradients`: Sunset, Candy, Breeze, Raindrop, Midnight, and Forest.
  - `Solid Canvas`: Clean White and Dark Charcoal.
- **Universal Capture Modes**:
  - `󰹑 Fullscreen`: Captures the currently focused monitor or spans displays.
  - ` Active Window`: Automatically detects focused Hyprland window boundaries.
  - `󰒅 Area Selection`: Drag to snip any region, terminal snippet, or code block into a presentation-ready card.
  - `󰈙 OCR (Optical Character Recognition)`: Snip any image or screen text to extract plain text directly to the clipboard via Tesseract.
  - `󰕧 / 󰑋 Screen Recording`: High-performance H.264/MP4 recording with isolated PID lifecycle and one-click playback.
- **Rich Notification Preview Cards**:
  - Full-width image thumbnail preview directly in SwayNC / notification center.
  - Interactive action buttons: `[ Annotate ]` (Swappy) and `[ Delete ]` (removes file from disk).
  - Clean status messages without raw path clutter.

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
- `swappy` (optional, for screenshot annotation)

### Arch Linux

```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify tesseract tesseract-data-eng wf-recorder swappy
```

---

## Build and Installation

### Arch Linux (AUR)

`shotdock` is available on Arch Linux via the AUR:

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
shotdock -f        # Capture focused screen
shotdock -a        # Interactively snip selected area
shotdock -w        # Capture active window
shotdock -t        # Extract text from area (OCR)
shotdock -r        # Toggle fullscreen video recording
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
  "save_dir": "~/Pictures/Screenshots"
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

# Layer rules for backdrop blur
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

### Sway

Add to `~/.config/sway/config`:

```ini
bindsym $mod+Shift+d exec shotdock
```

---

## License

MIT License. Copyright (c) 2026 Yasir.
