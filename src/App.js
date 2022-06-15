import logo from './logo.svg';
import './App.css';
import {useState} from 'react'
import {useAsync} from 'react-async'

import { invoke } from '@tauri-apps/api/tauri'

const invokeNamespaces = async () => {
  invoke("get_cloud_namespaces")
    .then(res => {return res;})
    .catch(err => {alert(err)});

} 

const invokePods = async () => {
  await invoke("get_pods", {"ns" : "default"})
    .then((res) => {res.map((obj) => {
      alert(obj)
    });})
    .catch(err => {alert(err)})
    .finally(() => {
      alert("done")
    });

}

const invoke_cmd_a = async() => {
  invoke("cmd_a")
    .then((res) => {alert(res);return res;})
    .catch((err) => {alert(err)})
    .finally(() => 
    {
      alert("done")
    })
}

const invoke_cmd_err = async() => {
  await invoke("cmd_err")
    .then((res) => {;return res;})
    .catch((err) => {alert("Error : ", err)})
    .finally(() => {

    })
}

function Namespace() {
  const { execute, status, value, error } = useAsync(invokeNamespaces, false);
  return (
    <div>
      {status === "idle" && <div>Start your journey by clicking a button</div>}
      {status === "success" && <div>{value}</div>}
      {status === "error" && <div>{error}</div>}
      <button onClick={execute} disabled={status === "pending"}>
        {status !== "pending" ? "Click me" : "Loading..."}
      </button>
    </div>
  );
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        {/* <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a> */}
        <Namespace/>
        <button onClick={() => {invokeNamespaces()}}> Invoke namespaces </button>
        <button onClick={() => {invokePods()}}> Invoke pods </button>
        <button onClick={() => {invoke_cmd_a()}}> Invoke cmd_a </button>
        <button onClick={() => {invoke_cmd_err()}}> Invoke cmd_err </button>
        </header>
    </div>
  );
}

export default App;
