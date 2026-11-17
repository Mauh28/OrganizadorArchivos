<script lang="ts">
  import { onMount } from "svelte";
  import { SvelteSet } from "svelte/reactivity";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import "../styles/theme.css";
  import "../styles/page.css";
  import Header from "../lib/Header.svelte";
  import CategoryCard from "../lib/CategoryCard.svelte";
  import FileDetailPanel from "../lib/FileDetailPanel.svelte";
  import ActionBar from "../lib/ActionBar.svelte";
  import ReportModal from "../lib/ReportModal.svelte";
  import Icon from "../lib/Icon.svelte";
  import {
    checkUndoAvailable,
    getDefaultDownloadsDir,
    organizeSelected,
    scanFolder,
    undoLast,
  } from "../lib/api";
  import type { FileItem, OrganizeReport, ScanResult, UndoReport } from "../lib/types";

  let folderPath = $state("");
  let isScanning = $state(false);
  let isOrganizing = $state(false);
  let canUndo = $state(false);
  let errorMessage = $state<string | null>(null);

  let scanResult = $state<ScanResult | null>(null);
  let selectedIds = new SvelteSet<string>();
  let activeFile = $state<FileItem | null>(null);

  let organizeReport = $state<OrganizeReport | null>(null);
  let undoReport = $state<UndoReport | null>(null);

  // Archivos planos acumulados para conteo y selección
  let allFiles = $derived(
    scanResult ? scanResult.groups.flatMap((g) => g.files) : []
  );

  let selectedFiles = $derived(
    allFiles.filter((f) => selectedIds.has(f.id))
  );

  let selectedBytes = $derived(
    selectedFiles.reduce((sum, f) => sum + f.size, 0)
  );

  onMount(async () => {
    try {
      folderPath = await getDefaultDownloadsDir();
      await handleScan();
    } catch (e) {
      errorMessage = String(e);
    }
  });

  async function handleScan() {
    if (!folderPath) return;
    isScanning = true;
    errorMessage = null;

    try {
      scanResult = await scanFolder(folderPath);
      selectedIds.clear();
      activeFile = scanResult.groups[0]?.files[0] ?? null;
      canUndo = await checkUndoAvailable(folderPath);
    } catch (e) {
      errorMessage = String(e);
      scanResult = null;
      activeFile = null;
    } finally {
      isScanning = false;
    }
  }

  function handleToggleFile(id: string) {
    if (selectedIds.has(id)) {
      selectedIds.delete(id);
    } else {
      selectedIds.add(id);
    }
  }

  function handleInspectFile(file: FileItem) {
    activeFile = file;
  }

  function handleToggleGroup(groupKey: string) {
    if (!scanResult) return;
    const group = scanResult.groups.find((g) => g.key === groupKey);
    if (!group) return;

    const groupIds = group.files.map((f) => f.id);
    const allSelected = groupIds.every((id) => selectedIds.has(id));

    if (allSelected) {
      groupIds.forEach((id) => selectedIds.delete(id));
    } else {
      groupIds.forEach((id) => selectedIds.add(id));
    }
  }

  function handleSelectAll() {
    allFiles.forEach((f) => selectedIds.add(f.id));
  }

  function handleDeselectAll() {
    selectedIds.clear();
  }

  async function handleOrganize() {
    if (selectedFiles.length === 0 || !folderPath) return;
    isOrganizing = true;

    try {
      organizeReport = await organizeSelected(folderPath, selectedFiles);
      await handleScan();
    } catch (e) {
      errorMessage = String(e);
    } finally {
      isOrganizing = false;
    }
  }

  async function handleUndo() {
    if (!folderPath) return;
    try {
      undoReport = await undoLast(folderPath);
      await handleScan();
    } catch (e) {
      errorMessage = String(e);
    }
  }

  function closeReportModal() {
    organizeReport = null;
    undoReport = null;
  }
</script>

<div class="app-layout">
  <Header
    bind:folderPath
    {isScanning}
    {canUndo}
    onScan={handleScan}
    onUndo={handleUndo}
    onChangePath={(newPath) => {
      folderPath = newPath;
      handleScan();
    }}
  />

  <main class="content">
    {#if errorMessage}
      <div class="alert danger">
        <Icon name="alert" size={16} />
        <span>{errorMessage}</span>
      </div>
    {/if}

    {#if isScanning}
      <div class="loading-state">
        <div class="big-spinner"></div>
        <p>Analizando archivos del primer nivel...</p>
      </div>
    {:else if scanResult}
      {#if scanResult.total_files === 0}
        <div class="empty-state">
          <div class="empty-icon-wrap">
            <Icon name="check" size={32} />
          </div>
          <h3>No hay archivos sueltos para organizar</h3>
          <p>
            No se encontraron archivos en la raíz de <code>{scanResult.target_path}</code>.
            Las carpetas existentes no fueron alteradas.
          </p>
        </div>
      {:else}
        <div class="workspace-grid">
          <section class="groups-pane">
            {#each scanResult.groups as group (group.key)}
              <CategoryCard
                {group}
                {selectedIds}
                activeFileId={activeFile?.id ?? null}
                onToggleGroup={handleToggleGroup}
                onToggleFile={handleToggleFile}
                onInspect={handleInspectFile}
              />
            {/each}
          </section>

          <section class="details-pane">
            <FileDetailPanel
              file={activeFile}
              selected={activeFile ? selectedIds.has(activeFile.id) : false}
              onToggleSelect={handleToggleFile}
            />
          </section>
        </div>
      {/if}
    {/if}
  </main>

  {#if scanResult && scanResult.total_files > 0}
    <ActionBar
      selectedCount={selectedIds.size}
      {selectedBytes}
      totalCount={scanResult.total_files}
      {isOrganizing}
      onSelectAll={handleSelectAll}
      onDeselectAll={handleDeselectAll}
      onOrganize={handleOrganize}
    />
  {/if}

  {#if organizeReport || undoReport}
    <ReportModal
      {organizeReport}
      {undoReport}
      onClose={closeReportModal}
    />
  {/if}
</div>
