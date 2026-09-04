# Optical Character Recognition (OCR)

Extract plain text directly from any screen region into your Wayland clipboard.

---

## Usage

1. Trigger OCR mode via the floating dock or CLI:
   ```sh
   shotdock -t
   ```
2. Drag a rectangular selection over any on-screen text, terminal buffer, error dialog, or image.
3. The extracted text is immediately written to your Wayland clipboard (`wl-clipboard`) and an interactive notification displays a preview of the recognized text.

---

## Configuration

Configure the language model in `~/.config/shotdock/config.json`:

```json
{
  "ocr_lang": "eng"
}
```

The language code matches Tesseract dataset codes (e.g., `eng`, `deu`, `fra`, `spa`, `jpn`).

---

## Dependencies

- `tesseract`: The OCR engine
- `tesseract-data-eng`: English language trained data (or language pack of your choice)
- `wl-clipboard`: Wayland clipboard tool (`wl-copy`)
