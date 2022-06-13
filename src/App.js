import logo from './logo.svg';
import './App.css';
import {useState} from 'react'
import {useAsync} from 'react-async'

import { invoke } from '@tauri-apps/api/tauri'

async function invokeDB(){
  const value = await invoke("ctx::get_db");
  alert(value);
}

async function incrementDB() {
  const value = await invoke("increment_db");
}

async function kubeNamespace() {
  const value = await invoke("namespaces")
  return value;
}

const loadPlayer = async ({ playerId }, { signal }) => {
  const res = await fetch(`/api/players/${playerId}`, { signal })
  if (!res.ok) throw new Error(res.statusText)
  return res.json()
}

const invokeNamespaces = async () => {
  console.log(invoke)
  await invoke("kube::discovery::namespaces", {cl: null})
    .then(res => {alert(res);return res})
    .catch(err => {alert(err)})
    .finally(alert("finally"));
} 

function Namespace() {
  const {data, error, isLoading} = useAsync(invokeNamespaces());
  if (isLoading) return <p>Loading...</p>
  if (error) return <p>Error: {error.message}</p>
  if(data)
    return <p>{data}</p>
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <Namespace/>
        <p> Hello </p>
        <button onClick={() => {invokeDB()}}>Invoke DB</button>
        <button onClick={() => {alert("button")}}> test </button>
        <button onClick={() => {invokeNamespaces()}}>Increment DB</button>
        </header>
    </div>
  );
}

export default App;
