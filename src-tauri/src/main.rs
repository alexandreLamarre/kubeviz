#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod cloud;
pub mod ctx;

pub static APP_NAME: &str = "Kubeviz";
pub static VERSION: &str = "0.1.0";

pub static mut DB: i32 = 1;

use cloud::discovery::get_cloud_namespaces;
pub fn app_to_string() -> String {
    format!("{} : {}", APP_NAME, VERSION)
}

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        // .setup(|app| {
        //     {
        //         let window = app.get_window("main").unwrap();
        //         window.open_devtools();
        //         window.close_devtools();
        //     }
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![get_cloud_namespaces])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
