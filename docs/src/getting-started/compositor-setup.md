# Compositor Setup

## Hyprland

Add keybindings and layer-shell blur rules to `~/.config/hypr/hyprland.conf`:

```ini
# Trigger floating dock
bind = SUPER SHIFT, D, exec, shotdock

# Direct shortcuts
bind = SUPER, P, exec, shotdock -a
bind = SUPER CTRL, P, exec, shotdock -w
bind = SUPER ALT, P, exec, shotdock -f

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
```

---

## Niri

Add to `~/.config/niri/config.kdl`:

```kdl
binds {
    Mod+Shift+D { spawn "shotdock"; }
    Mod+P { spawn "shotdock" "-a"; }
}
```
