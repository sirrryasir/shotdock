use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compositor {
    Hyprland,
    Sway,
    Niri,
    Other,
}

pub fn detect() -> Compositor {
    if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return Compositor::Hyprland;
    }
    if std::env::var("SWAYSOCK").is_ok() {
        return Compositor::Sway;
    }
    if std::env::var("NIRI_SOCKET").is_ok() {
        return Compositor::Niri;
    }
    if Command::new("niri")
        .args(["msg", "version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Compositor::Niri;
    }
    if Command::new("hyprctl")
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Compositor::Hyprland;
    }
    if Command::new("swaymsg")
        .arg("-v")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Compositor::Sway;
    }
    Compositor::Other
}

pub fn focused_output() -> Option<String> {
    match detect() {
        Compositor::Hyprland => {
            let out = Command::new("hyprctl")
                .args(["monitors", "-j"])
                .output()
                .ok()?;
            let monitors: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            monitors.as_array()?.iter().find_map(|m| {
                if m.get("focused").and_then(|f| f.as_bool()) == Some(true) {
                    m.get("name").and_then(|n| n.as_str()).map(String::from)
                } else {
                    None
                }
            })
        }
        Compositor::Sway => {
            let out = Command::new("swaymsg")
                .args(["-t", "get_outputs"])
                .output()
                .ok()?;
            let outputs: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            outputs.as_array()?.iter().find_map(|o| {
                if o.get("focused").and_then(|f| f.as_bool()) == Some(true) {
                    o.get("name").and_then(|n| n.as_str()).map(String::from)
                } else {
                    None
                }
            })
        }
        Compositor::Niri => {
            let out = Command::new("niri")
                .args(["msg", "--json", "outputs"])
                .output()
                .ok()?;
            let val: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            if let Some(map) = val.as_object() {
                for (name, data) in map {
                    if data.get("is_focused").and_then(|f| f.as_bool()) == Some(true) {
                        return Some(name.clone());
                    }
                }
            } else if let Some(arr) = val.as_array() {
                for item in arr {
                    if item.get("is_focused").and_then(|f| f.as_bool()) == Some(true) {
                        return item.get("name").and_then(|n| n.as_str()).map(String::from);
                    }
                }
            }
            None
        }
        Compositor::Other => None,
    }
}

pub fn outputs() -> Vec<(String, String)> {
    match detect() {
        Compositor::Hyprland => {
            let mut list = Vec::new();
            if let Ok(out) = Command::new("hyprctl").args(["monitors", "-j"]).output()
                && let Ok(monitors) = serde_json::from_slice::<serde_json::Value>(&out.stdout)
                && let Some(arr) = monitors.as_array()
            {
                for m in arr {
                    if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                        let desc = m.get("description").and_then(|d| d.as_str()).unwrap_or("");
                        list.push((name.to_string(), desc.to_string()));
                    }
                }
            }
            list
        }
        Compositor::Sway => {
            let mut list = Vec::new();
            if let Ok(out) = Command::new("swaymsg").args(["-t", "get_outputs"]).output()
                && let Ok(outputs) = serde_json::from_slice::<serde_json::Value>(&out.stdout)
                && let Some(arr) = outputs.as_array()
            {
                for o in arr {
                    if let Some(name) = o.get("name").and_then(|n| n.as_str()) {
                        let desc = o.get("model").and_then(|d| d.as_str()).unwrap_or("");
                        list.push((name.to_string(), desc.to_string()));
                    }
                }
            }
            list
        }
        Compositor::Niri => {
            let mut list = Vec::new();
            if let Ok(out) = Command::new("niri")
                .args(["msg", "--json", "outputs"])
                .output()
                && let Ok(val) = serde_json::from_slice::<serde_json::Value>(&out.stdout)
                && let Some(map) = val.as_object()
            {
                for (name, _) in map {
                    list.push((name.clone(), String::new()));
                }
            }
            list
        }
        Compositor::Other => Vec::new(),
    }
}

pub fn window_geometry_by_address(addr: &str) -> Option<String> {
    let out = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .ok()?;
    let clients: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
    let arr = clients.as_array()?;
    for c in arr {
        if c.get("address").and_then(|a| a.as_str()) == Some(addr) {
            let at = c.get("at").and_then(|v| v.as_array())?;
            let size = c.get("size").and_then(|v| v.as_array())?;
            let x = at.first()?.as_i64()?;
            let y = at.get(1)?.as_i64()?;
            let w = size.first()?.as_i64()?;
            let h = size.get(1)?.as_i64()?;
            return Some(format!("{},{} {}x{}", x, y, w, h));
        }
    }
    None
}

