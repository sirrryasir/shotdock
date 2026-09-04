mod capture;
mod compositor;
mod config;
mod image;
mod runtime;
mod theme;
mod ui;

use capture::{CaptureMode, execute_capture};
use config::Config;
use gtk4::Application;
use gtk4::prelude::*;
use std::env;

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

fn main() {
    if env::var("GSK_RENDERER").is_err() {
        unsafe {
            env::set_var("GSK_RENDERER", "gl");
        }
    }

    let args: Vec<String> = env::args().collect();
    let config = Config::load();

    if args.len() == 1 && toggle_if_running() {
        return;
    }

    // Direct CLI flags for instant shortcuts
    if args.len() > 1 {
        match args[1].as_str() {
            "--full" | "-f" => {
                execute_capture(CaptureMode::FullScreen, &config);
                return;
            }
            "--area" | "-a" => {
                execute_capture(CaptureMode::Area, &config);
                return;
            }
            "--window" | "-w" => {
                execute_capture(CaptureMode::ActiveWindow, &config);
                return;
            }
            "--record" | "-r" => {
                execute_capture(CaptureMode::RecordScreen, &config);
                return;
            }
            "--record-area" => {
                execute_capture(CaptureMode::RecordArea, &config);
                return;
            }
            "--text" | "-t" => {
                execute_capture(CaptureMode::TextOcr, &config);
                return;
            }
            "--help" | "-h" => {
                println!(
                    "shotdock - macOS-style floating screenshot & recording toolbar for Wayland"
                );
                println!();
                println!("Usage: shotdock [OPTIONS]");
                println!();
                println!("Options:");
                println!("  (none)            Launch floating macOS toolbar");
                println!("  -f, --full        Instantly capture full screen");
                println!("  -a, --area        Instantly capture selected area");
                println!("  -w, --window      Instantly capture active window");
                println!("  -t, --text        Extract text from selected area (OCR)");
                println!("  -r, --record      Toggle screen recording");
                println!("  --record-area     Toggle area screen recording");
                println!("  -h, --help        Show this help message");
                return;
            }
            _ => {}
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
