# Estándares de Desarrollo del Proyecto — Organizador

Este documento define las directrices y buenas prácticas que rigen la arquitectura y el código de esta aplicación.

---

## 1. Simplicidad y Eficacia (KISS & YAGNI)
- **Soluciones directas:** No introducir patrones abstractos cuando una función pura o una estructura simple resuelve el problema.
- **Sin dependencias superfluas:** Cada crate o librería npm añadida debe estar estrictamente justificada.
- **Código legible antes que "inteligente":** Es preferible un código que se entienda a primera vista a un "one-liner" difícil de mantener.

---

## 2. Tamaño de Clases, Estructuras y Módulos (Principio de Responsabilidad Única)
- **Límite de tamaño:** Ningún archivo, clase o módulo debe superar preferiblemente las **100-150 líneas de código**.
- **Separación de estilos (CSS desacoplado):**
  - **No embeber bloques `<style>` dentro de los componentes `.svelte`:** Todos los estilos deben residir en archivos `.css` independientes y organizados dentro del directorio `src/styles/`.
  - Cada componente importa su hoja de estilo correspondiente (ej. `import "../styles/header.css";`).
  - Mantiene los archivos Svelte enfocados exclusivamente en la reactividad y estructura del markup.
- **División oportuna:**
  - Si un struct o clase acumula más de 3-4 responsabilidades distintas, se divide inmediatamente en submódulos especializados.
  - En el frontend (Svelte), cada componente maneja únicamente su parte visual y lógica directa (ej. `FileRow.svelte`, `CategoryGroup.svelte`, `ScanControls.svelte`, `UndoModal.svelte`).
  - En el backend (Rust), se separan estrictamente:
    - `scanner.rs`: Lectura de disco y extracción de metadatos.
    - `sorter.rs`: Categorización y agrupación de extensiones (imágenes unificadas, documentos, etc.).
    - `organizer.rs`: Creación de carpetas, resolución de colisiones y movimiento de archivos.
    - `ledger.rs`: Registro del historial de operaciones para la función Deshacer.
    - `models.rs`: Definición de tipos y estructuras de datos puras.

---

## 3. Estética y Reglas de Interfaz
- **Prohibido el uso de emojis en la interfaz:**
  - No utilizar emojis (como 📁, ✏️, 🔍, 📦, ↩, 🎉, ⚠️, ▶) en ningún componente, botón, etiqueta o modal.
  - Utilizar iconos vectoriales **SVG limpios** o glifos tipográficos estándar para garantizar una estética de aplicación nativa de escritorio profesional y consistente en cualquier sistema operativo.
- **Distribución de pantalla:**
  - Layout maestro-detalle: **70% a la izquierda** para la lista y agrupación de archivos y **30% a la derecha** para el panel lateral de detalles e inspección (previsualización de imágenes/videos, propiedades y metadatos).
- **Separación estricta de CSS:**
  - No embeber bloques `<style>` dentro de `.svelte`. Toda regla visual reside en archivos `.css` dentro de `src/styles/`.

---

## 4. Documentación y Comentarios Útiles
- **Enfocarse en el "por qué" y en las condiciones límite:**
  - Evitar comentarios redundantes que solo repiten el código (`// incrementa i en 1`).
  - Documentar contratos de funciones: qué espera de entrada, qué retorna y qué errores puede arrojar.
  - Señalar explícitamente decisiones clave (por ejemplo: exclusión de carpetas en escaneo, manejo de colisiones de nombre, permisos).
- **Auto-documentación con nombres expresivos:** Nombres de variables y funciones consistentes y descriptivos.

---

## 4. Prácticas Adicionales

### A. Manejo Seguro y Atómico de Archivos
- **Nunca sobrescribir datos:** Ante cualquier coincidencia de nombres, aplicar sufijos numéricos automáticos (`archivo (1).pdf`).
- **Comprobaciones preventivas:** Verificar existencia y permisos antes de mover.
- **Transaccionalidad (Undo Ledger):** Toda operación se escribe en un registro antes de ejecutarse para asegurar reversibilidad.

### B. Tipado Estricto de Extremo a Extremo
- **Backend (Rust):** Uso exhaustivo del sistema de tipos (`Result<T, AppError>`, `Option<T>`). Cero `unwrap()` en rutas de ejecución en producción.
- **Frontend (TypeScript):** Tipos sincronizados con los modelos de Rust (`FileEntry`, `GroupStats`, `OrganizeReport`). Cero uso de `any`.

### C. Detección Temprana y Pruebas Unitarias
- Pruebas unitarias automatizadas con carpetas temporales (`tempfile`) para validar la lógica sin tocar archivos reales.
