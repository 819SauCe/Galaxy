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

  function handleOverlayKeydown(e: KeyboardEvent) {
    if ((e.key === "Enter" || e.key === " ") && isOpen) {
      close();
    }
  }

  function handleModalKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" || e.key === " ") {
      e.stopPropagation();
    }
  }

  function selectTheme(id: string) {
    if (id === currentThemeId) return;
    currentThemeId = id;
    dispatch("changeTheme", { id });
    if (isDesktop) applyDesktopAppearance();
  }

  let systemPrompt: string = "";
  let appLanguage: string = "pt-BR";

  let apiKeys: Record<string, string> = {
    openai: "",
    copilot: "",
    anthropic: "",
  };

  const aiProviders = [
    {
      id: "openai",
      name: "OpenAI",
      bullets: [
        "Excelente geração de linguagem natural e compreensão de contexto",
        "Versátil para conversas, resumos, tradução e criatividade",
        "Modelos disponíveis para maior capacidade (gpt-4) ou custo reduzido (gpt-3.5)",
      ],
      models: ["gpt-4", "gpt-4o", "gpt-3.5-turbo"],
    },
    {
      id: "copilot",
      name: "Copilot",
      bullets: [
        "Otimizado para assistência de programação e completions de código",
        "Útil para gerar snippets, refatoração e explicações de trechos",
        "Melhor para produtividade dev — integra bem com fluxos de IDE",
      ],
      models: ["copilot-code-x", "copilot-chat"],
    },
    {
      id: "anthropic",
      name: "Anthropic",
      bullets: [
        "Mais focado em segurança e alinhamento",
        "Bom para instruções sensíveis e completions longos",
      ],
      models: ["claude-2", "claude-instant"],
    },
  ];

  let primaryAI: string = aiProviders[0].id;

  let selectedModels: Record<string, string> = {
    openai: aiProviders[0].models[0],
    copilot: aiProviders[1].models[0],
    anthropic: aiProviders[2].models[0],
  };

  let isDesktop = true;
  let desktopTransparency = 100;
  let desktopBlur = 0;

  function hexToRgba(hex: string, a: number) {
    let h = hex.replace("#", "").trim();
    if (!h) return hex;
    if (h.length === 3) {
      h = h
        .split("")
        .map((c) => c + c)
        .join("");
    }
    const r = parseInt(h.substring(0, 2), 16);
    const g = parseInt(h.substring(2, 4), 16);
    const b = parseInt(h.substring(4, 6), 16);
    return `rgba(${r}, ${g}, ${b}, ${a})`;
  }

  function applyDesktopAppearance() {
    const root = document.documentElement;
    const alpha = Math.max(0, Math.min(1, desktopTransparency / 100));

    root.style.setProperty("--app-blur", `${desktopBlur}px`);

    const surfaceVars = [
      "--bg-body",
      "--bg-surface",
      "--bg-surface-muted",
      "--sidebar-bg",
      "--chat-bot-bg",
      "--input-bg",
    ];

    const cs = getComputedStyle(root);
    surfaceVars.forEach((v) => {
      const val = cs.getPropertyValue(v).trim();
      if (!val) return;
      if (val.startsWith("rgb")) {
        const nums = val.match(/\d+/g);
        if (nums && nums.length >= 3) {
          const r = nums[0],
            g = nums[1],
            b = nums[2];
          root.style.setProperty(v, `rgba(${r}, ${g}, ${b}, ${alpha})`);
        }
      } else if (val.startsWith("#")) {
        try {
          root.style.setProperty(v, hexToRgba(val, alpha));
        } catch (e) {
        }
      }
    });

    dispatch("changeAppearance", {
      transparency: desktopTransparency,
      blur: desktopBlur,
    });
  }

  function updateGeneral() {
    dispatch("updateGeneral", {
      systemPrompt,
      appLanguage,
      apiKeys,
      primaryAI,
      selectedModels,
    });
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
    try {
      isDesktop =
        !/Mobi|Android|iPhone|iPad/.test(navigator.userAgent) &&
        window.matchMedia &&
        window.matchMedia("(pointer: fine)").matches;
    } catch (e) {
      isDesktop = true;
    }
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

{#if isOpen}
  <div
    class="settings-overlay"
    on:click|self={close}
    on:keydown={handleOverlayKeydown}
    role="button"
    tabindex="0"
    aria-label="Fechar configurações"
  >
    <div
      class="settings-modal"
      on:click|stopPropagation
      on:keydown={handleModalKeydown}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
    >
      <header class="settings-header drag-region">
        <h2>Configurações</h2>

        <nav class="settings-tabs-top no-drag">
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

        <button
          class="settings-close no-drag"
          on:click={close}
          aria-label="Fechar"
        >
          ✕
        </button>
      </header>

      <section class="settings-body">
        {#if activeTab === "general"}
          <div class="settings-section">
            <h3>Geral</h3>
            <p class="section-description">
              Ajuste preferências básicas do chatbot: personalidade, idioma,
              chaves de API, modelos e IA principal.
            </p>

            <label class="field">
              <span>System prompt (personalidade da IA)</span>
              <textarea
                rows="4"
                placeholder="Descreva a personalidade, tom e instruções iniciais para a IA..."
                on:input={(e) => {
                  systemPrompt = (e.target as HTMLTextAreaElement).value;
                  updateGeneral();
                }}
              ></textarea>
            </label>

            <label class="field">
              <span>Idioma do app</span>
              <select bind:value={appLanguage} on:change={updateGeneral}>
                <option value="pt-BR">Português (pt-BR)</option>
                <option value="en-US">English (en-US)</option>
                <option value="es-ES">Español (es-ES)</option>
              </select>
            </label>

            <div class="settings-subsection">
              <h4>Chaves de API</h4>
              <p class="section-description">
                Insira as chaves para cada provedor (ex.: OpenAI, Copilot,
                Afims).
              </p>

              {#each Object.keys(apiKeys) as k}
                <label class="field">
                  <span>{k} key</span>
                  <input
                    type="text"
                    value={apiKeys[k]}
                    placeholder={`Insira a chave para ${k}`}
                    on:input={(e) => {
                      apiKeys[k] = (e.target as HTMLInputElement).value;
                      updateGeneral();
                    }}
                  />
                </label>
              {/each}
            </div>

            <div class="settings-subsection">
              <h4>Modelos e IA principal</h4>
              <p class="section-description">
                Escolha o modelo por provedor e selecione qual IA será a
                principal (rádio).
              </p>

              <div class="ai-providers-list">
                {#each aiProviders as p}
                  <div
                    class="ai-provider-row"
                    role="group"
                    aria-label={`Config ${p.name}`}
                  >
                    <div class="ai-provider-header">
                      <label class="ai-provider-radio">
                        <input
                          type="radio"
                          name="primaryAI"
                          value={p.id}
                          bind:group={primaryAI}
                          on:change={updateGeneral}
                        />
                        <span class="ai-provider-name">{p.name}</span>
                      </label>
                    </div>

                    <div class="ai-provider-desc">
                      <ul class="ai-bullets">
                        {#each p.bullets as b}
                          <li>{b}</li>
                        {/each}
                      </ul>
                    </div>

                    <label class="field">
                      <span>Modelo ({p.name})</span>
                      <select
                        value={selectedModels[p.id]}
                        on:change={(e) => {
                          selectedModels[p.id] = (
                            e.target as HTMLSelectElement
                          ).value;
                          updateGeneral();
                        }}
                      >
                        {#each p.models as m}
                          <option value={m}>{m}</option>
                        {/each}
                      </select>
                    </label>
                  </div>
                {/each}
              </div>
            </div>
          </div>
        {/if}

        {#if activeTab === "theme"}
          <div class="settings-section">
            <h3>Tema</h3>

            {#if isDesktop}
              <div class="settings-subsection">
                <h4>Aparência (Desktop)</h4>
                <p class="section-description">
                  Ajuste a transparência e o desfoque do app desktop (apenas em
                  PCs).
                </p>

                <label class="field">
                  <span>Transparência da janela: {desktopTransparency}%</span>
                  <input
                    type="range"
                    min="20"
                    max="100"
                    step="1"
                    bind:value={desktopTransparency}
                    on:input={() => applyDesktopAppearance()}
                  />
                </label>

                <label class="field">
                  <span>Blur (backdrop): {desktopBlur}px</span>
                  <input
                    type="range"
                    min="0"
                    max="30"
                    step="1"
                    bind:value={desktopBlur}
                    on:input={() => applyDesktopAppearance()}
                  />
                </label>
              </div>
            {/if}

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
    width: min(92vw, 60rem);
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
    transition:
      background 0.15s ease,
      color 0.15s ease;
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

  .field textarea {
    background: var(--input-bg);
    border: 1px solid var(--input-border);
    border-radius: var(--radius-md);
    padding: 8px 10px;
    color: var(--text-primary);
    font-size: 14px;
    resize: none;
    min-height: 88px;
    max-height: 220px;
    box-sizing: border-box;
  }

  .field input[type="text"]:focus,
  .field select:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent-soft);
  }

  .field textarea:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent-soft);
  }

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
    transition:
      border 0.15s ease,
      box-shadow 0.15s ease,
      transform 0.08s ease;
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

  .ai-providers-list {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 12px;
    margin-top: 8px;
  }

  .ai-provider-row {
    background: var(--bg-surface-muted);
    border: 1px solid var(--border-subtle);
    padding: 14px;
    border-radius: var(--radius-md);
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-items: stretch;
    justify-content: space-between;
    min-height: 160px;
  }

  .ai-provider-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .ai-provider-radio {
    display: flex;
    gap: 8px;
    align-items: center;
    cursor: pointer;
  }

  .ai-provider-name {
    font-weight: 600;
  }

  .ai-bullets {
    margin: 0;
    padding-left: 18px;
    color: var(--text-secondary);
    font-size: 13px;
    line-height: 1.3;
  }

  .ai-provider-desc {
    flex: 1 1 auto;
    display: flex;
    align-items: flex-start;
  }

  .ai-provider-desc .ai-bullets {
    margin-top: 2px;
  }

  .ai-provider-row .field select {
    width: 100%;
    box-sizing: border-box;
  }

  .ai-provider-row .field {
    margin-top: 6px;
  }

  .ai-provider-radio input[type="radio"] {
    width: 18px;
    height: 18px;
  }

  @media (max-width: 600px) {
    .settings-modal {
      width: calc(100vw - 16px);
      max-width: 100%;
      max-height: 100%;
      max-height: 100%;
      border-radius: 12px;
      margin: 8px;
      padding-bottom: calc(12px + env(safe-area-inset-bottom));
      padding-left: calc(12px + env(safe-area-inset-left));
      padding-right: calc(12px + env(safe-area-inset-right));
    }

    .settings-header {
      padding-top: 12px;
      gap: 8px;
    }

    .settings-close {
      top: 10px;
      right: 10px;
    }

    .ai-providers-list {
      grid-template-columns: 1fr;
      gap: 10px;
    }

    .ai-provider-row {
      min-height: 140px;
      padding: 12px;
    }

    .ai-provider-radio input[type="radio"] {
      width: 22px;
      height: 22px;
    }

    .ai-provider-name {
      font-size: 16px;
    }

    .ai-provider-row .field select {
      font-size: 15px;
      padding: 10px;
    }

    .field input[type="text"],
    .field select,
    textarea {
      font-size: 15px;
      padding: 10px;
    }
  }
</style>
