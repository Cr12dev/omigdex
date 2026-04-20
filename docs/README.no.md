# Omigdex

En åpen kildekode-videodownloader-applikasjon bygget med Tauri, React, TypeScript og Rust. Last ned videoer fra YouTube, Instagram, TikTok og Pinterest enkelt.

## Funksjoner

- **Multiplattform-støtte**: Last ned videoer fra YouTube, Instagram, TikTok og Pinterest
- **Flere formater**: Last ned i MP4 (video) eller MP3 (lyd) format
- **Kvalitetsvalg**: Velg mellom Best, Høy (1080p), Medium (720p) eller Lav kvalitet
- **Download-kø**: Styr opptil 3 samtidige nedlastinger automatisk
- **Download-historikk**: Spor alle nedlastingene dine med vedvarende lagring
- **Mørk/Lys tema**: Bytt mellom mørk og lys modus
- **Toast-varsler**: Sanntidstilbakemelding på nedlastingsstatus
- **Minimalistisk UI**: Rent og moderne design uten emojis
- **Portable versjon**: Kjør uten installasjon (inkluderer yt-dlp)
- **Raske nedlastinger**: Optimalisert for hastighet (15-minutters video på ~10 sekunder)

## Tech Stack

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust med Tauri
- **Styling**: Tailwind CSS + shadcn/ui
- **Videodownloader**: yt-dlp (innebygd)
- **Build-system**: NSIS (installasjonsprogram) + GitHub Actions

## Installasjon

### Fra Release

1. Last ned nyeste versjon fra [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. Velg mellom:
   - **Installatør (NSIS)**: Kjør `.exe`-installasjonsprogrammet
   - **Portable**: Pakk ut `.zip` og kjør `omigdex-app.exe`

### Fra Kildekode

**Forutsetninger**:
- Node.js (v18 eller nyere)
- pnpm
- Rust toolchain
- yt-dlp (for utvikling)

**Trinn**:

```bash
# Klon depotet
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Installer avhengigheter
pnpm install

# Kjør i utviklingsmodus
pnpm tauri dev

# Bygg for produksjon
pnpm tauri build
```

## Bruk

1. **Last ned en video**:
   - Lim inn video-URL i inntastingsfeltet
   - Velg format (MP4 eller MP3)
   - Velg kvalitet (Best, Høy, Medium, Lav)
   - Klikk "Download"

2. **Styr nedlastinger**:
   - Vis aktive nedlastinger i "Queue"-fanen
   - Avbryt nedlastinger om nødvendig
   - Tøm fullførte nedlastinger

3. **Vis historikk**:
   - Bytt til "History"-fanen
   - Se alle tidligere nedlastinger
   - Fjern enkelte oppføringer eller tøm alt

4. **Bytt tema**:
   - Klikk på temaknappen i overskriften
   - Bytt mellom mørk og lys modus

## Prosjektstruktur

```
omigdex-app/
├── src/                    # React Frontend
│   ├── components/         # React-komponenter
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/ui-komponenter
│   ├── lib/               # Verktøy
│   │   └── utils.ts
│   ├── App.tsx            # Hovedkomponent
│   └── main.tsx           # Inngangspunkt
├── src-tauri/             # Rust Backend
│   ├── src/
│   │   ├── main.rs        # Inngangspunkt
│   │   ├── lib.rs         # Tauri-kommandoer
│   │   ├── downloader.rs  # yt-dlp-integrasjon
│   │   ├── queue.rs       # Kø-styring
│   │   ├── history.rs     # Vedvarende historikk
│   │   └── types.rs       # Datastrukturer
│   └── Cargo.toml         # Rust-avhengigheter
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD-pipeline
└── create-portable.bat    # Portable build-skript
```

## Rust Backend-arkitektur

Backend består av flere moduler:

- **`main.rs`**: Applikasjonens inngangspunkt
- **`lib.rs`**: Tauri-kommandodefinisjoner og initialisering
- **`downloader.rs`**: yt-dlp-integrasjon for videonedlasting
- **`queue.rs`**: Download-kø med samtidig styring
- **`history.rs`**: Vedvarende download-historikk i JSON
- **`types.rs`**: Datastrukturer og enums

## Utvikling

**Kjør utviklingsserver**:
```bash
pnpm tauri dev
```

**Kjør kun frontend** (uten backend):
```bash
npm run dev
```

**Bygg installasjonsprogram**:
```bash
pnpm tauri build
```

**Lag portable versjon**:
```bash
create-portable.bat
```

## Lage Releases

Releases lages automatisk via GitHub Actions når du skyver en tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Arbeidsflyten lager:
- NSIS-installatør med yt-dlp inkludert
- Portable ZIP med yt-dlp inkludert

## Kjente Problemer

- **Antivirus-advarsler**: Usignerte kjørbare filer kan utløse Windows Defender eller annen antivirusprogramvare. Dette er normalt for usignerte applikasjoner. Vurder å legge til et unntak eller bruke den portable versjonen.
- **Kodesignering**: For å unngå antivirus-advarsler trenger applikasjonen en digital signatur. For åpen kildekode-prosjekter tilbyr [SignPath.io](https://signpath.io/) gratis kodesignering.

## Bidrag

Bidrag er velkomne! Ikke nøl med å sende en Pull Request.

## Lisens

Dette prosjektet er åpen kildekode og er tilgjengelig under GPL-3.0-lisensen.

## Anerkjennelser

- [Tauri](https://tauri.app/) - Multiplattform desktop-rammeverk
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Videodownloader
- [Tailwind CSS](https://tailwindcss.com/) - CSS-rammeverk
- [shadcn/ui](https://ui.shadcn.com/) - UI-komponenter

## Lenker

- [GitHub-repository](https://github.com/Cr12dev/omigdex)
- [Releases](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Anbefalt IDE-konfigurasjon

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
