<script lang="ts">
  import { onMount } from "svelte";
  import { applyTheme } from "./script/apply-themes";
  import Config from "./components/Config.svelte";

  export let data: {
    themes: Record<string, any>;
    defaultTheme: string;
  };

  const { themes, defaultTheme } = data;

  onMount(() => {
    const theme = themes[defaultTheme];
    if (theme) applyTheme(theme);
  });

  function changeTheme(e: Event) {
    const name = (e.target as HTMLSelectElement).value;
    const theme = themes[name];
    if (theme) applyTheme(theme);
  }
</script>


<select on:change={changeTheme}>
  {#each Object.keys(themes) as name}
    <option value={name}>{themes[name].name}</option>
  {/each}
</select>

<Config />