export interface FileIconInfo {
  iconClass: string;
  colorClass: string;
}

/**
 * Mapea cada extensión a un icono específico de Font Awesome y su clase de color,
 * evitando iconos genéricos repetidos y facilitando la identificación visual estilo VS Code.
 */
export function getFileIconInfo(ext: string): FileIconInfo {
  const clean = ext.toLowerCase().replace(/^\./, "");

  if (clean === "pdf") {
    return { iconClass: "fa-solid fa-file-pdf", colorClass: "icon-pdf" };
  }
  if (["doc", "docx", "odt", "rtf"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-word", colorClass: "icon-word" };
  }
  if (["xls", "xlsx", "csv"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-excel", colorClass: "icon-excel" };
  }
  if (["ppt", "pptx"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-powerpoint", colorClass: "icon-ppt" };
  }
  if (["md", "markdown", "txt", "log"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-lines", colorClass: "icon-text" };
  }
  if (["zip", "rar", "7z", "tar", "gz", "bz2"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-zipper", colorClass: "icon-archive" };
  }
  if (
    ["png", "jpg", "jpeg", "webp", "gif", "svg", "bmp", "ico", "tiff", "avif", "imagenes"].includes(
      clean
    )
  ) {
    return { iconClass: "fa-solid fa-file-image", colorClass: "icon-image" };
  }
  if (["mp4", "mkv", "avi", "mov", "webm", "flv", "wmv"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-video", colorClass: "icon-video" };
  }
  if (["mp3", "wav", "flac", "ogg", "m4a", "aac"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-audio", colorClass: "icon-audio" };
  }
  if (["exe", "msi", "bat", "cmd", "ps1"].includes(clean)) {
    return { iconClass: "fa-brands fa-windows", colorClass: "icon-exec" };
  }
  if (["iso", "img"].includes(clean)) {
    return { iconClass: "fa-solid fa-compact-disc", colorClass: "icon-iso" };
  }
  if (["drawio", "mdj", "uml"].includes(clean)) {
    return { iconClass: "fa-solid fa-diagram-project", colorClass: "icon-diagram" };
  }
  if (["js", "ts", "py", "rs", "html", "css", "json", "xml", "sql", "yaml", "yml"].includes(clean)) {
    return { iconClass: "fa-solid fa-file-code", colorClass: "icon-code" };
  }

  return { iconClass: "fa-regular fa-file", colorClass: "icon-default" };
}

/**
 * Determina si la extensión corresponde a un archivo previsualizable como imagen.
 */
export function isImageFile(ext: string): boolean {
  const clean = ext.toLowerCase().replace(/^\./, "");
  return ["png", "jpg", "jpeg", "webp", "gif", "svg", "bmp", "ico"].includes(clean);
}

/**
 * Determina si la extensión corresponde a un archivo de video compatible.
 */
export function isVideoFile(ext: string): boolean {
  const clean = ext.toLowerCase().replace(/^\./, "");
  return ["mp4", "webm", "ogg"].includes(clean);
}
