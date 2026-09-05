# Screen Recording

`shotdock` uses `wf-recorder` for H.264 video recordings.

---

## Recording Profile

Default configuration:
- Codec: `libx264`
- Rate control: Constant Rate Factor `crf=18`
- Frame rate: 60 FPS (`record_fps`)
- Pixel format: `yuv420p`
- Preset: `veryfast`

---

## Interactive Target Selection

Trigger screen recording:

```sh
shotdock -r
```

`shotdock` opens a selection menu (via `rofi` if installed):
1. **Display**: Choose a monitor to record.
2. **Window or Area**: Click a window or drag a region using `slurp`.

To start region recording directly without the menu:

```sh
shotdock --record-area
```

---

## Process Management

- **Start**: Run `shotdock -r`.
- **Stop**: Run `shotdock -r` again.
- **Signal**: `wf-recorder` is sent `SIGINT` to cleanly finalize the MP4 container.
- **Clipboard**: File path to the `.mp4` is copied to clipboard.
- **Default Directory**: `~/Videos/Recordings/`.
- **PID File**: Tracked in `$XDG_RUNTIME_DIR/shotdock/record.pid`.