#[allow(dead_code)]
pub fn active_window_geometry() -> Option<String> {
    match detect() {
        Compositor::Hyprland => {
            let out = Command::new("hyprctl")
                .args(["activewindow", "-j"])
                .output()
                .ok()?;
            let win: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            let at = win.get("at").and_then(|v| v.as_array())?;
            let size = win.get("size").and_then(|v| v.as_array())?;
            if at.len() == 2 && size.len() == 2 {
                let x = at[0].as_i64().unwrap_or(0);
                let y = at[1].as_i64().unwrap_or(0);
                let w = size[0].as_i64().unwrap_or(0);
                let h = size[1].as_i64().unwrap_or(0);
                if w > 10 && h > 10 {
                    return Some(format!("{},{} {}x{}", x, y, w, h));
                }
            }
            None
        }
        Compositor::Sway => {
            let out = Command::new("swaymsg")
                .args(["-t", "get_tree"])
                .output()
                .ok()?;
            let tree: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            find_sway_focused_rect(&tree)
        }
        Compositor::Niri => {
            let out = Command::new("niri")
                .args(["msg", "--json", "focused-window"])
                .output()
                .ok()?;
            let win: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            if let Some(rect) = win.get("rect").or_else(|| win.get("layout_tile")) {
                let x = rect.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
                let y = rect.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
                let w = rect.get("width").and_then(|v| v.as_i64()).unwrap_or(0);
                let h = rect.get("height").and_then(|v| v.as_i64()).unwrap_or(0);
                if w > 10 && h > 10 {
                    return Some(format!("{},{} {}x{}", x, y, w, h));
                }
            }
            None
        }
        Compositor::Other => None,
    }
}

pub fn window_boxes() -> Option<String> {
    match detect() {
        Compositor::Hyprland => {
            let out = Command::new("hyprctl")
                .args(["clients", "-j"])
                .output()
                .ok()?;
            let clients: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            let arr = clients.as_array()?;
            let mut boxes = String::new();
            for win in arr {
                let mapped = win.get("mapped").and_then(|v| v.as_bool()).unwrap_or(false);
                let ws = win
                    .get("workspace")
                    .and_then(|v| v.get("id"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                if !mapped || ws <= 0 {
                    continue;
                }
                if let (Some(a), Some(s)) = (
                    win.get("at").and_then(|v| v.as_array()),
                    win.get("size").and_then(|v| v.as_array()),
                ) && a.len() == 2
                    && s.len() == 2
                {
                    let x = a[0].as_i64().unwrap_or(0);
                    let y = a[1].as_i64().unwrap_or(0);
                    let w = s[0].as_i64().unwrap_or(0);
                    let h = s[1].as_i64().unwrap_or(0);
                    if w > 10 && h > 10 {
                        boxes.push_str(&format!("{},{} {}x{}\n", x, y, w, h));
                    }
                }
            }
            if boxes.is_empty() { None } else { Some(boxes) }
        }
        Compositor::Sway => {
            let out = Command::new("swaymsg")
                .args(["-t", "get_tree"])
                .output()
                .ok()?;
            let tree: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            let mut boxes = String::new();
            collect_sway_boxes(&tree, &mut boxes);
            if boxes.is_empty() { None } else { Some(boxes) }
        }
        Compositor::Niri => {
            let out = Command::new("niri")
                .args(["msg", "--json", "windows"])
                .output()
                .ok()?;
            let windows: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
            let arr = windows.as_array()?;
            let mut boxes = String::new();
            for win in arr {
                if let Some(rect) = win.get("rect").or_else(|| win.get("layout_tile")) {
                    let x = rect.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
                    let y = rect.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
                    let w = rect.get("width").and_then(|v| v.as_i64()).unwrap_or(0);
                    let h = rect.get("height").and_then(|v| v.as_i64()).unwrap_or(0);
                    if w > 10 && h > 10 {
                        boxes.push_str(&format!("{},{} {}x{}\n", x, y, w, h));
                    }
                }
            }
            if boxes.is_empty() { None } else { Some(boxes) }
        }
        Compositor::Other => None,
    }
}

#[allow(dead_code)]
fn find_sway_focused_rect(node: &serde_json::Value) -> Option<String> {
    if node.get("focused").and_then(|f| f.as_bool()) == Some(true) {
        let rect = node.get("rect")?;
        let x = rect.get("x")?.as_i64()?;
        let y = rect.get("y")?.as_i64()?;
        let w = rect.get("width")?.as_i64()?;
        let h = rect.get("height")?.as_i64()?;
        if w > 10 && h > 10 {
            return Some(format!("{},{} {}x{}", x, y, w, h));
        }
    }
    if let Some(nodes) = node.get("nodes").and_then(|n| n.as_array()) {
        for child in nodes {
            if let Some(r) = find_sway_focused_rect(child) {
                return Some(r);
            }
        }
    }
    if let Some(floating) = node.get("floating_nodes").and_then(|n| n.as_array()) {
        for child in floating {
            if let Some(r) = find_sway_focused_rect(child) {
                return Some(r);
            }
        }
    }
    None
}

fn collect_sway_boxes(node: &serde_json::Value, boxes: &mut String) {
    if let Some(rect) = node.get("rect")
        && (node.get("app_id").is_some() || node.get("window").is_some())
    {
        let x = rect.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
        let y = rect.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
        let w = rect.get("width").and_then(|v| v.as_i64()).unwrap_or(0);
        let h = rect.get("height").and_then(|v| v.as_i64()).unwrap_or(0);
        if w > 10 && h > 10 {
            boxes.push_str(&format!("{},{} {}x{}\n", x, y, w, h));
        }
    }
    if let Some(nodes) = node.get("nodes").and_then(|n| n.as_array()) {
        for child in nodes {
            collect_sway_boxes(child, boxes);
        }
    }
    if let Some(floating) = node.get("floating_nodes").and_then(|n| n.as_array()) {
        for child in floating {
            collect_sway_boxes(child, boxes);
        }
    }
}
