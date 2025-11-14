<script lang="ts">
    import "../../routes/style.css";
    import "./SideTab.css";
    import { onMount, onDestroy } from "svelte";

    export let chats: { id: string | number; title: string }[] = [];
    export let activeChatId: string | number | null = null;
    export let onNewChat: () => void = () => {};
    export let onSelectChat: (id: string | number) => void = () => {};
    export let onDeleteChat: (id: string | number) => void = () => {};
    export let onDeleteAllChats: () => void = () => {};
    export let openConfig: () => void = () => {};
    export let onRenameChat: (id: string | number, title: string) => void = () => {};
    export let userName: string = "Você";
    export let userAvatarUrl: string = "";

    let collapsed = false;
    const toggleCollapsed = () => {
        collapsed = !collapsed;
    };

    let sidebarWidth = 260;
    const MIN_WIDTH = 160;
    const MAX_WIDTH = 420;

    let isResizing = false;

    function startResize(event: MouseEvent) {
        if (collapsed) return;
        isResizing = true;
        document.body.style.userSelect = "none";
    }

    function stopResize() {
        if (!isResizing) return;
        isResizing = false;
        document.body.style.userSelect = "";
    }

    function handleMouseMove(event: MouseEvent) {
        if (!isResizing || collapsed) return;

        const newWidth = Math.min(
            MAX_WIDTH,
            Math.max(MIN_WIDTH, event.clientX),
        );

        sidebarWidth = newWidth;
    }

    onMount(() => {
        window.addEventListener("mousemove", handleMouseMove);
        window.addEventListener("mouseup", stopResize);
    });

    onDestroy(() => {
        window.removeEventListener("mousemove", handleMouseMove);
        window.removeEventListener("mouseup", stopResize);
    });
</script>

<aside
    class="sidebar no-drag"
    class:collapsed
    style={`width: ${collapsed ? "72px" : sidebarWidth + "px"}`}
