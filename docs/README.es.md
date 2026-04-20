# Omigdex

Una aplicación de descarga de videos de código abierto construida con Tauri, React, TypeScript y Rust. Descarga videos de YouTube, Instagram, TikTok y Pinterest con facilidad.

## Características

- **Soporte multiplataforma**: Descarga videos de YouTube, Instagram, TikTok y Pinterest
- **Múltiples formatos**: Descarga en formatos MP4 (video) o MP3 (audio)
- **Selección de calidad**: Elige entre Mejor, Alta (1080p), Media (720p) o Baja calidad
- **Cola de descargas**: Gestiona hasta 3 descargas simultáneas automáticamente
- **Historial de descargas**: Rastrea todas tus descargas con almacenamiento persistente
- **Tema claro/oscuro**: Alterna entre modos claro y oscuro
- **Notificaciones toast**: Feedback en tiempo real del estado de descarga
- **Interfaz minimalista**: Diseño limpio y moderno sin emojis
- **Versión portable**: Ejecuta sin instalación (incluye yt-dlp)
- **Descargas rápidas**: Optimizado para velocidad (video de 15 minutos en ~10 segundos)
- **Sistema de plugins Lua**: Extiende la funcionalidad con plugins personalizados

## Stack Tecnológico

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust con Tauri
- **Estilos**: Tailwind CSS + shadcn/ui
- **Descargador de videos**: yt-dlp (incrustado)
- **Sistema de build**: NSIS (instalador) + GitHub Actions

## Instalación

### Desde Release

1. Descarga el último release desde [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. Elige entre:
   - **Instalador (NSIS)**: Ejecuta el instalador `.exe`
   - **Portable**: Extrae el `.zip` y ejecuta `omigdex-app.exe`

### Desde Código Fuente

**Requisitos previos**:
- Node.js (v18 o superior)
- pnpm
- Toolchain de Rust
- yt-dlp (para desarrollo)

**Pasos**:

```bash
# Clona el repositorio
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Instala dependencias
pnpm install

# Ejecuta en modo desarrollo
pnpm tauri dev

# Construye para producción
pnpm tauri build
```

## Uso

1. **Descargar un video**:
   - Pega la URL del video en el campo de entrada
   - Selecciona formato (MP4 o MP3)
   - Selecciona calidad (Mejor, Alta, Media, Baja)
   - Haz clic en "Download"

2. **Gestionar descargas**:
   - Ver descargas activas en la pestaña "Queue"
   - Cancela descargas si es necesario
   - Limpia descargas completadas

3. **Ver historial**:
   - Cambia a la pestaña "History"
   - Ver todas las descargas pasadas
   - Elimina entradas individuales o limpia todo

4. **Alternar tema**:
   - Haz clic en el botón de tema en el encabezado
   - Alterna entre modos claro y oscuro

## Estructura del Proyecto

```
omigdex-app/
├── src/                    # Frontend React
│   ├── components/         # Componentes React
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # Componentes shadcn/ui
│   ├── lib/               # Utilidades
│   │   └── utils.ts
│   ├── App.tsx            # Componente principal
│   └── main.tsx           # Punto de entrada
├── src-tauri/             # Backend Rust
│   ├── src/
│   │   ├── main.rs        # Punto de entrada
│   │   ├── lib.rs         # Comandos Tauri
│   │   ├── downloader.rs  # Integración yt-dlp
│   │   ├── queue.rs       # Gestión de cola
│   │   ├── history.rs     # Historial persistente
│   │   └── types.rs       # Estructuras de datos
│   └── Cargo.toml         # Dependencias Rust
├── .github/
│   └── workflows/
│       └── build.yml      # Pipeline CI/CD
└── create-portable.bat    # Script de build portable
```

## Arquitectura del Backend Rust

El backend consta de varios módulos:

- **`main.rs`**: Punto de entrada de la aplicación
- **`lib.rs`**: Definiciones de comandos Tauri e inicialización
- **`downloader.rs`**: Integración con yt-dlp para descargar videos
- **`queue.rs`**: Cola de descargas con gestión concurrente
- **`history.rs`**: Historial de descargas persistente en JSON
- **`types.rs`**: Estructuras de datos y enums

## Desarrollo

**Ejecutar servidor de desarrollo**:
```bash
pnpm tauri dev
```

**Ejecutar solo frontend** (sin backend):
```bash
npm run dev
```

**Construir instalador**:
```bash
pnpm tauri build
```

**Crear versión portable**:
```bash
create-portable.bat
```

## Construir Releases

Los releases se construyen automáticamente mediante GitHub Actions cuando envías un tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

El workflow crea:
- Instalador NSIS con yt-dlp incluido
- ZIP portable con yt-dlp incluido

## Problemas Conocidos

- **Advertencias de antivirus**: Los ejecutables sin firmar pueden activar Windows Defender u otro software antivirus. Esto es normal para aplicaciones sin firmar. Considera agregar una excepción o usar la versión portable.
- **Firma de código**: Para evitar advertencias de antivirus, la aplicación necesita una firma digital. Para proyectos de código abierto, [SignPath.io](https://signpath.io/) ofrece firma de código gratuita.

## Contribución

¡Las contribuciones son bienvenidas! No dudes en enviar un Pull Request.

## Licencia

Este proyecto es de código abierto y está disponible bajo la Licencia GPL-3.0.

## Agradecimientos

- [Tauri](https://tauri.app/) - Framework de escritorio multiplataforma
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Descargador de videos
- [Tailwind CSS](https://tailwindcss.com/) - Framework CSS
- [shadcn/ui](https://ui.shadcn.com/) - Componentes UI

## Enlaces

- [Repositorio GitHub](https://github.com/Cr12dev/omigdex)
- [Releases](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Configuración de IDE Recomendada

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
