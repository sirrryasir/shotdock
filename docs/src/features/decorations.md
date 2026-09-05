# Window Framing & Canvas Themes

`shotdock` applies rounded corners, drop shadows, window titlebars, and background canvas gradients.

---

## Window Framing

When framing is applied:

1. **Rounded Corners**: 16px anti-aliased corner radius.
2. **Drop Shadow**: Multi-pass Gaussian drop shadow cast evenly on all sides.
3. **Window Titlebar**: Dark mock titlebar with macOS-style window controls.

![Framed Window](../images/framed_window.png)

### Capture Commands

```sh
# Framed capture (titlebar + shadow + canvas)
shotdock -w

# Framed capture without shadow
shotdock -w --no-shadow

# Framed capture without titlebar
shotdock -w --no-titlebar
```

---

## Canvas Themes

| Theme | Description |
|:---|:---|
| `Transparent` | Clean alpha PNG with soft drop shadow |
| `FollowSystem` | Gradient matching active wallpaper palette (Wallbash) |
| `RealWallpaper` | Centers screenshot over blurred desktop wallpaper |
| `Sunset` | Pink to Purple gradient (`#f43f5e` to `#8b5cf6`) |
| `Candy` | Vibrant Violet gradient (`#ec4899` to `#a855f7`) |
| `Breeze` | Cyan to Blue gradient (`#06b6d4` to `#3b82f6`) |
| `Raindrop` | Ocean Indigo gradient (`#3b82f6` to `#6366f1`) |
| `Midnight` | Deep Indigo gradient (`#1e1b4b` to `#0f172a`) |
| `Forest` | Emerald gradient (`#059669` to `#10b981`) |
| `White` / `Black` | Minimalist solid backdrops |

![Canvas Presentation](../images/canvas_presentation.png)

---

## Framing Existing Images (`shotdock frame`)

Frame existing images via CLI:

```sh
# Basic framing
shotdock frame screenshot.png

# Frame with custom output file
shotdock frame terminal.png -o docs/terminal_framed.png

# Apply canvas theme
shotdock frame window.png --theme Sunset -o framed_card.png

# Copy framed output directly to clipboard
shotdock frame code.png -c

# Strip titlebar
shotdock frame app.png --no-titlebar --theme Breeze
```
