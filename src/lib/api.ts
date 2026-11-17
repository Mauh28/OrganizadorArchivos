import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import type { FileItem, OrganizeReport, ScanResult, UndoReport } from "./types";

export { convertFileSrc };

/**
 * Obtiene la previsualización en Data URL base64 de una imagen local.
 */
export async function getImageDataUrl(path: string): Promise<string> {
  return await invoke<string>("get_image_data_url", { path });
}

/**
 * Obtiene la ruta por defecto de la carpeta Descargas desde el backend.
 */
export async function getDefaultDownloadsDir(): Promise<string> {
  return await invoke<string>("get_default_downloads_dir");
}

/**
 * Escanea los archivos sueltos en el primer nivel del directorio dado.
 */
export async function scanFolder(path: string): Promise<ScanResult> {
  return await invoke<ScanResult>("scan_folder", { path });
}

/**
 * Empaqueta los archivos seleccionados en sus respectivas carpetas por extensión y fecha.
 */
export async function organizeSelected(
  baseDir: string,
  files: FileItem[]
): Promise<OrganizeReport> {
  return await invoke<OrganizeReport>("organize_selected", {
    baseDir,
    files,
  });
}

/**
 * Comprueba si existe un registro de empaquetado previo para habilitar la reversión.
 */
export async function checkUndoAvailable(baseDir: string): Promise<boolean> {
  return await invoke<boolean>("check_undo_available", { baseDir });
}

/**
 * Revierte el último empaquetado efectuado devolviendo los archivos a su lugar original.
 */
export async function undoLast(baseDir: string): Promise<UndoReport> {
  return await invoke<UndoReport>("undo_last", { baseDir });
}

/**
 * Formatea bytes a una cadena legible (B, KB, MB, GB).
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}

