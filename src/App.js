import logo from './logo.svg';
import './App.css';
import {useState} from 'react'
import {useAsync} from 'react-async'

const invoke = window.__TAURI__.invoke;

async function invokeDB(){
  const value = await invoke("get_db");
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
  await invoke("namespaces")
    .then(res => {console.log(res);return res})
    .catch(err => {console.log(err)})
    .finally();
} 

function Namespace() {
  const {data, error, isLoading} = useAsync(invokeNamespaces);
  if (isLoading) return <p>Loading...</p>
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
        <button onClick={invokeNamespaces}>Invoke DB</button>
      </header>
    </div>
  );
}

export default App;
