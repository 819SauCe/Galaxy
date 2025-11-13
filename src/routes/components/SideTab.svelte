<script lang="ts">
    import "../style.css";
    import { onMount, onDestroy } from "svelte";

    export let chats: { id: string | number; title: string }[] = [];
    export let activeChatId: string | number | null = null;
    export let onNewChat: () => void = () => {};
    export let onSelectChat: (id: string | number) => void = () => {};
    export let onDeleteChat: (id: string | number) => void = () => {};
    export let onDeleteAllChats: () => void = () => {};
    export let openConfig: () => void = () => {};
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
            aria-label={collapsed
                ? "Expandir barra lateral"
                : "Recolher barra lateral"}
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
                        class="chat-item {chat.id === activeChatId
                            ? 'active'
                            : ''}"
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
                        <div class="chat-title" title={chat.title}>
                            {chat.title}
                        </div>
                        <button
                            class="icon-btn small"
                            on:click|stopPropagation={() =>
                                onDeleteChat(chat.id)}
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
            <span class="icon"
                ><svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    fill="currentColor"
                    viewBox="0 0 16 16"
                >
                    <path
                        d="M8.932.727c-.243-.97-1.62-.97-1.864 0l-.071.286a.96.96 0 0 1-1.622.434l-.205-.211c-.695-.719-1.888-.03-1.613.931l.08.284a.96.96 0 0 1-1.186 1.187l-.284-.081c-.96-.275-1.65.918-.931 1.613l.211.205a.96.96 0 0 1-.434 1.622l-.286.071c-.97.243-.97 1.62 0 1.864l.286.071a.96.96 0 0 1 .434 1.622l-.211.205c-.719.695-.03 1.888.931 1.613l.284-.08a.96.96 0 0 1 1.187 1.187l-.081.283c-.275.96.918 1.65 1.613.931l.205-.211a.96.96 0 0 1 1.622.434l.071.286c.243.97 1.62.97 1.864 0l.071-.286a.96.96 0 0 1 1.622-.434l.205.211c.695.719 1.888.03 1.613-.931l-.08-.284a.96.96 0 0 1 1.187-1.187l.283.081c.96.275 1.65-.918.931-1.613l-.211-.205a.96.96 0 0 1 .434-1.622l.286-.071c.97-.243.97-1.62 0-1.864l-.286-.071a.96.96 0 0 1-.434-1.622l.211-.205c.719-.695.03-1.888-.931-1.613l-.284.08a.96.96 0 0 1-1.187-1.186l.081-.284c.275-.96-.918-1.65-1.613-.931l-.205.211a.96.96 0 0 1-1.622-.434zM8 12.997a4.998 4.998 0 1 1 0-9.995 4.998 4.998 0 0 1 0 9.996z"
                    />
                </svg></span
            >
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

    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="resize-handle" on:mousedown={startResize}></div>
</aside>

