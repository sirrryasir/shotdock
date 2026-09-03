use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct ThemeColors {
    pub bg_css: String,
    pub border_css: String,
    pub accent_hex: String,
    pub accent_hover: String,
    pub active_mode_bg: String,
    pub text_color: String,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_css: "rgba(30, 34, 42, 0.85)".to_string(),
            border_css: "rgba(255, 255, 255, 0.16)".to_string(),
            accent_hex: "#007aff".to_string(),
            accent_hover: "#0069d9".to_string(),
            active_mode_bg: "rgba(255, 255, 255, 0.22)".to_string(),
            text_color: "#ffffff".to_string(),
        }
    }
}

impl ThemeColors {
    pub fn load() -> Self {
        let home = match std::env::var("HOME") {
            Ok(h) => h,
            Err(_) => return Self::default(),
        };

        let dcol_path = PathBuf::from(home).join(".cache/dotfiles/wall.dcol");
        if !dcol_path.exists() {
            return Self::default();
        }

        let content = match fs::read_to_string(dcol_path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };

        let mut vars = HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if let Some((k, v)) = line.split_once('=') {
                let val = v.trim_matches('"').trim();
                vars.insert(k.trim().to_string(), val.to_string());
            }
        }

        // Primary dark background from wallpaper
        let bg_hex = vars
            .get("dcol_pry1")
            .cloned()
            .unwrap_or_else(|| "1e222a".to_string());
        let (bg_r, bg_g, bg_b) = parse_hex(&bg_hex).unwrap_or((30, 34, 42));

        // Primary vibrant accent from wallpaper
        let accent_hex = vars
            .get("dcol_1xa6")
            .or_else(|| vars.get("dcol_pry4"))
            .cloned()
            .unwrap_or_else(|| "007aff".to_string());

        let (acc_r, acc_g, acc_b) = parse_hex(&accent_hex).unwrap_or((0, 122, 255));

        // Secondary / hover accent
        let accent_hover_hex = vars
            .get("dcol_1xa7")
            .cloned()
            .unwrap_or_else(|| accent_hex.clone());

        let text_color = vars
            .get("dcol_txt1")
            .map(|t| format!("#{}", t))
            .unwrap_or_else(|| "#ffffff".to_string());

        Self {
            bg_css: format!("rgba({}, {}, {}, 0.84)", bg_r, bg_g, bg_b),
            border_css: format!("rgba({}, {}, {}, 0.28)", acc_r, acc_g, acc_b),
            accent_hex: format!("#{}", accent_hex),
            accent_hover: format!("#{}", accent_hover_hex),
            active_mode_bg: format!("rgba({}, {}, {}, 0.30)", acc_r, acc_g, acc_b),
            text_color,
        }
    }

    pub fn generate_css(&self) -> String {
        format!(
            r#"
window.shotdock-window {{
    background-color: transparent;
}}

.shotdock-pill {{
    background: {bg_css};
    border: 1px solid {border_css};
    border-radius: 999px;
    padding: 6px 14px;
    box-shadow: 0 16px 42px rgba(0, 0, 0, 0.65), inset 0 1px 0 rgba(255, 255, 255, 0.18);
}}

.icon-btn {{
    background: transparent;
    border: none;
    border-radius: 999px;
    padding: 6px 14px;
    color: #d0d4dc;
    font-size: 16px;
    transition: all 150ms cubic-bezier(0.4, 0, 0.2, 1);
}}

.icon-btn:hover {{
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
}}

.icon-btn.active-mode {{
    background: {active_mode_bg};
    color: #ffffff;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.25), inset 0 0 0 1px rgba(255, 255, 255, 0.15);
}}

.close-btn {{
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 999px;
    padding: 6px 12px;
    color: #a0a6b2;
    font-size: 13px;
    font-weight: 600;
    transition: all 120ms ease-in-out;
}}

.close-btn:hover {{
    background: #e63946;
    border-color: #e63946;
    color: #ffffff;
}}

.options-btn {{
    background: transparent;
    border: none;
    border-radius: 999px;
    padding: 6px 12px;
    color: #d0d4dc;
    font-size: 13px;
    font-weight: 500;
}}

.options-btn:hover {{
    background: rgba(255, 255, 255, 0.12);
    color: #ffffff;
}}

.action-btn {{
    background: {accent_hex};
    border: none;
    border-radius: 999px;
    padding: 6px 20px;
    color: {text_color};
    font-size: 13px;
    font-weight: 600;
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.35);
}}

.action-btn:hover {{
    background: {accent_hover};
}}

.action-btn.record-mode {{
    background: #e63946;
    box-shadow: 0 4px 14px rgba(230, 57, 70, 0.4);
}}

.action-btn.record-mode:hover {{
    background: #c92a37;
}}

separator {{
    background-color: rgba(255, 255, 255, 0.12);
    margin: 4px 4px;
}}

.options-popover {{
    background: {bg_css};
    border: 1px solid {border_css};
    border-radius: 14px;
    padding: 12px 16px;
    color: {text_color};
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.7);
}}

.popover-header {{
    font-size: 11px;
    color: #8892b0;
    font-weight: 700;
    margin-top: 6px;
    margin-bottom: 2px;
}}
"#,
            bg_css = self.bg_css,
            border_css = self.border_css,
            accent_hex = self.accent_hex,
            accent_hover = self.accent_hover,
            active_mode_bg = self.active_mode_bg,
            text_color = self.text_color,
        )
    }
}

fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}
