<script lang="ts">
  import "../styles/category-card.css";
  import type { CategoryGroup, FileItem } from "./types";
  import FileRow from "./FileRow.svelte";
  import Icon from "./Icon.svelte";
  import { formatBytes } from "./api";
  import { getFileIconInfo } from "./fileIcons";

  interface Props {
    group: CategoryGroup;
    selectedIds: Set<string>;
    activeFileId: string | null;
    onToggleGroup: (groupKey: string) => void;
    onToggleFile: (id: string) => void;
    onInspect: (file: FileItem) => void;
  }

  let {
    group,
    selectedIds,
    activeFileId,
    onToggleGroup,
    onToggleFile,
    onInspect,
  }: Props = $props();

  let isExpanded = $state(true);

  let groupFileIds = $derived(group.files.map((f) => f.id));
  let selectedInGroup = $derived(
    groupFileIds.filter((id) => selectedIds.has(id)).length
  );
  let allSelected = $derived(
    group.files.length > 0 && selectedInGroup === group.files.length
  );
  let isIndeterminate = $derived(
    selectedInGroup > 0 && selectedInGroup < group.files.length
  );

  let groupIconInfo = $derived(getFileIconInfo(group.extensions[0] || group.key));

  function toggleExpand() {
    isExpanded = !isExpanded;
  }
</script>

<div class="category-card">
  <div class="card-header">
    <div class="header-left">
      <input
        type="checkbox"
        checked={allSelected}
        indeterminate={isIndeterminate}
        onchange={() => onToggleGroup(group.key)}
        title="Seleccionar todo el grupo"
      />

      <button class="expand-btn" onclick={toggleExpand} title="Desplegar / Colapsar">
        <Icon name={isExpanded ? "chevron-down" : "chevron-right"} size={11} />
        <Icon name={groupIconInfo.iconClass} class={groupIconInfo.colorClass} size={14} />
        <span class="group-title">{group.label}</span>
      </button>

      {#if group.extensions.length > 1}
        <span class="ext-tags">
          {group.extensions.join(", ")}
        </span>
      {/if}
    </div>

    <div class="header-right">
      <span class="count-badge">{group.count} {group.count === 1 ? 'archivo' : 'archivos'}</span>
      <span class="size-badge">{formatBytes(group.total_size)}</span>
    </div>
  </div>

  {#if isExpanded}
    <div class="files-list">
      {#each group.files as file (file.id)}
        <FileRow
          {file}
          selected={selectedIds.has(file.id)}
          active={activeFileId === file.id}
          onToggle={onToggleFile}
          {onInspect}
        />
      {/each}
    </div>
  {/if}
</div>
