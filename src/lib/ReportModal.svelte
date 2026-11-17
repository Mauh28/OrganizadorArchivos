<script lang="ts">
  import "../styles/report-modal.css";
  import Icon from "./Icon.svelte";
  import type { OrganizeReport, UndoReport } from "./types";

  interface Props {
    organizeReport: OrganizeReport | null;
    undoReport: UndoReport | null;
    onClose: () => void;
  }

  let { organizeReport, undoReport, onClose }: Props = $props();
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && onClose()} />

<div class="modal-backdrop" onclick={onClose} role="presentation">
  <div
    class="modal-box"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="modal-header">
      <div class="modal-title">
        <span class="modal-title-icon">
          <Icon name={organizeReport ? "package" : "undo"} size={14} />
        </span>
        <span>{organizeReport ? "Resumen del Empaquetado" : "Resumen de Reversión"}</span>
      </div>
      <button class="btn-icon-close" onclick={onClose} title="Cerrar">
        <Icon name="close" size={14} />
      </button>
    </div>

    <div class="modal-body">
      {#if organizeReport}
        <div class="stats-list">
          <div class="stat-row success">
            <span class="stat-label">
              <span class="stat-label-icon"><Icon name="check" size={13} /></span>
              <span>Archivos movidos</span>
            </span>
            <span class="stat-num">{organizeReport.moved_count}</span>
          </div>

          {#if organizeReport.renamed_count > 0}
            <div class="stat-row warning">
              <span class="stat-label">
                <span class="stat-label-icon"><Icon name="edit" size={13} /></span>
                <span>Renombrados por colisión</span>
              </span>
              <span class="stat-num">{organizeReport.renamed_count}</span>
            </div>
          {/if}

          {#if organizeReport.error_count > 0}
            <div class="stat-row danger">
              <span class="stat-label">
                <span class="stat-label-icon"><Icon name="alert" size={13} /></span>
                <span>Errores</span>
              </span>
              <span class="stat-num">{organizeReport.error_count}</span>
            </div>
          {/if}
        </div>

        {#if organizeReport.errors.length > 0}
          <div class="error-list">
            <h4>Detalle de incidencias:</h4>
            <ul>
              {#each organizeReport.errors as err}
                <li>{err}</li>
              {/each}
            </ul>
          </div>
        {/if}
      {:else if undoReport}
        <div class="stats-list">
          <div class="stat-row success">
            <span class="stat-label">
              <span class="stat-label-icon"><Icon name="undo" size={13} /></span>
              <span>Archivos devueltos a la raíz</span>
            </span>
            <span class="stat-num">{undoReport.restored_count}</span>
          </div>

          {#if undoReport.error_count > 0}
            <div class="stat-row danger">
              <span class="stat-label">
                <span class="stat-label-icon"><Icon name="alert" size={13} /></span>
                <span>Errores</span>
              </span>
              <span class="stat-num">{undoReport.error_count}</span>
            </div>
          {/if}
        </div>

        {#if undoReport.errors.length > 0}
          <div class="error-list">
            <h4>Detalle de incidencias:</h4>
            <ul>
              {#each undoReport.errors as err}
                <li>{err}</li>
              {/each}
            </ul>
          </div>
        {/if}
      {/if}
    </div>

    <div class="modal-footer">
      <button class="btn-close" onclick={onClose}>Aceptar</button>
    </div>
  </div>
</div>
