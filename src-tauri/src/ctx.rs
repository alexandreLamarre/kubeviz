use std::fs;
use std::path::Path;
static KUBEVIZ_CTX: &str = "~/.kubeviz";
use crate::app_to_string;
use crate::DB;

/// Creates a kube_config for the Kubeviz app
pub fn create_kube_config(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(KUBEVIZ_CTX).exists() {
        fs::create_dir_all(KUBEVIZ_CTX)?;
    }
    if !Path::new(file_path).is_file() {
        return Err(format!("{} is not a file", file_path).into());
    }
    fs::copy(file_path, KUBEVIZ_CTX)?;
    Ok(())
}

/// Makes kubeviz use this kubeconfig
pub fn use_kube_config(identifer: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(format!("{}/{}", KUBEVIZ_CTX, identifer).as_str()).exists() {
        std::env::set_var("KUBECONFIG", format!("{}/{}", &KUBEVIZ_CTX, &identifer));
        return Ok(());
    }
    Err(format!("{} was not found by {}", identifer, app_to_string()).into())
}

#[tauri::command]
pub fn increment_db() {
    unsafe {
        DB += 1;
    }
}

#[tauri::command]
pub fn get_db() -> i32 {
    unsafe { DB }
}
