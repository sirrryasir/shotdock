#!/usr/bin/env bash
set -e

OUTPUT_DIR="/mnt/shared/Videos"
OUTPUT_FILE="$OUTPUT_DIR/shotdock-demo.mp4"
PID_FILE="/tmp/shotdock_demo_wf.pid"

mkdir -p "$OUTPUT_DIR"

if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    echo "Stopping existing recording process: $OLD_PID"
    kill -INT "$OLD_PID" 2>/dev/null || true
    rm -f "$PID_FILE"
    exit 0
fi

echo "Starting shotdock demo recording to $OUTPUT_FILE..."
wf-recorder -f "$OUTPUT_FILE" -c libx264 -p crf=18 -p preset=fast &
WF_PID=$!
echo "$WF_PID" > "$PID_FILE"
echo "Recording started (PID: $WF_PID)."
echo "Follow the storyboard in launch/DEMO_VIDEO_SCRIPT.md."
echo "Run this script again or press Ctrl+C to finish."

trap 'kill -INT $WF_PID 2>/dev/null; rm -f "$PID_FILE"; echo "Saved to $OUTPUT_FILE"; exit 0' SIGINT SIGTERM
wait "$WF_PID"
