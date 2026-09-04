# Capture Modes

`shotdock` provides 4 primary capture modes, accessible from the floating dock or direct CLI flags.

![Toolbar](../images/toolbar.png)

---

## 1. Fullscreen

Captures the currently focused monitor across Hyprland, Sway, and Niri:

```sh
shotdock -f
```

---

## 2. Active Window

Queries active window coordinates directly from the compositor IPC (`hyprctl activewindow`, `swaymsg -t get_tree`, or `niri msg --json focused-window`):

```sh
shotdock -w
```

---

## 3. Area Selection

Interactive drag-selection for arbitrary rectangular regions using `slurp`:

```sh
shotdock -a
```

---

## 4. Optical Character Recognition (OCR)

Selects a screen region, runs Tesseract OCR engine, and copies plain text directly to the Wayland clipboard:

```sh
shotdock -t
```
