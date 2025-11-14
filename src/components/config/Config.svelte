<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import { saveSetting, loadSetting } from "../../routes/lib/data/settingsStore";
  import "./Config.css";
  import type { GeneralSettings } from "../../routes/lib/config";

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

  let systemPrompt: string = "";
  let appLanguage: string = "pt-BR";

  let apiKeys: Record<string, string> = {
    openai: "",
    copilot: "",
    anthropic: ""
  };

  const aiProviders = [
    {
      id: "openai",
      name: "OpenAI",
      bullets: [
        "Excelente geração de linguagem natural e compreensão de contexto",
        "Versátil para conversas, resumos, tradução e criatividade",
        "Modelos disponíveis para maior capacidade (gpt-4) ou custo reduzido (gpt-3.5)"
      ],
      models: ["gpt-5.1-2025-11-13", "sora-2", "gpt-realtime-2025-08-28"]
    },
    {
      id: "copilot",
      name: "Copilot",
      bullets: [
        "Otimizado para assistência de programação e completions de código",
        "Útil para gerar snippets, refatoração e explicações de trechos",
        "Melhor para produtividade dev — integra bem com fluxos de IDE"
      ],
      models: ["copilot-code-x", "copilot-chat"]
    },
    {
      id: "anthropic",
      name: "Anthropic",
      bullets: [
        "Mais focado em segurança e alinhamento",
        "Bom para instruções sensíveis e completions longos"
      ],
      models: ["claude-2", "claude-instant"]
    }
  ];

  let primaryAI: string = aiProviders[0].id;

  let selectedModels: Record<string, string> = {
    openai: aiProviders[0].models[0],
    copilot: aiProviders[1].models[0],
    anthropic: aiProviders[2].models[0]
  };

  let isDesktop = true;
  let desktopTransparency = 100;
  let desktopBlur = 0;

  async function applyDesktopAppearance() {
    const root = document.documentElement;
    const alpha = Math.max(0, Math.min(1, desktopTransparency / 100));

    root.style.setProperty("--app-blur", `${desktopBlur}px`);

    const surfaceVars = [
      "--bg-body",
      "--bg-surface",
      "--bg-surface-muted",
      "--sidebar-bg",
      "--chat-bot-bg",
      "--input-bg"
    ];

    const cs = getComputedStyle(root);
    surfaceVars.forEach((v) => {
      const val = cs.getPropertyValue(v).trim();
      if (!val) return;
      if (val.startsWith("rgb")) {
        const nums = val.match(/\d+/g);
        if (nums && nums.length >= 3) {
          const r = nums[0];
          const g = nums[1];
          const b = nums[2];
          root.style.setProperty(v, `rgba(${r}, ${g}, ${b}, ${alpha})`);
        }
      } else if (val.startsWith("#")) {
        try {
          root.style.setProperty(v, hexToRgba(val, alpha));
        } catch (e) {}
      }
    });

    dispatch("changeAppearance", {
      transparency: desktopTransparency,
      blur: desktopBlur
    });

    await saveSetting("appearance", {
      desktopTransparency,
      desktopBlur
    });
  }

  async function selectTheme(id: string) {
    if (id === currentThemeId) return;
    currentThemeId = id;
    dispatch("changeTheme", { id });
    await saveSetting("theme", { themeId: id });
    if (isDesktop) applyDesktopAppearance();
  }

  async function updateGeneral() {
    const general: GeneralSettings = {
      systemPrompt,
      appLanguage,
      apiKeys,
      primaryAI,
      selectedModels
    };
    dispatch("updateGeneral", general);
    await saveSetting("general", general);
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

    (async () => {
      const general = await loadSetting("general", {
        systemPrompt,
        appLanguage,
        primaryAI,
        selectedModels,
        apiKeys
      });

      systemPrompt = general.systemPrompt;
      appLanguage = general.appLanguage;
      primaryAI = general.primaryAI;
      selectedModels = general.selectedModels;
      apiKeys = general.apiKeys;

      const appearance = await loadSetting("appearance", {
        desktopTransparency,
        desktopBlur
      });

      desktopTransparency = appearance.desktopTransparency;
      desktopBlur = appearance.desktopBlur;

      const themeSetting = await loadSetting("theme", {
        themeId: currentThemeId
      });

      if (themeSetting.themeId && themeSetting.themeId !== currentThemeId) {
        currentThemeId = themeSetting.themeId;
        dispatch("changeTheme", { id: currentThemeId });
      }

      if (isDesktop) {
        applyDesktopAppearance();
      }
    })();
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
                bind:value={systemPrompt}
                on:input={updateGeneral}
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
                    bind:value={apiKeys[k]}
                    placeholder={`Insira a chave para ${k}`}
                    on:input={updateGeneral}
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
                        bind:value={selectedModels[p.id]}
                        on:change={updateGeneral}
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
