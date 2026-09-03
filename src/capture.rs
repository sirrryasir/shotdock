use crate::config::{CanvasTheme, Config};
use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    FullScreen,
    ActiveWindow,
    Area,
    TextOcr,
    RecordScreen,
    RecordArea,
}

pub fn execute_capture(mode: CaptureMode, config: &Config) {
    // Countdown timer if configured
    if config.timer_seconds > 0 {
        let _ = Command::new("notify-send")
            .args([
                "-a",
                "shotdock",
                "-t",
                "1000",
                &format!("Taking screenshot in {}s...", config.timer_seconds),
            ])
            .spawn();
        thread::sleep(Duration::from_secs(config.timer_seconds as u64));
    }

    match mode {
        CaptureMode::FullScreen => capture_fullscreen(config),
        CaptureMode::ActiveWindow => capture_active_window(config),
        CaptureMode::Area => capture_area(config),
        CaptureMode::TextOcr => capture_ocr(config),
        CaptureMode::RecordScreen => toggle_screen_recording(false),
        CaptureMode::RecordArea => toggle_screen_recording(true),
    }
}

fn get_save_path(config: &Config) -> PathBuf {
    let now = Local::now();
    let filename = now.format("%Y%m%d_%Hh%Mm%Ss_screenshot.png").to_string();
    let dir = shellexpand(&config.save_dir);
    let _ = fs::create_dir_all(&dir);
    dir.join(filename)
}

