#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod cloud;
pub mod ctx;
use tauri::Manager;

pub static APP_NAME: &str = "Kubeviz";
pub static VERSION: &str = "0.1.0";

pub static mut DB: i32 = 1;

use cloud::discovery::{
    get_cloud_namespaces, get_deployments, get_pods, get_replicasets, get_services,
    get_statefulsets, main_watch,
};
pub fn app_to_string() -> String {
    format!("{} : {}", APP_NAME, VERSION)
}

#[tauri::command]
fn cmd_a() -> String {
    format!("{} : {}", APP_NAME, VERSION)
}

#[tauri::command]
fn cmd_err() -> Result<String, String> {
    Err(format!("{} : {}", APP_NAME, VERSION))
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_watch().await?;
    Ok(())
    // tauri::Builder::default()
    //     // This is where you pass in your commands
    //     // .setup(|app| {
    //     //     {
    //     //         let window = app.get_window("main").unwrap();
    //     //         window.open_devtools();
    //     //         window.close_devtools();
    //     //     }
    //     //     Ok(())
    //     // })
    //     .invoke_handler(tauri::generate_handler![
    //         get_cloud_namespaces,
    //         get_pods,
    //         get_replicasets,
    //         get_services,
    //         get_statefulsets,
    //         get_deployments,
    //         cmd_a,
    //         cmd_err,
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("failed to run app");
}
