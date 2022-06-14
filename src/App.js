import logo from './logo.svg';
import './App.css';
import {useState} from 'react'
import {useAsync} from 'react-async'

import { invoke } from '@tauri-apps/api/tauri'

const invokeNamespaces = async () => {
  console.log(invoke)
  await invoke("get_cloud_namespaces", {cl: null})
    .then(res => {alert(res);return res})
    .catch(err => {alert(err)});
} 

function Namespace() {
  const {data, error, isLoading} = useAsync(invokeNamespaces());
  if (isLoading) return <p>Loading...</p>
  if (error) return <p>Error: {error.message}</p>
  if(data) {
    const k = data.split("\n");
    const namespaces = k.map((item, index) => {
      <li> {item} </li>
    });
    return <ul> {namespaces} </ul>
  }
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
        <button onClick={() => {alert(invokeNamespaces())}}> Invoke namespaces </button>
        </header>
    </div>
  );
}

export default App;
