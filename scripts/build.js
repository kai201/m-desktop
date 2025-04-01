import path from "node:path";
import fs from "node:fs";
import { mergeConfig } from "vite";

let configName = path.join("src-tauri/tauri.conf.json");
let configStr = fs.readFileSync(configName, "utf8");

const config = JSON.parse(configStr);
const env = process.env.NODE_ENV || "development";
const version = process.env.VERSION || "0.1.0";

let opts = {};

opts = {
  version: version,
  bundle: {
    windows: {
      webviewInstallMode: {
        type: "embedBootstrapper",
      },
      nsis: {
        minimumWebview2Version: "110.0.1531.0",
      },
    },
  },
  plugins: {},
};

if (env == "pro") {
  config.plugins.updater.endpoints = null;
  opts.plugins = {
    updater: {
      endpoints: [
        "https://whelp.ares-ai.cn/sys/version/{{target}}/{{arch}}/{{current_version}}",
      ],
    },
  };
}

opts = mergeConfig(config, opts);

const stable = path.join("src-tauri/tauri.conf.stable.json");
const standard = path.join("src-tauri/tauri.conf.standard.json");
fs.writeFileSync(standard, JSON.stringify(opts, null, "\t"));
opts.bundle.windows.webviewInstallMode = {
  type: "fixedRuntime",
  path: "./Microsoft.WebView2.FixedVersionRuntime.134.0.3124.93.x64/",
};
fs.writeFileSync(stable, JSON.stringify(opts, null, "\t"));
