import { invoke } from '@tauri-apps/api/tauri'

const invokeNamespaces = async () => {
    return invoke("get_cloud_namespaces")
} 


export {invokeNamespaces}