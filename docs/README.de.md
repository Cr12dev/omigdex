# Omigdex

Eine Open-Source-Video-Downloader-Anwendung, die mit Tauri, React, TypeScript und Rust erstellt wurde. Laden Sie Videos von YouTube, Instagram, TikTok und Pinterest einfach herunter.

## Funktionen

- **Multiplattform-Unterstützung**: Laden Sie Videos von YouTube, Instagram, TikTok und Pinterest herunter
- **Mehrere Formate**: Laden Sie in MP4 (Video) oder MP3 (Audio) Format herunter
- **Qualitätsauswahl**: Wählen Sie zwischen Beste, Hoch (1080p), Mittel (720p) oder Niedrig Qualität
- **Download-Warteschlange**: Verwalten Sie bis zu 3 gleichzeitige Downloads automatisch
- **Download-Verlauf**: Verfolgen Sie alle Ihre Downloads mit dauerhaftem Speicher
- **Hell/Dunkel-Thema**: Wechseln Sie zwischen hellen und dunklen Modi
- **Toast-Benachrichtigungen**: Echtzeit-Feedback zum Download-Status
- **Minimalistische Benutzeroberfläche**: Sauberes und modernes Design ohne Emojis
- **Portable-Version**: Führen Sie ohne Installation aus (inklusive yt-dlp)
- **Schnelle Downloads**: Optimiert für Geschwindigkeit (15-Minuten-Video in ~10 Sekunden)

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust mit Tauri
- **Styling**: Tailwind CSS + shadcn/ui
- **Video-Downloader**: yt-dlp (eingebettet)
- **Build-System**: NSIS (Installer) + GitHub Actions

## Installation

### Von Release

1. Laden Sie die neueste Version von [GitHub Releases](https://github.com/Cr12dev/omigdex/releases) herunter
2. Wählen Sie zwischen:
   - **Installer (NSIS)**: Führen Sie den `.exe`-Installer aus
   - **Portable**: Extrahieren Sie die `.zip`-Datei und führen Sie `omigdex-app.exe` aus

### Von Quellcode

**Voraussetzungen**:
- Node.js (v18 oder höher)
- pnpm
- Rust-Toolchain
- yt-dlp (für Entwicklung)

**Schritte**:

```bash
# Klonen Sie das Repository
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Installieren Sie Abhängigkeiten
pnpm install

# Führen Sie im Entwicklungsmodus aus
pnpm tauri dev

# Bauen Sie für die Produktion
pnpm tauri build
```

## Verwendung

1. **Video herunterladen**:
   - Fügen Sie die Video-URL in das Eingabefeld ein
   - Wählen Sie das Format (MP4 oder MP3)
   - Wählen Sie die Qualität (Beste, Hoch, Mittel, Niedrig)
   - Klicken Sie auf "Download"

2. **Downloads verwalten**:
   - Aktive Downloads im Tab "Queue" anzeigen
   - Downloads bei Bedarf abbrechen
   - Abgeschlossene Downloads löschen

3. **Verlauf anzeigen**:
   - Wechseln Sie zum Tab "History"
   - Alle vergangenen Downloads anzeigen
   - Einzelne Einträge entfernen oder alles löschen

4. **Thema wechseln**:
   - Klicken Sie auf die Theme-Schaltfläche im Header
   - Wechseln Sie zwischen hellen und dunklen Modi

## Projektstruktur

```
omigdex-app/
├── src/                    # React Frontend
│   ├── components/         # React-Komponenten
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/ui-Komponenten
│   ├── lib/               # Hilfsprogramme
│   │   └── utils.ts
│   ├── App.tsx            # Hauptkomponente
│   └── main.tsx           # Einstiegspunkt
├── src-tauri/             # Rust Backend
│   ├── src/
│   │   ├── main.rs        # Einstiegspunkt
│   │   ├── lib.rs         # Tauri-Befehle
│   │   ├── downloader.rs  # yt-dlp-Integration
│   │   ├── queue.rs       # Warteschlangenverwaltung
│   │   ├── history.rs     Dauerhafter Verlauf
│   │   └── types.rs       # Datenstrukturen
│   └── Cargo.toml         # Rust-Abhängigkeiten
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD-Pipeline
└── create-portable.bat    # Portable-Build-Skript
```

## Rust-Backend-Architektur

Das Backend besteht aus mehreren Modulen:

- **`main.rs`**: Einstiegspunkt der Anwendung
- **`lib.rs`**: Tauri-Befehlsdefinitionen und Initialisierung
- **`downloader.rs`**: yt-dlp-Integration für Video-Downloads
- **`queue.rs`**: Download-Warteschlange mit gleichzeitiger Verwaltung
- **`history.rs`**: Dauerhafter Download-Verlauf in JSON
- **`types.rs`**: Datenstrukturen und Enums

## Entwicklung

**Entwicklungsserver ausführen**:
```bash
pnpm tauri dev
```

**Nur Frontend ausführen** (ohne Backend):
```bash
npm run dev
```

**Installer bauen**:
```bash
pnpm tauri build
```

**Portable-Version erstellen**:
```bash
create-portable.bat
```

## Releases Erstellen

Releases werden automatisch über GitHub Actions erstellt, wenn Sie ein Tag pushen:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Der Workflow erstellt:
- NSIS-Installer mit yt-dlp inklusive
- Portable ZIP mit yt-dlp inklusive

## Bekannte Probleme

- **Antivirus-Warnungen**: Unsignierte ausführbare Dateien können Windows Defender oder andere Antivirus-Software auslösen. Dies ist normal für unsignierte Anwendungen. Erwägen Sie, eine Ausnahme hinzuzufügen oder die portable Version zu verwenden.
- **Code-Signierung**: Um Antivirus-Warnungen zu vermeiden, benötigt die Anwendung eine digitale Signatur. Für Open-Source-Projekte bietet [SignPath.io](https://signpath.io/) kostenlose Code-Signierung.

## Beiträge

Beiträge sind willkommen! Zögern Sie nicht, einen Pull Request zu senden.

## Lizenz

Dieses Projekt ist Open Source und unter der MIT-Lizenz verfügbar.

## Danksagungen

- [Tauri](https://tauri.app/) - Multiplattform-Desktop-Framework
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Video-Downloader
- [Tailwind CSS](https://tailwindcss.com/) - CSS-Framework
- [shadcn/ui](https://ui.shadcn.com/) - UI-Komponenten

## Links

- [GitHub-Repository](https://github.com/Cr12dev/omigdex)
- [Releases](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Empfohlene IDE-Konfiguration

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
