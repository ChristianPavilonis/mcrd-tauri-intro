// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rs_ray::{ray, Ray};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu, WindowEvent,
};
use tauri_plugin_positioner::{Position, WindowExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // let tray = SystemTray::new().with_menu(
    //     SystemTrayMenu::new()
    //         .add_item(CustomMenuItem::new("hello".to_string(), "Hello"))
    //         .add_submenu(SystemTraySubmenu::new(
    //             "Submenu",
    //             SystemTrayMenu::new().add_item(CustomMenuItem::new("submenu-item", "Submenu Item")),
    //         ))
    //         .add_item(CustomMenuItem::new("quit".to_string(), "Quit")),
    // );

    let tray = SystemTray::new();

    let mut app = tauri::Builder::default()
        // plugin will place our window in the correct spot.
        .plugin(tauri_plugin_positioner::init())
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    // this could be done in a much cleanr way probably
                    if let Some(tray) = app.get_window("tray") {
                        if tray.is_visible().is_ok_and(|is_visible| is_visible) {
                            let _ = tray.hide();
                        } else {
                            let _ = tray.show();
                        }
                    } else {
                        let result = tauri::WindowBuilder::new(
                            app,
                            "tray",
                            tauri::WindowUrl::App("index.html".into()),
                        )
                        .inner_size(500 as f64, 900 as f64)
                        .decorations(false)
                        .focused(true)
                        .always_on_top(true)
                        .build();

                        if let Ok(window) = result {
                            let _ = window.move_window(Position::TrayCenter);

                            let window_handler = window.clone();

                            window.on_window_event(move |event| match event {
                                WindowEvent::Focused(focused) if !focused => {
                                    let _ = window_handler.hide();
                                }
                                _ => {}
                            });
                        }
                    }
                }
                // SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                //     "quit" => {
                //         std::process::exit(0);
                //     }
                //     "hello" => {
                //         ray!("Hello, from the system tray").color("blue");
                //     }
                //     _ => {}
                // },
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    // keeps our app out of the dock on mac
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
