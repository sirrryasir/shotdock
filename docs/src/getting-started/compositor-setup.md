# Compositor Keybindings

Example keybindings for Wayland compositors.

---

## Hyprland (`hyprland.conf`)

```ini
# Screenshots
bind = SUPER, P, exec, shotdock -a
bind = SUPER CTRL, P, exec, shotdock -a --freeze
bind = SUPER ALT, P, exec, shotdock -w
bind = , Print, exec, shotdock -p

# Utilities
bind = SUPER CTRL, T, exec, shotdock -t
bind = SUPER, R, exec, shotdock -r
bind = SUPER SHIFT, D, exec, shotdock

# Floating dock blur rules
layerrule = blur, shotdock
layerrule = ignorezero, shotdock
```

---

## Sway (`config`)

```ini
bindsym $mod+p exec shotdock -a
bindsym $mod+Ctrl+p exec shotdock -a --freeze
bindsym $mod+Alt+p exec shotdock -w
bindsym Print exec shotdock -p
bindsym $mod+Ctrl+t exec shotdock -t
bindsym $mod+r exec shotdock -r
bindsym $mod+Shift+d exec shotdock
```

---

## Niri (`config.kdl`)

```kdl
binds {
    Mod+P { spawn "shotdock" "-a"; }
    Mod+Ctrl+P { spawn "shotdock" "-a" "--freeze"; }
    Mod+Alt+P { spawn "shotdock" "-w"; }
    Print { spawn "shotdock" "-p"; }
    Mod+Ctrl+T { spawn "shotdock" "-t"; }
    Mod+R { spawn "shotdock" "-r"; }
    Mod+Shift+D { spawn "shotdock"; }
}
```
