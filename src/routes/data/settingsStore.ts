import Database from "@tauri-apps/plugin-sql";
import type { GeneralSettings } from "../lib/config";

let dbPromise: Promise<any> | null = null;

async function getDb() {
  if (!dbPromise) {
    dbPromise = (async () => {
      const db = await Database.load("sqlite:app.db");

      await db.execute(`
        CREATE TABLE IF NOT EXISTS settings (
          key TEXT PRIMARY KEY,
          value TEXT NOT NULL
        )
      `);

      return db;
    })();
  }
  return dbPromise;
}

export async function saveSetting(key: string, value: any) {
  const db = await getDb();

  await db.execute(
    `INSERT INTO settings (key, value)
     VALUES ($1, $2)
     ON CONFLICT(key) DO UPDATE SET value = excluded.value`,
    [key, JSON.stringify(value)],
  );
}

type SettingRow = { value: string };

export async function loadSetting<T>(key: string, fallback: T): Promise<T> {
  const db = await getDb();
  const rows = (await db.select(
    "SELECT value FROM settings WHERE key = $1",
    [key],
  )) as SettingRow[];

  if (!rows.length) return fallback;

  try {
    return JSON.parse(rows[0].value) as T;
  } catch {
    return fallback;
  }
}

// helper específico
export async function loadGeneralSettings(): Promise<GeneralSettings> {
  const fallback: GeneralSettings = {
    systemPrompt: "",
    appLanguage: "pt-BR",
    apiKeys: { openai: "", copilot: "", anthropic: "" },
    primaryAI: "openai",
    selectedModels: {
      openai: "gpt-4",
      copilot: "copilot-code-x",
      anthropic: "claude-2",
    },
  };

  return loadSetting<GeneralSettings>("general", fallback);
}