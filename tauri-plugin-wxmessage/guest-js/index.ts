import { invoke } from '@tauri-apps/api/core'

export async function isEnabled(): Promise<boolean> {
  return await invoke('plugin:wxmessage|is_enabled')
}

export async function enable(args: Array<string>): Promise<void> {
  await invoke('plugin:wxmessage|enable', { args })
}

export async function disable(): Promise<void> {
  await invoke('plugin:wxmessage|disable')
}