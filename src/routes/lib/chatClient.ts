import type { GeneralSettings } from "./config";

export type ChatMessage = {
  role: "system" | "user" | "assistant";
  content: string;
};

export async function sendChat(
  settings: GeneralSettings,
  history: ChatMessage[],
): Promise<string> {
  const { primaryAI } = settings;

  if (primaryAI === "openai") {
    return sendOpenAI(settings, history);
  }

  return "Ainda não implementei este provedor.";
}

async function sendOpenAI(
  settings: GeneralSettings,
  history: ChatMessage[],
): Promise<string> {
  const apiKey = settings.apiKeys["openai"];
  const model = settings.selectedModels["openai"] ?? "gpt-4o";

  if (!apiKey) {
    throw new Error("Chave da OpenAI não configurada nas Configurações.");
  }

  const messages: ChatMessage[] = [];

  messages.push({
    role: "system",
    content:
      settings.systemPrompt ||
      `Você é um assistente útil. Responda em ${settings.appLanguage}.`,
  });

  for (const m of history) {
    if (m.role === "system") continue;
    messages.push(m);
  }

  const res = await fetch("https://api.openai.com/v1/chat/completions", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${apiKey}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ model, messages }),
  });

  if (!res.ok) {
    const text = await res.text();
    console.error(text);
    throw new Error("Erro ao chamar a API da OpenAI.");
  }

  const data = await res.json();
  return data.choices?.[0]?.message?.content ?? "";
}