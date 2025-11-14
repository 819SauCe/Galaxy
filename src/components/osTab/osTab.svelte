<script lang="ts">
	import "./osTab.css"
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	let isMaximized = false;
	let isControlsHovered = false;

	onMount(async () => {
		try {
			const result = await invoke<boolean>('plugin:window|is_maximized');
			isMaximized = result;
		} catch (e) {
			console.error('Erro ao verificar maximizado:', e);
		}
	});

	const handleMinimize = async () => {
		await invoke('plugin:window|minimize');
	};

	const handleMaximize = async () => {
		if (isMaximized) {
			await invoke('plugin:window|unmaximize');
		} else {
			await invoke('plugin:window|maximize');
		}
		isMaximized = !isMaximized;
	};

	const handleClose = async () => {
		await invoke('plugin:window|close');
	};
</script>

<div class="title-bar" data-tauri-drag-region>
	<div class="title-content">
		<span class="app-title">Galaxy</span>
	</div>

	<div class="title-spacer"></div>

	<div
		class="window-controls {isControlsHovered ? 'show-icons' : ''}"
		on:mouseenter={() => (isControlsHovered = true)}
		on:mouseleave={() => (isControlsHovered = false)}
		role="group"
	>
		<button
			class="window-btn minimize"
			on:click|stopPropagation={handleMinimize}
			title="Minimizar"
			data-tauri-drag-region="false"
		>
			<span class="glyph">–</span>
		</button>

		<button
			class="window-btn maximize"
			on:click|stopPropagation={handleMaximize}
			title={isMaximized ? 'Restaurar' : 'Maximizar'}
			data-tauri-drag-region="false"
		>
			<span class="glyph">{isMaximized ? '▢' : '◻'}</span>
		</button>

        <button
			class="window-btn close"
			on:click|stopPropagation={handleClose}
			title="Fechar"
			data-tauri-drag-region="false"
		>
			<span class="glyph">×</span>
		</button>
	</div>
</div>