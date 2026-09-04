# Compositor Setup & Keybindings

`shotdock` is designed to be invoked directly from your compositor keybindings.

> [!NOTE]
> **Keybindings are completely user-defined!**  
> `shotdock` does not force any specific key combinations on your environment. You are free to choose and map any shortcuts that fit your workflow to the corresponding `shotdock` CLI commands.

---

## Suggested Configurations

Below are recommended examples showing how you can map `shotdock` commands in your compositor. Feel free to modify the key combinations to match your personal preferences.

### Hyprland (`~/.config/hypr/hyprland.conf`)

```ini
# --- Suggested shotdock Keybindings (Customize to your preference) ---

# Clean raw snip (drag region or click window) -> clipboard & notification
bind = SUPER, P, exec, shotdock -a

# Frozen screen snip (freezes moving content and video during snip)
bind = SUPER CTRL, P, exec, shotdock -a --freeze

# Studio presentation capture (applies rounded corners, shadow & canvas)
bind = SUPER ALT, P, exec, shotdock -w

# Full desktop capture across all monitors
bind = , Print, exec, shotdock -p

# OCR text extraction to clipboard
bind = SUPER CTRL, T, exec, shotdock -t

# Toggle 60 FPS screen recording (prompts for display, window, or area)
bind = SUPER, R, exec, shotdock -r

# Optional: Launch floating visual dock
bind = SUPER SHIFT, D, exec, shotdock

# Layer rules for dock backdrop blur
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

---

### Sway (`~/.config/sway/config`)

```ini
# --- Suggested shotdock Keybindings ---

# Clean raw snip
bindsym $mod+p exec shotdock -a

# Frozen screen snip
bindsym $mod+Ctrl+p exec shotdock -a --freeze

# Studio presentation capture
bindsym $mod+Alt+p exec shotdock -w

# Full desktop capture across all monitors
bindsym Print exec shotdock -p

# OCR text extraction
bindsym $mod+Ctrl+t exec shotdock -t

# Screen recording
bindsym $mod+r exec shotdock -r

# Optional: Floating visual dock
bindsym $mod+Shift+d exec shotdock
```

---

### Niri (`~/.config/niri/config.kdl`)

```kdl
// --- Suggested shotdock Keybindings ---
binds {
    // Clean raw snip
    Mod+P { spawn "shotdock" "-a"; }

    // Frozen screen snip
    Mod+Ctrl+P { spawn "shotdock" "-a" "--freeze"; }

    // Studio presentation capture
    Mod+Alt+P { spawn "shotdock" "-w"; }

    // Full desktop capture across all monitors
    Print { spawn "shotdock" "-p"; }

    // OCR text extraction
    Mod+Ctrl+T { spawn "shotdock" "-t"; }

    // Screen recording
    Mod+R { spawn "shotdock" "-r"; }

    // Optional: Floating visual dock
    Mod+Shift+D { spawn "shotdock"; }
}
```
