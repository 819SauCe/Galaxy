<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Database from "@tauri-apps/plugin-sql";
  import { applyTheme } from "./script/apply-themes";

  import Config from "./components/Config.svelte";
  import OsTab from "./components/osTab.svelte";
  import SideTab from "./components/SideTab.svelte";
  import Chat from "./components/Chat.svelte";

  import { loadGeneralSettings } from "./data/settingsStore";
  import type { GeneralSettings } from "./lib/config";

  // vem do +layout.ts
  export let data: {
    themes: Record<string, any>;
    defaultTheme: string;
  };

  const { themes, defaultTheme } = data;

  let isConfigOpen = false;
  let currentThemeId = defaultTheme;
  let windowWidth = 800;
  let windowHeight = 600;

  type ChatItem = { id: number; title: string };

  let chats: ChatItem[] = [];
  let activeChatId: number | null = null;

  // settings gerais usados pelo chat
  let general: GeneralSettings = {
    systemPrompt: "",
    appLanguage: "pt-BR",
    apiKeys: {
      openai: "",
      copilot: "",
      anthropic: "",
    },
    primaryAI: "openai",
    selectedModels: {
      openai: "gpt-4",
      copilot: "copilot-code-x",
      anthropic: "claude-2",
    },
  };

  type DB = Awaited<ReturnType<typeof Database.load>>;
  let db: DB | null = null;

  async function handleNewChat() {
    const id = Date.now();
    const newChat: ChatItem = {
      id,
      title: "Novo chat",
    };

    chats = [newChat, ...chats];
    activeChatId = id;

    if (db) {
      await db.execute(
        "INSERT INTO chats (id, title, created_at) VALUES ($1, $2, $3)",
        [id, newChat.title, Date.now()],
      );
    }
  }

  function handleSelectChat(id: string | number) {
    const numericId = Number(id);
    activeChatId = numericId;
  }

  async function handleDeleteChat(id: string | number) {
    const numericId = Number(id);

    chats = chats.filter((c) => c.id !== numericId);
    if (activeChatId === numericId) {
      activeChatId = chats[0]?.id ?? null;
    }

    if (db) {
      await db.execute("DELETE FROM chats WHERE id = $1", [numericId]);
    }
  }

  async function handleDeleteAllChats() {
    chats = [];
    activeChatId = null;

    if (db) {
      await db.execute("DELETE FROM chats");
    }
  }

  onMount(async () => {
    console.log("[+page] onMount");

    // aplica tema inicial
    const theme = themes[defaultTheme];
    if (theme) applyTheme(theme);

    // carrega configurações gerais salvas no SQLite
    general = await loadGeneralSettings();
    console.log("[+page] general carregado", general);

    // abre banco/local
    db = await Database.load("sqlite:app.db");

    // garante tabela dos chats
    await db.execute(`
      CREATE TABLE IF NOT EXISTS chats (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        created_at INTEGER NOT NULL
      )
    `);

    type ChatRow = { id: number; title: string; created_at: number };

    const rows = (await db.select(
      "SELECT id, title, created_at FROM chats ORDER BY created_at DESC",
    )) as ChatRow[];

    if (rows.length > 0) {
      chats = rows.map((r) => ({
        id: r.id,
        title: r.title,
      }));
      activeChatId = chats[0].id;
    } else {
      // cria um chat padrão se ainda não existir nada
      const firstId = 1;
      const firstTitle = "Novo chat";

      chats = [{ id: firstId, title: firstTitle }];
      activeChatId = firstId;

      await db.execute(
        "INSERT INTO chats (id, title, created_at) VALUES ($1, $2, $3)",
        [firstId, firstTitle, Date.now()],
      );
    }

    console.log("[+page] chats carregados", chats, "active:", activeChatId);

    // tamanho inicial da janela
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

  function handleUpdateGeneral(e: CustomEvent<GeneralSettings>) {
    general = e.detail;
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
      on:updateGeneral={handleUpdateGeneral}
    />

    <!-- AQUI é a área do chat -->
    <Chat {activeChatId} {general} />
  </div>
</div>
