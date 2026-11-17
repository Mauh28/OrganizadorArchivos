<script lang="ts">
  import "../styles/file-row.css";
  import type { FileItem } from "./types";
  import Icon from "./Icon.svelte";
  import { formatBytes } from "./api";
  import { getFileIconInfo } from "./fileIcons";

  interface Props {
    file: FileItem;
    selected: boolean;
    active: boolean;
    onToggle: (id: string) => void;
    onInspect: (file: FileItem) => void;
  }

  let { file, selected, active, onToggle, onInspect }: Props = $props();
  let iconInfo = $derived(getFileIconInfo(file.extension));

  function handleClick(e: MouseEvent) {
    // Si no se hizo clic en el checkbox ni su etiqueta, seleccionar para inspección de detalles
    const target = e.target as HTMLElement;
    if (target.tagName !== "INPUT" && target.tagName !== "LABEL") {
      onInspect(file);
    }
  }
</script>

<div
  class="file-row"
  class:selected
  class:active
  onclick={handleClick}
  onkeydown={(e) => e.key === "Enter" && onInspect(file)}
  role="button"
  tabindex="0"
>
  <label class="checkbox-container">
    <input
      type="checkbox"
      checked={selected}
      onclick={(e) => e.stopPropagation()}
      onchange={() => onToggle(file.id)}
    />
  </label>

  <div class="file-icon-wrap">
    <Icon name={iconInfo.iconClass} class={iconInfo.colorClass} size={14} />
  </div>

  <div class="file-info" title={file.name}>
    <span class="file-name">{file.name}</span>
    <span class="dest-badge">
      <Icon name="chevron-right" size={10} /> {file.group_key}/{file.date_folder}/
    </span>
  </div>

  <div class="meta-col">
    <span class="meta-label">Modificado:</span>
    <span class="meta-value">{file.modified_at}</span>
  </div>

  <div class="meta-col hide-mobile">
    <span class="meta-label">Creado:</span>
    <span class="meta-value">{file.created_at}</span>
  </div>

  <div class="size-col">
    {formatBytes(file.size)}
  </div>
</div>