>
    <div class="sidebar-header">
        <div class="app-brand">
            <div class="logo">G</div>
            <div class="app-text">
                <span class="app-name">Galaxy</span>
                <span class="app-subtitle">sempre online</span>
            </div>
        </div>

        <button
            class="icon-btn toggle"
            on:click={toggleCollapsed}
            title={collapsed ? "Expandir barra" : "Recolher barra"}
            aria-label={collapsed ? "Expandir barra lateral" : "Recolher barra lateral"}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                fill="currentColor"
                viewBox="0 0 16 16"
                class="chevron-icon"
            >
                <path
                    fill-rule="evenodd"
                    d="M10.146 3.646a.5.5 0 0 1 .708.708L7.707 7.5l3.147 3.146a.5.5 0 0 1-.708.708l-3.5-3.5a.5.5 0 0 1 0-.708z"
                />
            </svg>
        </button>
    </div>

    <div class="new-chat-wrapper">
        <button class="primary-btn" on:click={onNewChat}>
            <span class="icon">＋</span>
            <span class="label">Novo chat</span>
        </button>
    </div>

    <div class="section history-section">
        <div class="section-header">
            <span class="section-title">Chats recentes</span>
            {#if chats.length > 0}
                <button class="link-btn" on:click={onDeleteAllChats}>
                    Limpar tudo
                </button>
            {/if}
        </div>

        {#if chats.length === 0}
            <div class="empty-state">
                Nenhum chat ainda.
                <br />
                Clique em <strong>Novo chat</strong> para começar.
            </div>
        {:else}
            <div class="chat-list">
                {#each chats as chat}
                    <div
                        class="chat-item {chat.id === activeChatId ? 'active' : ''}"
                        role="button"
                        tabindex="0"
                        on:click={() => onSelectChat(chat.id)}
                        on:keydown={(e) => {
                            if (e.key === "Enter" || e.key === " ") {
                                e.preventDefault();
                                onSelectChat(chat.id);
                            }
                        }}
                    >
                        <div
                            class="chat-title"
                            title={chat.title}
                            contenteditable="true"
                            on:focus={(e) => {
                                const el = e.target as HTMLDivElement;
                                const range = document.createRange();
                                const sel = window.getSelection();
                                range.selectNodeContents(el);
                                range.collapse(false);
                                sel?.removeAllRanges();
                                sel?.addRange(range);
                            }}
                            on:blur={(e) => {
                                const el = e.target as HTMLDivElement;
                                const newTitle = el.innerText.trim();
                                if (newTitle && newTitle !== chat.title) {
                                    onRenameChat(chat.id, newTitle);
                                } else {
                                    el.innerText = chat.title;
                                }
                            }}
                            on:keydown={(e) => {
                                if (e.key === "Enter") {
                                    e.preventDefault();
                                    (e.target as HTMLDivElement).blur();
                                }
                                if (e.key === "Escape") {
                                    (e.target as HTMLDivElement).innerText = chat.title;
                                    (e.target as HTMLDivElement).blur();
                                }
                            }}
                        >
                            {chat.title}
                        </div>
                        <button
                            class="icon-btn small"
                            on:click|stopPropagation={() => onDeleteChat(chat.id)}
                            title="Deletar chat"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                fill="currentColor"
                                viewBox="0 0 16 16"
                            >
                                <path
                                    d="M6.5 1h3a.5.5 0 0 1 .5.5V2h2.5a.5.5 0 0 1 0 1h-.538l-.853 9.66A1.5 1.5 0 0 1 9.616 14H6.384a1.5 1.5 0 0 1-1.493-1.34L4.038 3H3.5a.5.5 0 0 1 0-1H6V1.5a.5.5 0 0 1 .5-.5Z"
                                />
                            </svg>
                        </button>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <div class="sidebar-footer">
        <button class="secondary-btn" on:click={openConfig}>
            <span class="icon">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    fill="currentColor"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M8.932.727c-.243-.97-1.62-.97-1.864 0l-.071.286a.96.96 0 0 1-1.622.434l-.205-.211c-.695-.719-1.888-.03-1.613.931l.08.284a.96.96 0 0 1-1.186 1.187l-.284-.081c-.96-.275-1.65.918-.931 1.613l.211.205a.96.96 0 0 1-.434 1.622l-.286.071c-.97.243-.97 1.62 0 1.864l.286.071a.96.96 0 0 1 .434 1.622l-.211.205c-.719.695-.03 1.888.931 1.613l.284-.08a.96.96 0 0 1 1.187 1.187l-.081.283c-.275.96.918 1.65 1.613.931l.205-.211a.96.96 0 0 1 1.622.434l.071.286c.243.97 1.62.97 1.864 0l.071-.286a.96.96 0 0 1 1.622-.434l.205.211c.695.719 1.888.03 1.613-.931l-.08-.284a.96.96 0 0 1 1.187-1.187l.283.081c.96.275 1.65-.918.931-1.613l-.211-.205a.96.96 0 0 1 .434-1.622l.286-.071c.97-.243.97-1.62 0-1.864l-.286-.071a.96.96 0 0 1-.434-1.622l.211-.205c.719-.695.03-1.888-.931-1.613l-.284.08a.96.96 0 0 1-1.187-1.186l.081-.284c.275-.96-.918-1.65-1.613-.931l-.205.211a.96.96 0 0 1-1.622-.434zM8 12.997a4.998 4.998 0 1 1 0-9.995 4.998 4.998 0 0 1 0 9.996z"
                    />
                </svg>
            </span>
            <span class="label">Configurações</span>
        </button>

        <div class="user-area">
            {#if userAvatarUrl}
                <img
                    class="avatar"
                    src={userAvatarUrl}
                    alt="Avatar do usuário"
                />
            {:else}
                <div class="avatar placeholder">
                    {userName ? userName[0].toUpperCase() : "U"}
                </div>
            {/if}
            <div class="user-info">
                <span class="user-name">{userName}</span>
                <span class="user-status">
                    <span class="status-dot"></span>
                    online
                </span>
            </div>
        </div>
    </div>

    <div class="resize-handle" on:mousedown={startResize}></div>
</aside>