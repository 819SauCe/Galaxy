<script lang="ts">
  import type { GeneralSettings } from "../lib/config";
  import { sendChat, type ChatMessage } from "../lib/chatClient";

  export let activeChatId: number | null;
  export let general: GeneralSettings;

  let conversations: Record<number, ChatMessage[]> = {};

  $: messages =
    activeChatId != null
      ? conversations[activeChatId] ?? []
      : ([] as ChatMessage[]);

  let input = "";
  let isLoading = false;
  let errorMsg = "";

  async function handleSend() {
    const text = input.trim();
    if (!text || isLoading || activeChatId == null) return;

    errorMsg = "";
    input = "";

    const current: ChatMessage[] = conversations[activeChatId] ?? [];

    const userMsg: ChatMessage = {
      role: "user",
      content: text,
    };

    const nextHistory: ChatMessage[] = [...current, userMsg];

    conversations = {
      ...conversations,
      [activeChatId]: nextHistory,
    };

    try {
      isLoading = true;
      const reply = await sendChat(general, nextHistory);

      const assistantMsg: ChatMessage = {
        role: "assistant",
        content: reply,
      };

      const finalHistory: ChatMessage[] = [...nextHistory, assistantMsg];

      conversations = {
        ...conversations,
        [activeChatId]: finalHistory,
      };
    } catch (err: any) {
      console.error(err);
      errorMsg = err?.message ?? "Erro ao enviar mensagem.";
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      if (e.ctrlKey || !e.shiftKey) {
        e.preventDefault();
        handleSend();
      }
    }
  }
</script>


<div class="chat-root">
  {#if activeChatId == null}
    <div class="chat-empty">
      Selecione ou crie um chat à esquerda.
    </div>
  {:else}
    <div class="chat-messages">
      {#if messages.length === 0}
        <div class="chat-empty">
          Comece a conversar ✨
        </div>
      {/if}

      {#each messages as m}
        <div class="message-row {m.role}">
          <div class="message-bubble">
            <div class="message-label">
              {m.role === "user" ? "Você" : "Assistente"}
            </div>
            <div class="message-content">{m.content}</div>
          </div>
        </div>
      {/each}

      {#if isLoading}
        <div class="message-row assistant">
          <div class="message-bubble">
            <div class="message-label">Pensando...</div>
            <div class="typing-dots">
              <span></span><span></span><span></span>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <div class="chat-input">
      <textarea
        rows="1"
        bind:value={input}
        on:keydown={handleKeydown}
        placeholder="Digite uma mensagem..."
      ></textarea>
      <button on:click={handleSend} disabled={isLoading || !input.trim()}>
        {#if isLoading}Enviando...{:else}Enviar{/if}
      </button>
    </div>

    {#if errorMsg}
      <div class="chat-error">{errorMsg}</div>
    {/if}
  {/if}
</div>

<style>
  .chat-root {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 16px;
    color: var(--text-primary);
  }

  .chat-messages {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .chat-empty {
    margin: auto;
    text-align: center;
    color: var(--text-secondary);
  }

  .message-row {
    display: flex;
  }

  .message-row.user {
    justify-content: flex-end;
  }

  .message-row.assistant {
    justify-content: flex-start;
  }

  .message-bubble {
    max-width: 70%;
    padding: 8px 10px;
    border-radius: var(--radius-md);
    background: var(--chat-bot-bg);
    border: 1px solid var(--border-subtle);
    font-size: 14px;
  }

  .message-row.user .message-bubble {
    background: var(--chat-user-bg);
  }

  .message-label {
    font-size: 11px;
    color: var(--text-secondary);
    margin-bottom: 4px;
  }

  .message-content {
    white-space: pre-wrap;
    word-break: break-word;
  }

  .chat-input {
    margin-top: 10px;
    border-top: 1px solid var(--border-subtle);
    padding-top: 8px;
    display: flex;
    gap: 8px;
  }

  .chat-input textarea {
    flex: 1;
    resize: none;
    background: var(--input-bg);
    border-radius: var(--radius-md);
    border: 1px solid var(--input-border);
    padding: 8px 10px;
    color: var(--text-primary);
  }

  .chat-input button {
    border: none;
    border-radius: var(--radius-md);
    padding: 8px 12px;
    background: var(--accent);
    color: var(--accent-contrast);
    cursor: pointer;
  }

  .chat-input button:disabled {
    opacity: 0.6;
    cursor: default;
  }

  .chat-error {
    margin-top: 6px;
    font-size: 12px;
    color: #ff6b6b;
  }

  .typing-dots {
    display: inline-flex;
    gap: 3px;
  }

  .typing-dots span {
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: var(--text-secondary);
    animation: blink 1s infinite;
  }

  .typing-dots span:nth-child(2) {
    animation-delay: 0.2s;
  }

  .typing-dots span:nth-child(3) {
    animation-delay: 0.4s;
  }

  @keyframes blink {
    0%, 80%, 100% {
      opacity: 0.3;
      transform: translateY(0);
    }
    40% {
      opacity: 1;
      transform: translateY(-3px);
    }
  }
</style>
