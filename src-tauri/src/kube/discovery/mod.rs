use k8s_openapi::api::apps::v1::{Deployment, ReplicaSet, StatefulSet};
use k8s_openapi::api::core::v1::{Namespace, Pod, Service};
use kube::{
    api::{Api, ListParams, PostParams, ResourceExt},
    Client,
};
use std::error::Error;

/// Gets all namespaces in current context
#[tauri::command]
pub async fn get_namespaces(cl: kube::Client) -> Result<Vec<String>, Box<dyn Error>> {
    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let ns: Api<Namespace> = Api::default_namespaced(cl);
    let mut res: Vec<String> = vec![];
    for n in ns.list(&ListParams::default()).await? {
        println!("found pod {}", n.name());
        res.push(n.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_pods(cl: kube::Client, ns: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let pods: Api<Pod> = Api::namespaced(cl, ns);
    let mut res: Vec<String> = vec![];
    for p in pods.list(&ListParams::default()).await? {
        println!("found pod {}", p.name());
        res.push(p.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_deployments(cl: kube::Client, ns: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let deployments: Api<Deployment> = Api::namespaced(cl, ns);
    let mut res: Vec<String> = vec![];
    for d in deployments.list(&ListParams::default()).await? {
        println!("found deployment {}", d.name());
        res.push(d.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_replicasets(cl: kube::Client, ns: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let replicasets: Api<ReplicaSet> = Api::namespaced(cl, ns);
    let mut res: Vec<String> = vec![];
    for r in replicasets.list(&ListParams::default()).await? {
        println!("found replicaset {}", r.name());
        res.push(r.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_statefulsets(cl: kube::Client, ns: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let statefulsets: Api<StatefulSet> = Api::namespaced(cl, ns);
    let mut res: Vec<String> = vec![];
    for s in statefulsets.list(&ListParams::default()).await? {
        println!("found statefulset {}", s.name());
        res.push(s.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_services(cl: kube::Client, ns: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let pods: Api<Service> = Api::namespaced(cl, ns);
    let mut res: Vec<String> = vec![];
    for p in pods.list(&ListParams::default()).await? {
        println!("found pod {}", p.name());
        res.push(p.name());
    }
    Ok(res)
}
