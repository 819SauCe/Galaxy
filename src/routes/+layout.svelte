<script lang="ts">
  import { onMount } from "svelte";
  import { applyTheme } from "./script/apply-themes";
  import Config from "./components/Config.svelte";
  import "./style.css"

  export let data: {
    themes: Record<string, any>;
    defaultTheme: string;
  };

  const { themes, defaultTheme } = data;

  let isConfigOpen = false;
  let currentThemeId = defaultTheme;

  onMount(() => {
    const theme = themes[defaultTheme];
    if (theme) applyTheme(theme);
  });

  function openConfig() {
    isConfigOpen = true;
  }

  function handleThemeChange(id: string) {
    const theme = themes[id];
    if (!theme) return;
    currentThemeId = id;
    applyTheme(theme);
  }
</script>

<!-- Exemplo: botão na sidebar ou no header -->
<button on:click={openConfig}>
  ⚙️ Configurações
</button>

<Config
  bind:isOpen={isConfigOpen}
  {themes}
  {currentThemeId}
  on:changeTheme={(e) => handleThemeChange(e.detail.id)}
/>
