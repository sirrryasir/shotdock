## Dependencies

`shotdock` uses native Wayland and GTK4 layer-shell libraries.

### Arch Linux

Install the core dependencies via `pacman`:

```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify tesseract tesseract-data-eng wf-recorder swappy
```

### Fedora

```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel grim slurp ImageMagick wl-clipboard libnotify tesseract wf-recorder swappy
```

## Installation

### Arch Linux (AUR)

`shotdock` is available in the Arch User Repository (AUR):

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

Or install directly into Cargo bin:

```sh
cargo install --path .
```
