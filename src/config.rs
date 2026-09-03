use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CanvasTheme {
    #[default]
    Transparent,
    FollowSystem,
    RealWallpaper,
    White,
    Black,
    Sunset,
    Candy,
    Breeze,
    Raindrop,
    Midnight,
    Forest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub show_cursor: bool,
    #[serde(default = "default_true")]
    pub window_shadow: bool,
    #[serde(default = "default_true")]
    pub macos_titlebar: bool,
    #[serde(default)]
    pub canvas_theme: CanvasTheme,
    pub timer_seconds: u32,
    pub save_to_disk: bool,
    pub copy_to_clipboard: bool,
    pub open_in_editor: bool,
    pub save_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        let pictures_dir = std::env::var("XDG_PICTURES_DIR")
            .unwrap_or_else(|_| format!("{}/Pictures", std::env::var("HOME").unwrap_or_default()));
        let save_dir = format!("{}/Screenshots", pictures_dir);

        Self {
            show_cursor: false,
            window_shadow: true,
            macos_titlebar: true,
            canvas_theme: CanvasTheme::Transparent,
            timer_seconds: 0,
            save_to_disk: true,
            copy_to_clipboard: true,
            open_in_editor: false,
            save_dir,
        }
    }
}

impl Config {
    fn config_path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        Some(PathBuf::from(home).join(".config/shotdock/config.json"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path()
            && let Ok(content) = fs::read_to_string(path)
            && let Ok(config) = serde_json::from_str(&content)
        {
            return config;
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }
}
