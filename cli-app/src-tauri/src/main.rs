// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rs_ray::{ray, Ray};
use tauri::{api::cli::SubcommandMatches, window::WindowBuilder, Manager};

struct Name(String);

#[tauri::command]
fn get_name(state: tauri::State<Name>) -> String {
    state.0.clone()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_name])
        .setup(|app| {
            match app.get_cli_matches() {
                // `matches` here is a Struct with { args, subcommand }.
                // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
                // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
                Ok(matches) => {
                    if let Some(subcommand_matches) = matches.subcommand {
                        if subcommand_matches.name == "greet".to_string() {
                            if let Some(name) = subcommand_matches.matches.args.get("name") {
                                let name = trim_quotes(name.value.as_str().unwrap()).to_string();

                                app.manage(Name(name));

                                let _ = WindowBuilder::new(
                                    app,
                                    "greet",
                                    tauri::WindowUrl::App("index.html".into()),
                                )
                                .build();
                            }
                        }
                    }
                    else {
                        std::process::exit(0);
                    }
                }
                Err(_) => {}
            };
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// would probably be better as a trait
fn trim_quotes(string: &str) -> &str {
    let bytes = string.as_bytes();
    let len = string.len();
    if len > 1 && (bytes[0] == b'"' || bytes[0] == b'\'') && bytes[0] == bytes[len - 1] {
        &string[1..len - 1]
    } else {
        &string[..]
    }
}
