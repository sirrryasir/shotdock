## Isolated Recording Engine

`shotdock` integrates with `wf-recorder` for lightweight MP4/H.264 video captures:

- **PID Tracking**: Ensures recording processes can be started and stopped reliably without interfering with other compositor tasks.
- **Region Recording**: Interactively select the screen area you want to capture as video.
- **Interactive Open Action**: Click **Open Video** directly from the notification to play back the recording.

```sh
shotdock -r            # Toggle fullscreen recording
shotdock --record-area # Toggle area recording
```
