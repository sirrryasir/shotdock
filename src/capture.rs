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
    let apply_shadow = config.window_shadow;
    let add_titlebar = config.macos_titlebar;
    run_grim_pipeline(&args, config, apply_shadow, add_titlebar);
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

    let apply_shadow = config.window_shadow;
    let add_titlebar = config.macos_titlebar;
    run_grim_pipeline(&args, config, apply_shadow, add_titlebar);
}

fn capture_active_window(freeze: bool, config: &Config) {
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

        // Guard against accidental tiny clicks (e.g. 1x1 pixel)
        if let Some((_, size)) = geom.split_once(' ')
            && let Some((w_s, h_s)) = size.split_once('x')
            && let (Ok(w), Ok(h)) = (w_s.parse::<u32>(), h_s.parse::<u32>())
            && (w < 10 || h < 10)
        {
            return;
        }

        let mut args = vec!["-g".to_string(), geom];
        if config.show_cursor {
            args.push("-c".to_string());
        }

        // Studio framing: user wants macOS titlebar, shadow, and canvas applied according to config!
        let add_titlebar = config.macos_titlebar;
        let apply_shadow = config.window_shadow;

        let mut frame_config = config.clone();
        if std::env::args().any(|a| a == "-w" || a == "--window")
            && !std::env::args().any(|a| a == "-e" || a == "--edit")
        {
            frame_config.open_in_editor = false;
        }

        run_grim_pipeline(&args, &frame_config, apply_shadow, add_titlebar);
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

        // Guard against accidental tiny clicks (e.g. 1x1 pixel)
        if let Some((_, size)) = geom.split_once(' ')
            && let Some((w_s, h_s)) = size.split_once('x')
            && let (Ok(w), Ok(h)) = (w_s.parse::<u32>(), h_s.parse::<u32>())
            && (w < 10 || h < 10)
        {
            return;
        }

        let mut args = vec!["-g".to_string(), geom];
        if config.show_cursor {
            args.push("-c".to_string());
        }

        let apply_shadow = config.window_shadow;
        let add_titlebar = config.macos_titlebar;
        run_grim_pipeline(&args, config, apply_shadow, add_titlebar);
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
    cmd.args(["-l", "1"]); // Fast lossless PNG compression (sub-150ms)
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
        // Launch editor immediately in milliseconds
        let _ = Command::new(&editor_bin)
            .args(["-f", editor_target])
            .spawn();
        return;
    }

    let _ = Command::new("notify-send")
        .args([
            "-a",
            "shotdock",
            "-i",
            &preview_str,
            "-h",
            &format!("string:image-path:{}", preview_str),
            "-t",
            "3000",
            "Screenshot Captured",
            body_text,
        ])
        .spawn();
}

enum RecordTarget {
    Output(String),
    Geometry(String),
}

