<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { applyTheme } from "./script/apply-themes";
  import Config from "./components/Config.svelte";
  import OsTab from "./components/osTab.svelte";
  import SideTab from "./components/SideTab.svelte";
  import "./style.css";

  export let data: {
    themes: Record<string, any>;
    defaultTheme: string;
  };

  const { themes, defaultTheme } = data;

  let isConfigOpen = false;
  let currentThemeId = defaultTheme;
  let windowWidth = 800;
  let windowHeight = 600;

  type Chat = { id: number; title: string };

  let chats: Chat[] = [{ id: 1, title: "Bem-vindo à IA do Tauri" }];
  let activeChatId: number | null = 1;

  let modelOptions: string[] = [
    "Modelo Turbo",
    "Modelo Padrão",
    "Modelo Barato",
  ];
  let selectedModel: string = modelOptions[0];

  function handleNewChat() {
    const id = Date.now();
    const newChat: Chat = {
      id,
      title: "Novo chat",
    };
    chats = [newChat, ...chats];
    activeChatId = id;
  }

  function handleSelectChat(id: string | number) {
    const numericId = Number(id);
    activeChatId = numericId;
  }

  function handleDeleteChat(id: string | number) {
    const numericId = Number(id);

    chats = chats.filter((c) => c.id !== numericId);
    if (activeChatId === numericId) {
      activeChatId = chats[0]?.id ?? null;
    }
  }

  function handleDeleteAllChats() {
    chats = [];
    activeChatId = null;
  }

  function handleSelectModel(model: string) {
    selectedModel = model;
  }

  onMount(() => {
    const theme = themes[defaultTheme];
    if (theme) applyTheme(theme);

    invoke("plugin:window|inner_size")
      .then((size: any) => {
        windowWidth = size.width;
        windowHeight = size.height;
      })
      .catch(console.error);
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

  async function startResize(event: MouseEvent, direction: string) {
    event.preventDefault();
    const startX = event.clientX;
    const startY = event.clientY;

    const currentSize: any = await invoke("plugin:window|inner_size");
    const currentPosition: any = await invoke("plugin:window|outer_position");

    const onMouseMove = async (moveEvent: MouseEvent) => {
      const deltaX = moveEvent.clientX - startX;
      const deltaY = moveEvent.clientY - startY;

      let newWidth = currentSize.width;
      let newHeight = currentSize.height;
      let newX = currentPosition.x;
      let newY = currentPosition.y;

      if (direction.includes("e"))
        newWidth = Math.max(300, currentSize.width + deltaX);
      if (direction.includes("w")) {
        newWidth = Math.max(300, currentSize.width - deltaX);
        newX = currentPosition.x + deltaX;
      }
      if (direction.includes("s"))
        newHeight = Math.max(200, currentSize.height + deltaY);
      if (direction.includes("n")) {
        newHeight = Math.max(200, currentSize.height - deltaY);
        newY = currentPosition.y + deltaY;
      }

      if (direction.includes("w") || direction.includes("n")) {
        await invoke("plugin:window|set_position", { x: newX, y: newY });
      }

      await invoke("plugin:window|set_size", {
        width: newWidth,
        height: newHeight,
      });
    };

    const onMouseUp = () => {
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
    };

    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
  }
</script>

<div class="app-container">
  <OsTab />

  <div class="content">
    <SideTab
      {chats}
      {activeChatId}
      onNewChat={handleNewChat}
      onSelectChat={handleSelectChat}
      onDeleteChat={handleDeleteChat}
      onDeleteAllChats={handleDeleteAllChats}
      {openConfig}
      userName="Dev Tauri"
    />

    <Config
      bind:isOpen={isConfigOpen}
      {themes}
      {currentThemeId}
      on:changeTheme={(e) => handleThemeChange(e.detail.id)}
    />

    <slot />
  </div>
</div>
