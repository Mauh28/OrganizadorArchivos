use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::models::{LedgerRecord, MoveAction, UndoReport};

/// Obtiene la ruta del archivo que almacena el último historial de movimientos para ese directorio.
fn get_ledger_file_path(base_dir: &str) -> PathBuf {
    Path::new(base_dir).join(".organizador_ledger.json")
}

/// Guarda el registro de operaciones del empaquetado para permitir la función Deshacer.
pub fn save_ledger(base_dir: &str, actions: Vec<MoveAction>) -> Result<String, String> {
    if actions.is_empty() {
        return Ok("vacio".to_string());
    }

    let id = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let record = LedgerRecord {
        id: id.clone(),
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        base_dir: base_dir.to_string(),
        actions,
    };

    let ledger_path = get_ledger_file_path(base_dir);
    let json = serde_json::to_string_pretty(&record)
        .map_err(|e| format!("Error serializando ledger: {}", e))?;

    fs::write(&ledger_path, json)
        .map_err(|e| format!("Error guardando ledger de deshacer: {}", e))?;

    Ok(id)
}

/// Verifica si existe una operación previa pendiente de poder deshacer.
pub fn has_undo(base_dir: &str) -> bool {
    get_ledger_file_path(base_dir).exists()
}

/// Revierte el último empaquetado registrado, devolviendo los archivos a su posición original.
pub fn undo_last_organize(base_dir: &str) -> Result<UndoReport, String> {
    let ledger_path = get_ledger_file_path(base_dir);
    if !ledger_path.exists() {
        return Err("No hay ninguna operación de empaquetado reciente para deshacer.".to_string());
    }

    let content = fs::read_to_string(&ledger_path)
        .map_err(|e| format!("No se pudo leer el archivo de historial: {}", e))?;

    let record: LedgerRecord = serde_json::from_str(&content)
        .map_err(|e| format!("El formato del historial es inválido: {}", e))?;

    let mut restored_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for action in &record.actions {
        let dest = Path::new(&action.destination_path);
        let orig = Path::new(&action.original_path);

        if !dest.exists() {
            error_count += 1;
            errors.push(format!("No se encontró en destino: {}", dest.display()));
            continue;
        }

        // Mover de vuelta a la raíz de origen
        if let Err(e) = fs::rename(dest, orig) {
            error_count += 1;
            errors.push(format!("Error al restaurar {}: {}", orig.display(), e));
        } else {
            restored_count += 1;
            // Limpiar carpeta contenedora si quedó vacía (fecha y grupo)
            clean_empty_parent_dirs(dest, Path::new(base_dir));
        }
    }

    // Eliminar el archivo ledger para no repetir la reversión
    let _ = fs::remove_file(&ledger_path);

    Ok(UndoReport {
        restored_count,
        error_count,
        errors,
    })
}

/// Elimina carpetas vacías generadas tras devolver los archivos, sin sobrepasar la carpeta base.
fn clean_empty_parent_dirs(file_path: &Path, base_dir: &Path) {
    let mut current = file_path.parent();
    while let Some(dir) = current {
        if dir == base_dir {
            break;
        }
        if let Ok(mut entries) = fs::read_dir(dir) {
            if entries.next().is_none() {
                let _ = fs::remove_dir(dir);
            } else {
                break;
            }
        } else {
            break;
        }
        current = dir.parent();
    }
}
