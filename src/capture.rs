use crate::compositor;
use crate::config::Config;
use crate::image;
use crate::runtime;
use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    AllScreens,
    FullScreen,
    ActiveWindow,
    Area,
    TextOcr,
    RecordScreen,
    RecordArea,
}

pub struct ScreenFreeze {
    child: Option<std::process::Child>,
}

impl ScreenFreeze {
    pub fn new() -> Option<Self> {
        let child = Command::new("hyprpicker")
            .args(["-r", "-z"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        thread::sleep(Duration::from_millis(150));
        Some(Self { child: Some(child) })
    }
}

impl Drop for ScreenFreeze {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

pub fn execute_capture(mode: CaptureMode, freeze: bool, config: &Config) {
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
        CaptureMode::AllScreens => capture_all_screens(config),
        CaptureMode::FullScreen => capture_fullscreen(config),
        CaptureMode::ActiveWindow => capture_active_window(freeze, config),
        CaptureMode::Area => capture_area(freeze, config),
        CaptureMode::TextOcr => capture_ocr(config),
        CaptureMode::RecordScreen => toggle_screen_recording(false, config),
        CaptureMode::RecordArea => toggle_screen_recording(true, config),
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

fn capture_all_screens(config: &Config) {
    let mut args = Vec::new();
    if config.show_cursor {
        args.push("-c".to_string());
    }
    // No -o and no -g => grim captures all connected monitors
    let apply_decorations = config.window_shadow || config.macos_titlebar;
    run_grim_pipeline(&args, config, apply_decorations, config.macos_titlebar);
}

fn capture_fullscreen(config: &Config) {
    let mut args = Vec::new();
    if config.show_cursor {
        args.push("-c".to_string());
    }

    if let Some(name) = compositor::focused_output() {
        args.push("-o".to_string());
        args.push(name);
    }

    let apply_decorations = config.window_shadow || config.macos_titlebar;
    run_grim_pipeline(&args, config, apply_decorations, config.macos_titlebar);
}

fn capture_active_window(freeze: bool, config: &Config) {
    let active_geom = compositor::active_window_geometry();
    if let Some(g) = active_geom {
        let mut args = vec!["-g".to_string(), g];
        if config.show_cursor {
            args.push("-c".to_string());
        }
        run_grim_pipeline(&args, config, config.window_shadow, config.macos_titlebar);
    } else {
        capture_area(freeze, config);
    }
}

fn capture_area(freeze: bool, config: &Config) {
    let _freeze_guard = if freeze || config.freeze {
        ScreenFreeze::new()
    } else {
        None
    };

    let boxes = compositor::window_boxes();
    let mut slurp_cmd = Command::new("slurp");
    slurp_cmd.args([
        "-d",
        "-b",
        "#00000088",
        "-c",
        "#7AA4C2",
        "-s",
        "#7AA4C222",
        "-w",
        "2",
    ]);

    let slurp_out = if let Some(ref b) = boxes {
        slurp_cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut c| {
                if let Some(mut stdin) = c.stdin.take() {
                    let _ = stdin.write_all(b.as_bytes());
                }
                c.wait_with_output()
            })
            .ok()
    } else {
        slurp_cmd.output().ok()
    };

    drop(_freeze_guard);

    if let Some(output) = slurp_out
        && output.status.success()
    {
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
}

fn capture_ocr(config: &Config) {
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
        let lang = config.ocr_lang.as_deref().unwrap_or("eng");

        if let Ok(grim_res) = grim_out
            && grim_res.status.success()
            && let Ok(mut tess) = Command::new("tesseract")
                .args(["stdin", "stdout", "-l", lang])
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

fn resolve_preferred_editor(config: &Config) -> String {
    if let Some(ref ed) = config.editor {
        let trimmed = ed.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if Command::new("which")
        .arg("satty")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        "satty".to_string()
    } else {
        "swappy".to_string()
    }
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
        image::apply_macos_decorations(&raw_output, add_titlebar, config.canvas_theme)
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

    let runtime_dir = runtime::get_runtime_dir();
    let preview_path = runtime_dir.join("latest.png");
    let preview_str = preview_path.to_string_lossy().to_string();
    let _ = fs::write(&preview_path, &output);

    let (saved_file_arg, body_text) = match (&save_path, config.copy_to_clipboard) {
        (Some(p), true) => (
            p.to_string_lossy().to_string(),
            "Copied to clipboard & saved",
        ),
        (Some(p), false) => (p.to_string_lossy().to_string(), "Saved to Screenshots"),
        (None, true) => (String::new(), "Copied to clipboard"),
        (None, false) => (String::new(), "Capture complete"),
    };

    let editor_bin = resolve_preferred_editor(config);

    if config.open_in_editor {
        let editor_target = if !saved_file_arg.is_empty() {
            &saved_file_arg
        } else {
            &preview_str
        };
        let _ = Command::new(&editor_bin)
            .args(["-f", editor_target])
            .spawn();
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
    if command -v "$4" >/dev/null 2>&1; then
        "$4" -f "$target"
    elif command -v satty >/dev/null 2>&1; then
        satty -f "$target"
    elif command -v swappy >/dev/null 2>&1; then
        swappy -f "$target"
    fi
elif [ "$action" = "delete" ] && [ -n "$2" ]; then
    rm -f -- "$2"
    notify-send -a "shotdock" -t 2000 "Screenshot Deleted" "File removed."
fi
"#,
            "shotdock-action",
            &preview_str,
            &saved_file_arg,
            body_text,
            &editor_bin,
        ])
        .spawn();
}

fn toggle_screen_recording(is_area: bool, config: &Config) {
    let runtime_dir = runtime::get_runtime_dir();
    let pid_file = runtime_dir.join("record.pid");
    let file_track = runtime_dir.join("record_file.txt");

    if pid_file.exists() {
        if let Ok(pid_str) = fs::read_to_string(&pid_file)
            && let Ok(pid) = pid_str.trim().parse::<u32>()
            && runtime::is_process_running_with_comm(pid, "wf-recorder")
        {
            runtime::send_signal(pid, runtime::SIGINT);
        }
        let _ = fs::remove_file(&pid_file);

        let target_file = fs::read_to_string(&file_track).unwrap_or_default();
        let _ = fs::remove_file(&file_track);

        let target_clean = target_file.trim().to_string();
        let _ = Command::new("sh")
            .args([
                "-c",
                r#"
if [ -n "$1" ] && [ -f "$1" ]; then
    action=$(notify-send -a "shotdock" -A "open=Open Video" "Recording Stopped" "Saved to Videos/Recordings")
    if [ "$action" = "open" ]; then
        xdg-open -- "$1"
    fi
else
    notify-send -a "shotdock" "Recording Stopped" "Saved to Videos/Recordings"
fi
"#,
                "shotdock-record-action",
                &target_clean,
            ])
            .spawn();
        return;
    }

    if runtime::is_user_process_running("wf-recorder") {
        runtime::pkill_user_process("wf-recorder", "-INT");
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

        if config.studio_quality {
            let fps = config.record_fps.max(30).to_string();
            cmd.args([
                "-c",
                "libx264",
                "-p",
                "crf=18",
                "-p",
                "preset=veryfast",
                "-p",
                "pix_fmt=yuv420p",
                "-r",
                &fps,
            ]);
        }

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
            let _ = fs::write(&pid_file, pid.to_string());
            let _ = fs::write(&file_track, &save_file);

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

pub fn frame_existing_image(
    input_path: &std::path::Path,
    output_path: Option<&std::path::Path>,
    theme_override: Option<crate::config::CanvasTheme>,
    shadow: bool,
    titlebar: bool,
    copy_clipboard: bool,
    config: &Config,
) -> Result<PathBuf, String> {
    if !input_path.exists() {
        return Err(format!("Input file not found: {:?}", input_path));
    }

    let raw_bytes = fs::read(input_path).map_err(|e| e.to_string())?;
    let png_bytes = if image::get_png_dimensions(&raw_bytes).is_some() {
        raw_bytes
    } else {
        let mut cmd = Command::new("magick");
        cmd.args(["-", "png32:-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let mut child = cmd.spawn().map_err(|e| e.to_string())?;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(&raw_bytes);
        }
        let out = child.wait_with_output().map_err(|e| e.to_string())?;
        if !out.status.success() || out.stdout.is_empty() {
            return Err("Failed to convert image to PNG".to_string());
        }
        out.stdout
    };

    let theme = theme_override.unwrap_or(config.canvas_theme);
    let framed = if shadow || titlebar || theme != crate::config::CanvasTheme::Transparent {
        image::apply_macos_decorations(&png_bytes, titlebar, theme)
    } else {
        png_bytes
    };

    let target_path = if let Some(p) = output_path {
        p.to_path_buf()
    } else {
        let stem = input_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");
        let dir = shellexpand(&config.save_dir);
        let _ = fs::create_dir_all(&dir);
        dir.join(format!("{}_framed.png", stem))
    };

    fs::write(&target_path, &framed).map_err(|e| e.to_string())?;

    if copy_clipboard
        && let Ok(mut child) = Command::new("wl-copy")
            .args(["-t", "image/png"])
            .stdin(Stdio::piped())
            .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(&framed);
        }
        let _ = child.wait();
    }

    let target_str = target_path.to_string_lossy().to_string();
    let action_script = format!(
        r#"action=$(notify-send -a "shotdock" -i "{target}" -h "string:image-path:{target}" -A "open=Open" "Image Framed" "Saved to {target}")
if [ "$action" = "open" ]; then
    xdg-open "{target}"
fi"#,
        target = target_str
    );
    let _ = Command::new("sh").args(["-c", &action_script]).spawn();

    Ok(target_path)
}
