# Demo Video Specification & Storyboard: shotdock

Target Duration: 30 to 45 seconds  
Aspect Ratio: 16:9 (1920x1080) or 9:16 for vertical shorts  
Format: MP4 (H.264), 60 FPS, crisp UI text, high contrast  
Tone: Fast, functional, silent with UI clicks or subtle mechanical typing sounds. No filler narration.

---

## Shot List & Sequence

### Shot 1: The Problem & Trigger (0:00 - 0:06)
- **Visual**: Clean Hyprland desktop with a terminal window open (Neovim or btop).
- **Action**: User presses keybinding (`SUPER + SHIFT + D`).
- **Effect**: The `shotdock` pill smoothly floats into view at the bottom edge with background blur.
- **On-Screen Label**: "Native Wayland Floating Dock"

### Shot 2: Window Snip & Automatic Framing (0:06 - 0:14)
- **Visual**: Click "Window Capture" (``).
- **Action**: Click the terminal window.
- **Effect**: Shotdock captures the window, applies 16px anti-aliased rounded corners, and generates an omnidirectional Gaussian drop shadow with mock traffic lights.
- **Feedback**: SwayNC notification slides in showing the formatted thumbnail preview card.
- **On-Screen Label**: "Automatic 16px Rounded Corners & Soft Drop Shadow"

### Shot 3: Presentation Canvas Themes (0:14 - 0:24)
- **Visual**: Open the `Options` popover on the dock.
- **Action**: Change "Canvas Background" from `Transparent` to `Follow System (Wallbash)`, then `Real Wallpaper (Blurred)`, then `Sunset`.
- **Action**: Take an area capture (`󰒅`) of a code block in Neovim.
- **Effect**: Output image is seamlessly framed on the chosen canvas theme, ready for social media or technical documentation.
- **On-Screen Label**: "Canvas Backgrounds: Wallpaper Blur & System Palettes"

### Shot 4: OCR Text Extraction (0:24 - 0:31)
- **Visual**: Terminal showing an error message or non-selectable text.
- **Action**: Click OCR (`󰈙`), drag selection box over the text.
- **Effect**: Notification toasts: "OCR Completed: Copied to clipboard". User pastes directly into an editor buffer.
- **On-Screen Label**: "Direct-to-Clipboard OCR"

### Shot 5: Screen Recording & Interactive Notifications (0:31 - 0:38)
- **Visual**: Click "Record" (`󰕧`). Dock indicates recording state.
- **Action**: Do a quick window movement or terminal command. Click Stop (`󰑋`).
- **Effect**: Notification arrives with `[ Open Video ]` button. User clicks button, video plays immediately.
- **On-Screen Label**: "H.264 Recording with Process Isolation"

### Shot 6: End Card / Call to Action (0:38 - 0:42)
- **Visual**: Terminal centered on screen.
- **Command**: `yay -S shotdock`
- **Subtext**:
  - GitHub: github.com/sirrryasir/shotdock
  - Docs: sirrryasir.github.io/shotdock
  - License: MIT | Written in Rust
