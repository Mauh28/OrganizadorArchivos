# Organizador de Descargas

Aplicación de escritorio moderna, minimalista y de alto rendimiento creada con **Tauri 2 (Rust)** y **Svelte 5 (TypeScript)** para organizar archivos acumulados en la carpeta de descargas sin tocar carpetas previas.

---

## Características principales

1. **Escaneo inteligente de primer nivel:**
   - Detecta automáticamente la carpeta de descargas del usuario activo (`Users>Nombre>Descargas`, o la ruta que configures).
   - Analiza **únicamente** los archivos sueltos en la raíz de dicha carpeta e ignora automáticamente todas las subcarpetas preexistentes para proteger tus proyectos y estructuras previas.
   - Descarta archivos temporales (`.crdownload`, `.part`, `.tmp`) y de sistema (`desktop.ini`).

2. **Organización por extensión y fecha:**
   - **Extensiones limpias:** Carpetas en minúsculas sin punto (ej. `pdf/`, `docx/`, `exe/`, `zip/`).
   - **Consolidación de imágenes:** Todas las extensiones de imagen (`png`, `jpg`, `jpeg`, `webp`, `gif`, `svg`, etc.) se reúnen en una única carpeta central: `imagenes/`.
   - **Subcarpetas por fecha de modificación:** Cada archivo se ubica dentro de `YYYY-MM-DD/` según su última fecha de modificación.

3. **Seguridad y resolución de colisiones:**
   - **Cero pérdida de datos:** Si ya existe un archivo con el mismo nombre en la carpeta de destino, se renombra automáticamente con un sufijo secuencial: `documento (1).pdf`.
   - Se utiliza `rename` nativo del sistema de archivos: instantáneo en la misma unidad y preserva las fechas de creación/modificación originales.

4. **Botón Deshacer (Reversión completa):**
   - Cada empaquetado genera un registro de transacciones (*ledger*).
   - Con un clic en **"Deshacer"** todos los archivos vuelven a su posición exacta original y las carpetas vacías se limpian.

5. **Interfaz estilo Visual Studio Code (70% / 30%) con tema automático:**
   - Detecta automáticamente el tema del sistema (Claro / Oscuro) de Windows mediante `@media (prefers-color-scheme)`.
   - **70% izquierda:** Explorador agrupado por extensión con iconos vectoriales de **Font Awesome** diferenciados por formato.
   - **30% derecha:** Panel de detalles del archivo seleccionado con previsualización de imágenes/videos, metadatos y ruta de destino planificada.
   - Barra de estado inferior con conteo de seleccionados, tamaño total y botón para empaquetar.

---

## Estructura del código

Diseñado bajo el principio de responsabilidad única y módulos pequeños (< 100-150 líneas por archivo):

```
organizador/
├── src/                          # Frontend en Svelte 5 + TypeScript
│   ├── lib/
│   │   ├── types.ts              # Modelos tipados sincronizados con Rust
│   │   ├── api.ts                # Clientes para invoke() y formateadores
│   │   ├── fileIcons.ts          # Mapeo de extensiones a iconos Font Awesome
│   │   ├── Icon.svelte           # Componente de iconos vectoriales
│   │   ├── Header.svelte         # Barra superior con ruta, escaneo y deshacer
│   │   ├── CategoryCard.svelte   # Secciones colapsables por extensión
│   │   ├── FileRow.svelte        # Fila de archivo individual con fechas y tamaño
│   │   ├── FileDetailPanel.svelte# Panel lateral derecho con propiedades y vista previa
│   │   ├── ActionBar.svelte      # Barra inferior de estado y empaquetado
│   │   └── ReportModal.svelte    # Modal con resumen de archivos movidos y errores
│   ├── styles/                   # Hojas de estilo CSS desacopladas por componente
│   │   ├── theme.css             # Paleta VS Code Light/Dark Modern
│   │   ├── page.css              # Distribución 70/30 maestro-detalle
│   │   ├── header.css
│   │   ├── category-card.css
│   │   ├── file-row.css
│   │   ├── details-panel.css
│   │   ├── action-bar.css
│   │   ├── report-modal.css
│   │   └── icon.css
│   └── routes/
│       └── +page.svelte          # Vista principal reactiva
│
├── src-tauri/                    # Backend en Rust (Alto rendimiento)
│   ├── src/
│   │   ├── models.rs             # Structs de datos puros (FileItem, ScanResult, etc.)
│   │   ├── sorter.rs             # Clasificación de extensiones y fechas
│   │   ├── scanner.rs            # Lectura del primer nivel del directorio
│   │   ├── organizer.rs          # Creación de carpetas y resolución de nombres
│   │   ├── ledger.rs             # Historial y reversión transaccional
│   │   ├── commands.rs           # Comandos expuestos a Tauri
│   │   ├── lib.rs                # Registro de handlers y plugins
│   │   └── main.rs               # Entrada de la aplicación nativa
│   └── Cargo.toml
│
├── CODING_STANDARDS.md           # Estándares de desarrollo aplicados
└── package.json
```

---

## Cómo ejecutar la aplicación

### Modo Desarrollo (con recarga en vivo)
```powershell
npm run tauri dev
```

### Ejecutar el binario ya compilado directamente
El ejecutable compilado se encuentra en:
- Versión optimizada (Release): `src-tauri\target\release\organizador.exe`
- Versión de depuración (Debug): `src-tauri\target\debug\organizador.exe`

### Generar instalador de Windows (.msi / .exe)
```powershell
npm run tauri build
```
