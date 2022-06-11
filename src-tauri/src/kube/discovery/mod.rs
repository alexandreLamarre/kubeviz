use k8s_openapi::api::core::v1::{Namespace, Pod};
use kube::{
    api::{Api, ListParams, PostParams, ResourceExt},
    Client,
};
use std::error::Error;

#[tauri::command]
pub async fn namespaces() -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::try_default().await?;

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let pods: Api<Namespace> = Api::default_namespaced(client);
    let mut res: Vec<String> = vec![];
    for p in pods.list(&ListParams::default()).await? {
        println!("found pod {}", p.name());
        res.push(p.name());
    }
    Ok(res)
}
