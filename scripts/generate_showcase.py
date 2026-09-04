#!/usr/bin/env python3
import os
import subprocess
import cairo
import gi
gi.require_version('Pango', '1.0')
gi.require_version('PangoCairo', '1.0')
from gi.repository import Pango, PangoCairo

def round_rect(ctx, x, y, w, h, r):
    ctx.new_sub_path()
    ctx.arc(x + w - r, y + r, r, -1.57079632679, 0)
    ctx.arc(x + w - r, y + h - r, r, 0, 1.57079632679)
    ctx.arc(x + r, y + h - r, r, 1.57079632679, 3.14159265359)
    ctx.arc(x + r, y + r, r, 3.14159265359, 4.71238898038)
    ctx.close_path()

def draw_pill(ctx, cx, cy, active_mode="area"):
    # Pill dimensions: 680x50
    pw, ph = 680, 50
    px = cx - pw / 2
    py = cy - ph / 2

    # Drop shadow for pill
    ctx.save()
    round_rect(ctx, px, py + 4, pw, ph, 25)
    ctx.set_source_rgba(0, 0, 0, 0.45)
    ctx.fill()
    ctx.restore()

    # Pill background (dark glassmorphism)
    round_rect(ctx, px, py, pw, ph, 25)
    ctx.set_source_rgba(0.12, 0.14, 0.18, 0.92)
    ctx.fill_preserve()
    ctx.set_source_rgba(1, 1, 1, 0.18)
    ctx.set_line_width(1.2)
    ctx.stroke()

    # Close button (circle at left)
    close_x = px + 28
    ctx.arc(close_x, cy, 14, 0, 6.28318)
    ctx.set_source_rgba(1, 1, 1, 0.08)
    ctx.fill_preserve()
    ctx.set_source_rgba(1, 1, 1, 0.14)
    ctx.set_line_width(1)
    ctx.stroke()

    # ✕ text
    layout = PangoCairo.create_layout(ctx)
    desc = Pango.FontDescription("JetBrainsMono Nerd Font Semi-Bold 10")
    layout.set_font_description(desc)
    layout.set_text("✕", -1)
    ink, logical = layout.get_pixel_extents()
    ctx.move_to(close_x - logical.width / 2, cy - logical.height / 2)
    ctx.set_source_rgba(0.65, 0.7, 0.78, 1)
    PangoCairo.show_layout(ctx, layout)

    # Separator 1
    sep1_x = px + 54
    ctx.move_to(sep1_x, cy - 14)
    ctx.line_to(sep1_x, cy + 14)
    ctx.set_source_rgba(1, 1, 1, 0.15)
    ctx.set_line_width(1)
    ctx.stroke()

    # Mode icons: Screen, Window, Area, OCR
    icons = [
        ("screen", "󰹑"),
        ("window", ""),
        ("area", "󰒅"),
        ("ocr", "󰈙"),
    ]
    cur_x = sep1_x + 28
    desc_icon = Pango.FontDescription("JetBrainsMono Nerd Font 14")
    layout.set_font_description(desc_icon)

    for mode_id, sym in icons:
        if mode_id == active_mode:
            # Active badge
            round_rect(ctx, cur_x - 18, cy - 18, 36, 36, 18)
            ctx.set_source_rgba(0.2, 0.5, 0.85, 0.35)
            ctx.fill_preserve()
            ctx.set_source_rgba(1, 1, 1, 0.25)
            ctx.set_line_width(1)
            ctx.stroke()
            ctx.set_source_rgba(1, 1, 1, 1)
        else:
            ctx.set_source_rgba(0.8, 0.84, 0.9, 0.85)

        layout.set_text(sym, -1)
        ink, logical = layout.get_pixel_extents()
        ctx.move_to(cur_x - logical.width / 2, cy - logical.height / 2)
        PangoCairo.show_layout(ctx, layout)
        cur_x += 44

    # Separator 2
    sep2_x = cur_x - 10
    ctx.move_to(sep2_x, cy - 14)
    ctx.line_to(sep2_x, cy + 14)
    ctx.set_source_rgba(1, 1, 1, 0.15)
    ctx.stroke()

    # Record buttons: Rec Screen, Rec Area
    rec_icons = [("rec_screen", "󰕧"), ("rec_area", "󰑋")]
    cur_x = sep2_x + 28
    for mode_id, sym in rec_icons:
        if mode_id == active_mode:
            round_rect(ctx, cur_x - 18, cy - 18, 36, 36, 18)
            ctx.set_source_rgba(0.9, 0.2, 0.25, 0.4)
            ctx.fill()
            ctx.set_source_rgba(1, 0.4, 0.4, 1)
        else:
            ctx.set_source_rgba(0.8, 0.84, 0.9, 0.85)

        layout.set_text(sym, -1)
        ink, logical = layout.get_pixel_extents()
        ctx.move_to(cur_x - logical.width / 2, cy - logical.height / 2)
        PangoCairo.show_layout(ctx, layout)
        cur_x += 44

    # Separator 3
    sep3_x = cur_x - 10
    ctx.move_to(sep3_x, cy - 14)
    ctx.line_to(sep3_x, cy + 14)
    ctx.set_source_rgba(1, 1, 1, 0.15)
    ctx.stroke()

    # Options button
    opt_x = sep3_x + 52
    desc_txt = Pango.FontDescription("JetBrainsMono Nerd Font Semi-Bold 10")
    layout.set_font_description(desc_txt)
    layout.set_text("Options ▾", -1)
    ink, logical = layout.get_pixel_extents()
    ctx.move_to(opt_x - logical.width / 2, cy - logical.height / 2)
    ctx.set_source_rgba(0.85, 0.88, 0.94, 0.95)
    PangoCairo.show_layout(ctx, layout)

    # Action button (Capture pill)
    btn_w, btn_h = 92, 34
    btn_x = px + pw - btn_w - 8
    btn_y = cy - btn_h / 2
    round_rect(ctx, btn_x, btn_y, btn_w, btn_h, 17)
    if "rec" in active_mode:
        ctx.set_source_rgba(0.9, 0.22, 0.27, 1)
    else:
        ctx.set_source_rgba(0.05, 0.48, 0.95, 1)
    ctx.fill()

    action_label = "Record" if "rec" in active_mode else "Capture"
    layout.set_text(action_label, -1)
    ink, logical = layout.get_pixel_extents()
    ctx.move_to(btn_x + (btn_w - logical.width) / 2, cy - logical.height / 2)
    ctx.set_source_rgba(1, 1, 1, 1)
    PangoCairo.show_layout(ctx, layout)


