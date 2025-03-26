import { openUrl } from "@tauri-apps/plugin-opener";
import { invoke } from "@tauri-apps/api/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { check } from "@tauri-apps/plugin-updater";
export { exit, relaunch } from "@tauri-apps/plugin-process";

export const checkUpdate = check;

export async function openBrowser(url: string) {
  return await openUrl(url);
}

export async function appStart(): Promise<boolean> {
  return await invoke("background_task_start");
}

export async function appStop(): Promise<boolean> {
  return await invoke("background_task_stop");
}

export async function getWindows() {
  return await invoke("get_window_all");
}

export async function windowCaptureStop(): Promise<boolean> {
  return await invoke("window_stop");
}

export async function windowCaptureStart(): Promise<boolean> {
  return await invoke("window_start");
}

export async function getSessionUserId(): Promise<string> {
  return await invoke("get_session_id");
}

export async function setSessionUserId(sessionId: string | null) {
  return await invoke("set_session_id", { sessionId });
}

export async function hideWindow(label?: string) {
  let window = label
    ? await WebviewWindow.getByLabel(label)
    : WebviewWindow.getCurrent();
  await window?.hide();
}

export async function showWindow(label?: string) {
  let window = label
    ? await WebviewWindow.getByLabel(label)
    : WebviewWindow.getCurrent();
  await window?.show();
}

export async function isVisibleWindow(label?: string) {
  let window = label
    ? await WebviewWindow.getByLabel(label)
    : WebviewWindow.getCurrent();
  return await window?.isVisible();
}