fn select_recording_target(is_area: bool) -> Option<RecordTarget> {
    if is_area {
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
        let boxes = compositor::window_boxes();
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

        let geom = slurp_out.and_then(|o| {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if !s.is_empty() { Some(s) } else { None }
            } else {
                None
            }
        })?;
        return Some(RecordTarget::Geometry(geom));
    }

    // First priority: Standard Hyprland screen cast picker (hyprland-share-picker)
    if compositor::detect() == compositor::Compositor::Hyprland
        && Command::new("which")
            .arg("hyprland-share-picker")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        && let Ok(picker_out) = Command::new("hyprland-share-picker").output()
        && picker_out.status.success()
    {
        let out_str = String::from_utf8_lossy(&picker_out.stdout);
        for line in out_str.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("[SELECTION]/") {
                if let Some(screen_name) = rest.strip_prefix("screen:") {
                    let name = screen_name.trim();
                    if !name.is_empty() {
                        return Some(RecordTarget::Output(name.to_string()));
                    }
                } else if let Some(addr) = rest.strip_prefix("window:") {
                    if let Some(geom) = compositor::window_geometry_by_address(addr.trim()) {
                        return Some(RecordTarget::Geometry(geom));
                    }
                } else if let Some(region_str) = rest.strip_prefix("region:") {
                    let parts: Vec<&str> = region_str.split_whitespace().collect();
                    if parts.len() >= 5 {
                        let geom = format!("{},{} {}x{}", parts[1], parts[2], parts[3], parts[4]);
                        return Some(RecordTarget::Geometry(geom));
                    }
                }
            }
        }
        // If user closed or cancelled the picker window, cleanly return None
        return None;
    }

    let outputs = compositor::outputs();
    if outputs.len() <= 1 {
        return compositor::focused_output()
            .or_else(|| outputs.first().map(|(n, _)| n.clone()))
            .map(RecordTarget::Output);
    }

    let rofi_check = Command::new("which")
        .arg("rofi")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if rofi_check {
        let mut menu_items = Vec::new();
        for (name, desc) in &outputs {
            if desc.is_empty() {
                menu_items.push(format!("Display: {}", name));
            } else {
                menu_items.push(format!("Display: {} ({})", name, desc));
            }
        }
        menu_items.push("Window or Custom Area".to_string());

        let input_text = menu_items.join("\n");
        let mut rofi_cmd = Command::new("rofi");
        rofi_cmd.args([
            "-dmenu",
            "-p",
            "Record Target",
            "-mesg",
            "Choose display, window, or region to record",
            "-theme-str",
            "window {width: 450px;}",
        ]);
        rofi_cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

        if let Ok(mut child) = rofi_cmd.spawn() {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(input_text.as_bytes());
            }
            if let Ok(out) = child.wait_with_output()
                && out.status.success()
            {
                let selection = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if selection == "Window or Custom Area" {
                    return select_recording_target(true);
                }
                for (name, _) in &outputs {
                    if selection.contains(name) {
                        return Some(RecordTarget::Output(name.clone()));
                    }
                }
            }
        }
        None
    } else {
        compositor::focused_output().map(RecordTarget::Output)
    }
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
            // Wait up to 1.5s for wf-recorder to cleanly flush MP4 headers and exit
            for _ in 0..15 {
                if !runtime::is_process_running_with_comm(pid, "wf-recorder") {
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
        let _ = fs::remove_file(&pid_file);

        let target_file = fs::read_to_string(&file_track).unwrap_or_default();
        let _ = fs::remove_file(&file_track);

        let target_clean = target_file.trim().to_string();

        if !target_clean.is_empty() && std::path::Path::new(&target_clean).exists() {
            // Copy saved video file path to clipboard
            if let Ok(mut child) = Command::new("wl-copy").stdin(Stdio::piped()).spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = stdin.write_all(target_clean.as_bytes());
                }
                let _ = child.wait();
            }

            let fname = std::path::Path::new(&target_clean)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("recording.mp4");

            let _ = Command::new("notify-send")
                .args([
                    "-a",
                    "shotdock",
                    "-i",
                    "video-x-generic",
                    "-t",
                    "3000",
                    "Recording Stopped",
                    &format!("Saved: {}\n(Path copied to clipboard)", fname),
                ])
                .spawn();
        } else {
            let _ = Command::new("notify-send")
                .args([
                    "-a",
                    "shotdock",
                    "-t",
                    "3000",
                    "Recording Stopped",
                    "Saved to Videos/Recordings",
                ])
                .spawn();
        }
        return;
    }

    if runtime::is_user_process_running("wf-recorder") {
        runtime::pkill_user_process("wf-recorder", "-INT");
        thread::sleep(Duration::from_millis(300));
        let _ = Command::new("notify-send")
            .args([
                "-a",
                "shotdock",
                "-t",
                "3000",
                "Recording Stopped",
                "Saved to Videos/Recordings",
            ])
            .spawn();
        return;
    }

    let target = match select_recording_target(is_area) {
        Some(t) => t,
        None => return, // User cancelled target selection (e.g. Escape in rofi)
    };

    let check_rec = Command::new("which").arg("wf-recorder").output();
    if check_rec.map(|o| o.status.success()).unwrap_or(false) {
        let now = Local::now();
        let filename = now.format("%Y%m%d_%Hh%Mm%Ss_recording.mp4").to_string();
        let videos_dir = std::env::var("HOME").unwrap_or_default() + "/Videos/Recordings";
        let _ = fs::create_dir_all(&videos_dir);
        let save_file = format!("{}/{}", videos_dir, filename);

        let mut cmd = Command::new("wf-recorder");
        cmd.arg("-y");
        cmd.args(["-f", &save_file]);
        cmd.stdin(Stdio::null());

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

        match target {
            RecordTarget::Output(out_name) => {
                cmd.args(["-o", &out_name]);
            }
            RecordTarget::Geometry(geom) => {
                cmd.args(["-g", &geom]);
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
                    "-t",
                    "3000",
                    "Recording Started",
                    "Run shotdock -r again to stop.",
                ])
                .spawn();
        }
    } else {
        let _ = Command::new("notify-send")
            .args([
                "-a",
                "shotdock",
                "-t",
                "3000",
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
    let _ = Command::new("notify-send")
        .args([
            "-a",
            "shotdock",
            "-i",
            &target_str,
            "-h",
            &format!("string:image-path:{}", target_str),
            "-t",
            "3000",
            "Image Framed",
            &format!("Saved to {}", target_str),
        ])
        .spawn();

    Ok(target_path)
}
