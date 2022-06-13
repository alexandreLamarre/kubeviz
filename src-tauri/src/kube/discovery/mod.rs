use k8s_openapi::api::apps::v1::{Deployment, ReplicaSet, StatefulSet};
use k8s_openapi::api::core::v1::{Namespace, Pod, Service};
use kube::{
    api::{Api, ListParams, PostParams, ResourceExt},
    Client,
};
use std::error::Error;

/// Gets all namespaces in current context
#[tauri::command]
pub async fn get_namespaces(mut cl: Option<kube::Client>) -> Result<Vec<String>, Box<dyn Error>> {
    // Read pods in the configured namespace into the typed interface from k8s-openapi
    if cl.is_none() {
        cl = Some(Client::try_default().await?);
    }
    let ns: Api<Namespace> = Api::default_namespaced(cl.unwrap());
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
