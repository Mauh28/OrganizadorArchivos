use serde::{Deserialize, Serialize};

/// Representa un archivo individual encontrado en el primer nivel del directorio escaneado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub id: String,
    pub name: String,
    pub path: String,
    pub extension: String,
    pub group_key: String,
    pub size: u64,
    pub created_at: String,
    pub modified_at: String,
    pub date_folder: String, // Formato YYYY-MM-DD según última modificación
}

/// Grupo de archivos clasificados (por extensión individual o por categoría unificada como imágenes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryGroup {
    pub key: String,
    pub label: String,
    pub extensions: Vec<String>,
    pub files: Vec<FileItem>,
    pub count: usize,
    pub total_size: u64,
}

/// Resultado completo de un escaneo de directorio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub target_path: String,
    pub groups: Vec<CategoryGroup>,
    pub total_files: usize,
    pub total_size: u64,
}

/// Registro de un archivo movido para permitir auditoría y reversión (Deshacer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAction {
    pub original_path: String,
    pub destination_path: String,
}

/// Registro histórico de una operación de empaquetado.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerRecord {
    pub id: String,
    pub timestamp: String,
    pub base_dir: String,
    pub actions: Vec<MoveAction>,
}

/// Resumen devuelto tras ejecutar el empaquetado de archivos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizeReport {
    pub moved_count: usize,
    pub renamed_count: usize,
    pub error_count: usize,
    pub errors: Vec<String>,
    pub ledger_id: Option<String>,
}

/// Resumen devuelto tras ejecutar la reversión (Deshacer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoReport {
    pub restored_count: usize,
    pub error_count: usize,
    pub errors: Vec<String>,
}
