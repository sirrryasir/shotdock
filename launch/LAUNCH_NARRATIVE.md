# Product Launch Narrative: shotdock

## Category & Positioning
- **Product Name**: shotdock
- **Category**: Wayland Desktop Productivity / Screen Capture Utility
- **Core Value Proposition**: An anchored, floating screenshot and screen recording dock for Wayland compositors with native window framing and presentation canvas backgrounds.

---

## The Strategic Narrative

### 1. Why Now (The Context)
Wayland compositors (specifically Hyprland and Sway) have matured from experimental enthusiast rices to daily-driver production environments for tens of thousands of developers, designers, and Linux power users. 

However, screen capture on Linux has remained fragmented:
- Users typically rely on loose combinations of `grim`, `slurp`, `swappy`, and dozens of lines of unmaintained Bash scripts bound to arbitrary keys.
- Sharing clean screenshots requires external websites or manual post-processing in GIMP or Figma to add shadows, rounded borders, or padded background frames.
- There is no unified, aesthetic on-screen utility that provides immediate visual controls, countdown timers, area recording, OCR, and presentation-ready output in a single native tool.

The modern Linux desktop deserves a dedicated, cohesive tool built natively for Wayland layer-shell protocols.

### 2. Why Us (The Architecture)
`shotdock` is built in Rust using GTK4 and `gtk4-layer-shell`. It integrates directly with Wayland compositor state:
- Zero overhead: Parses PNG dimensions directly from in-memory byte headers without invoking slow external subprocesses like ImageMagick `identify`.
- Compositor Native: Automatically queries `hyprctl` for active window boundaries, monitors, and geometry.
- Environment Aware: Integrates directly with the user's active desktop wallpaper and color scheme (Wallbash palettes and blurred wallpaper caches).
- Zero Node.js footprint: Pure Rust from binary execution to the `mdBook` documentation engine.

### 3. Why This (Product Philosophy)
`shotdock` does not try to be an electron bloatware tool or a simple headless script. It combines:
1. **Dock-first Ergonomics**: A responsive floating pill dock anchored to the screen bottom with backdrop blur, smooth keyboard controls (`Escape` to close, `Enter` to capture), and options popovers.
2. **Built-in Presentation Framing**: Automatic 16px anti-aliased rounded corners, omnidirectional Gaussian drop shadows, and titlebar decorations so captures are immediately presentation-ready without post-editing.
3. **Canvas Palettes**: Dynamic backgrounds that match the user's active wallpaper, system colors, or clean solid studio backdrops.
4. **Rich Notification Cards**: Immediate visual feedback in SwayNC with interactive action buttons (`[ Annotate ]` via Swappy and `[ Delete ]` to clean disk).
