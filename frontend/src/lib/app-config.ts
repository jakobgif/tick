// Jakob Frenzel
// 05/01/25
// configuration of the app

import { appConfigDir, join } from "@tauri-apps/api/path"
import { readTextFile, writeTextFile, exists, mkdir } from "@tauri-apps/plugin-fs"

export type AppConfig = {
  backendUrl: string
}

const DEFAULT_CONFIG: AppConfig = {
  backendUrl: "https://tick.example.local",
}

const CONFIG_FILE = "tick-config.json"

async function getConfigPath(): Promise<string> {
  const dir = await appConfigDir()
  await mkdir(dir, { recursive: true })
  return join(dir, CONFIG_FILE)
}

export async function loadAppConfig(): Promise<AppConfig> {
  try {
    const path = await getConfigPath()
    const fileExists = await exists(path)
    if (fileExists) {
      const data = await readTextFile(path)
      const loaded = JSON.parse(data) as Partial<AppConfig>
      const merged = { ...DEFAULT_CONFIG, ...loaded }
      return merged
    }
    return { ...DEFAULT_CONFIG }
  } catch (err) {
    console.error("[config] Error loading config:", err)
    return { ...DEFAULT_CONFIG }
  }
}

export async function saveAppConfig(config: AppConfig): Promise<void> {
  try {
    const path = await getConfigPath()
    const data = JSON.stringify(config, null, 2)
    await writeTextFile(path, data)
  } catch (err) {
    console.error("[config] Error saving config:", err)
  }
}