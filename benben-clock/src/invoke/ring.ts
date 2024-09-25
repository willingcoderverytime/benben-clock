import { invoke } from "@tauri-apps/api/tauri";

export function start_music() {
  return invoke("start_music", {});
}

export function stop_music() {
  return invoke("stop_music", {});
}
