// Definición de tipos TypeScript sincronizados con los modelos de Rust (src-tauri/src/models.rs)

export interface FileItem {
  id: string;
  name: string;
  path: string;
  extension: string;
  group_key: string;
  size: number;
  created_at: string;
  modified_at: string;
  date_folder: string;
}

export interface CategoryGroup {
  key: string;
  label: string;
  extensions: string[];
  files: FileItem[];
  count: number;
  total_size: number;
}

export interface ScanResult {
  target_path: string;
  groups: CategoryGroup[];
  total_files: number;
  total_size: number;
}

export interface OrganizeReport {
  moved_count: number;
  renamed_count: number;
  error_count: number;
  errors: string[];
  ledger_id?: string;
}

export interface UndoReport {
  restored_count: number;
  error_count: number;
  errors: string[];
}
