// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, NativeImage};
use tauri::{CustomMenuItem, SystemTrayMenuItem, MenuEntry, SystemTraySubmenu};
mod keyboard_listener;
use std::thread;

#[derive(Clone, serde::Serialize)]
struct Payload {
    mode: String,
    message: String,
}

fn main() {

  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
let enable = CustomMenuItem::new("enable".to_string(), "Enable").accelerator("Cmd+E");
let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
let volume_slider = CustomMenuItem::new("volume".to_string(), "Volume");

let volume_menu = SystemTrayMenu::new().add_item(volume_slider);
let volume = SystemTraySubmenu::new("Volume", volume_menu);

let switches_menu = SystemTrayMenu::new();
let switches = SystemTraySubmenu::new("Switches", switches_menu);

let tray_menu = SystemTrayMenu::new()
.add_item(enable)
.add_native_item(SystemTrayMenuItem::Separator)
.add_submenu(volume)
.add_submenu(switches)

.add_native_item(SystemTrayMenuItem::Separator)

.add_item(quit);
  let system_tray = SystemTray::new()
    .with_menu(tray_menu);
  tauri::Builder::default()
    .system_tray(system_tray)
    .on_system_tray_event(|_app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "quit" => {
            std::process::exit(0);
          }
          _ => {}
        }
      }
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
