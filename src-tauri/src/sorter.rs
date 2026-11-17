use std::time::SystemTime;
use chrono::{DateTime, Local};

/// Extensiones de imagen que se unifican bajo la carpeta 'imagenes'.
const IMAGE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "gif", "bmp", "svg", "ico", "tiff", "tif", "avif", "heic",
];

/// Archivos y extensiones temporales o de sistema que deben ignorarse en el escaneo.
const IGNORED_EXTENSIONS: &[&str] = &["crdownload", "part", "tmp", "download"];
const IGNORED_FILENAMES: &[&str] = &["desktop.ini", "thumbs.db", ".ds_store"];

/// Determina si un archivo debe ser ignorado por ser temporal o del sistema operativo.
pub fn is_ignored_file(filename: &str, ext: &str) -> bool {
    let lower_name = filename.to_lowercase();
    let lower_ext = ext.to_lowercase();

    IGNORED_FILENAMES.contains(&lower_name.as_str())
        || IGNORED_EXTENSIONS.contains(&lower_ext.as_str())
}

/// Clasifica una extensión en su clave de grupo y etiqueta legible.
/// Las imágenes se agrupan en 'imagenes'. Las demás usan su propia extensión en minúsculas.
pub fn classify_extension(ext: &str) -> (String, String) {
    let clean_ext = ext.trim().trim_start_matches('.').to_lowercase();

    if clean_ext.is_empty() {
        return ("sin_extension".to_string(), "Sin extensión".to_string());
    }

    if IMAGE_EXTENSIONS.contains(&clean_ext.as_str()) {
        ("imagenes".to_string(), "Imágenes".to_string())
    } else {
        let label = clean_ext.to_uppercase();
        (clean_ext, label)
    }
}

/// Convierte un SystemTime a fecha legible local (YYYY-MM-DD HH:MM:SS) y a carpeta (YYYY-MM-DD).
pub fn format_system_time(time: Option<SystemTime>) -> (String, String) {
    match time {
        Some(t) => {
            let datetime: DateTime<Local> = t.into();
            (
                datetime.format("%Y-%m-%d %H:%M").to_string(),
                datetime.format("%Y-%m-%d").to_string(),
            )
        }
        None => ("Desconocida".to_string(), "sin_fecha".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_images() {
        let (key_png, label_png) = classify_extension("png");
        let (key_jpg, label_jpg) = classify_extension(".JPG");
        assert_eq!(key_png, "imagenes");
        assert_eq!(key_jpg, "imagenes");
        assert_eq!(label_png, "Imágenes");
        assert_eq!(label_jpg, "Imágenes");
    }

    #[test]
    fn test_classify_standard_extensions() {
        let (key_pdf, label_pdf) = classify_extension("pdf");
        assert_eq!(key_pdf, "pdf");
        assert_eq!(label_pdf, "PDF");

        let (key_docx, label_docx) = classify_extension(".DOCX");
        assert_eq!(key_docx, "docx");
        assert_eq!(label_docx, "DOCX");
    }

    #[test]
    fn test_classify_empty_extension() {
        let (key, label) = classify_extension("");
        assert_eq!(key, "sin_extension");
        assert_eq!(label, "Sin extensión");
    }

    #[test]
    fn test_ignored_files() {
        assert!(is_ignored_file("archivo.crdownload", "crdownload"));
        assert!(is_ignored_file("desktop.ini", "ini"));
        assert!(!is_ignored_file("documento.pdf", "pdf"));
    }
}
