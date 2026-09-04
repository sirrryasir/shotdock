# Contributing to shotdock

Check [ROADMAP.md](ROADMAP.md) for open items and planned milestones.

---

## Codebase Architecture

The project is structured into focused modules:

| Module | Responsibility |
| :--- | :--- |
| [`src/main.rs`](src/main.rs) | CLI flag parsing, single-instance runtime PID toggling, GTK4 application launch. |
| [`src/capture.rs`](src/capture.rs) | Capture pipelines, multi-compositor detection (Hyprland, Sway, Niri), ImageMagick 4K processing, `wf-recorder` lifecycle. |
| [`src/ui.rs`](src/ui.rs) | Floating dock widget tree, GTK4 LayerShell anchoring, focused monitor placement, options popover. |
| [`src/config.rs`](src/config.rs) | JSON persistence at `~/.config/shotdock/config.json`, defaults, and backward compatibility. |
| [`src/theme.rs`](src/theme.rs) | CSS generation, wallbash/pywal color extraction, responsive glassmorphism styles. |

---

## Development Setup

### Prerequisites

- Rust stable (2024 edition)
- GTK4 & GTK4 Layer Shell development libraries
- A Wayland session with layer-shell support (Hyprland, Sway, Niri, River, Wayfire)

**Arch Linux:**
```sh
sudo pacman -S gtk4 gtk4-layer-shell grim slurp imagemagick wl-clipboard libnotify tesseract wf-recorder swappy satty hyprpicker
```

**Fedora:**
```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel grim slurp ImageMagick wl-clipboard libnotify tesseract wf-recorder hyprpicker
```

**Ubuntu / Debian (24.04+):**
```sh
sudo apt install libgtk-4-dev libgtk4-layer-shell-dev grim slurp imagemagick wl-clipboard libnotify-bin tesseract-ocr wf-recorder hyprpicker
```

---

## Local Build & Verification

Always verify your changes compile with zero warnings before opening a PR:

```sh
# Fast syntax & type check
cargo check

# Enforce strict clippy linter standards (zero warnings)
cargo clippy -- -D warnings

# Ensure code formatting matches rustfmt
cargo fmt --check

# Test release build
cargo build --release

# Run locally
./target/release/shotdock
```

---

## Engineering & Security Standards

1. **Memory & File Safety**:
   - Never write sensitive user data (previews, PIDs, recordings) to world-writable `/tmp`. Always use `capture::get_runtime_dir()` (`$XDG_RUNTIME_DIR/shotdock`, mode `0700`).
2. **Process Verification**:
   - Never kill processes blindly by PID without verifying process identity (e.g. via `is_process_running_with_comm`).
   - Scope any process searches to the current user (`-u <UID>`).
3. **Idiomatic Rust**:
   - Zero tolerance for uncontrolled panics (`unwrap()` / `expect()`) in runtime code. Use `?`, `if let`, or `let-else` chains.
   - Avoid unnecessary allocations and unnecessary `.clone()` calls.
4. **Git Commit Messages**:
   - Follow Conventional Commits: `feat: ...`, `fix: ...`, `docs: ...`, `refactor: ...`, `perf: ...`.

---

## Submitting a Pull Request

1. Fork the repository and create a descriptive branch:
   ```sh
   git checkout -b feat/phase2-xdg-portal
   ```
2. Commit your changes with clear messages.
3. Open a Pull Request referencing the related issue or roadmap item.
4. Pull requests are reviewed within 48 hours.
