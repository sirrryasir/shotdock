## Hyprland

Add keybindings and layer-shell blur rules to your Hyprland configuration (`~/.config/hypr/hyprland.conf` or `binds.lua`):

```ini
# Trigger floating dock
bind = SUPER SHIFT, D, exec, shotdock

# Direct scriptable shortcuts
bind = SUPER, P, exec, shotdock -a
bind = SUPER CTRL, P, exec, shotdock -w
bind = SUPER ALT, P, exec, shotdock -f

# Layer rules for backdrop blur
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

## Sway

Add to `~/.config/sway/config`:

```ini
bindsym $mod+Shift+d exec shotdock
```
