export const ssr = false;

import type { LayoutLoad } from './$types';
import themesJson from './data/themes.json';

export const load: LayoutLoad = async () => {
  return {
    themes: themesJson.themes,
    defaultTheme: themesJson.defaultTheme
  };
};
