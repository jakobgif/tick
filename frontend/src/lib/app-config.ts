// Jakob Frenzel
// 05/01/25
// configuration of the app

export type AppConfig = {
  backendUrl: string
}

export function loadAppConfig(): AppConfig {
  return {
    backendUrl: "https://tick.jakobfrenzel.com",
  }
}

export function saveAppConfig(_config: AppConfig): void {
  
}