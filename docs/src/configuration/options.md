## Configuration File

The configuration file is located at `~/.config/shotdock/config.json`:

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

### Schema

- `show_cursor` (boolean): Include mouse cursor in screenshots.
- `window_shadow` (boolean): Render 16px rounded corners and Gaussian drop shadow.
- `macos_titlebar` (boolean): Add dark mock titlebar with traffic light buttons.
- `canvas_theme` (string): Background canvas preset.
- `timer_seconds` (number): Countdown delay before capture (0, 3, 5, 10).
- `save_to_disk` (boolean): Write screenshot PNG to `save_dir`.
- `copy_to_clipboard` (boolean): Copy screenshot PNG to Wayland clipboard.
- `open_in_editor` (boolean): Immediately launch `swappy` after capture.
- `save_dir` (string): Directory for saved screenshots (supports `~/`).
