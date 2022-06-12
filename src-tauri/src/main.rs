#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod ctx;
mod kube;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::Manager;

pub static APP_NAME: &str = "Kubeviz";
pub static VERSION: &str = "0.1.0";

pub static mut DB: i32 = 1;

pub fn app_to_string() -> String {
  format!("{} : {}", APP_NAME, VERSION)
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
        window.close_devtools();
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect(format!("error while running {}", app_to_string()).as_str());
}
