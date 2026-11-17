<script lang="ts">
  import "../styles/icon.css";

  interface Props {
    name: string;
    size?: number;
    class?: string;
  }

  let { name, size = 14, class: className = "" }: Props = $props();

  const UI_ICONS: Record<string, string> = {
    folder: "fa-solid fa-folder-open",
    search: "fa-solid fa-magnifying-glass",
    undo: "fa-solid fa-rotate-left",
    edit: "fa-solid fa-pen",
    package: "fa-solid fa-box-archive",
    check: "fa-solid fa-check",
    close: "fa-solid fa-xmark",
    alert: "fa-solid fa-triangle-exclamation",
    "chevron-right": "fa-solid fa-chevron-right",
    "chevron-down": "fa-solid fa-chevron-down",
    "arrow-right": "fa-solid fa-arrow-right",
    document: "fa-regular fa-file",
  };

  let faClass = $derived(name.startsWith("fa-") ? name : (UI_ICONS[name] ?? "fa-regular fa-file"));
</script>

<i
  class="app-icon {faClass} {className}"
  style="font-size: {size}px; width: {size + 2}px;"
  aria-hidden="true"
></i>
