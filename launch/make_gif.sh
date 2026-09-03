#!/usr/bin/env bash
set -e

INPUT="${1:-/mnt/shared/Videos/shotdock-demo.mp4}"
OUTPUT="${2:-/mnt/shared/Videos/shotdock-demo.gif}"

if [ ! -f "$INPUT" ]; then
    echo "Error: Input file $INPUT does not exist."
    exit 1
fi

echo "Generating high-quality GIF from $INPUT..."
PALETTE="/tmp/shotdock_palette.png"
ffmpeg -y -i "$INPUT" -vf "fps=20,scale=960:-1:flags=lanczos,palettegen" "$PALETTE"
ffmpeg -y -i "$INPUT" -i "$PALETTE" -filter_complex "fps=20,scale=960:-1:flags=lanczos[x];[x][1:v]paletteuse" "$OUTPUT"
rm -f "$PALETTE"

echo "GIF generated: $OUTPUT ($(du -h "$OUTPUT" | cut -f1))"
