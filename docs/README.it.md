# Omigdex

Un'applicazione di download video open source costruita con Tauri, React, TypeScript e Rust. Scarica video da YouTube, Instagram, TikTok e Pinterest con facilità.

## Caratteristiche

- **Supporto multipiattaforma**: Scarica video da YouTube, Instagram, TikTok e Pinterest
- **Formati multipli**: Scarica in formati MP4 (video) o MP3 (audio)
- **Selezione qualità**: Scegli tra Migliore, Alta (1080p), Media (720p) o Bassa qualità
- **Coda di download**: Gestisci fino a 3 download simultanei automaticamente
- **Cronologia download**: Traccia tutti i tuoi download con archiviazione persistente
- **Tema chiaro/scuro**: Alterna tra modalità chiara e scura
- **Notifiche toast**: Feedback in tempo reale sullo stato del download
- **Interfaccia minimalista**: Design pulito e moderno senza emoji
- **Versione portatile**: Esegui senza installazione (include yt-dlp)
- **Download veloci**: Ottimizzato per la velocità (video di 15 minuti in ~10 secondi)

## Stack Tecnologico

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust con Tauri
- **Stili**: Tailwind CSS + shadcn/ui
- **Scaricatore video**: yt-dlp (integrato)
- **Sistema di build**: NSIS (installatore) + GitHub Actions

## Installazione

### Dalla Release

1. Scarica l'ultima release da [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. Scegli tra:
   - **Installatore (NSIS)**: Esegui l'installatore `.exe`
   - **Portatile**: Estrai lo `.zip` ed esegui `omigdex-app.exe`

### Dal Codice Sorgente

**Prerequisiti**:
- Node.js (v18 o superiore)
- pnpm
- Toolchain Rust
- yt-dlp (per lo sviluppo)

**Passaggi**:

```bash
# Clona il repository
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Installa le dipendenze
pnpm install

# Esegui in modalità sviluppo
pnpm tauri dev

# Costruisci per la produzione
pnpm tauri build
```

## Utilizzo

1. **Scaricare un video**:
   - Incolla l'URL del video nel campo di input
   - Seleziona il formato (MP4 o MP3)
   - Seleziona la qualità (Migliore, Alta, Media, Bassa)
   - Clicca su "Download"

2. **Gestire i download**:
   - Visualizza i download attivi nella scheda "Queue"
   - Annulla i download se necessario
   - Pulisci i download completati

3. **Visualizzare la cronologia**:
   - Passa alla scheda "History"
   - Visualizza tutti i download passati
   - Rimuovi singole voci o pulisci tutto

4. **Alterare il tema**:
   - Clicca sul pulsante del tema nell'intestazione
   - Alterna tra modalità chiara e scura

## Struttura del Progetto

```
omigdex-app/
├── src/                    # Frontend React
│   ├── components/         # Componenti React
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # Componenti shadcn/ui
│   ├── lib/               # Utilità
│   │   └── utils.ts
│   ├── App.tsx            # Componente principale
│   └── main.tsx           # Punto di ingresso
├── src-tauri/             # Backend Rust
│   ├── src/
│   │   ├── main.rs        # Punto di ingresso
│   │   ├── lib.rs         # Comandi Tauri
│   │   ├── downloader.rs  # Integrazione yt-dlp
│   │   ├── queue.rs       # Gestione coda
│   │   ├── history.rs     # Cronologia persistente
│   │   └── types.rs       # Strutture dati
│   └── Cargo.toml         # Dipendenze Rust
├── .github/
│   └── workflows/
│       └── build.yml      # Pipeline CI/CD
└── create-portable.bat    # Script build portatile
```

## Architettura Backend Rust

Il backend consiste di diversi moduli:

- **`main.rs`**: Punto di ingresso dell'applicazione
- **`lib.rs`**: Definizioni dei comandi Tauri e inizializzazione
- **`downloader.rs`**: Integrazione con yt-dlp per il download video
- **`queue.rs`**: Coda di download con gestione concorrente
- **`history.rs`**: Cronologia download persistente in JSON
- **`types.rs`**: Strutture dati ed enum

## Sviluppo

**Eseguire server di sviluppo**:
```bash
pnpm tauri dev
```

**Eseguire solo frontend** (senza backend):
```bash
npm run dev
```

**Costruire installatore**:
```bash
pnpm tauri build
```

**Creare versione portatile**:
```bash
create-portable.bat
```

## Creare Release

Le release vengono create automaticamente tramite GitHub Actions quando invii un tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Il workflow crea:
- Installatore NSIS con yt-dlp incluso
- ZIP portatile con yt-dlp incluso

## Problemi Conosciuti

- **Avvisi antivirus**: Gli eseguibili non firmati possono attivare Windows Defender o altro software antivirus. Questo è normale per applicazioni non firmate. Considera di aggiungere un'eccezione o usare la versione portatile.
- **Firma codice**: Per evitare avvisi antivirus, l'applicazione necessita di una firma digitale. Per progetti open source, [SignPath.io](https://signpath.io/) offre firma codice gratuita.

## Contributi

I contributi sono benvenuti! Non esitare a inviare una Pull Request.

## Licenza

Questo progetto è open source ed è disponibile sotto la Licenza GPL-3.0.

## Riconoscimenti

- [Tauri](https://tauri.app/) - Framework desktop multipiattaforma
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Scaricatore video
- [Tailwind CSS](https://tailwindcss.com/) - Framework CSS
- [shadcn/ui](https://ui.shadcn.com/) - Componenti UI

## Link

- [Repository GitHub](https://github.com/Cr12dev/omigdex)
- [Release](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Configurazione IDE Raccomandata

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