def draw_options_popover(ctx, px, py):
    pw, ph = 270, 310
    round_rect(ctx, px, py, pw, ph, 14)
    ctx.set_source_rgba(0.1, 0.12, 0.16, 0.96)
    ctx.fill_preserve()
    ctx.set_source_rgba(0.2, 0.5, 0.85, 0.45)
    ctx.set_line_width(1.2)
    ctx.stroke()

    layout = PangoCairo.create_layout(ctx)
    desc_hdr = Pango.FontDescription("JetBrainsMono Nerd Font Bold 8")
    desc_item = Pango.FontDescription("JetBrainsMono Nerd Font Regular 9")
    desc_val = Pango.FontDescription("JetBrainsMono Nerd Font Semi-Bold 9")

    # Header 1
    layout.set_font_description(desc_hdr)
    layout.set_text("OPTIONS", -1)
    ctx.move_to(px + 16, py + 14)
    ctx.set_source_rgba(0.55, 0.65, 0.8, 1)
    PangoCairo.show_layout(ctx, layout)

    # Options Checkboxes
    opts = [
        ("Show Mouse Pointer", False),
        ("Window Shadow & Corners", True),
        ("macOS Window Titlebar", True),
    ]
    cy = py + 34
    for label, checked in opts:
        layout.set_font_description(desc_item)
        round_rect(ctx, px + 16, cy + 1, 12, 12, 3)
        if checked:
            ctx.set_source_rgba(0.05, 0.48, 0.95, 1)
            ctx.fill()
            ctx.set_source_rgba(1, 1, 1, 1)
            ctx.arc(px + 22, cy + 7, 2.5, 0, 6.28)
            ctx.fill()
        else:
            ctx.set_source_rgba(1, 1, 1, 0.15)
            ctx.fill()

        layout.set_text(label, -1)
        ctx.move_to(px + 36, cy)
        ctx.set_source_rgba(0.9, 0.92, 0.96, 1)
        PangoCairo.show_layout(ctx, layout)
        cy += 24

    # Header 2: Canvas Background
    layout.set_font_description(desc_hdr)
    layout.set_text("CANVAS BACKGROUND", -1)
    ctx.move_to(px + 16, cy + 6)
    ctx.set_source_rgba(0.55, 0.65, 0.8, 1)
    PangoCairo.show_layout(ctx, layout)
    cy += 24

    # Dropdown pill
    round_rect(ctx, px + 16, cy, pw - 32, 26, 6)
    ctx.set_source_rgba(1, 1, 1, 0.08)
    ctx.fill_preserve()
    ctx.set_source_rgba(1, 1, 1, 0.18)
    ctx.stroke()

    layout.set_font_description(desc_val)
    layout.set_text("Sunset (Pink / Purple) ▾", -1)
    ctx.move_to(px + 26, cy + 5)
    ctx.set_source_rgba(0.95, 0.96, 0.98, 1)
    PangoCairo.show_layout(ctx, layout)
    cy += 36

    # Header 3: Destination
    layout.set_font_description(desc_hdr)
    layout.set_text("SAVE DESTINATION", -1)
    ctx.move_to(px + 16, cy + 4)
    ctx.set_source_rgba(0.55, 0.65, 0.8, 1)
    PangoCairo.show_layout(ctx, layout)
    cy += 22

    dests = [
        ("Save to Screenshots", True),
        ("Copy to Clipboard", True),
        ("Open in Editor (Satty)", False),
    ]
    for label, checked in dests:
        layout.set_font_description(desc_item)
        round_rect(ctx, px + 16, cy + 1, 12, 12, 3)
        if checked:
            ctx.set_source_rgba(0.05, 0.48, 0.95, 1)
            ctx.fill()
            ctx.set_source_rgba(1, 1, 1, 1)
            ctx.arc(px + 22, cy + 7, 2.5, 0, 6.28)
            ctx.fill()
        else:
            ctx.set_source_rgba(1, 1, 1, 0.15)
            ctx.fill()

        layout.set_text(label, -1)
        ctx.move_to(px + 36, cy)
        ctx.set_source_rgba(0.9, 0.92, 0.96, 1)
        PangoCairo.show_layout(ctx, layout)
        cy += 22


