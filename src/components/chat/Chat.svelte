<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import "./Chat.css";
  import type { GeneralSettings } from "../../routes/lib/config";
  import { invoke } from "@tauri-apps/api/core";
  import Database from "@tauri-apps/plugin-sql";
  import { renderMarkdown } from "../../routes/lib/markdown";

  type ImageAttachment = {
    id: string;
    name: string;
    size: number;
    url: string;
  };
  type FileAttachment = {
    id: string;
    name: string;
    size: number;
    type: string;
  };
  type AudioAttachment = { blob: Blob; url: string; duration: number };
  type Attachments = {
    images: ImageAttachment[];
    files: FileAttachment[];
    audio: AudioAttachment | null;
  };

  type ChatMessage = {
    id: number;
    chatId: number;
    role: "user" | "assistant";
    content: string;
    createdAt: number;
  };

  export let general: GeneralSettings = {
    systemPrompt: "",
    appLanguage: "pt-BR",
    apiKeys: {
      openai: "",
      copilot: "",
      anthropic: "",
    },
    primaryAI: "openai",
    selectedModels: {
      openai: "",
      copilot: "",
      anthropic: "",
    },
  };

  export let activeChatId: number | null = null;

  type DB = Awaited<ReturnType<typeof Database.load>>;
  let db: DB | null = null;

  let messages: ChatMessage[] = [];

  let text = "";
  let attachments: Attachments = {
    images: [],
    files: [],
    audio: null,
  };

  let recording = false;
  let mediaRecorder: MediaRecorder | null = null;
  let audioChunks: Blob[] = [];
  let audioSeconds = 0;
  let audioTimer: number | null = null;

  let textareaEl: HTMLTextAreaElement | null = null;
  let imageInputEl: HTMLInputElement | null = null;
  let fileInputEl: HTMLInputElement | null = null;

  let isThinking = false;

  $: charCount = text.length;
  $: tokensFillWidth = 20 + Math.min(1, charCount / 4000) * 60;

  async function initDb() {
    if (db) return;
    db = await Database.load("sqlite:app.db");
    await db.execute(`
      CREATE TABLE IF NOT EXISTS messages (
        id INTEGER PRIMARY KEY,
        chat_id INTEGER NOT NULL,
        role TEXT NOT NULL,
        content TEXT NOT NULL,
        created_at INTEGER NOT NULL
      )
    `);
  }

  async function loadMessagesForChat(chatId: number) {
    if (!db) return;
    type Row = {
      id: number;
      chat_id: number;
      role: string;
      content: string;
      created_at: number;
    };
    const rows = (await db.select(
      "SELECT id, chat_id, role, content, created_at FROM messages WHERE chat_id = $1 ORDER BY created_at ASC",
      [chatId],
    )) as Row[];
    messages = rows.map((r) => ({
      id: r.id,
      chatId: r.chat_id,
      role: r.role === "user" ? "user" : "assistant",
      content: r.content,
      createdAt: r.created_at,
    }));
  }

  let currentLoadedChatId: number | null = null;

  $: if (db && activeChatId !== null && activeChatId !== currentLoadedChatId) {
    currentLoadedChatId = activeChatId;
    loadMessagesForChat(activeChatId);
  }

  onMount(async () => {
    await initDb();
    if (activeChatId !== null) {
      currentLoadedChatId = activeChatId;
      await loadMessagesForChat(activeChatId);
    }
  });

  function autoResize() {
    if (!textareaEl) return;
    textareaEl.style.height = "auto";
    const maxHeight = 240;
    textareaEl.style.height =
      Math.min(textareaEl.scrollHeight, maxHeight) + "px";
  }

  function tickThen(cb: () => void) {
    requestAnimationFrame(() => requestAnimationFrame(cb));
  }

  function openImagePicker() {
    if (isThinking) return;
    imageInputEl?.click();
  }

  function handleImageChange(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const files = Array.from(input.files || []);
    files.forEach((file) => {
      if (!file.type.startsWith("image/")) return;
      const id = `${Date.now()}-${Math.random().toString(16).slice(2)}`;
      const reader = new FileReader();
      reader.onload = (e) => {
        const url = (e.target?.result ?? "") as string;
        attachments = {
          ...attachments,
          images: [
            ...attachments.images,
            {
              id,
              name: file.name,
              size: file.size,
              url,
            },
          ],
        };
      };
      reader.readAsDataURL(file);
    });
    input.value = "";
  }

  function removeImage(id: string) {
    attachments = {
      ...attachments,
      images: attachments.images.filter((img) => img.id !== id),
    };
  }

  function openFilePicker() {
    if (isThinking) return;
    fileInputEl?.click();
  }

  function handleFileChange(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const files = Array.from(input.files || []);
    files.forEach((file) => {
      const id = `${Date.now()}-${Math.random().toString(16).slice(2)}`;
      attachments = {
        ...attachments,
        files: [
          ...attachments.files,
          {
            id,
            name: file.name,
            size: file.size,
            type: file.type,
          },
        ],
      };
    });
    input.value = "";
  }

  function removeFile(id: string) {
    attachments = {
      ...attachments,
      files: attachments.files.filter((f) => f.id !== id),
    };
  }

  function formatSeconds(seconds: number) {
    const m = String(Math.floor(seconds / 60)).padStart(2, "0");
    const s = String(seconds % 60).padStart(2, "0");
    return `${m}:${s}`;
  }

  async function startRecording() {
    if (isThinking) return;

    if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
      alert("Seu ambiente não suporta gravação de áudio (MediaRecorder).");
      return;
    }

    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorder = new MediaRecorder(stream);
      audioChunks = [];
      audioSeconds = 0;

      mediaRecorder.ondataavailable = (e) => {
        if (e.data.size > 0) audioChunks.push(e.data);
      };

      mediaRecorder.onstop = () => {
        const blob = new Blob(audioChunks, { type: "audio/webm" });
        const url = URL.createObjectURL(blob);
        attachments = {
          ...attachments,
          audio: {
            blob,
            url,
            duration: audioSeconds,
          },
        };
        stream.getTracks().forEach((t) => t.stop());
      };

      mediaRecorder.start();
      recording = true;

      audioTimer = window.setInterval(() => {
        audioSeconds += 1;
      }, 1000) as unknown as number;
    } catch (err) {
      console.error(err);
      alert("Não foi possível acessar o microfone.");
    }
  }

  function stopRecording() {
    if (mediaRecorder && recording) {
      mediaRecorder.stop();
    }
    if (audioTimer !== null) {
      clearInterval(audioTimer);
      audioTimer = null;
    }
    recording = false;
  }

  function toggleRecording() {
    if (recording) {
      stopRecording();
    } else {
      startRecording();
    }
  }

  function removeAudio() {
    if (attachments.audio?.url) {
      URL.revokeObjectURL(attachments.audio.url);
    }
    attachments = { ...attachments, audio: null };
    audioSeconds = 0;
  }

  async function handleSend() {
    if (isThinking) return;

    const trimmed = text.trim();
    const hasAttachments =
      attachments.images.length > 0 ||
      attachments.files.length > 0 ||
      attachments.audio !== null;

    if (!trimmed && !hasAttachments) {
      textareaEl?.focus();
      return;
    }

    const chatIdAtSend = activeChatId;

    if (chatIdAtSend === null) {
      alert("Nenhum chat ativo.");
      return;
    }

    if (!db) {
      await initDb();
      if (!db) {
        alert("Banco de dados não inicializado.");
        return;
      }
    }

    const provider = general.primaryAI;
    const model = general.selectedModels[provider];
    const apiKey = general.apiKeys[provider];

    if (!apiKey) {
      alert(
        `Você precisa configurar a chave de API para ${provider} nas Configurações.`,
      );
      return;
    }

    const now = Date.now();
    const userMsgId = now;

    await db.execute(
      "INSERT INTO messages (id, chat_id, role, content, created_at) VALUES ($1, $2, $3, $4, $5)",
      [userMsgId, chatIdAtSend, "user", trimmed, now],
    );

    messages = [
      ...messages,
      {
        id: userMsgId,
        chatId: chatIdAtSend,
        role: "user",
        content: trimmed,
        createdAt: now,
      },
    ];

    const conversation = [
      general.systemPrompt
        ? { role: "system", content: general.systemPrompt }
        : null,
      ...messages.map((m) => ({
        role: m.role,
        content: m.content,
      })),
    ].filter(
      (m): m is { role: "system" | "user" | "assistant"; content: string } =>
        !!m && m.content.trim() !== "",
    );

    const payload = {
      provider,
      model,
      apiKey,
      systemPrompt: general.systemPrompt,
      message: trimmed,
      messages: conversation,
      attachments: {
        images: attachments.images,
        files: attachments.files,
        hasAudio: !!attachments.audio,
      },
    };

    text = "";
    attachments = { images: [], files: [], audio: null };
    audioSeconds = 0;
    autoResize();

    isThinking = true;

    try {
      const response = await invoke<{ text: string }>("chat_completion", {
        payload,
      });
      const aiText = response.text;
      const aiNow = Date.now();
      const aiMsgId = aiNow + 1;

      await db.execute(
        "INSERT INTO messages (id, chat_id, role, content, created_at) VALUES ($1, $2, $3, $4, $5)",
        [aiMsgId, chatIdAtSend, "assistant", aiText, aiNow],
      );

      if (activeChatId === chatIdAtSend) {
        messages = [
          ...messages,
          {
            id: aiMsgId,
            chatId: chatIdAtSend,
            role: "assistant",
            content: aiText,
            createdAt: aiNow,
          },
        ];
      }
    } catch (err) {
      console.error(err);
      alert("Erro ao chamar a IA. Veja o console para detalhes.");
    } finally {
      isThinking = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (isThinking) return;
    if (event.key === "Enter" && event.shiftKey) {
      return;
    }
    if (event.key === "Enter") {
      event.preventDefault();
      handleSend();
    }
  }

  onDestroy(() => {
    if (audioTimer !== null) clearInterval(audioTimer);
    if (attachments.audio?.url) {
      URL.revokeObjectURL(attachments.audio.url);
    }
  });
</script>

{#if activeChatId === null}
  <div class="chat-empty-screen">
    <div class="logo-big">🤖</div>
    <div class="empty-text">Selecione um chat ou crie um novo</div>
  </div>
{:else}
  <div class="chat-frame">
    <div class="chat-inner">
      <header class="chat-header" aria-label="Cabeçalho do assistente de IA">
        <div class="chat-title">
          <span class="dot"></span>
          <span>Assistente</span>
        </div>
      </header>

      <section class="messages-panel" aria-label="Histórico de mensagens">
        {#if messages.length === 0}
          <div class="messages-empty">
            Comece digitando uma mensagem para este chat.
          </div>
        {:else}
          <div class="messages-list">
            {#each messages as msg (msg.id)}
              <div
                class={`message-row ${msg.role === "user" ? "row-user" : "row-assistant"}`}
              >
                <div
                  class={`message-bubble ${msg.role === "user" ? "message-user" : "message-assistant"}`}
                >
                  <div class="message-meta">
                    <span class="message-role">
                      {msg.role === "user" ? "Você" : "Assistente"}
                    </span>
                  </div>
                  <div
                    class={`message-text ${
                      msg.role === "assistant" ? "markdown" : ""
                    }`}
                  >
                    {#if msg.role === "assistant"}
                      {@html renderMarkdown(msg.content)}
                    {:else}
                      {msg.content}
                    {/if}
                  </div>
                </div>
              </div>
            {/each}

            {#if isThinking}
              <div class="message-row row-assistant">
                <div class="message-bubble message-assistant typing">
                  <div class="message-meta">
                    <span class="message-role">Assistente</span>
                  </div>
                  <div class="typing-dots" aria-label="Assistente digitando">
                    <span></span>
                    <span></span>
                    <span></span>
                  </div>
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </section>

      <section class="input-panel" aria-label="Entrada para o assistente de IA">
        <div class="input-inner">
          <div class="textarea-block">
            <div class="textarea-header">
              <span class="label-main">Prompt</span>
              <span class="label-sub">
                {#if isThinking}
                  aguardando resposta da IA...
                {:else}
                  descreva o que você precisa
                {/if}
              </span>
            </div>

            <textarea
              bind:this={textareaEl}
              bind:value={text}
              rows="2"
              placeholder={isThinking
                ? "Aguarde a resposta..."
                : "Pergunte qualquer coisa..."}
              aria-label="Digite sua mensagem para o assistente de IA"
              on:input={autoResize}
              on:keydown={handleKeydown}
              disabled={isThinking}
            ></textarea>

            <div class="attachments" aria-live="polite">
              {#each attachments.images as img (img.id)}
                <div class="chip">
                  <div class="chip-thumb chip-thumb-image">
                    <img src={img.url} alt={img.name} />
                  </div>
                  <div class="chip-info">
                    <span class="chip-name">{img.name}</span>
                    <span class="chip-meta">imagem</span>
                  </div>
                  <button
                    class="chip-remove"
                    title="Remover imagem"
                    on:click={() => removeImage(img.id)}
                  >
                    ✕
                  </button>
                </div>
              {/each}

              {#each attachments.files as file (file.id)}
                <div class="chip">
                  <div class="chip-thumb">
                    {file.name.split(".").pop()?.toUpperCase().slice(0, 4) ||
                      "ARQ"}
                  </div>
                  <div class="chip-info">
                    <span class="chip-name">{file.name}</span>
                    <span class="chip-meta">anexo</span>
                  </div>
                  <button
                    class="chip-remove"
                    title="Remover arquivo"
                    on:click={() => removeFile(file.id)}
                  >
                    ✕
                  </button>
                </div>
              {/each}

              {#if recording || attachments.audio}
                <div class="chip chip-audio">
                  <div class="chip-thumb">●</div>
                  <div class="chip-info">
                    <span class="chip-name">
                      {#if recording}
                        Gravando áudio...
                      {:else}
                        Áudio gravado
                      {/if}
                    </span>
                    <span class="chip-meta">
                      {formatSeconds(
                        recording
                          ? audioSeconds
                          : attachments.audio?.duration || 0,
                      )}
                    </span>
                  </div>

                  <div class="chip-audio-extra">
                    {#if !recording && attachments.audio}
                      <audio controls src={attachments.audio.url}></audio>
                    {/if}

                    <button
                      class="chip-remove"
                      title={recording ? "Parar gravação" : "Remover áudio"}
                      on:click={() =>
                        recording ? stopRecording() : removeAudio()}
                    >
                      ✕
                    </button>
                  </div>
                </div>
              {/if}
            </div>
          </div>

          <div class="toolbar">
            <div class="toolbar-left">
              <button
                type="button"
                class="icon-btn"
                title="Anexar imagens"
                aria-label="Anexar imagens"
                on:click={openImagePicker}
                disabled={isThinking}
              >
                <svg viewBox="0 0 24 24" fill="none">
                  <rect x="4" y="4" width="16" height="16" rx="3" />
                  <circle cx="9" cy="9" r="2" />
                  <path d="M7 17L11 13L14.5 16.5L16 15L19 18" />
                </svg>
              </button>

              <button
                type="button"
                class={`icon-btn ${recording ? "icon-btn-danger" : ""}`}
                title={recording ? "Parar gravação" : "Gravar áudio"}
                aria-label={recording ? "Parar gravação" : "Gravar áudio"}
                on:click={toggleRecording}
                disabled={isThinking}
              >
                {#if recording}
                  <svg viewBox="0 0 24 24" fill="none">
                    <rect x="7" y="7" width="10" height="10" rx="2" />
                  </svg>
                {:else}
                  <svg viewBox="0 0 24 24" fill="none">
                    <rect x="9" y="4" width="6" height="11" rx="3" />
                    <path
                      d="M7 11C7 13.2091 8.79086 15 11 15H13C15.2091 15 17 13.2091 17 11"
                    />
                    <path d="M12 16V19" />
                    <path d="M9.5 19H14.5" />
                  </svg>
                {/if}
              </button>

              <button
                type="button"
                class="icon-btn"
                title="Anexar arquivo"
                aria-label="Anexar arquivo"
                on:click={openFilePicker}
                disabled={isThinking}
              >
                <svg viewBox="0 0 24 24" fill="none">
                  <path
                    d="M15 3H8C6.89543 3 6 3.89543 6 5V19C6 20.1046 6.89543 21 8 21H16C17.1046 21 18 20.1046 18 19V8L15 3Z"
                  />
                  <path d="M15 3V8H18" />
                  <path d="M10 12H14" />
                  <path d="M10 15H12.5" />
                </svg>
              </button>
            </div>

            <div class="toolbar-right">
              <button
                type="button"
                class="btn-send"
                title={isThinking
                  ? "Aguardando resposta..."
                  : "Enviar mensagem"}
                aria-label="Enviar mensagem"
                on:click={handleSend}
                disabled={isThinking}
              >
                <span>{isThinking ? "Respondendo..." : "Enviar"}</span>
                <svg viewBox="0 0 24 24" fill="none">
                  <path d="M4.5 4.5L19.5 12L4.5 19.5L7.5 12L4.5 4.5Z" />
                </svg>
              </button>
            </div>
          </div>
        </div>

        <input
          bind:this={imageInputEl}
          type="file"
          accept="image/*"
          multiple
          hidden
          on:change={handleImageChange}
        />
        <input
          bind:this={fileInputEl}
          type="file"
          multiple
          hidden
          on:change={handleFileChange}
        />
      </section>
    </div>
  </div>
{/if}