<style>
    .sidebar {
        position: relative;
        height: 98vh;
        display: flex;
        flex-direction: column;
        padding: 16px 12px;
        box-sizing: border-box;
        background: var(--sidebar-bg, var(--bg-surface-muted));
        border-right: 1px solid var(--sidebar-border, var(--border-subtle));
        color: var(--text-primary);
        backdrop-filter: var(--sidebar-blur, none);
        font-family:
            system-ui,
            -apple-system,
            "SF Pro Text",
            sans-serif;
        transition:
            padding 0.18s ease,
            width 0.18s ease;
    }

    .sidebar.collapsed {
        padding-inline: 8px;
    }

    .sidebar-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 8px;
        margin-bottom: 12px;
    }

    .app-brand {
        display: flex;
        align-items: center;
        gap: 8px;
        overflow: hidden;
    }

    .logo {
        width: 28px;
        height: 28px;
        border-radius: var(--radius-lg);
        background: var(--bg-surface);
        border: 1px solid var(--border-subtle);
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 16px;
        color: var(--accent);
    }

    .app-text {
        display: flex;
        flex-direction: column;
    }

    .app-name {
        font-size: 14px;
        font-weight: 600;
        color: var(--text-primary);
    }

    .app-subtitle {
        font-size: 11px;
        color: var(--text-secondary);
    }

    .sidebar.collapsed .app-text {
        display: none;
    }

    .icon-btn {
        border: none;
        background: transparent;
        color: var(--text-secondary);
        cursor: pointer;
        border-radius: 999px;
        padding: 6px;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        font-size: 16px;
        transition:
            background 0.14s ease,
            transform 0.08s ease,
            color 0.14s ease,
            opacity 0.12s ease;
    }

    .icon-btn.toggle {
        padding: 4px 6px;
    }

    .icon-btn.toggle:hover {
        background: var(--bg-surface);
        color: var(--text-primary);
    }

    .icon-btn:active {
        transform: scale(0.96);
    }

    .icon-btn.small {
        padding: 4px;
        font-size: 14px;
    }

    .chevron-icon {
        transition: transform 0.16s ease;
    }

    .sidebar.collapsed .chevron-icon {
        transform: rotate(180deg);
    }

    .sidebar.collapsed .primary-btn,
    .sidebar.collapsed .secondary-btn {
        justify-content: center;
        padding-inline: 0;
    }

    .sidebar.collapsed .sidebar-header {
        justify-content: center;
        gap: 4px;
    }

    .sidebar.collapsed .app-brand {
        margin-right: 0;
    }

    .sidebar.collapsed .user-area {
        justify-content: center;
        padding-inline: 0;
    }

    .sidebar.collapsed .chat-title {
        display: none;
    }

    .sidebar.collapsed .chat-item {
        justify-content: center;
    }

    .new-chat-wrapper {
        padding: 4px 0 10px;
    }

    .primary-btn,
    .secondary-btn {
        width: 100%;
        border-radius: var(--radius-lg);
        border: none;
        display: inline-flex;
        align-items: center;
        justify-content: flex-start;
        gap: 8px;
        cursor: pointer;
        padding: 9px 12px;
        font-size: 13px;
        font-weight: 500;
        transition:
            background 0.16s ease,
            transform 0.08s ease,
            box-shadow 0.16s ease,
            filter 0.12s ease;
        box-sizing: border-box;
        white-space: nowrap;
    }

    .primary-btn {
        background: var(--accent);
        color: var(--text-primary);
        box-shadow: var(--shadow-soft);
    }

    .primary-btn:hover {
        transform: translateY(-1px);
        filter: brightness(1.08);
    }

    .primary-btn:active {
        transform: translateY(0);
        box-shadow: none;
    }

    .secondary-btn {
        background: var(--bg-surface);
        color: var(--text-primary);
        border: 1px solid var(--border-subtle);
    }

    .secondary-btn:hover {
        background: var(--bg-surface-muted);
    }

    .icon {
        font-size: 16px;
        flex-shrink: 0;
    }

    .label {
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .sidebar.collapsed .label {
        display: none;
    }

    .section {
        margin-top: 12px;
    }

    .section-title {
        font-size: 11px;
        text-transform: uppercase;
        letter-spacing: 0.06em;
        color: var(--text-secondary);
        margin-bottom: 6px;
    }

    .history-section {
        flex: 1;
        display: flex;
        flex-direction: column;
        min-height: 0;
        margin-top: 14px;
    }

    .section-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .link-btn {
        border: none;
        background: transparent;
        color: var(--text-secondary);
        font-size: 11px;
        cursor: pointer;
        padding: 0;
    }

    .link-btn:hover {
        color: var(--text-primary);
        text-decoration: underline;
    }

    .sidebar.collapsed .link-btn,
    .sidebar.collapsed .section-header .section-title {
        display: none;
    }

    .chat-list {
        margin-top: 6px;
        padding-right: 4px;
        overflow-y: auto;
        scrollbar-width: thin;
        scrollbar-color: var(--scrollbar-thumb) transparent;
    }

    .chat-list::-webkit-scrollbar {
        width: 8px;
    }

    .chat-list::-webkit-scrollbar-thumb {
        background: var(--scrollbar-thumb);
        border-radius: 999px;
    }

    .chat-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 6px;
        padding: 7px 8px;
        border-radius: var(--radius-md);
        cursor: pointer;
        font-size: 13px;
        color: var(--text-primary);
        transition:
            background 0.12s ease,
            transform 0.06s ease,
            opacity 0.12s ease;
    }

    .chat-item:hover {
        background: var(--bg-surface);
    }

    .chat-item.active {
        background: var(--accent);
        color: var(--text-primary);
    }

    .chat-title {
        flex: 1;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .empty-state {
        margin-top: 12px;
        font-size: 12px;
        color: var(--text-secondary);
        line-height: 1.4;
    }

    .sidebar.collapsed .empty-state {
        display: none;
    }

    .sidebar-footer {
        margin-top: 10px;
        border-top: 1px solid var(--border-subtle);
        padding-top: 8px;
        display: flex;
        flex-direction: column;
        gap: 6px;
    }

    .user-area {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 6px 4px;
        border-radius: var(--radius-lg);
    }

    .avatar {
        width: 28px;
        height: 28px;
        border-radius: 999px;
        object-fit: cover;
    }

    .avatar.placeholder {
        background: var(--accent);
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: 600;
        color: var(--text-primary);
    }

    .user-info {
        display: flex;
        flex-direction: column;
        font-size: 11px;
    }

    .user-name {
        font-weight: 500;
    }

    .user-status {
        color: var(--text-secondary);
        display: inline-flex;
        align-items: center;
        gap: 4px;
    }

    .status-dot {
        width: 6px;
        height: 6px;
        border-radius: 999px;
        background: var(--accent);
    }

    .sidebar.collapsed .user-info {
        display: none;
    }

    .resize-handle {
        position: absolute;
        top: 0;
        right: -3px;
        width: 6px;
        height: 100%;
        cursor: col-resize;
        background: transparent;
    }

    .resize-handle:hover {
        background: rgba(255, 255, 255, 0.04);
    }

    .sidebar.collapsed .resize-handle {
        display: none;
    }
</style>
