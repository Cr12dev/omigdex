# Omigdex

An open-source video downloader application built with Tauri, React, TypeScript, and Rust. Download videos from YouTube, Instagram, TikTok, and Pinterest with ease.

## Languages

- [English](README.md)
- [Español](docs/README.es.md)
- [Italiano](docs/README.it.md)
- [Français](docs/README.fr.md)
- [Deutsch](docs/README.de.md)
- [Polski](docs/README.pl.md)
- [中文](docs/README.zh.md)
- [Русский](docs/README.ru.md)
- [Suomi](docs/README.fi.md)
- [العربية](docs/README.ar.md)
- [日本語](docs/README.ja.md)
- [Norsk](docs/README.no.md)
- [Ελληνικά](docs/README.el.md)
- [Türkçe](docs/README.tr.md)

## Features

- **Multi-platform support**: Download videos from YouTube, Instagram, TikTok, and Pinterest
- **Multiple formats**: Download in MP4 (video) or MP3 (audio) formats
- **Quality selection**: Choose from Best, High (1080p), Medium (720p), or Low quality
- **Download queue**: Manage up to 3 concurrent downloads automatically
- **Download history**: Track all your downloads with persistent storage
- **Dark/Light theme**: Toggle between dark and light modes
- **Toast notifications**: Real-time feedback on download status
- **Minimalist UI**: Clean and modern interface without emojis
- **Portable version**: Run without installation (includes yt-dlp)
- **Fast downloads**: Optimized for speed (15-minute video in ~10 seconds)
- **Lua plugin system**: Extend functionality with custom plugins

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust with Tauri
- **Styling**: Tailwind CSS + shadcn/ui
- **Video Downloader**: yt-dlp (embedded)
- **Build System**: NSIS (installer) + GitHub Actions

## Installation

### From Release

1. Download the latest release from [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. Choose either:
   - **Installer (NSIS)**: Run the `.exe` installer
   - **Portable**: Extract the `.zip` and run `omigdex-app.exe`

### From Source

**Prerequisites**:
- Node.js (v18 or higher)
- pnpm
- Rust toolchain
- yt-dlp (for development)

**Steps**:

```bash
# Clone the repository
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Usage

1. **Download a video**:
   - Paste the video URL in the input field
   - Select format (MP4 or MP3)
   - Select quality (Best, High, Medium, Low)
   - Click "Download"

2. **Manage downloads**:
   - View active downloads in the "Queue" tab
   - Cancel downloads if needed
   - Clear completed downloads

3. **View history**:
   - Switch to "History" tab
   - View all past downloads
   - Remove individual entries or clear all

4. **Toggle theme**:
   - Click the theme button in the header
   - Switch between light and dark modes

## Project Structure

```
omigdex-app/
├── src/                    # React frontend
│   ├── components/         # React components
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/ui components
│   ├── lib/               # Utilities
│   │   └── utils.ts
│   ├── App.tsx            # Main app component
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   ├── lib.rs         # Tauri commands
│   │   ├── downloader.rs  # yt-dlp integration
│   │   ├── queue.rs       # Download queue management
│   │   ├── history.rs     # Download history persistence
│   │   └── types.rs       # Data structures
│   └── Cargo.toml         # Rust dependencies
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD pipeline
└── create-portable.bat    # Portable build script
```

## Rust Backend Architecture

The backend consists of several modules:

- **`main.rs`**: Application entry point
- **`lib.rs`**: Tauri command definitions and app initialization
- **`downloader.rs`**: yt-dlp integration for video downloading
- **`queue.rs`**: Download queue with concurrent download management
- **`history.rs`**: Persistent download history in JSON
- **`types.rs`**: Data structures and enums

## Development

**Run development server**:
```bash
pnpm tauri dev
```

**Run frontend only** (without backend):
```bash
npm run dev
```

**Build installer**:
```bash
pnpm tauri build
```

**Create portable version**:
```bash
create-portable.bat
```

## Building Releases

Releases are automatically built via GitHub Actions when you push a tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

The workflow creates:
- NSIS installer with yt-dlp included
- Portable ZIP with yt-dlp included

## Known Issues

- **Antivirus warnings**: Unsigned executables may trigger Windows Defender or other antivirus software. This is normal for unsigned applications. Consider adding an exception or using the portable version.
- **Code signing**: To avoid antivirus warnings, the application needs a digital signature. For open-source projects, [SignPath.io](https://signpath.io/) offers free code signing.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the GPL-3.0 License.

## Acknowledgments

- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Video downloader
- [Tailwind CSS](https://tailwindcss.com/) - CSS framework
- [shadcn/ui](https://ui.shadcn.com/) - UI components

## Links

- [GitHub Repository](https://github.com/Cr12dev/omigdex)
- [Releases](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
