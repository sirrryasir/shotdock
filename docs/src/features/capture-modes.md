## 1. Fullscreen (`󰹑`)

Captures the active, focused monitor with display resolution fidelity:

```sh
shotdock -f
```

## 2. Active Window (``)

Automatically queries `hyprctl activewindow` to get the focused window coordinates, trimming extraneous borders or shadow artifacts:

```sh
shotdock -w
```

## 3. Area Selection (`󰒅`)

Allows you to drag-select any rectangular portion of the screen using `slurp`:

```sh
shotdock -a
```

## 4. Optical Character Recognition (`󰈙`)

Select any area containing text, error dialogs, or documentation. `shotdock` runs Tesseract OCR and copies plain text immediately to your Wayland clipboard:

```sh
shotdock -t
```
