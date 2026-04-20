# Omigdex

Aplikacja do pobierania filmów typu open source zbudowana przy użyciu Tauri, React, TypeScript i Rust. Pobieraj filmy z YouTube, Instagram, TikTok i Pinterest z łatwością.

## Funkcje

- **Obsługa wielu platform**: Pobieraj filmy z YouTube, Instagram, TikTok i Pinterest
- **Wiele formatów**: Pobieraj w formatach MP4 (wideo) lub MP3 (audio)
- **Wybór jakości**: Wybierz spośród Najlepsza, Wysoka (1080p), Średnia (720p) lub Niska jakość
- **Kolejka pobierania**: Zarządzaj do 3 jednoczesnych pobrań automatycznie
- **Historia pobierania**: Śledź wszystkie swoje pobrania z trwałym przechowywaniem
- **Motyw jasny/ciemny**: Przełączaj między trybami jasnym i ciemnym
- **Powiadomienia toast**: Informacje zwrotne w czasie rzeczywistym o stanie pobierania
- **Minimalistyczny interfejs**: Czysty i nowoczesny design bez emoji
- **Wersja przenośna**: Uruchom bez instalacji (zawiera yt-dlp)
- **Szybkie pobieranie**: Zoptymalizowane pod kątem szybkości (film 15-minutowy w ~10 sekund)

## Stack Technologiczny

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust z Tauri
- **Style**: Tailwind CSS + shadcn/ui
- **Pobieracz filmów**: yt-dlp (wbudowany)
- **System budowania**: NSIS (instalator) + GitHub Actions

## Instalacja

### Z Release

1. Pobierz najnowszą wersję z [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. Wybierz między:
   - **Instalator (NSIS)**: Uruchom instalator `.exe`
   - **Przenośny**: Rozpakuj `.zip` i uruchom `omigdex-app.exe`

### Z Kodu Źródłowego

**Wymagania wstępne**:
- Node.js (v18 lub nowszy)
- pnpm
- Rust toolchain
- yt-dlp (do rozwoju)

**Kroki**:

```bash
# Sklonuj repozytorium
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Zainstaluj zależności
pnpm install

# Uruchom w trybie deweloperskim
pnpm tauri dev

# Zbuduj dla produkcji
pnpm tauri build
```

## Użycie

1. **Pobieranie filmu**:
   - Wklej URL filmu w pole wprowadzania
   - Wybierz format (MP4 lub MP3)
   - Wybierz jakość (Najlepsza, Wysoka, Średnia, Niska)
   - Kliknij "Download"

2. **Zarządzanie pobraniami**:
   - Wyświetl aktywne pobrania na zakładce "Queue"
   - Anuluj pobrania w razie potrzeby
   - Wyczyść ukończone pobrania

3. **Wyświetlanie historii**:
   - Przełącz na zakładkę "History"
   - Wyświetl wszystkie przeszłe pobrania
   - Usuń pojedyncze wpisy lub wyczyść wszystko

4. **Przełączanie motywu**:
   - Kliknij przycisk motywu w nagłówku
   - Przełączaj między trybami jasnym i ciemnym

## Struktura Projektu

```
omigdex-app/
├── src/                    # Frontend React
│   ├── components/         # Komponenty React
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # Komponenty shadcn/ui
│   ├── lib/               # Narzędzia
│   │   └── utils.ts
│   ├── App.tsx            # Główny komponent
│   └── main.tsx           # Punkt wejścia
├── src-tauri/             # Backend Rust
│   ├── src/
│   │   ├── main.rs        # Punkt wejścia
│   │   ├── lib.rs         # Komendy Tauri
│   │   ├── downloader.rs  # Integracja yt-dlp
│   │   ├── queue.rs       # Zarządzanie kolejką
│   │   ├── history.rs     # Trwała historia
│   │   └── types.rs       # Struktury danych
│   └── Cargo.toml         # Zależności Rust
├── .github/
│   └── workflows/
│       └── build.yml      # Pipeline CI/CD
└── create-portable.bat    # Skrypt budowania przenośnego
```

## Architektura Backend Rust

Backend składa się z kilku modułów:

- **`main.rs`**: Punkt wejścia aplikacji
- **`lib.rs`**: Definicje komend Tauri i inicjalizacja
- **`downloader.rs`**: Integracja z yt-dlp do pobierania filmów
- **`queue.rs`**: Kolejka pobierania z zarządzaniem współbieżnym
- **`history.rs`**: Trwała historia pobierania w JSON
- **`types.rs`**: Struktury danych i enumy

## Rozwój

**Uruchom serwer deweloperski**:
```bash
pnpm tauri dev
```

**Uruchom tylko frontend** (bez backend):
```bash
npm run dev
```

**Zbuduj instalator**:
```bash
pnpm tauri build
```

**Utwórz wersję przenośną**:
```bash
create-portable.bat
```

## Tworzenie Release

Release są tworzone automatycznie przez GitHub Actions, gdy wypchniesz tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Workflow tworzy:
- Instalator NSIS z yt-dlp włączonym
- ZIP przenośny z yt-dlp włączonym

## Znane Problemy

- **Ostrzeżenia antywirusowe**: Niepodpisane pliki wykonywalne mogą wywołać Windows Defender lub inne oprogramowanie antywirusowe. Jest to normalne dla niepodpisanych aplikacji. Rozważ dodanie wyjątku lub użycie wersji przenośnej.
- **Podpisywanie kodu**: Aby uniknąć ostrzeżeń antywirusowych, aplikacja potrzebuje podpisu cyfrowego. Dla projektów open source [SignPath.io](https://signpath.io/) oferuje bezpłatne podpisywanie kodu.

## Wkład

Wkłady są mile widziane! Nie wahaj się wysłać Pull Request.

## Licencja

Ten projekt jest open source i jest dostępny na licencji GPL-3.0.

## Podziękowania

- [Tauri](https://tauri.app/) - Framework desktop wieloplatformowy
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Pobieracz filmów
- [Tailwind CSS](https://tailwindcss.com/) - Framework CSS
- [shadcn/ui](https://ui.shadcn.com/) - Komponenty UI

## Linki

- [Repozytorium GitHub](https://github.com/Cr12dev/omigdex)
- [Release](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Zalecana Konfiguracja IDE

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