def render_mock_terminal(filename, w=760, h=420):
    surf = cairo.ImageSurface(cairo.FORMAT_ARGB32, w, h)
    ctx = cairo.Context(surf)

    # Terminal background
    ctx.set_source_rgb(0.09, 0.11, 0.14)
    ctx.paint()

    # Terminal content
    layout = PangoCairo.create_layout(ctx)
    desc = Pango.FontDescription("JetBrainsMono Nerd Font 11")
    layout.set_font_description(desc)

    lines = [
        ("<span color='#7aa4c2'>yasir@archlinux</span>:<span color='#89ddff'>~/dev/shotdock</span>$ <span color='#c3e88d'>cargo build --release</span>", 24),
        ("<span color='#82aaff'>   Compiling</span> shotdock v0.2.0 (/home/yasir/dev/shotdock)", 54),
        ("<span color='#c3e88d'>    Finished</span> release [optimized] target(s) in 1.45s", 80),
        ("", 104),
        ("<span color='#7aa4c2'>yasir@archlinux</span>:<span color='#89ddff'>~/dev/shotdock</span>$ <span color='#c3e88d'>shotdock --area</span>", 114),
        ("<span color='#bb80ff'>[shotdock]</span> Captured 4K selection via grim + slurp", 144),
        ("<span color='#bb80ff'>[shotdock]</span> Applied macOS frame, 16px radius, drop shadow", 170),
        ("<span color='#c3e88d'>[shotdock]</span> Copied to clipboard &amp; saved to Screenshots", 196),
        ("", 222),
        ("<span color='#7aa4c2'>yasir@archlinux</span>:<span color='#89ddff'>~/dev/shotdock</span>$ <span color='#ffffff'>_</span>", 232),
    ]

    for markup, y in lines:
        if markup:
            layout.set_markup(markup, -1)
            ctx.set_source_rgb(0.88, 0.92, 0.96)
            ctx.move_to(28, y)
            PangoCairo.show_layout(ctx, layout)

    surf.write_to_png(filename)


