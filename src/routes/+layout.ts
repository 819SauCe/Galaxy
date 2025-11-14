export const ssr = false;

import type { LayoutLoad } from './$types';
import themesJson from './lib/data/themes.json';

export const load: LayoutLoad = async () => {
  return {
    themes: themesJson.themes,
    defaultTheme: themesJson.defaultTheme
  };
};

window.addEventListener("contextmenu", (event) => {
  event.preventDefault();
});

window.addEventListener("keydown", (e) => {
  if (e.key === "F12") {
    e.preventDefault();
  }
  if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "i") {
    e.preventDefault();
  }
  if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "c") {
    e.preventDefault();
  }
  if (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "j") {
    e.preventDefault();
  }
});
