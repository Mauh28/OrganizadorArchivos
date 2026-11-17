use std::env;
use std::path::PathBuf;

use crate::ledger::{has_undo, undo_last_organize};
use crate::models::{FileItem, OrganizeReport, ScanResult, UndoReport};
use crate::organizer::organize_files;
use crate::scanner::scan_directory;

/// Obtiene la ruta por defecto de la carpeta Descargas del usuario de Windows.
#[tauri::command]
pub fn get_default_downloads_dir() -> String {
    if let Ok(profile) = env::var("USERPROFILE") {
        let downloads = PathBuf::from(profile).join("Downloads");
        if downloads.exists() {
            return downloads.to_string_lossy().to_string();
        }
    }
    r"C:\Users\Mauro\Downloads".to_string()
}

/// Comando Tauri para escanear los archivos sueltos en el primer nivel.
#[tauri::command]
pub fn scan_folder(path: String) -> Result<ScanResult, String> {
    scan_directory(&path)
}

/// Comando Tauri para empaquetar los archivos seleccionados en sus carpetas por extensión y fecha.
#[tauri::command]
pub fn organize_selected(base_dir: String, files: Vec<FileItem>) -> Result<OrganizeReport, String> {
    organize_files(&base_dir, files)
}

/// Verifica si hay operaciones previas disponibles para deshacer.
#[tauri::command]
pub fn check_undo_available(base_dir: String) -> bool {
    has_undo(&base_dir)
}

/// Revierte el último empaquetado efectuado devolviendo los archivos a su ubicación original.
#[tauri::command]
pub fn undo_last(base_dir: String) -> Result<UndoReport, String> {
    undo_last_organize(&base_dir)
}

/// Lee una imagen del disco y devuelve su Data URL en base64 para previsualización inmediata.
#[tauri::command]
pub fn get_image_data_url(path: String) -> Result<String, String> {
    let p = std::path::Path::new(&path);
    if !p.exists() || !p.is_file() {
        return Err("El archivo no existe".to_string());
    }

    let metadata = std::fs::metadata(p).map_err(|e| e.to_string())?;
    // Limitar base64 a archivos menores a 15 MB para optimizar rendimiento
    if metadata.len() > 15 * 1024 * 1024 {
        return Err("El archivo es demasiado grande para previsualizar".to_string());
    }

    let bytes = std::fs::read(p).map_err(|e| format!("Error leyendo archivo: {}", e))?;
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "bmp" => "image/bmp",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    };

    const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut b64 = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };
        b64.push(B64_CHARS[(b0 >> 2) as usize] as char);
        b64.push(B64_CHARS[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            b64.push(B64_CHARS[(((b1 & 15) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            b64.push('=');
        }
        if chunk.len() > 2 {
            b64.push(B64_CHARS[(b2 & 63) as usize] as char);
        } else {
            b64.push('=');
        }
    }

    Ok(format!("data:{};base64,{}", mime, b64))
}
