import logo from './logo.svg';
import './App.css';
import {useState, Async, useEffect, useCallback} from 'react'

import { invoke } from '@tauri-apps/api/tauri'

const invokeNamespaces = async () => {
  return invoke("get_cloud_namespaces")

} 

// Hook
const useAsync = (asyncFunction, immediate = true) => {
  const [status, setStatus] = useState("idle");
  const [value, setValue] = useState(null);
  const [error, setError] = useState(null);
  // The execute function wraps asyncFunction and
  // handles setting state for pending, value, and error.
  // useCallback ensures the below useEffect is not called
  // on every render, but only if asyncFunction changes.
  const execute = useCallback(() => {
    setStatus("pending");
    setValue(null);
    setError(null);
    return asyncFunction()
      .then((response) => {
        setValue(response);
        setStatus("success");
      })
      .catch((error) => {
        setError(error);
        setStatus("error");
      });
  }, [asyncFunction]);
  // Call execute if we want to fire it right away.
  // Otherwise execute can be called later, such as
  // in an onClick handler.
  useEffect(() => {
    if (immediate) {
      execute();
    }
  }, [execute, immediate]);
  return { execute, status, value, error };
};

const MyComponent = () => (
  <Async promiseFn={invokeNamespaces}>
    {({ data, error, isPending }) => {
      if (isPending) return "Loading..."
      if (error) return `Something went wrong: ${error.message}`
      if (data)
        return (
          <div>
            <strong>Player data:</strong>
            <pre>{JSON.stringify(data, null, 2)}</pre>
          </div>
        )
      return null
    }}
  </Async>
)

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

const Namespace = () => {
  const { execute, status, value, error } = useAsync(invokeNamespaces);
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
        {/* <NS/> */}
        <Namespace/>
        {/* <MyComponent/> */}
        {/* <button onClick={() => {invokeNamespaces()}}> Invoke namespaces </button>
        <button onClick={() => {invokePods()}}> Invoke pods </button>
        <button onClick={() => {invoke_cmd_a()}}> Invoke cmd_a </button>
        <button onClick={() => {invoke_cmd_err()}}> Invoke cmd_err </button> */}
        </header>
    </div>
  );
}

export default App;
