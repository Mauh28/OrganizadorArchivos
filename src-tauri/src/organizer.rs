use std::fs;
use std::path::{Path, PathBuf};

use crate::ledger::save_ledger;
use crate::models::{FileItem, MoveAction, OrganizeReport};

/// Resuelve colisiones de nombre si ya existe un archivo con el mismo nombre en la carpeta destino.
/// Genera sufijos secuenciales: "archivo (1).ext", "archivo (2).ext", etc.
pub fn resolve_collision_path(target_folder: &Path, original_name: &str) -> (PathBuf, bool) {
    let direct_path = target_folder.join(original_name);
    if !direct_path.exists() {
        return (direct_path, false);
    }

    let stem = Path::new(original_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(original_name);

    let ext = Path::new(original_name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!(".{}", e))
        .unwrap_or_default();

    let mut counter = 1;
    loop {
        let candidate_name = format!("{} ({}){}", stem, counter, ext);
        let candidate_path = target_folder.join(&candidate_name);
        if !candidate_path.exists() {
            return (candidate_path, true);
        }
        counter += 1;
    }
}

/// Ejecuta el empaquetado de los archivos seleccionados hacia sus respectivas carpetas por extensión y fecha.
pub fn organize_files(base_dir: &str, files: Vec<FileItem>) -> Result<OrganizeReport, String> {
    let base_path = Path::new(base_dir);
    if !base_path.exists() || !base_path.is_dir() {
        return Err(format!("El directorio base '{}' no es válido.", base_dir));
    }

    let mut moved_count = 0;
    let mut renamed_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();
    let mut successful_actions = Vec::new();

    for file in files {
        let src_path = Path::new(&file.path);
        if !src_path.exists() {
            error_count += 1;
            errors.push(format!("El archivo ya no existe: {}", file.name));
            continue;
        }

        // Estructura destino: base_dir / <extension_o_imagenes> / <YYYY-MM-DD>
        let target_folder = base_path.join(&file.group_key).join(&file.date_folder);

        if let Err(e) = fs::create_dir_all(&target_folder) {
            error_count += 1;
            errors.push(format!("No se pudo crear carpeta para {}: {}", file.name, e));
            continue;
        }

        let (dest_path, was_renamed) = resolve_collision_path(&target_folder, &file.name);

        match fs::rename(src_path, &dest_path) {
            Ok(_) => {
                moved_count += 1;
                if was_renamed {
                    renamed_count += 1;
                }
                successful_actions.push(MoveAction {
                    original_path: file.path,
                    destination_path: dest_path.to_string_lossy().to_string(),
                });
            }
            Err(e) => {
                error_count += 1;
                errors.push(format!("Error al mover {}: {}", file.name, e));
            }
        }
    }

    // Guardar ledger histórico para permitir deshacer
    let ledger_id = match save_ledger(base_dir, successful_actions) {
        Ok(id) => Some(id),
        Err(e) => {
            errors.push(format!("Aviso: no se pudo guardar historial para deshacer: {}", e));
            None
        }
    };

    Ok(OrganizeReport {
        moved_count,
        renamed_count,
        error_count,
        errors,
        ledger_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_collision() {
        let dir = tempdir().unwrap();
        let folder = dir.path();

        // Crear "prueba.txt" existente
        File::create(folder.join("prueba.txt")).unwrap();

        let (path1, renamed1) = resolve_collision_path(folder, "prueba.txt");
        assert!(renamed1);
        assert_eq!(path1.file_name().unwrap(), "prueba (1).txt");

        // Crear "prueba (1).txt" también
        File::create(&path1).unwrap();

        let (path2, renamed2) = resolve_collision_path(folder, "prueba.txt");
        assert!(renamed2);
        assert_eq!(path2.file_name().unwrap(), "prueba (2).txt");
    }
}
