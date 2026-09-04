# Capture Modes

`shotdock` provides 6 scriptable capture modes, accessible from compositor keybindings, CLI flags, or the floating dock.

![Toolbar](../images/toolbar.png)

---

## 1. Area Selection (with Window Snapping)

Interactive selection via `slurp`. Drag any arbitrary rectangular box, or **single-click on any window** to snap to its exact geometry:

```sh
shotdock -a
```

---

## 2. Screen Freeze Selection

Freezes display buffers and moving video/animations during area selection via `hyprpicker`:

```sh
# Freeze screen during snip
shotdock -a --freeze

# Or shorthand
shotdock -z
```

---

## 3. Active Window

Captures the currently focused window directly via compositor IPC (`hyprctl activewindow`, `swaymsg -t get_tree`, or `niri msg --json focused-window`):

```sh
shotdock -w
```

---

## 4. Focused Monitor

Captures the entire screen of the currently focused monitor:

```sh
shotdock -f
```

---

## 5. All Connected Monitors

Captures the full spanning desktop across all connected monitors (ideal for the `Print` key):

```sh
shotdock --all

# Or shorthand
shotdock -p
```

---

## 6. Optical Character Recognition (OCR)

Selects a screen region, runs Tesseract OCR engine, and copies plain text directly to the Wayland clipboard:

```sh
shotdock -t
```
