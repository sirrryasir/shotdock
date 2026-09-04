# Roadmap

Development milestones for shotdock on modern Wayland compositors (Hyprland, Sway, Niri, River, Wayfire).

## Architecture

- UI: GTK4 LayerShell (`zwlr_layer_shell_v1`).
- Capture: grim + slurp (`wlr-screencopy-unstable-v1`).
- Recording: wf-recorder (Hardware-accelerated H.264/HEVC via wlroots).
- Audio: PipeWire / PulseAudio via wf-recorder `-a`.
- Theming: Dynamic dotfile/wallbash palette extraction and CSS runtime injection.

## Phase 1: Core (Complete)

- [x] GTK4 LayerShell pill dock with backdrop blur integration.
- [x] macOS window framing: rounded corners, drop shadow, mock titlebar.
- [x] 60 FPS H.264 recording profile (CRF 18) via wf-recorder.
- [x] Native compositor query integration:
  - Hyprland (`hyprctl`)
  - Sway (`swaymsg`)
  - Niri (`niri msg`)
- [x] Wallbash/Pywal and blurred wallpaper canvas backdrops.
- [x] OCR via Tesseract directly to clipboard.
- [x] Isolated runtime security (`$XDG_RUNTIME_DIR/shotdock`, mode 0700).
- [x] Annotation editor auto-detection (satty, swappy).

## Phase 2: Capture & Recording Enhancements (Active)

- [ ] Audio recording toggle in options popover: system audio and microphone via PipeWire (`wf-recorder -a`).
- [ ] Aspect ratio canvas presets: padded framing for 16:9, 9:16 (Shorts/Reels), 1:1, and 4:3.
- [ ] Cursor highlight and click indicator overlay during recording.
- [ ] Language selection dropdown for Tesseract OCR in options popover.
- [ ] Quick video format exporter: MP4 to GIF (`gifski`) and WebM directly from notification cards.

## Phase 3: Compositor Deepening

- [ ] River compositor integration via `riverctl` window and output queries.
- [ ] Fractional scaling pixel-perfection checks across mixed DPI multi-monitor setups.
- [ ] Custom layer-shell entry and exit animations.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions, code standards, and PR workflows.
