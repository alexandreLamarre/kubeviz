#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
pub mod cloud;
pub mod ctx;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::Manager;

pub static APP_NAME: &str = "Kubeviz";
pub static VERSION: &str = "0.1.0";

pub static mut DB: i32 = 1;

use cloud::discovery::{
  get_cloud_namespaces, get_deployments, get_pods, get_replicasets, get_services, get_statefulsets,
};
pub fn app_to_string() -> String {
  format!("{} : {}", APP_NAME, VERSION)
}

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

fn main() {
  // tauri::Builder::default()
  //   // .setup(|app| {
  //   //   {
  //   //     let window = app.get_window("main").unwrap();
  //   //     window.open_devtools();
  //   //     window.close_devtools();
  //   //   }
  //   //   Ok(())
  //   // })
  //   .invoke_handler(tauri::generate_context!(my_custom_command))
  //   .run(tauri::generate_context!())
  //   .expect(format!("error while running {}", app_to_string()).as_str());

  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
