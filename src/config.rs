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

impl CanvasTheme {
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().replace(['-', '_', ' '], "").as_str() {
            "transparent" => Some(CanvasTheme::Transparent),
            "followsystem" | "wallbash" | "system" => Some(CanvasTheme::FollowSystem),
            "realwallpaper" | "wallpaper" => Some(CanvasTheme::RealWallpaper),
            "white" => Some(CanvasTheme::White),
            "black" => Some(CanvasTheme::Black),
            "sunset" => Some(CanvasTheme::Sunset),
            "candy" => Some(CanvasTheme::Candy),
            "breeze" => Some(CanvasTheme::Breeze),
            "raindrop" => Some(CanvasTheme::Raindrop),
            "midnight" => Some(CanvasTheme::Midnight),
            "forest" => Some(CanvasTheme::Forest),
            _ => None,
        }
    }
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
    #[serde(default)]
    pub editor: Option<String>,
    #[serde(default)]
    pub ocr_lang: Option<String>,
    #[serde(default = "default_true")]
    pub studio_quality: bool,
    #[serde(default = "default_fps")]
    pub record_fps: u32,
    #[serde(default)]
    pub freeze: bool,
}

fn default_fps() -> u32 {
    60
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
            editor: None,
            ocr_lang: None,
            studio_quality: true,
            record_fps: 60,
            freeze: false,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let conf = Config::default();
        assert!(conf.copy_to_clipboard);
        assert!(conf.save_to_disk);
        assert!(conf.studio_quality);
        assert_eq!(conf.record_fps, 60);
        assert_eq!(conf.canvas_theme, CanvasTheme::Transparent);
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let conf = Config::default();
        let serialized = serde_json::to_string(&conf).expect("serialization failed");
        let deserialized: Config =
            serde_json::from_str(&serialized).expect("deserialization failed");
        assert_eq!(conf.copy_to_clipboard, deserialized.copy_to_clipboard);
        assert_eq!(conf.save_dir, deserialized.save_dir);
    }
}
