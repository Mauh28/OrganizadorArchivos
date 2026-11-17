<script lang="ts">
  import "../styles/action-bar.css";
  import Icon from "./Icon.svelte";
  import { formatBytes } from "./api";

  interface Props {
    selectedCount: number;
    selectedBytes: number;
    totalCount: number;
    isOrganizing: boolean;
    onSelectAll: () => void;
    onDeselectAll: () => void;
    onOrganize: () => void;
  }

  let {
    selectedCount,
    selectedBytes,
    totalCount,
    isOrganizing,
    onSelectAll,
    onDeselectAll,
    onOrganize,
  }: Props = $props();
</script>

<div class="action-bar">
  <div class="stats-side">
    <span class="selection-pill">
      {selectedCount} de {totalCount} seleccionados
    </span>
    {#if selectedCount > 0}
      <span class="size-pill">
        ({formatBytes(selectedBytes)})
      </span>
      <button class="btn-link" onclick={onDeselectAll}>Deseleccionar</button>
    {:else if totalCount > 0}
      <button class="btn-link" onclick={onSelectAll}>Seleccionar todos</button>
    {/if}
  </div>

  <div class="button-side">
    <button
      class="btn-organize"
      disabled={selectedCount === 0 || isOrganizing}
      onclick={onOrganize}
    >
      {#if isOrganizing}
        <span class="spinner"></span>
        <span>Empaquetando...</span>
      {:else}
        <Icon name="package" size={15} />
        <span>Empaquetar seleccionados ({selectedCount})</span>
      {/if}
    </button>
  </div>
</div>
