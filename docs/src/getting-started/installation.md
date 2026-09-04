# Installation

`shotdock` is built with native Wayland protocols and GTK4 layer-shell libraries.

---

## Dependencies

### Core Requirements
- `gtk4` & `gtk4-layer-shell`
- `grim` & `slurp`
- `imagemagick` (ImageMagick 7 for shadow and canvas pipelines)
- `wl-clipboard`
- `libnotify` (`notify-send`)

### Optional Tools
- `hyprpicker`: Freezes screen animations during area snips (`--freeze` / `-z`)
- `tesseract` & `tesseract-data-eng`: Optical Character Recognition (`-t`)
- `wf-recorder`: Video screen recording (`-r`, `--record-area`)
- `satty` or `swappy`: Interactive annotation editor
- `rofi`: Interactive recording target selection menu

---

## Package Manager Installation

### Arch Linux (AUR)

```sh
# Using yay
yay -S shotdock

# Using paru
paru -S shotdock
```

Install recommended optional dependencies on Arch:
```sh
sudo pacman -S hyprpicker tesseract tesseract-data-eng wf-recorder swappy rofi
```

### Fedora

```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel grim slurp ImageMagick wl-clipboard libnotify tesseract wf-recorder swappy rofi
```

---

## Building from Source

Ensure Rust stable is installed:

```sh
git clone https://github.com/sirrryasir/shotdock.git
cd shotdock
cargo build --release
sudo install -Dm755 target/release/shotdock /usr/local/bin/shotdock
```

### Cargo Install

Install directly into Cargo's binary directory:

```sh
cargo install --path .
```
