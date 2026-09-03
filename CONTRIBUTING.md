# Contributing to shotdock

Thank you for your interest in contributing to `shotdock`! We welcome bug reports, feature requests, documentation improvements, and code contributions.

## Development Workflow

### Prerequisites

- Rust 1.85+ (or latest stable)
- GTK4 & GTK4 Layer Shell development libraries
- A Wayland session (Hyprland, Sway, etc.)

On Arch Linux:
```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify tesseract wf-recorder swappy
```

### Local Build & Testing

```sh
# Clone your fork
git clone https://github.com/your-username/shotdock.git
cd shotdock

# Check lints
cargo clippy -- -D warnings

# Format check
cargo fmt --check

# Build and run
cargo run --
```

### Coding Standards

- Run `cargo fmt` prior to committing.
- Ensure `cargo clippy -- -D warnings` produces zero warnings.
- Keep subprocess communication robust and process-isolated.
- Write commit messages following the Conventional Commits specification (`feat: ...`, `fix: ...`, `docs: ...`, `refactor: ...`).

## Pull Requests

1. Fork the repo and create your branch from `main`.
2. Ensure existing functionality continues to work.
3. Submit a Pull Request describing your changes clearly.
