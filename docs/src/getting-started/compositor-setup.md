# Compositor Setup

## Hyprland

Add keybindings and layer-shell blur rules to `~/.config/hypr/hyprland.conf`:

```ini
# Trigger floating dock
bind = SUPER SHIFT, D, exec, shotdock

# Direct shortcuts
bind = SUPER, P, exec, shotdock -a                  # Area selection or click window
bind = SUPER CTRL, P, exec, shotdock -a --freeze    # Frozen screen area selection
bind = SUPER ALT, P, exec, shotdock -f              # Focused monitor
bind = , Print, exec, shotdock --all                # All connected monitors

# Layer rules for backdrop blur
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

---

## Sway

Add to `~/.config/sway/config`:

```ini
bindsym $mod+Shift+d exec shotdock
bindsym $mod+p exec shotdock -a
bindsym $mod+Ctrl+p exec shotdock -a --freeze
bindsym $mod+Alt+p exec shotdock -f
bindsym Print exec shotdock --all
```

---

## Niri

Add to `~/.config/niri/config.kdl`:

```kdl
binds {
    Mod+Shift+D { spawn "shotdock"; }
    Mod+P { spawn "shotdock" "-a"; }
    Mod+Ctrl+P { spawn "shotdock" "-a" "--freeze"; }
    Mod+Alt+P { spawn "shotdock" "-f"; }
    Print { spawn "shotdock" "--all"; }
}
```
