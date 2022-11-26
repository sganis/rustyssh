import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Files from "./Files";
import Login from "./Login";

function App() {
  const [files, setFiles] = useState({});
  const [currentPath, setCurrentPath] = useState("");
  const [isConnecting, setIsConnecting] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [server, setServer] = useState("");
  const [user, setUser] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [message, setMessage] = useState("");
  const [needPassword, setNeedPassword] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        setMessage("Connecting...");
        setIsConnecting(true);
        let settings = await invoke("read_settings");
        setServer(settings.server);
        setUser(settings.user);
        try {
          console.log(settings);
          await invoke("connect_with_key", { settings });
          setIsConnected(true);
          setMessage("");
          await getFiles("/");
        } catch (e) {
          setNeedPassword(true);
          setIsConnecting(false);
          setError(e);
        }
      } catch (e) {
        setError(`Error: ${e}`);
      }
    })();
  }, []);

  function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  const handleSubmit = async (event) => {
    event.preventDefault();
    setIsConnecting(true);
    setError("");
    //await sleep(1000);
    const settings = {
      server: server,
      user: user,
      password: password,
      port: 22,
    };
    console.log(settings);
    try {
      await invoke("connect", { settings: settings });
      setIsConnected(true);
      await getFiles("/");
    } catch (e) {
      console.log(e);
      setError(e);
    }
    setIsConnecting(false);
  };

  const getFiles = async (path) => {
    try {
      console.log("listing:" + path);
      const r = await invoke("get_files", { path });
      //console.log(r);
      const jsfiles = JSON.parse(r);
      setFiles(jsfiles);
      setCurrentPath(path);
      console.log(files);
    } catch (e) {
      console.log("error");
    }
  };

  const goUp = async (path) => {
    console.log(`go up: ${path}`);
    const index = path.lastIndexOf("/") == 0 ? 1 : path.lastIndexOf("/");
    const parent = path.substring(0, index);
    console.log(parent);
    await getFiles(parent);
  };

  const logout = async () => {
    try {
      const r = await invoke("disconnect");
      setIsConnected(false);
      console.log("disconnected");
    } catch (e) {
      console.log(e);
    }
  };

  return (
    <div className="container-sm">
      <h1>
        <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        &nbsp;Rusty SSH
      </h1>
      {needPassword && !isConnected && (
        <Login
          isConnecting={isConnecting}
          handleSubmit={handleSubmit}
          server={server}
          setServer={setServer}
          user={user}
          setUser={setUser}
          password={password}
          setPassword={setPassword}
          error={error}
        />
      )}
      {isConnected && (
        <div>
          <p className="float-end">
            <a href="#" onClick={logout}>
              Logout
            </a>
          </p>
          <Files
            files={files}
            handleRowClick={getFiles}
            goUp={goUp}
            currentPath={currentPath}
          />
        </div>
      )}
      <br />
      <div>
        {!error && message && <p className="alert alert-info">{message}</p>}
        {error && <p className="alert alert-warning">{error}</p>}
      </div>
    </div>
  );
}

export default App;
