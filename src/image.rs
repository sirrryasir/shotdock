use crate::config::CanvasTheme;
use crate::theme;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn get_png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 24 || &data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let w = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let h = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Some((w, h))
}

pub fn apply_macos_decorations(input: &[u8], add_titlebar: bool, theme: CanvasTheme) -> Vec<u8> {
    let (w, mut h) = match get_png_dimensions(input) {
        Some(dim) => dim,
        None => return input.to_vec(),
    };

    let mut cmd = Command::new("magick");
    cmd.args([
        "-limit", "memory", "512MiB", "-limit", "map", "1GiB", "-filter", "Lanczos", "-quality",
        "100",
    ]);
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
            let (c1, c2) = theme::get_wallbash_gradient();
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
            let candidates = [
                format!("{}/.cache/dotfiles/wall.blur", home),
                format!("{}/.cache/wallbash/wall.blur", home),
                format!("{}/.cache/dotfiles/wall.set", home),
                format!("{}/.cache/wallbash/wall.set", home),
                format!("{}/.cache/current_wallpaper", home),
                format!("{}/.current_wallpaper", home),
            ];

            if let Some(path) = candidates.iter().find(|p| Path::new(p).exists()) {
                cmd.args([
                    "(",
                    path,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_dimensions_valid() {
        let mut data = vec![0u8; 32];
        data[0..8].copy_from_slice(b"\x89PNG\r\n\x1a\n");
        data[16..20].copy_from_slice(&1920u32.to_be_bytes());
        data[20..24].copy_from_slice(&1080u32.to_be_bytes());

        assert_eq!(get_png_dimensions(&data), Some((1920, 1080)));
    }

    #[test]
    fn test_png_dimensions_invalid() {
        assert_eq!(get_png_dimensions(b"not a png"), None);
        assert_eq!(get_png_dimensions(&[]), None);
    }
}
