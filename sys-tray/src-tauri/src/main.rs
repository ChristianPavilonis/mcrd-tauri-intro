// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rs_ray::{ray, Ray};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    WindowEvent,
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
    //         .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
    //         .add_item(CustomMenuItem::new("hide".to_string(), "Hide")),
    // );

    let tray = SystemTray::new();

    tauri::Builder::default()
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
                    if let Some(tray) = app.get_window("tray") {
                        if tray.is_visible().is_ok_and(|v| v) {
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
                SystemTrayEvent::RightClick { position, size, .. } => {}
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
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
