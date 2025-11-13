<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";

  export let isOpen = false;
  export let themes: Record<string, any> = {};
  export let currentThemeId: string;

  const dispatch = createEventDispatcher();

  type TabId = "general" | "theme" | "shortcuts";
  let activeTab: TabId = "theme";

  function close() {
    isOpen = false;
    dispatch("close");
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && isOpen) {
      close();
    }
  }

  function selectTheme(id: string) {
    if (id === currentThemeId) return;
    currentThemeId = id;
    dispatch("changeTheme", { id });
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

{#if isOpen}
  <div class="settings-overlay" on:click|self={close}>
    <div class="settings-modal" on:click|stopPropagation>
      <header class="settings-header">
        <h2>Configurações</h2>

        <nav class="settings-tabs-top">
          <button
            class:active={activeTab === "general"}
            on:click={() => (activeTab = "general")}
          >
            Geral
          </button>

          <button
            class:active={activeTab === "theme"}
            on:click={() => (activeTab = "theme")}
          >
            Tema
          </button>

          <button
            class:active={activeTab === "shortcuts"}
            on:click={() => (activeTab = "shortcuts")}
          >
            Atalhos
          </button>
        </nav>

        <button class="settings-close" on:click={close} aria-label="Fechar">
          ✕
        </button>
      </header>

      <section class="settings-body">
        {#if activeTab === "general"}
          <div class="settings-section">
            <h3>Geral</h3>
            <p class="section-description">
              Ajuste preferências básicas do chatbot.
            </p>

            <!-- Só exemplos, você pode trocar tudo aqui -->
            <label class="field">
              <span>Nome do assistente</span>
              <input type="text" placeholder="Ex.: Nébula, Pixel, etc." />
            </label>

            <label class="field">
              <span>Estilo de resposta padrão</span>
              <select>
                <option>Equilibrado</option>
                <option>Resumido</option>
                <option>Detalhado</option>
                <option>Mais técnico</option>
              </select>
            </label>
          </div>
        {/if}

        {#if activeTab === "theme"}
          <div class="settings-section">
            <h3>Tema</h3>
            <p class="section-description">
              Escolha um tema visual. A prévia abaixo mostra como o chat fica.
            </p>

            <div class="theme-options-grid">
              {#each Object.entries(themes) as [id, theme]}
                <label
                  class="theme-option"
                  class:active={id === currentThemeId}
                >
                  <input
                    type="radio"
                    name="theme"
                    value={id}
                    checked={id === currentThemeId}
                    on:change={() => selectTheme(id)}
                  />

                  <div
                    class="theme-preview-card"
                    style={`--preview-bg:${theme.cssVars["--bg-surface"]};
                            --preview-muted:${theme.cssVars["--bg-surface-muted"]};
                            --preview-accent:${theme.cssVars["--accent"]};
                            --preview-user:${theme.cssVars["--chat-user-bg"]};
                            --preview-bot:${theme.cssVars["--chat-bot-bg"]};
                            --preview-text:${theme.cssVars["--text-primary"]};
                            --preview-border:${theme.cssVars["--border-subtle"]};`}
                  >
                    <div class="theme-preview-header">
                      <span class="theme-name">{theme.name}</span>
                      <span class="theme-mode">
                        {theme.mode === "dark" ? "Escuro" : "Claro"}
                      </span>
                    </div>

                    <div class="theme-preview-chat">
                      <div class="bubble bot"></div>
                      <div class="bubble user"></div>
                    </div>
                  </div>
                </label>
              {/each}
            </div>
          </div>
        {/if}

        {#if activeTab === "shortcuts"}
          <div class="settings-section">
            <h3>Atalhos de teclado</h3>
            <p class="section-description">
              Deixe o uso do chatbot mais rápido com atalhos.
            </p>

            <div class="shortcut-list">
              <div class="shortcut-row">
                <div class="shortcut-keys">
                  <span class="keycap">Ctrl</span>
                  <span class="shortcut-plus">+</span>
                  <span class="keycap">Enter</span>
                </div>
                <div class="shortcut-description">Enviar mensagem</div>
              </div>

              <div class="shortcut-row">
                <div class="shortcut-keys">
                  <span class="keycap">Shift</span>
                  <span class="shortcut-plus">+</span>
                  <span class="keycap">Enter</span>
                </div>
                <div class="shortcut-description">Quebrar linha</div>
              </div>

              <div class="shortcut-row">
                <div class="shortcut-keys">
                  <span class="keycap">Ctrl</span>
                  <span class="shortcut-plus">+</span>
                  <span class="keycap">K</span>
                </div>
                <div class="shortcut-description">Focar no input</div>
              </div>

              <div class="shortcut-row">
                <div class="shortcut-keys">
                  <span class="keycap">Ctrl</span>
                  <span class="shortcut-plus">+</span>
                  <span class="keycap">Shift</span>
                  <span class="shortcut-plus">+</span>
                  <span class="keycap">N</span>
                </div>
                <div class="shortcut-description">Nova conversa</div>
              </div>

              <div class="shortcut-row">
                <div class="shortcut-keys">
                  <span class="keycap">Ctrl</span>
                  <span class="shortcut-plus">+</span>
                  <span class="keycap">,</span>
                </div>
                <div class="shortcut-description">Abrir configurações</div>
              </div>
            </div>
          </div>
        {/if}
      </section>
    </div>
  </div>
{/if}

<style>
  .settings-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.55);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
  }

  .settings-modal {
    background: var(--bg-surface);
    color: var(--text-primary);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-strong);
    width: 60rem;
    height: 80vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    border: 1px solid var(--border-subtle);
  }

  .settings-header {
    position: relative;
    padding: 14px 18px 0 18px;
    border-bottom: 1px solid var(--border-subtle);
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .settings-header h2 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
  }

  .settings-close {
    position: absolute;
    top: 10px;
    right: 12px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 18px;
    cursor: pointer;
    border-radius: 999px;
    width: 28px;
    height: 28px;
    display: grid;
    place-items: center;
  }

  .settings-close:hover {
    background: rgba(255, 255, 255, 0.06);
  }

  /* Abas no topo */
  .settings-tabs-top {
    display: flex;
    gap: 6px;
    margin-bottom: 6px;
  }

  .settings-tabs-top button {
    flex: 0 0 auto;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
    padding: 6px 10px;
    border-radius: 999px;
    cursor: pointer;
    transition: background 0.15s ease, color 0.15s ease;
  }

  .settings-tabs-top button:hover {
    background: rgba(255, 255, 255, 0.04);
    color: var(--text-primary);
  }

  .settings-tabs-top button.active {
    background: var(--accent-soft);
    color: var(--accent-contrast);
    font-weight: 500;
  }

  .settings-body {
    padding: 16px 18px 18px 18px;
    overflow-y: auto;
  }

  .settings-section h3 {
    margin: 0 0 4px;
    font-size: 17px;
  }

  .section-description {
    margin: 0 0 16px;
    font-size: 13px;
    color: var(--text-secondary);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 14px;
    font-size: 14px;
  }

  .field span {
    color: var(--text-secondary);
  }

  .field input[type="text"],
  .field select {
    background: var(--input-bg);
    border: 1px solid var(--input-border);
    border-radius: var(--radius-md);
    padding: 8px 10px;
    color: var(--text-primary);
    font-size: 14px;
  }

  .field input[type="text"]:focus,
  .field select:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent-soft);
  }

  /* Grid de temas */
  .theme-options-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 10px;
  }

  .theme-option {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 8px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-subtle);
    background: var(--bg-surface-muted);
    cursor: pointer;
    transition: border 0.15s ease, box-shadow 0.15s ease, transform 0.08s ease;
  }

  .theme-option:hover {
    transform: translateY(-1px);
    box-shadow: var(--shadow-soft);
  }

  .theme-option.active {
    border-color: var(--accent);
  }

  .theme-option input[type="radio"] {
    margin-bottom: 4px;
    accent-color: var(--accent);
  }

  .theme-preview-card {
    border-radius: var(--radius-sm);
    padding: 8px;
    background: var(--preview-bg);
    border: 1px solid var(--preview-border);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .theme-preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 12px;
    color: var(--preview-text);
  }

  .theme-name {
    font-weight: 500;
  }

  .theme-mode {
    font-size: 11px;
    opacity: 0.8;
  }

  .theme-preview-chat {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .bubble {
    height: 12px;
    border-radius: 999px;
  }

  .bubble.bot {
    width: 70%;
    background: var(--preview-bot);
  }

  .bubble.user {
    width: 55%;
    align-self: flex-end;
    background: var(--preview-user);
  }

  /* Lista de atalhos */
  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .shortcut-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.03);
  }

  .shortcut-keys {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    align-items: center;
  }

  .keycap {
    padding: 3px 7px;
    border-radius: 5px;
    border: 1px solid var(--border-subtle);
    background: var(--bg-surface-muted);
    font-size: 11px;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
      "Liberation Mono", "Courier New", monospace;
  }

  .shortcut-plus {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .shortcut-description {
    font-size: 13px;
    color: var(--text-secondary);
    flex: 1;
    text-align: right;
  }

  @media (max-width: 600px) {
    .settings-modal {
      max-height: 90vh;
    }

    .settings-header {
      padding-top: 12px;
    }

    .settings-close {
      top: 8px;
      right: 8px;
    }
  }
</style>
