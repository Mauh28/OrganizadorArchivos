<script lang="ts">
  import "../styles/details-panel.css";
  import type { FileItem } from "./types";
  import Icon from "./Icon.svelte";
  import { convertFileSrc, formatBytes, getImageDataUrl } from "./api";
  import { getFileIconInfo, isImageFile, isVideoFile } from "./fileIcons";

  interface Props {
    file: FileItem | null;
    selected: boolean;
    onToggleSelect: (id: string) => void;
  }

  let { file, selected, onToggleSelect }: Props = $props();

  let previewUrl = $state<string | null>(null);
  let isImage = $derived(file ? isImageFile(file.extension) : false);
  let isVideo = $derived(file ? isVideoFile(file.extension) : false);
  let iconInfo = $derived(
    file ? getFileIconInfo(file.extension) : { iconClass: "fa-regular fa-file", colorClass: "icon-default" }
  );

  $effect(() => {
    if (file && isImage) {
      previewUrl = null;
      getImageDataUrl(file.path)
        .then((url) => {
          previewUrl = url;
        })
        .catch(() => {
          // Fallback a convertFileSrc si getImageDataUrl falla
          previewUrl = convertFileSrc(file.path);
        });
    } else if (file && isVideo) {
      previewUrl = convertFileSrc(file.path);
    } else {
      previewUrl = null;
    }
  });
</script>

<aside class="detail-panel">
  {#if !file}
    <div class="empty-detail">
      <Icon name="document" size={42} />
      <p>Selecciona un archivo para ver sus detalles y previsualización</p>
    </div>
  {:else}
    <div class="panel-header">
      <Icon name={iconInfo.iconClass} class={iconInfo.colorClass} size={18} />
      <div class="panel-title-wrap">
        <h3 class="panel-title" title={file.name}>{file.name}</h3>
        <span class="panel-subtitle">{file.extension ? file.extension : "Sin extensión"}</span>
      </div>
    </div>

    <div class="panel-content">
      <!-- Sección de previsualización -->
      <div class="preview-container">
        {#if isImage && previewUrl}
          <img src={previewUrl} alt={file.name} class="preview-img" />
        {:else if isVideo && previewUrl}
          <!-- svelte-ignore a11y_media_has_caption -->
          <video src={previewUrl} controls class="preview-video"></video>
        {:else}
          <div class="preview-placeholder">
            <Icon name={iconInfo.iconClass} class={iconInfo.colorClass} size={44} />
            <span class="preview-ext-badge">{file.extension || "ARCHIVO"}</span>
          </div>
        {/if}
      </div>

      <!-- Sección de propiedades -->
      <div class="props-section">
        <h4>Propiedades del Archivo</h4>

        <div class="prop-row">
          <span class="prop-label">Tamaño:</span>
          <span class="prop-value">{formatBytes(file.size)} ({file.size.toLocaleString()} bytes)</span>
        </div>

        <div class="prop-row">
          <span class="prop-label">Última modificación:</span>
          <span class="prop-value">{file.modified_at}</span>
        </div>

        <div class="prop-row">
          <span class="prop-label">Fecha de creación:</span>
          <span class="prop-value">{file.created_at}</span>
        </div>

        <div class="prop-row">
          <span class="prop-label">Destino planificado:</span>
          <span class="prop-value code-style">{file.group_key}/{file.date_folder}/{file.name}</span>
        </div>

        <div class="prop-row">
          <span class="prop-label">Ruta de origen:</span>
          <span class="prop-value code-style" title={file.path}>{file.path}</span>
        </div>
      </div>
    </div>

    <div class="panel-footer">
      <button
        class="btn-toggle-select"
        class:is-selected={selected}
        onclick={() => onToggleSelect(file.id)}
      >
        <Icon name={selected ? "check" : "package"} size={14} />
        <span>{selected ? "Seleccionado para empaquetar" : "Marcar para empaquetar"}</span>
      </button>
    </div>
  {/if}
</aside>

