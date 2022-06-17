import logo from './logo.svg';
import './App.css';
import {useAsync} from './hooks'
import {invokeNamespaces} from './tauri'

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
        <Namespace/>
        </header>
    </div>
  );
}

export default App;
