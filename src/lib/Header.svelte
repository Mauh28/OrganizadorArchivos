<script lang="ts">
  import "../styles/header.css";
  import Icon from "./Icon.svelte";

  interface Props {
    folderPath: string;
    isScanning: boolean;
    canUndo: boolean;
    onScan: () => void;
    onUndo: () => void;
    onChangePath: (newPath: string) => void;
  }

  let {
    folderPath = $bindable(),
    isScanning,
    canUndo,
    onScan,
    onUndo,
    onChangePath,
  }: Props = $props();

  let isEditing = $state(false);
  let editInput = $state(folderPath);

  function startEdit() {
    editInput = folderPath;
    isEditing = true;
  }

  function saveEdit() {
    if (editInput.trim().length > 0) {
      onChangePath(editInput.trim());
    }
    isEditing = false;
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter") saveEdit();
    if (e.key === "Escape") isEditing = false;
  }
</script>

<header class="header">
  <div class="brand">
    <div class="logo-icon">
      <Icon name="folder" size={24} />
    </div>
    <div class="title-wrap">
      <h1>Organizador de Archivos</h1>
      <div class="path-bar">
        {#if isEditing}
          <input
            class="path-input"
            bind:value={editInput}
            onkeydown={handleKeyDown}
            onblur={saveEdit}
            placeholder="Ruta de carpeta..."
          />
        {:else}
          <span class="path-label" title={folderPath}>{folderPath}</span>
          <button class="btn-text" onclick={startEdit} title="Cambiar ruta">
            <Icon name="edit" size={13} />
          </button>
        {/if}
      </div>
    </div>
  </div>

  <div class="actions">
    {#if canUndo}
      <button class="btn-secondary" onclick={onUndo} title="Revertir el último empaquetado">
        <Icon name="undo" size={14} />
        <span>Deshacer</span>
      </button>
    {/if}

    <button class="btn-primary" onclick={onScan} disabled={isScanning}>
      {#if isScanning}
        <span class="spinner"></span>
        <span>Escaneando...</span>
      {:else}
        <Icon name="search" size={14} />
        <span>Escanear</span>
      {/if}
    </button>
  </div>
</header>
