import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
import { check } from "@tauri-apps/plugin-updater";
function App() {
  useEffect(() => {
    let unlisten: () => void;

    (async () => {
      unlisten = await listen("capture", (e) => {
        console.log("start? ....");
        console.log(e.payload);
        let data = e.payload as any;
      });
    })();

    initOnApp();

    return () => unlisten?.();
  }, []);

  async function initOnApp() {
    await onOpenUrl((urls) => {
      console.log("deep link:", urls);
    });
  }

  async function hanlderSend() {
    invoke("send_text", { txt: "test...." });
  }
  async function hanlderStart() {
    invoke("window_start");
  }
  async function hanlderStop() {
    invoke("window_stop");
  }
  async function hanlderWins() {
    invoke("get_win_all");
  }
  async function hanlderUpdate() {
    const update = await check();
    if (update) {
      console.log(
        `found update ${update.version} from ${update.date} with notes ${update.body}`
      );
      let downloaded = 0;
      let contentLength = 0;
      // alternatively we could also call update.download() and update.install() separately
      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            contentLength = event.data.contentLength || 0;
            console.log(
              `started downloading ${event.data.contentLength} bytes`
            );
            break;
          case "Progress":
            downloaded += event.data.chunkLength;
            console.log(`downloaded ${downloaded} from ${contentLength}`);
            break;
          case "Finished":
            console.log("download finished");
            break;
        }
      });

      console.log("update installed");
      // await relaunch();
    } 
  }
  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>
      <input type="text" onFocus={hanlderSend} />

      <button onClick={hanlderSend}>test</button>
      <button onClick={hanlderWins}>windows</button>
      <button onClick={hanlderStart}>start</button>
      <button onClick={hanlderStop}>stop</button>
      <button onClick={hanlderUpdate}>update</button>
    </main>
  );
}

export default App;
