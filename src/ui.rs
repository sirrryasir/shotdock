use crate::capture::{CaptureMode, execute_capture};
use crate::config::{CanvasTheme, Config};
use crate::theme::ThemeColors;
use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, CheckButton, CssProvider, DropDown,
    EventControllerKey, Label, MenuButton, Orientation, Popover, Separator, StringList,
};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ui(app: &Application) {
    let theme = ThemeColors::load();
    let css_data = theme.generate_css();

    let provider = CssProvider::new();
    provider.load_from_data(&css_data);
    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let config = Rc::new(RefCell::new(Config::load()));
    let selected_mode = Rc::new(RefCell::new(CaptureMode::Area));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("shotdock")
        .css_classes(["shotdock-window"])
        .build();

    // LayerShell Setup
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    window.set_namespace(Some("shotdock"));
    window.set_anchor(Edge::Bottom, true);
    window.set_margin(Edge::Bottom, 28);

    // Target the currently focused monitor in Hyprland
    if let Some(display) = Display::default() {
        let focused_name = std::process::Command::new("hyprctl")
            .args(["monitors", "-j"])
            .output()
            .ok()
            .and_then(|out| serde_json::from_slice::<serde_json::Value>(&out.stdout).ok())
            .and_then(|monitors| {
                monitors.as_array().and_then(|arr| {
                    arr.iter()
                        .find(|m| m.get("focused").and_then(|f| f.as_bool()) == Some(true))
                        .and_then(|m| m.get("name").and_then(|n| n.as_str()).map(String::from))
                })
            });

        if let Some(name) = focused_name {
            let monitors = display.monitors();
            for i in 0..monitors.n_items() {
                if let Some(mon) = monitors.item(i).and_downcast::<gtk4::gdk::Monitor>()
                    && mon.connector().as_deref() == Some(&name)
                {
                    window.set_monitor(Some(&mon));
                    break;
                }
            }
        }
    }

    // Main Container Pill
    let main_box = Box::new(Orientation::Horizontal, 4);
    main_box.add_css_class("shotdock-pill");

    // Close Button (✕)
    let close_btn = Button::builder()
        .label("✕")
        .css_classes(["close-btn"])
        .tooltip_text("Close shotdock")
        .valign(gtk4::Align::Center)
        .build();
    {
        let win = window.clone();
        close_btn.connect_clicked(move |_| {
            win.close();
        });
    }
    main_box.append(&close_btn);

    let sep1 = Separator::new(Orientation::Vertical);
    main_box.append(&sep1);

    // Mode Buttons
    let btn_screen = Button::builder()
        .label("󰹑")
        .tooltip_text("Capture Entire Screen")
        .build();
    let btn_window = Button::builder()
        .label("")
        .tooltip_text("Capture Active Window")
        .build();
    let btn_area = Button::builder()
        .label("󰒅")
        .tooltip_text("Capture Selected Portion")
        .build();
    let btn_ocr = Button::builder()
        .label("󰈙")
        .tooltip_text("Extract Text (OCR to Clipboard)")
        .build();

    let btn_rec_screen = Button::builder()
        .label("󰕧")
        .tooltip_text("Record Entire Screen")
        .build();
    let btn_rec_area = Button::builder()
        .label("󰑋")
        .tooltip_text("Record Selected Portion")
        .build();

    let mode_buttons = [
        (CaptureMode::FullScreen, btn_screen.clone()),
        (CaptureMode::ActiveWindow, btn_window.clone()),
        (CaptureMode::Area, btn_area.clone()),
        (CaptureMode::TextOcr, btn_ocr.clone()),
        (CaptureMode::RecordScreen, btn_rec_screen.clone()),
        (CaptureMode::RecordArea, btn_rec_area.clone()),
    ];

    for (_, btn) in &mode_buttons {
        btn.add_css_class("icon-btn");
    }
    // Default active mode
    btn_area.add_css_class("active-mode");

    // Action Button (Capture / Record / Extract)
    let action_btn = Button::with_label("Capture");
    action_btn.add_css_class("action-btn");

    // Helper to update active mode styling
    let update_mode = {
        let selected_mode = selected_mode.clone();
        let mode_buttons = mode_buttons.clone();
        let action_btn = action_btn.clone();
        Rc::new(move |mode: CaptureMode| {
            *selected_mode.borrow_mut() = mode;
            for (m, btn) in &mode_buttons {
                if *m == mode {
                    btn.add_css_class("active-mode");
                } else {
                    btn.remove_css_class("active-mode");
                }
            }
            match mode {
                CaptureMode::RecordScreen | CaptureMode::RecordArea => {
                    action_btn.set_label("Record");
                    action_btn.add_css_class("record-mode");
                }
                CaptureMode::TextOcr => {
                    action_btn.set_label("Extract");
                    action_btn.remove_css_class("record-mode");
                }
                _ => {
                    action_btn.set_label("Capture");
                    action_btn.remove_css_class("record-mode");
                }
            }
        })
    };

    // Connect mode button clicks
    for (mode, btn) in &mode_buttons {
        let update_mode = update_mode.clone();
        let m = *mode;
        btn.connect_clicked(move |_| {
            update_mode(m);
        });
    }

    main_box.append(&btn_screen);
    main_box.append(&btn_window);
    main_box.append(&btn_area);
    main_box.append(&btn_ocr);

    let sep2 = Separator::new(Orientation::Vertical);
    main_box.append(&sep2);

    main_box.append(&btn_rec_screen);
    main_box.append(&btn_rec_area);

    let sep3 = Separator::new(Orientation::Vertical);
    main_box.append(&sep3);

    // Options Popover
    let options_btn = MenuButton::builder()
        .label("Options ▾")
        .css_classes(["options-btn"])
        .build();

    let popover = Popover::new();
    popover.add_css_class("options-popover");

    let pop_content = Box::new(Orientation::Vertical, 6);

    // Section 1: Capture options
    let lbl_capture = Label::builder()
        .label("OPTIONS")
        .xalign(0.0)
        .css_classes(["popover-header"])
        .build();
    pop_content.append(&lbl_capture);

    let chk_cursor = CheckButton::with_label("Show Mouse Pointer");
    chk_cursor.set_active(config.borrow().show_cursor);
    {
        let config = config.clone();
        chk_cursor.connect_toggled(move |btn| {
            config.borrow_mut().show_cursor = btn.is_active();
            config.borrow().save();
        });
    }
    pop_content.append(&chk_cursor);

    let chk_shadow = CheckButton::with_label("Window Shadow & Rounded Corners");
    chk_shadow.set_active(config.borrow().window_shadow);
    {
        let config = config.clone();
        chk_shadow.connect_toggled(move |btn| {
            config.borrow_mut().window_shadow = btn.is_active();
            config.borrow().save();
        });
    }
    pop_content.append(&chk_shadow);

    let chk_titlebar = CheckButton::with_label("macOS Header Mockup (🔴 🟡 🟢)");
    chk_titlebar.set_active(config.borrow().macos_titlebar);
    {
        let config = config.clone();
        chk_titlebar.connect_toggled(move |btn| {
            config.borrow_mut().macos_titlebar = btn.is_active();
            config.borrow().save();
        });
    }
    pop_content.append(&chk_titlebar);

    let lbl_canvas = Label::builder()
        .label("CANVAS BACKGROUND")
        .xalign(0.0)
        .css_classes(["popover-header"])
        .build();
    pop_content.append(&lbl_canvas);

    let canvas_options = StringList::new(&[
        "Transparent (Shadow Only)",
        "Follow System (Wallbash)",
        "Real Wallpaper (Blurred)",
        "White",
        "Black",
        "Sunset (Pink / Purple)",
        "Candy (Pink / Violet)",
        "Breeze (Cyan / Blue)",
        "Raindrop (Blue / Indigo)",
        "Midnight (Dark Indigo)",
        "Forest (Emerald / Green)",
    ]);
    let canvas_dropdown = DropDown::new(Some(canvas_options), None::<gtk4::Expression>);
    let initial_canvas_idx = match config.borrow().canvas_theme {
        CanvasTheme::Transparent => 0,
        CanvasTheme::FollowSystem => 1,
        CanvasTheme::RealWallpaper => 2,
        CanvasTheme::White => 3,
        CanvasTheme::Black => 4,
        CanvasTheme::Sunset => 5,
        CanvasTheme::Candy => 6,
        CanvasTheme::Breeze => 7,
        CanvasTheme::Raindrop => 8,
        CanvasTheme::Midnight => 9,
        CanvasTheme::Forest => 10,
    };
    canvas_dropdown.set_selected(initial_canvas_idx);
    {
        let config = config.clone();
        canvas_dropdown.connect_selected_notify(move |dd| {
            let theme = match dd.selected() {
                1 => CanvasTheme::FollowSystem,
                2 => CanvasTheme::RealWallpaper,
                3 => CanvasTheme::White,
                4 => CanvasTheme::Black,
                5 => CanvasTheme::Sunset,
                6 => CanvasTheme::Candy,
                7 => CanvasTheme::Breeze,
                8 => CanvasTheme::Raindrop,
                9 => CanvasTheme::Midnight,
                10 => CanvasTheme::Forest,
                _ => CanvasTheme::Transparent,
            };
            config.borrow_mut().canvas_theme = theme;
            config.borrow().save();
        });
    }
    pop_content.append(&canvas_dropdown);

    // Section 2: Timer
    let lbl_timer = Label::builder()
        .label("TIMER")
        .xalign(0.0)
        .css_classes(["popover-header"])
        .build();
    pop_content.append(&lbl_timer);

    let timer_options = StringList::new(&["None", "3 Seconds", "5 Seconds", "10 Seconds"]);
    let timer_dropdown = DropDown::new(Some(timer_options), None::<gtk4::Expression>);
    let initial_idx = match config.borrow().timer_seconds {
        3 => 1,
        5 => 2,
        10 => 3,
        _ => 0,
    };
    timer_dropdown.set_selected(initial_idx);
    {
        let config = config.clone();
        timer_dropdown.connect_selected_notify(move |dd| {
            let seconds = match dd.selected() {
                1 => 3,
                2 => 5,
                3 => 10,
                _ => 0,
            };
            config.borrow_mut().timer_seconds = seconds;
            config.borrow().save();
        });
    }
    pop_content.append(&timer_dropdown);

    // Section 3: Destination
    let lbl_save = Label::builder()
        .label("SAVE TO")
        .xalign(0.0)
        .css_classes(["popover-header"])
        .build();
    pop_content.append(&lbl_save);

    let chk_disk = CheckButton::with_label("Save to Screenshots");
    chk_disk.set_active(config.borrow().save_to_disk);
    {
        let config = config.clone();
        chk_disk.connect_toggled(move |btn| {
            config.borrow_mut().save_to_disk = btn.is_active();
            config.borrow().save();
        });
    }
    pop_content.append(&chk_disk);

    let chk_clip = CheckButton::with_label("Copy to Clipboard");
    chk_clip.set_active(config.borrow().copy_to_clipboard);
    {
        let config = config.clone();
        chk_clip.connect_toggled(move |btn| {
            config.borrow_mut().copy_to_clipboard = btn.is_active();
            config.borrow().save();
        });
    }
    pop_content.append(&chk_clip);

    let chk_editor = CheckButton::with_label("Open in Swappy Editor");
    chk_editor.set_active(config.borrow().open_in_editor);
    {
        let config = config.clone();
        chk_editor.connect_toggled(move |btn| {
            config.borrow_mut().open_in_editor = btn.is_active();
            config.borrow().save();
        });
    }
    pop_content.append(&chk_editor);

    popover.set_child(Some(&pop_content));
    options_btn.set_popover(Some(&popover));
    main_box.append(&options_btn);

    let sep4 = Separator::new(Orientation::Vertical);
    main_box.append(&sep4);

    // Connect Capture Button
    {
        let window = window.clone();
        let selected_mode = selected_mode.clone();
        let config = config.clone();
        action_btn.connect_clicked(move |_| {
            let mode = *selected_mode.borrow();
            let conf = config.borrow().clone();
            window.set_visible(false);
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(150), move || {
                execute_capture(mode, &conf);
                std::process::exit(0);
            });
        });
    }
    main_box.append(&action_btn);

    // Keyboard Controller (Escape to close, Enter to trigger capture)
    let key_controller = EventControllerKey::new();
    {
        let window = window.clone();
        let selected_mode = selected_mode.clone();
        let config = config.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, _| match keyval.name().as_deref() {
            Some("Escape") => {
                window.close();
                gtk4::glib::Propagation::Stop
            }
            Some("Return") => {
                let mode = *selected_mode.borrow();
                let conf = config.borrow().clone();
                window.set_visible(false);
                gtk4::glib::timeout_add_local_once(
                    std::time::Duration::from_millis(150),
                    move || {
                        execute_capture(mode, &conf);
                        std::process::exit(0);
                    },
                );
                gtk4::glib::Propagation::Stop
            }
            _ => gtk4::glib::Propagation::Proceed,
        });
    }
    window.add_controller(key_controller);

    window.set_child(Some(&main_box));
    window.present();
}
