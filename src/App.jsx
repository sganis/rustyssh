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
  const [remember_me, setRemember_me] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        let settings = await invoke("read_settings");
        console.log(settings);
        setServer(settings.server);
        setUser(settings.user);
        if (settings.remember_me) {
          setPassword(settings.password);
          setRemember_me(true);
        } else {
          setPassword("");
          setRemember_me(false);
        }
      } catch (e) {
        console.log(`Error: ${e}`);
      }
    })();
  }, []);

  function sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  const handleSubmit = async (event) => {
    event.preventDefault();
    setIsConnecting(true);
    //await sleep(1000);
    const settings = {
      server: server,
      user: user,
      password: remember_me ? password : "",
      port: 22,
      remember_me: remember_me,
    };
    console.log(settings);
    try {
      const r = await invoke("connect", { settings: settings });
      setIsConnected(true);
      await getFiles("/");
    } catch (e) {
      console.log(e);
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
        &nbsp;SSH File Browser
      </h1>
      {!isConnected && (
        <Login
          isConnecting={isConnecting}
          handleSubmit={handleSubmit}
          server={server}
          setServer={setServer}
          user={user}
          setUser={setUser}
          password={password}
          setPassword={setPassword}
          remember_me={remember_me}
          setRemember_me={setRemember_me}
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
    </div>
  );
}

export default App;