def main():
    os.makedirs("assets", exist_ok=True)
    os.makedirs("docs/src/images", exist_ok=True)

    # 1. Generate standalone toolbar (840x120 transparent canvas)
    surf1 = cairo.ImageSurface(cairo.FORMAT_ARGB32, 840, 120)
    ctx1 = cairo.Context(surf1)
    draw_pill(ctx1, 420, 60, "area")
    surf1.write_to_png("assets/toolbar.png")

    # 2. Generate toolbar with options popover open (840x440 transparent)
    surf2 = cairo.ImageSurface(cairo.FORMAT_ARGB32, 840, 440)
    ctx2 = cairo.Context(surf2)
    draw_options_popover(ctx2, 380, 20)
    draw_pill(ctx2, 420, 390, "area")
    surf2.write_to_png("assets/toolbar_options.png")

    # 3. Render clean mock terminal and apply macOS framing via ImageMagick
    render_mock_terminal("/tmp/mock_term.png")
    subprocess.run([
        "magick", "/tmp/mock_term.png",
        "-limit", "memory", "512MiB",
        "-filter", "Lanczos", "-quality", "100",
        "-background", "#1e222a",
        "-splice", "0x34",
        "-fill", "#ff5f56", "-draw", "circle 16,17 22,17",
        "-fill", "#ffbd2e", "-draw", "circle 36,17 42,17",
        "-fill", "#27c93f", "-draw", "circle 56,17 62,17",
        "-alpha", "set",
        "(", "+clone", "-alpha", "transparent", "-background", "none", "-fill", "white", "-draw", "roundrectangle 0,0 759,453 16,16", ")",
        "-compose", "DstIn", "-composite",
        "(", "+clone", "-background", "rgba(0,0,0,0.55)", "-shadow", "60x18+0+14", ")",
        "+swap", "-background", "none", "-compose", "Over", "-layers", "merge", "+repage",
        "assets/framed_window.png"
    ], check=True)

    # 4. Wrap the framed window in a Sunset presentation canvas
    subprocess.run([
        "magick", "assets/framed_window.png",
        "(", "-size", "1040x660", "gradient:#f43f5e-#8b5cf6", ")",
        "+swap", "-gravity", "center", "-composite",
        "assets/canvas_presentation.png"
    ], check=True)

    # 5. Build presentation slide frames for demo.gif
    # Standard 1100x640 dark wallpaper canvas for all frames
    subprocess.run([
        "magick", "-size", "1100x640", "gradient:#0f172a-#1e1b4b",
        "/tmp/demo_bg.png"
    ], check=True)

    # Frame 1: Dock idle at bottom
    subprocess.run([
        "magick", "/tmp/demo_bg.png", "assets/toolbar.png",
        "-gravity", "south", "-geometry", "+0+36", "-composite",
        "/tmp/f1.png"
    ], check=True)

    # Frame 2: Options open
    subprocess.run([
        "magick", "/tmp/demo_bg.png", "assets/toolbar_options.png",
        "-gravity", "south", "-geometry", "+0+12", "-composite",
        "/tmp/f2.png"
    ], check=True)

    # Frame 3: Framed window preview
    subprocess.run([
        "magick", "/tmp/demo_bg.png", "assets/framed_window.png",
        "-gravity", "center", "-composite",
        "/tmp/f3.png"
    ], check=True)

    # Frame 4: Canvas presentation preview
    subprocess.run([
        "magick", "/tmp/demo_bg.png",
        "(", "assets/canvas_presentation.png", "-resize", "880x560", ")",
        "-gravity", "center", "-composite",
        "/tmp/f4.png"
    ], check=True)

    # 6. Two-pass optimized ffmpeg palette GIF generation (<2MB)
    # Using split filter to properly feed palettegen and paletteuse
    subprocess.run([
        "ffmpeg", "-y",
        "-loop", "1", "-t", "2.2", "-i", "/tmp/f1.png",
        "-loop", "1", "-t", "2.2", "-i", "/tmp/f2.png",
        "-loop", "1", "-t", "2.5", "-i", "/tmp/f3.png",
        "-loop", "1", "-t", "2.5", "-i", "/tmp/f4.png",
        "-filter_complex",
        "[0:v][1:v][2:v][3:v]concat=n=4:v=1:a=0,fps=12,scale=880:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128[p];[s1][p]paletteuse=dither=sierra2_4a",
        "assets/demo.gif"
    ], check=True)

    # Copy all generated public assets to docs/src/images
    for f in ["toolbar.png", "toolbar_options.png", "framed_window.png", "canvas_presentation.png", "demo.gif"]:
        subprocess.run(["cp", f"assets/{f}", f"docs/src/images/{f}"], check=True)

    print("Successfully generated all clean public showcase assets!")

if __name__ == "__main__":
    main()
