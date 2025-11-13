export type GeneralSettings = {
  systemPrompt: string;
  appLanguage: string;
  apiKeys: Record<string, string>;
  primaryAI: string;
  selectedModels: Record<string, string>;
};