# Installation

`shotdock` uses native Wayland protocols and GTK4 layer-shell libraries.

---

## Dependencies

Runtime requirements:

- `gtk4` & `gtk4-layer-shell`
- `grim` & `slurp`
- `imagemagick` (ImageMagick 7 for shadow and canvas pipelines)
- `wl-clipboard`
- `libnotify` (`notify-send`)
- `tesseract` (optional, for OCR text extraction)
- `wf-recorder` (optional, for video screen recording)
- `satty` or `swappy` (optional, for screenshot annotation)

### Arch Linux

Install the runtime dependencies via `pacman`:

```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify tesseract tesseract-data-eng wf-recorder swappy
```

### Fedora

Install packages via `dnf`:

```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel grim slurp ImageMagick wl-clipboard libnotify tesseract wf-recorder swappy
```

---

## Installation Methods

### Arch Linux (AUR)

`shotdock` is available in the Arch User Repository:

```sh
# Using yay
yay -S shotdock

# Using paru
paru -S shotdock
```

---

## Building from Source

Ensure Rust is installed via `rustup`:

```sh
git clone https://github.com/sirrryasir/shotdock.git
cd shotdock
cargo build --release
sudo install -Dm755 target/release/shotdock /usr/local/bin/shotdock
```

### Cargo

Install directly into Cargo binary path:

```sh
cargo install --path .
```
