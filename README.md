# Organizador de Descargas

Aplicación de escritorio moderna, minimalista y de alto rendimiento creada con **Tauri 2 (Rust)** y **Svelte 5 (TypeScript)** para organizar archivos acumulados en la carpeta de descargas sin tocar carpetas previas.

---

## 🚀 Características principales

1. **Escaneo inteligente de primer nivel:**
   - Analiza **únicamente** los archivos sueltos en `C:\Users\Mauro\Downloads` (o la carpeta que elijas).
   - Ignora automáticamente todas las carpetas preexistentes para proteger tus proyectos y estructuras previas.
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
   - Si te arrepientes, con un clic en **"↩ Deshacer"** todos los archivos vuelven a su posición exacta original y las carpetas vacías se limpian.

5. **Diseño minimalista con tema automático:**
   - Detecta automáticamente el tema del sistema (Claro / Oscuro) de Windows mediante `@media (prefers-color-scheme)`.
   - Barra superior con selector/editor de ruta y botón de escaneo.
   - Tarjetas colapsables por grupo con selección rápida (todo el grupo o archivos individuales).
   - Barra inferior fija con conteo de archivos seleccionados, tamaño total y botón para empaquetar.

---

## 🛠️ Estructura del código

Diseñado bajo el principio de responsabilidad única y módulos pequeños (< 100-150 líneas por archivo):

```
organizador/
├── src/                          # Frontend en Svelte 5 + TypeScript
│   ├── lib/
│   │   ├── types.ts              # Modelos tipados sincronizados con Rust
│   │   ├── api.ts                # Clientes para invoke() y formateadores
│   │   ├── theme.css             # Variables CSS para modo claro y oscuro
│   │   ├── Header.svelte         # Barra superior con ruta, escaneo y deshacer
│   │   ├── CategoryCard.svelte   # Tarjetas colapsables por extensión
│   │   ├── FileRow.svelte        # Fila de archivo individual con fechas y tamaño
│   │   ├── ActionBar.svelte      # Barra inferior con botón de empaquetado
│   │   └── ReportModal.svelte    # Modal con resumen de archivos movidos y errores
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

## 💻 Cómo ejecutar la aplicación

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
