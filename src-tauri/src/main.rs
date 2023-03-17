use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};
use window_vibrancy::NSVisualEffectMaterial;

use helper::write_mouse_cfg;

mod helper;

#[tauri::command]
fn get_mouse_cfg() -> (usize, bool) {
    let (sen, acc_enabled) = helper::read_mouse_cfg().unwrap();
    (sen as usize - 100, acc_enabled)
}

#[tauri::command]
fn set_mouse_cfg(sen: usize, acc_enabled: bool) {
    let sen = std::cmp::min(sen, 99);
    write_mouse_cfg(sen as i32 + 100, acc_enabled).unwrap();
}

fn system_tray() -> SystemTray {
    // let about = CustomMenuItem::new("about", "About");
    let quit = CustomMenuItem::new("quit", "Quit");
    let tray_menu = SystemTrayMenu::new()
        // .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(system_tray())
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayCenter);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a right click");
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    // "hide" => {
                    //     let window = app.get_window("main").unwrap();
                    //     window.hide().unwrap();
                    // }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            tauri::WindowEvent::CloseRequested { .. } => {
                event.window().hide().unwrap();
            }
            tauri::WindowEvent::ThemeChanged(theme) => {
                println!("theme changed: {:?}", theme);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_mouse_cfg, set_mouse_cfg])
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();

            window_vibrancy::apply_vibrancy(
                &main_window,
                NSVisualEffectMaterial::HudWindow,
                None,
                Some(16f64),
            )
            .expect("unable to apply vibrancy");

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
