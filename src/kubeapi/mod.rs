use axum::{http::StatusCode, response::IntoResponse, Json};
use k8s_openapi::api::core::v1 as api;
use kube::{
    api::{Api, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    core::{CustomResourceExt, Resource},
    Client,
};
use serde::Deserialize;

/// lists the namespaces of the current client
pub async fn namespaces() -> impl IntoResponse {
    let client = Client::try_default().await.unwrap();
    let namespaces: Api<api::Namespace> = Api::all(client.clone());
    let namespaces = namespaces.list(&ListParams::default()).await.unwrap();
    let namespaces = namespaces.iter().map(|ns| ns.name()).collect::<Vec<_>>();
    Json(namespaces)
}

/// add a kubeconfig from the filesystem
pub async fn create_kubeconfig_file(Json(payload): Json<CreateKubeConfig>) -> impl IntoResponse {
    (StatusCode::OK, Json("OK"))
}

#[derive(Deserialize)]
pub struct CreateKubeConfig {
    file_system_path: String,
}
