# Screen Recording

`shotdock` integrates with `wf-recorder` for hardware-accelerated, visually lossless H.264 video recordings.

---

## Studio Quality Profile

When `studio_quality` is enabled in configuration (default):
- Codec: `libx264`
- Rate control: Constant Rate Factor `crf=18` (visually lossless)
- Frame rate: 60 FPS (`record_fps`)
- Pixel format: `yuv420p`
- Encoding preset: `veryfast`

---

## Interactive Target Selection

When you trigger screen recording:

```sh
shotdock -r
```

`shotdock` launches an interactive selector (using `rofi` if installed):
1. **Specific Display**: Pick which connected monitor to record.
2. **Window or Custom Area**: Select an active window or drag any custom region using `slurp`.

If you prefer to bypass the selection menu and immediately record a region:

```sh
shotdock --record-area
```

---

## Toggle & Process Management

- **Start**: Run `shotdock -r`. A notification confirms recording has started.
- **Stop**: Run `shotdock -r` again (or press your assigned stop shortcut).
- **Clean Output**: `wf-recorder` is cleanly signaled (`SIGINT`) to flush MP4 headers without corrupting video data.
- **Clipboard**: The full path to the recorded `.mp4` file is automatically copied to your Wayland clipboard (`wl-clipboard`).
- **Save Location**: Videos are stored in `~/Videos/Recordings/`.

---

## Process Isolation

Recording instances are tracked via `$XDG_RUNTIME_DIR/shotdock/record.pid`. Process checks are strictly scoped to the active user's UID to prevent process table collisions.
