use k8s_openapi::api::apps::v1::{Deployment, ReplicaSet, StatefulSet};
use k8s_openapi::api::core::v1::{Namespace, Pod, Service};
//use k8s_openapi::ListOptional;
use kube::{
    api::{Api, ListParams, /*PostParams,*/ ResourceExt},
    Client,
};
use std::error::Error;

/// Gets all namespaces in current context
#[tauri::command]
pub async fn get_cloud_namespaces() -> Result<Vec<String>, String> {
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(e) => return Err(format!("Connecting to default kubernetes : {}", e)),
    };
    let ns: Api<Namespace> = Api::all(client);
    let mut res: Vec<String> = vec![];
    for n in ns.list(&ListParams::default()).await.unwrap() {
        println!("Found namespace {}", n.name());
        res.push(n.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_pods(ns: &str) -> Result<Vec<String>, String> {
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(e) => return Err(format!("Connecting to default kubernetes : {}", e)),
    };
    let pods: Api<Pod> = Api::namespaced(client, ns);
    let mut res: Vec<String> = vec![];
    for p in pods.list(&ListParams::default()).await.unwrap() {
        println!("found pod {}", p.name());
        res.push(p.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_deployments(ns: &str) -> Result<Vec<String>, String> {
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(e) => return Err(format!("Connecting to default kubernetes : {}", e)),
    };
    let deployments: Api<Deployment> = Api::namespaced(client, ns);
    let mut res: Vec<String> = vec![];
    for d in deployments.list(&ListParams::default()).await.unwrap() {
        println!("found deployment {}", d.name());
        res.push(d.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_replicasets(ns: &str) -> Result<Vec<String>, String> {
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(e) => return Err(format!("Connecting to default kubernetes : {}", e)),
    };
    let replicasets: Api<ReplicaSet> = Api::namespaced(client, ns);
    let mut res: Vec<String> = vec![];
    for r in replicasets.list(&ListParams::default()).await.unwrap() {
        println!("found replicaset {}", r.name());
        res.push(r.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_statefulsets(ns: &str) -> Result<Vec<String>, String> {
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(e) => return Err(format!("Connecting to default kubernetes : {}", e)),
    };
    let statefulsets: Api<StatefulSet> = Api::namespaced(client, ns);
    let mut res: Vec<String> = vec![];
    for s in statefulsets.list(&ListParams::default()).await.unwrap() {
        println!("found statefulset {}", s.name());
        res.push(s.name());
    }
    Ok(res)
}

#[tauri::command]
pub async fn get_services(ns: &str) -> Result<Vec<String>, String> {
    let client = match Client::try_default().await {
        Ok(client) => client,
        Err(e) => return Err(format!("Connecting to default kubernetes : {}", e)),
    };
    let pods: Api<Service> = Api::namespaced(client, ns);
    let mut res: Vec<String> = vec![];
    for p in pods.list(&ListParams::default()).await.unwrap() {
        println!("found pod {}", p.name());
        res.push(p.name());
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_get_cloud_namespaces() {
        let res = aw!(get_cloud_namespaces());
        assert!(res.is_ok());
        assert!(res.unwrap().len() > 0);
    }
}

// ("APIService", "apiservices"),
// ("Binding", "bindings"),
// ("CertificateSigningRequest", "certificatesigningrequests"),
// ("ClusterRole", "clusterroles"), ("ClusterRoleBinding", "clusterrolebindings"),
// ("ComponentStatus", "componentstatuses"),
// ("ConfigMap", "configmaps"),
// ("ControllerRevision", "controllerrevisions"),
// ("CronJob", "cronjobs"),
// ("CSIDriver", "csidrivers"), ("CSINode", "csinodes"), ("CSIStorageCapacity", "csistoragecapacities"),
// ("CustomResourceDefinition", "customresourcedefinitions"),
// ("DaemonSet", "daemonsets"),
// ("Deployment", "deployments"),
// ("Endpoints", "endpoints"), ("EndpointSlice", "endpointslices"),
// ("Event", "events"),
// ("FlowSchema", "flowschemas"),
// ("HorizontalPodAutoscaler", "horizontalpodautoscalers"),
// ("Ingress", "ingresses"), ("IngressClass", "ingressclasses"),
// ("Job", "jobs"),
// ("Lease", "leases"),
// ("LimitRange", "limitranges"),
// ("LocalSubjectAccessReview", "localsubjectaccessreviews"),
// ("MutatingWebhookConfiguration", "mutatingwebhookconfigurations"),
// ("Namespace", "namespaces"),
// ("NetworkPolicy", "networkpolicies"),
// ("Node", "nodes"),
// ("PersistentVolumeClaim", "persistentvolumeclaims"),
// ("PersistentVolume", "persistentvolumes"),
// ("PodDisruptionBudget", "poddisruptionbudgets"),
// ("Pod", "pods"),
// ("PodSecurityPolicy", "podsecuritypolicies"),
// ("PodTemplate", "podtemplates"),
// ("PriorityClass", "priorityclasses"),
// ("PriorityLevelConfiguration", "prioritylevelconfigurations"),
// ("ReplicaSet", "replicasets"),
// ("ReplicationController", "replicationcontrollers"),
// ("ResourceQuota", "resourcequotas"),
// ("Role", "roles"), ("RoleBinding", "rolebindings"),
// ("RuntimeClass", "runtimeclasses"),
// ("Secret", "secrets"),
// ("SelfSubjectAccessReview", "selfsubjectaccessreviews"),
// ("SelfSubjectRulesReview", "selfsubjectrulesreviews"),
// ("ServiceAccount", "serviceaccounts"),
// ("Service", "services"),
// ("StatefulSet", "statefulsets"),
// ("StorageClass", "storageclasses"), ("StorageVersion", "storageversions"),
// ("SubjectAccessReview", "subjectaccessreviews"),
// ("TokenReview", "tokenreviews"),
// ("ValidatingWebhookConfiguration", "validatingwebhookconfigurations"),
// ("VolumeAttachment", "volumeattachments"),
