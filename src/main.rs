mod capture;
mod config;
mod theme;
mod ui;

use capture::{CaptureMode, execute_capture};
use config::Config;
use gtk4::Application;
use gtk4::prelude::*;
use std::env;

fn toggle_if_running() -> bool {
    let current_pid = std::process::id();
    if let Ok(output) = std::process::Command::new("pgrep")
        .args(["-x", "shotdock"])
        .output()
    {
        let pids = String::from_utf8_lossy(&output.stdout);
        let other_pids: Vec<&str> = pids
            .lines()
            .map(str::trim)
            .filter(|p| !p.is_empty() && *p != current_pid.to_string())
            .collect();
        if !other_pids.is_empty() {
            for pid in other_pids {
                let _ = std::process::Command::new("kill").arg(pid).status();
            }
            return true;
        }
    }
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

    // Otherwise launch the native floating GTK4 LayerShell toolbar
    let app = Application::builder()
        .application_id("org.yasir.shotdock")
        .build();

    app.connect_activate(ui::build_ui);
    app.run_with_args::<&str>(&[]);
}
