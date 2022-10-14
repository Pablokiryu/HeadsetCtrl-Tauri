#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent,SystemTrayMenuItem};
use tauri::Manager;
use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn grab_headset_capabilities(name: &str) -> String {
    let out = Command::new("headsetcontrol")
    .arg(name.to_ascii_lowercase())
    .output()
    .expect("CommandFailed");
    let s = String::from_utf8(out.stdout).unwrap(); 
    s
}

fn main() { 
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet,grab_headset_capabilities])
      .system_tray(SystemTray::new().with_menu(tray_menu))
      .on_system_tray_event(|app, event| match event {
        SystemTrayEvent::LeftClick {
          position: _,
          size: _,
          ..
        } => {
          println!("system tray received a left click");
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
        SystemTrayEvent::MenuItemClick { id, .. } => {
          match id.as_str() {
            "quit" => {
              std::process::exit(0);
            }
            "hide" => {
              let window = app.get_window("main").unwrap();
              window.hide().unwrap();
            }
            _ => {}
          }
        }
        _ => {}
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application")
}
