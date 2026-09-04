mod capture;
mod compositor;
mod config;
mod image;
mod runtime;
mod theme;
mod ui;

use capture::{CaptureMode, execute_capture, frame_existing_image};
use config::{CanvasTheme, Config};
use gtk4::Application;
use gtk4::prelude::*;
use std::env;
use std::path::Path;

fn toggle_if_running() -> bool {
    let pid_file = runtime::get_dock_pid_file();
    if let Ok(content) = std::fs::read_to_string(&pid_file)
        && let Ok(pid) = content.trim().parse::<u32>()
    {
        let current_pid = std::process::id();
        if pid != current_pid && runtime::is_process_running_with_comm(pid, "shotdock") {
            runtime::send_signal(pid, runtime::SIGTERM);
            runtime::cleanup_dock_pid();
            return true;
        }
    }
    runtime::cleanup_dock_pid();
    false
}

fn print_help() {
    println!("shotdock - Automated window framing & studio screen capture for Wayland");
    println!();
    println!("Usage:");
    println!("  shotdock [OPTIONS]");
    println!("  shotdock frame <FILE> [OPTIONS]");
    println!();
    println!("Capture Options:");
    println!("  (none)            Launch floating macOS toolbar (toggles if running)");
    println!("  -a, --area        Capture selected area or click window");
    println!("  -w, --window      Capture active window");
    println!("  -f, --full        Capture focused monitor");
    println!("  -p, --all         Capture all connected monitors");
    println!("  -z, --freeze      Freeze screen during area selection");
    println!("  -e, --edit        Open immediately in annotation editor (swappy/satty)");
    println!("  --no-edit         Do not open in editor");
    println!("  -t, --text        Extract text from selected area (OCR)");
    println!("  -r, --record      Toggle screen recording (60 FPS)");
    println!("  --record-area     Toggle area screen recording");
    println!("  -h, --help        Show this help message");
    println!();
    println!("Frame Subcommand:");
    println!("  shotdock frame <FILE> [-o OUTPUT] [--theme THEME] [--no-shadow] [--no-titlebar] [-c]");
    println!("  Apply macOS titlebar, Gaussian shadows, and canvas presets to an existing image.");
}

fn handle_frame_command(args: &[String], config: &Config) {
    if args.len() < 3 {
        eprintln!("Error: 'shotdock frame' requires an image file path.");
        eprintln!("Usage: shotdock frame <FILE> [-o OUTPUT] [--theme THEME] [--no-shadow] [--no-titlebar] [-c]");
        std::process::exit(1);
    }

    let input_file = &args[2];
    let mut output_file: Option<&Path> = None;
    let mut theme_override: Option<CanvasTheme> = None;
    let mut shadow = config.window_shadow;
    let mut titlebar = config.macos_titlebar;
    let mut copy_clipboard = config.copy_to_clipboard;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(Path::new(&args[i + 1]));
                    i += 1;
                }
            }
            "--theme" => {
                if i + 1 < args.len() {
                    if let Some(t) = CanvasTheme::from_str_loose(&args[i + 1]) {
                        theme_override = Some(t);
                    } else {
                        eprintln!("Warning: Unknown canvas theme '{}'. Using default.", args[i + 1]);
                    }
                    i += 1;
                }
            }
            "--no-shadow" => shadow = false,
            "--no-titlebar" => titlebar = false,
            "-c" | "--clipboard" => copy_clipboard = true,
            _ => {}
        }
        i += 1;
    }

    match frame_existing_image(
        Path::new(input_file),
        output_file,
        theme_override,
        shadow,
        titlebar,
        copy_clipboard,
        config,
    ) {
        Ok(path) => {
            println!("Framed image saved to: {}", path.display());
        }
        Err(e) => {
            eprintln!("Framing failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    if env::var("GSK_RENDERER").is_err() {
        unsafe {
            env::set_var("GSK_RENDERER", "gl");
        }
    }

    let args: Vec<String> = env::args().collect();
    let mut config = Config::load();

    if args.len() == 1 && toggle_if_running() {
        return;
    }

    if args.len() > 1 {
        let has_flag = |short: &str, long: &str| args.iter().any(|a| a == short || a == long);
        let freeze = has_flag("-z", "--freeze");

        if has_flag("-e", "--edit") {
            config.open_in_editor = true;
        } else if has_flag("--no-edit", "--no-editor") {
            config.open_in_editor = false;
        }

        if args[1] == "frame" {
            handle_frame_command(&args, &config);
            return;
        }

        if has_flag("-h", "--help") {
            print_help();
            return;
        }

        if has_flag("-p", "--all") {
            execute_capture(CaptureMode::AllScreens, freeze, &config);
            return;
        }

        if has_flag("-f", "--full") {
            execute_capture(CaptureMode::FullScreen, freeze, &config);
            return;
        }

        if has_flag("-w", "--window") {
            execute_capture(CaptureMode::ActiveWindow, freeze, &config);
            return;
        }

        if has_flag("-a", "--area") || (freeze && args.len() == 2) {
            execute_capture(CaptureMode::Area, freeze, &config);
            return;
        }

        if has_flag("-t", "--text") {
            execute_capture(CaptureMode::TextOcr, freeze, &config);
            return;
        }

        if has_flag("-r", "--record") {
            execute_capture(CaptureMode::RecordScreen, freeze, &config);
            return;
        }

        if args.iter().any(|a| a == "--record-area") {
            execute_capture(CaptureMode::RecordArea, freeze, &config);
            return;
        }
    }

    // Register active PID in secure runtime directory
    runtime::write_dock_pid();

    // Launch native floating GTK4 LayerShell toolbar
    let app = Application::builder()
        .application_id("org.yasir.shotdock")
        .build();

    app.connect_activate(ui::build_ui);
    app.run_with_args::<&str>(&[]);

    // Clean up pid on normal exit
    runtime::cleanup_dock_pid();
}
