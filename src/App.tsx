import { useState } from "react";
import { invoke } from "@tauri-apps/api/primitives";
import "./App.css";

function App() {
  const [data, setData] = useState<any>();
  const [name, setName] = useState("");

  async function greet() {
    setData(await invoke("query", { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <form
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <pre>{JSON.stringify(data, null, 2)}</pre>
    </div>
  );
}

export default App;
