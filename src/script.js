// When using the Tauri API npm package:
//import { invoke } from "@tauri-apps/api/tauri";
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.invoke;

// Invoke the command
invoke("my_custom_command");