fn shellexpand(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/")
        && let Ok(home) = std::env::var("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    PathBuf::from(path)
}

fn capture_fullscreen(config: &Config) {
    let mut args = Vec::new();
    if config.show_cursor {
        args.push("-c".to_string());
    }
    // Check focused monitor
    if let Ok(output) = Command::new("hyprctl").args(["monitors", "-j"]).output()
        && let Ok(monitors) = serde_json::from_slice::<serde_json::Value>(&output.stdout)
        && let Some(focused) = monitors.as_array().and_then(|arr| {
            arr.iter()
                .find(|m| m.get("focused").and_then(|f| f.as_bool()) == Some(true))
        })
        && let Some(name) = focused.get("name").and_then(|n| n.as_str())
    {
        args.push("-o".to_string());
        args.push(name.to_string());
    }

    let apply_decorations = config.window_shadow || config.macos_titlebar;
    run_grim_pipeline(&args, config, apply_decorations, config.macos_titlebar);
}

fn get_window_boxes() -> Option<String> {
    let output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .ok()?;
    let clients: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
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
        let at = win.get("at").and_then(|v| v.as_array());
        let size = win.get("size").and_then(|v| v.as_array());
        if let (Some(a), Some(s)) = (at, size)
            && a.len() == 2
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

fn get_active_window_geom() -> Option<String> {
    let output = Command::new("hyprctl")
        .args(["activewindow", "-j"])
        .output()
        .ok()?;
    let win: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let at = win.get("at").and_then(|v| v.as_array());
    let size = win.get("size").and_then(|v| v.as_array());
    if let (Some(a), Some(s)) = (at, size)
        && a.len() == 2
        && s.len() == 2
    {
        let x = a[0].as_i64().unwrap_or(0);
        let y = a[1].as_i64().unwrap_or(0);
        let w = s[0].as_i64().unwrap_or(0);
        let h = s[1].as_i64().unwrap_or(0);
        Some(format!("{},{} {}x{}", x, y, w, h))
    } else {
        None
    }
}

fn capture_active_window(config: &Config) {
    let active_geom = get_active_window_geom();

    let geom = if let Some(boxes) = get_window_boxes() {
        let child = Command::new("slurp")
            .args([
                "-d",
                "-b",
                "#00000088",
                "-c",
                "#7AA4C2",
                "-s",
                "#7AA4C222",
                "-w",
                "2",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .ok();

        if let Some(mut c) = child {
            if let Some(mut stdin) = c.stdin.take() {
                let _ = stdin.write_all(boxes.as_bytes());
            }
            if let Ok(out) = c.wait_with_output()
                && out.status.success()
            {
                let g = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if g.is_empty() { active_geom } else { Some(g) }
            } else {
                active_geom
            }
        } else {
            active_geom
        }
    } else {
        active_geom
    };

    if let Some(g) = geom {
        let mut args = vec!["-g".to_string(), g];
        if config.show_cursor {
            args.push("-c".to_string());
        }
        run_grim_pipeline(&args, config, config.window_shadow, config.macos_titlebar);
    } else {
        capture_area(config);
    }
}

fn capture_area(config: &Config) {
    // Run slurp to get geometry
    let slurp_out = Command::new("slurp")
        .args(["-d", "-b", "#00000088", "-c", "#00aaff", "-w", "2"])
        .output();

    match slurp_out {
        Ok(output) if output.status.success() => {
            let geom = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if geom.is_empty() {
                return;
            }
            let mut args = vec!["-g".to_string(), geom];
            if config.show_cursor {
                args.push("-c".to_string());
            }
            run_grim_pipeline(&args, config, config.window_shadow, config.macos_titlebar);
        }
        _ => {}
    }
}

fn capture_ocr(_config: &Config) {
    let slurp_out = Command::new("slurp")
        .args([
            "-d",
            "-b",
            "#00000088",
            "-c",
            "#7AA4C2",
            "-s",
            "#7AA4C222",
            "-w",
            "2",
        ])
        .output();

    if let Ok(output) = slurp_out
        && output.status.success()
    {
        let geom = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if geom.is_empty() {
            return;
        }

        let grim_out = Command::new("grim").args(["-g", &geom, "-"]).output();

        if let Ok(grim_res) = grim_out
            && grim_res.status.success()
            && let Ok(mut tess) = Command::new("tesseract")
                .args(["stdin", "stdout", "-l", "eng"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
        {
            if let Some(mut stdin) = tess.stdin.take() {
                let _ = stdin.write_all(&grim_res.stdout);
            }
            if let Ok(tess_out) = tess.wait_with_output() {
                let text = String::from_utf8_lossy(&tess_out.stdout).trim().to_string();
                if !text.is_empty() {
                    if let Ok(mut child) = Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
                        if let Some(mut stdin) = child.stdin.take() {
                            let _ = stdin.write_all(text.as_bytes());
                        }
                        let _ = child.wait();
                    }

                    let preview = if text.chars().count() > 80 {
                        format!("{}...", text.chars().take(80).collect::<String>())
                    } else {
                        text
                    };

                    let _ = Command::new("notify-send")
                        .args([
                            "-a",
                            "shotdock",
                            "-t",
                            "3000",
                            "Text Extracted to Clipboard",
                            &preview,
                        ])
                        .spawn();
                    return;
                }
            }
        }
    }

    let _ = Command::new("notify-send")
        .args([
            "-a",
            "shotdock",
            "-t",
            "2000",
            "OCR Notice",
            "No readable text found in selected area.",
        ])
        .spawn();
}

fn get_png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 24 || &data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let w = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let h = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Some((w, h))
}

fn get_wallbash_gradient() -> (String, String) {
    let home = std::env::var("HOME").unwrap_or_default();
    let dcol_path = format!("{}/.cache/dotfiles/wall.dcol", home);
    if let Ok(content) = fs::read_to_string(dcol_path) {
        let mut pry1 = String::new();
        let mut a6 = String::new();
        for line in content.lines() {
            let line = line.trim();
            if let Some((k, v)) = line.split_once('=') {
                let val = v.trim_matches('"').trim().to_string();
                if k == "dcol_pry1" {
                    pry1 = val;
                } else if k == "dcol_1xa6" {
                    a6 = val;
                }
            }
        }
        if !pry1.is_empty() && !a6.is_empty() {
            return (pry1, a6);
        }
    }
    ("313B42".to_string(), "7AA4C2".to_string())
}

fn apply_macos_decorations(input: &[u8], add_titlebar: bool, theme: CanvasTheme) -> Vec<u8> {
    let (w, mut h) = match get_png_dimensions(input) {
        Some(dim) => dim,
        None => return input.to_vec(),
    };

    let mut cmd = Command::new("magick");
    cmd.arg("-");

    if add_titlebar {
        h += 34;
        cmd.args([
            "-background",
            "#21252b",
            "-splice",
            "0x34",
            "-fill",
            "#ff5f56",
            "-draw",
            "circle 16,17 22,17",
            "-fill",
            "#ffbd2e",
            "-draw",
            "circle 36,17 42,17",
            "-fill",
            "#27c93f",
            "-draw",
            "circle 56,17 62,17",
        ]);
    }

    let rect_draw = format!(
        "roundrectangle 0,0 {},{} 16,16",
        w.saturating_sub(1),
        h.saturating_sub(1)
    );

    cmd.args([
        "-alpha",
        "set",
        "(",
        "+clone",
        "-alpha",
        "transparent",
        "-background",
        "none",
        "-fill",
        "white",
        "-draw",
        &rect_draw,
        ")",
        "-compose",
        "DstIn",
        "-composite",
        "(",
        "+clone",
        "-background",
        "rgba(0,0,0,0.45)",
        "-shadow",
        "60x18+0+12",
        ")",
        "+swap",
        "-background",
        "none",
        "-compose",
        "Over",
        "-layers",
        "merge",
        "+repage",
    ]);

    let cw = w + 120;
    let ch = h + 130;
    let size_arg = format!("{}x{}", cw, ch);

    match theme {
        CanvasTheme::Transparent => {
            cmd.arg("png32:-");
        }
        CanvasTheme::FollowSystem => {
            let (c1, c2) = get_wallbash_gradient();
            let grad = format!("gradient:#{}-#{}", c1, c2);
            cmd.args([
                "(",
                "-size",
                &size_arg,
                &grad,
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::RealWallpaper => {
            let home = std::env::var("HOME").unwrap_or_default();
            let blur_path = format!("{}/.cache/dotfiles/wall.blur", home);
            let set_path = format!("{}/.cache/dotfiles/wall.set", home);
            let wall_path = if std::path::Path::new(&blur_path).exists() {
                blur_path
            } else {
                set_path
            };

            if std::path::Path::new(&wall_path).exists() {
                cmd.args([
                    "(",
                    &wall_path,
                    "-resize",
                    &format!("{}^", size_arg),
                    "-gravity",
                    "center",
                    "-extent",
                    &size_arg,
                    ")",
                    "+swap",
                    "-gravity",
                    "center",
                    "-composite",
                    "png32:-",
                ]);
            } else {
                cmd.arg("png32:-");
            }
        }
        CanvasTheme::White => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "xc:#ffffff",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::Black => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "xc:#18181b",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::Sunset => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "gradient:#f43f5e-#8b5cf6",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::Candy => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "gradient:#ec4899-#a855f7",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::Breeze => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "gradient:#06b6d4-#3b82f6",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::Raindrop => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "gradient:#3b82f6-#6366f1",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::Midnight => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "gradient:#1e1b4b-#0f172a",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
        CanvasTheme::Forest => {
            cmd.args([
                "(",
                "-size",
                &size_arg,
                "gradient:#059669-#10b981",
                ")",
                "+swap",
                "-gravity",
                "center",
                "-composite",
                "png32:-",
            ]);
        }
    }

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    if let Ok(mut child) = cmd.spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input);
        }
        if let Ok(out) = child.wait_with_output()
            && out.status.success()
            && !out.stdout.is_empty()
        {
            return out.stdout;
        }
    }
    input.to_vec()
}

fn run_grim_pipeline(
    grim_args: &[String],
    config: &Config,
    apply_shadow: bool,
    add_titlebar: bool,
) {
    let mut cmd = Command::new("grim");
    for arg in grim_args {
        cmd.arg(arg);
    }
    // Output png data to stdout
    cmd.arg("-");

    let raw_output = match cmd.output() {
        Ok(out) if out.status.success() => out.stdout,
        _ => {
            let _ = Command::new("notify-send")
                .args([
                    "-a",
                    "shotdock",
                    "Capture Failed",
                    "Could not take screenshot.",
                ])
                .spawn();
            return;
        }
    };

    let output = if apply_shadow {
        apply_macos_decorations(&raw_output, add_titlebar, config.canvas_theme)
    } else {
        raw_output
    };

    let save_path = if config.save_to_disk {
        let path = get_save_path(config);
        if let Ok(mut file) = fs::File::create(&path) {
            let _ = file.write_all(&output);
            Some(path)
        } else {
            None
        }
    } else {
        None
    };

    if config.copy_to_clipboard
        && let Ok(mut child) = Command::new("wl-copy")
            .args(["--type", "image/png"])
            .stdin(Stdio::piped())
            .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(&output);
        }
        let _ = child.wait();
    }

    let preview_path = "/tmp/shotdock_latest.png";
    let _ = fs::write(preview_path, &output);

    let (saved_file_arg, body_text) = match (&save_path, config.copy_to_clipboard) {
        (Some(p), true) => (
            p.to_string_lossy().to_string(),
            "Copied to clipboard & saved",
        ),
        (Some(p), false) => (p.to_string_lossy().to_string(), "Saved to Screenshots"),
        (None, true) => (String::new(), "Copied to clipboard"),
        (None, false) => (String::new(), "Capture complete"),
    };

    if config.open_in_editor {
        let editor_target = if !saved_file_arg.is_empty() {
            &saved_file_arg
        } else {
            preview_path
        };
        let _ = Command::new("swappy").args(["-f", editor_target]).spawn();
    }

    let _ = Command::new("sh")
        .args([
            "-c",
            r#"
if [ -n "$2" ]; then
    action=$(notify-send -a "shotdock" -i "$1" -h "string:image-path:$1" -A "annotate=Annotate" -A "delete=Delete" "Screenshot Captured" "$3")
else
    action=$(notify-send -a "shotdock" -i "$1" -h "string:image-path:$1" -A "annotate=Annotate" "Screenshot Captured" "$3")
fi

if [ "$action" = "annotate" ]; then
    target="${2:-$1}"
    swappy -f "$target"
elif [ "$action" = "delete" ] && [ -n "$2" ]; then
    rm -f "$2"
    notify-send -a "shotdock" -t 2000 "Screenshot Deleted" "File removed."
fi
"#,
            "shotdock-action",
            preview_path,
            &saved_file_arg,
            body_text,
        ])
        .spawn();
}

fn toggle_screen_recording(is_area: bool) {
    let pid_file = "/tmp/shotdock_record.pid";
    let file_track = "/tmp/shotdock_record_file.txt";

    if std::path::Path::new(pid_file).exists() {
        if let Ok(pid_str) = fs::read_to_string(pid_file) {
            let pid = pid_str.trim();
            if !pid.is_empty() {
                let _ = Command::new("kill").args(["-INT", pid]).status();
            }
        }
        let _ = fs::remove_file(pid_file);

        let target_file = fs::read_to_string(file_track).unwrap_or_default();
        let _ = fs::remove_file(file_track);

        let _ = Command::new("sh")
            .args([
                "-c",
                r#"
if [ -n "$1" ] && [ -f "$1" ]; then
    action=$(notify-send -a "shotdock" -A "open=Open Video" "Recording Stopped" "Saved to Videos/Recordings")
    if [ "$action" = "open" ]; then
        xdg-open "$1"
    fi
else
    notify-send -a "shotdock" "Recording Stopped" "Saved to Videos/Recordings"
fi
"#,
                "shotdock-record-action",
                target_file.trim(),
            ])
            .spawn();
        return;
    }

    // Check if wf-recorder is running outside
    let check = Command::new("pgrep").args(["-x", "wf-recorder"]).output();
    if let Ok(res) = check
        && res.status.success()
    {
        let _ = Command::new("pkill")
            .args(["-INT", "-x", "wf-recorder"])
            .status();
        let _ = Command::new("notify-send")
            .args([
                "-a",
                "shotdock",
                "Recording Stopped",
                "Saved to Videos/Recordings",
            ])
            .spawn();
        return;
    }

    let check_rec = Command::new("which").arg("wf-recorder").output();
    if check_rec.map(|o| o.status.success()).unwrap_or(false) {
        let now = Local::now();
        let filename = now.format("%Y%m%d_%Hh%Mm%Ss_recording.mp4").to_string();
        let videos_dir = std::env::var("HOME").unwrap_or_default() + "/Videos/Recordings";
        let _ = fs::create_dir_all(&videos_dir);
        let save_file = format!("{}/{}", videos_dir, filename);

        let mut cmd = Command::new("wf-recorder");
        cmd.args(["-f", &save_file]);

        if is_area {
            if let Ok(slurp_out) = Command::new("slurp").output() {
                let geom = String::from_utf8_lossy(&slurp_out.stdout)
                    .trim()
                    .to_string();
                if geom.is_empty() {
                    return;
                }
                cmd.args(["-g", &geom]);
            } else {
                return;
            }
        }

        if let Ok(child) = cmd.spawn() {
            let pid = child.id();
            let _ = fs::write(pid_file, pid.to_string());
            let _ = fs::write(file_track, &save_file);

            let _ = Command::new("notify-send")
                .args([
                    "-a",
                    "shotdock",
                    "Recording Started",
                    "Press shotdock Record again to stop.",
                ])
                .spawn();
        }
    } else {
        let _ = Command::new("notify-send")
            .args([
                "-a",
                "shotdock",
                "Recorder Not Installed",
                "Install wf-recorder with: sudo pacman -S wf-recorder",
            ])
            .spawn();
    }
}
