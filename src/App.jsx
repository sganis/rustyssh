import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [number, setNumber] = useState(0);
  const [server, setServer] = useState("");
  const [user, setUser] = useState("");
  const [password, setPassword] = useState("");
  const [rememberMe, setRememberMe] = useState(true);

  async function read_settings() {
    return await invoke("read_settings");
  }

  useEffect(() => {
    (async () => {
      try {
        let settings = await read_settings();
        console.log(settings);
        setServer(settings.server);
        setUser(settings.user);
        setPassword(settings.password);
        setRememberMe(settings.remember_me);
      } catch (e) {
        console.log(`Error: ${e}`);
      }
    })();
  }, []);

  const handleSubmit = async (event) => {
    event.preventDefault();
    const settings = {
      server: server,
      user: user,
      password: password,
      remember_me: rememberMe,
    };
    await invoke("write_settings", { settings: settings });
    const n = await invoke("increment", { n: 2 });
    setNumber(n);
  };

  return (
    <div className="container">
      <h1>
        <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        &nbsp;&nbsp;Login
      </h1>

      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label>Server</label>
          <input
            type="text"
            value={server}
            className="form-control"
            id="server"
            placeholder="Enter remote ssh host name or IP address"
            onChange={(e) => setServer(e.target.value)}
          />
        </div>
        <div className="form-group">
          <label>User</label>
          <input
            type="user"
            value={user}
            className="form-control"
            id="user"
            placeholder="Enter username"
            onChange={(e) => setUser(e.target.value)}
          />
        </div>
        <div className="form-group">
          <label>Password</label>
          <input
            type="password"
            value={password}
            className="form-control"
            id="password"
            placeholder="Password"
            onChange={(e) => setPassword(e.target.value)}
          />
        </div>
        <div className="form-check">
          <input
            type="checkbox"
            value={rememberMe}
            className="form-check-input"
            id="rememberme"
            onChange={(e) => setRememberMe(Boolean(e.target.value))}
          />
          <label className="form-check-label">Remember me</label>
        </div>
        <button type="submit" className="btn btn-primary">
          Submit
        </button>
      </form>
      <p>{number}</p>
    </div>
  );
}

export default App;
