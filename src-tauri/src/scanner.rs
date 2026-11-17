use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::models::{CategoryGroup, FileItem, ScanResult};
use crate::sorter::{classify_extension, format_system_time, is_ignored_file};

/// Escanea únicamente los archivos que se encuentran directamente en el primer nivel del directorio.
/// Las subcarpetas se descartan intencionadamente para no alterar estructuras ya organizadas.
pub fn scan_directory(dir_path: &str) -> Result<ScanResult, String> {
    let path = Path::new(dir_path);

    if !path.exists() {
        return Err(format!("El directorio '{}' no existe.", dir_path));
    }
    if !path.is_dir() {
        return Err(format!("La ruta '{}' no es una carpeta válida.", dir_path));
    }

    let read_dir = fs::read_dir(path)
        .map_err(|e| format!("No se pudo leer el directorio: {}", e))?;

    let mut groups_map: HashMap<String, (String, Vec<String>, Vec<FileItem>, u64)> = HashMap::new();
    let mut total_files: usize = 0;
    let mut total_size: u64 = 0;

    for entry in read_dir.flatten() {
        // Validación estricta: Solo procesar archivos regulares, omitir carpetas y enlaces a carpetas
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if !file_type.is_file() {
            continue;
        }

        let file_path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();
        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_string();

        if is_ignored_file(&filename, &ext) {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size = metadata.len();
        let (created_at, _) = format_system_time(metadata.created().ok());
        let (modified_at, date_folder) = format_system_time(metadata.modified().ok());

        let (group_key, group_label) = classify_extension(&ext);

        let item = FileItem {
            id: file_path.to_string_lossy().to_string(),
            name: filename,
            path: file_path.to_string_lossy().to_string(),
            extension: ext.to_lowercase(),
            group_key: group_key.clone(),
            size,
            created_at,
            modified_at,
            date_folder,
        };

        total_files += 1;
        total_size += size;

        let entry_group = groups_map
            .entry(group_key.clone())
            .or_insert_with(|| (group_label, Vec::new(), Vec::new(), 0));

        if !ext.is_empty() && !entry_group.1.contains(&ext.to_lowercase()) {
            entry_group.1.push(ext.to_lowercase());
        }
        entry_group.2.push(item);
        entry_group.3 += size;
    }

    // Convertir el mapa a un vector ordenado por cantidad de archivos descendente
    let mut groups: Vec<CategoryGroup> = groups_map
        .into_iter()
        .map(|(key, (label, extensions, mut files, size_sum))| {
            // Ordenar los archivos internamente del más reciente al más antiguo
            files.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
            CategoryGroup {
                key,
                label,
                extensions,
                count: files.len(),
                total_size: size_sum,
                files,
            }
        })
        .collect();

    groups.sort_by(|a, b| b.count.cmp(&a.count));

    Ok(ScanResult {
        target_path: dir_path.to_string(),
        groups,
        total_files,
        total_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_scan_only_top_level_files() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        // Crear archivos sueltos
        File::create(base.join("nota.txt")).unwrap();
        File::create(base.join("foto.png")).unwrap();

        // Crear una subcarpeta con un archivo adentro (debe ser ignorado)
        let sub = base.join("subcarpeta");
        fs::create_dir(&sub).unwrap();
        File::create(sub.join("dentro.txt")).unwrap();

        let result = scan_directory(base.to_str().unwrap()).unwrap();

        // Solo debe encontrar 2 archivos (nota.txt y foto.png)
        assert_eq!(result.total_files, 2);
        assert_eq!(result.groups.len(), 2);
    }
}
