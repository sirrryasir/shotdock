# Screen Recording

`shotdock` integrates with `wf-recorder` for hardware-accelerated H.264 video captures.

---

## Studio Quality Profile

When `studio_quality` is enabled in configuration (default):
- Codec: `libx264`
- Rate control: Constant Rate Factor `crf=18` (visually lossless)
- Frame rate: 60 FPS (`record_fps`)
- Pixel format: `yuv420p`
- Encoding preset: `veryfast`

---

## Process Isolation

Recording instances are tracked via `$XDG_RUNTIME_DIR/shotdock/record.pid`. Starting or stopping recordings uses native POSIX `SIGINT` signaling without global process table collisions.

---

## CLI Shortcuts

```sh
shotdock -r            # Toggle fullscreen 60 FPS recording
shotdock --record-area # Toggle selected region recording
```

Stopping a recording generates a desktop notification with an `Open Video` action.
