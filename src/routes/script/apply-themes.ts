export function applyTheme(theme: any) {
  if (!theme || !theme.cssVars) return;

  const root = document.documentElement;

  Object.entries(theme.cssVars).forEach(([key, value]) => {
    root.style.setProperty(key, value as string);
  });
}
