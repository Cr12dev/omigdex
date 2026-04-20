# Omigdex

Avoimen lähdekoodin videosovellus, joka on rakennettu Tauri-, React-, TypeScript- ja Rust-tekniikoilla. Lataa videoita helposti YouTubesta, Instagramista, TikTokista ja Pinterestistä.

## Ominaisuudet

- **Monialustatuki**: Lataa videoita YouTubesta, Instagramista, TikTokista ja Pinterestistä
- **Useita formaatteja**: Lataa MP4 (video) tai MP3 (audio) -muodossa
- **Laadun valinta**: Valitse paras, korkea (1080p), keskikokoinen (720p) tai matala laatu
- **Latausjono**: Hallitse automaattisesti jopa 3 samanaikaista latausta
- **Lataushistoria**: Seuraa kaikkia latauksiasi pysyvällä tallennuksella
- **Tumma/vaalea teema**: Vaihda tumman ja vaalean tilan välillä
- **Toast-ilmoitukset**: Reaaliaikainen palaute lataustilasta
- **Minimalistinen käyttöliittymä**: Siisti ja moderni ilme ilman emojeja
- **Kannettava versio**: Käynnistä ilman asennusta (sisältää yt-dlp:n)
- **Nopeat lataukset**: Optimoidu nopeuteen (15 minuutin video ~10 sekunnissa)

## Teknologiapino

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust Tauri kanssa
- **Tyyli**: Tailwind CSS + shadcn/ui
- **Videolataaja**: yt-dlp (sisäänrakennettu)
- **Rakennusjärjestelmä**: NSIS (asennusohjelma) + GitHub Actions

## Asennus

### Julkaisusta

1. Lataa uusin versio [GitHub Releases](https://github.com/Cr12dev/omigdex/releases) -sivustolta
2. Valitse:
   - **Asennusohjelma (NSIS)**: Suorita `.exe`-asennusohjelma
   - **Kannettava**: Pura `.zip` ja suorita `omigdex-app.exe`

### Lähdekoodista

**Edellytykset**:
- Node.js (v18 tai uudempi)
- pnpm
- Rust-työkaluketju
- yt-dlp (kehitykseen)

**Vaiheet**:

```bash
# Kloonaa tietovarasto
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Asenna riippuvuudet
pnpm install

# Suorita kehitystilassa
pnpm tauri dev

# Rakenna tuotantoon
pnpm tauri build
```

## Käyttö

1. **Lataa video**:
   - Liitä videon URL-osoite syöttökenttään
   - Valitse muoto (MP4 tai MP3)
   - Valitse laatu (paras, korkea, keskikokoinen, matala)
   - Napsauta "Download"

2. **Hallitse latauksia**:
   - Näytä aktiiviset lataukset "Queue"-välilehdellä
   - Peruuta lataukset tarvittaessa
   - Tyhjennä valmiit lataukset

3. **Näytä historia**:
   - Vaihda "History"-välilehdelle
   - Näytä kaikki aiemmat lataukset
   - Poista yksittäisiä merkintöjä tai tyhjennä kaikki

4. **Vaihda teemaa**:
   - Napsauta teeman painiketta otsikossa
   - Vaihda tumman ja vaalean tilan välillä

## Projektin rakenne

```
omigdex-app/
├── src/                    # React Frontend
│   ├── components/         # React-komponentit
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/ui-komponentit
│   ├── lib/               # Apuohjelmat
│   │   └── utils.ts
│   ├── App.tsx            # Pääkomponentti
│   └── main.tsx           # Käynnistyspiste
├── src-tauri/             # Rust Backend
│   ├── src/
│   │   ├── main.rs        # Käynnistyspiste
│   │   ├── lib.rs         # Tauri-komennot
│   │   ├── downloader.rs  # yt-dlp-integraatio
│   │   ├── queue.rs       # Jonon hallinta
│   │   ├── history.rs     # Pysyvä historia
│   │   └── types.rs       # Tietorakenteet
│   └── Cargo.toml         # Rust-riippuvuudet
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD-putki
└── create-portable.bat    # Kannettavan rakennusohjelma
```

## Rust-backend-arkkitehtuuri

Backend koostuu useista moduuleista:

- **`main.rs`**: Sovelluksen käynnistyspiste
- **`lib.rs`**: Tauri-komentomääritykset ja alustus
- **`downloader.rs`**: yt-dlp-integraatio videoiden lataamiseen
- **`queue.rs`**: Latausjono rinnakkaisella hallinnalla
- **`history.rs`**: Pysyvä lataushistoria JSON-muodossa
- **`types.rs`**: Tietorakenteet ja enumit

## Kehitys

**Suorita kehityspalvelin**:
```bash
pnpm tauri dev
```

**Suorita vain frontend** (ilman backendia):
```bash
npm run dev
```

**Rakenna asennusohjelma**:
```bash
pnpm tauri build
```

**Luo kannettava versio**:
```bash
create-portable.bat
```

## Julkaisujen luominen

Julkaisut luodaan automaattisesti GitHub Actionsin kautta, kun lähetät tagin:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Työnkulku luo:
- NSIS-asennusohjelman, jossa on yt-dlp
- Kannettavan ZIP-tiedoston, jossa on yt-dlp

## Tunnetut ongelmat

- **Antivirusvaroitukset**: Allekirjoittamattavat suoritettavat tiedostot voivat laukaista Windows Defenderin tai muun antivirusohjelmiston. Tämä on normaalia allekirjoittamattomille sovelluksille. Harkitse poikkeuksen lisäämistä tai kannettavan version käyttöä.
- **Koodin allekirjoitus**: Antivirusvaroitusten välttämiseksi sovellus tarvitsee digitaalisen allekirjoituksen. Avoimen lähdekoodin projekteille [SignPath.io](https://signpath.io/) tarjoaa ilmaisen koodin allekirjoituksen.

## Osallistuminen

Osallistuminen on tervetullutta! Älä epäröi lähettää Pull Request -pyyntöä.

## Lisenssi

Tämä projekti on avointa lähdekoodia ja on saatavilla GPL-3.0-lisenssillä.

## Kiitokset

- [Tauri](https://tauri.app/) - Monialustainen työpöytäkehys
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Videonlataaja
- [Tailwind CSS](https://tailwindcss.com/) - CSS-kehys
- [shadcn/ui](https://ui.shadcn.com/) - UI-komponentit

## Linkit

- [GitHub-tietovarasto](https://github.com/Cr12dev/omigdex)
- [Julkaisut](https://github.com/Cr12dev/omigdex/releases)
- [Ongelmat](https://github.com/Cr12dev/omigdex/issues)

## Suositeltu IDE-kokoonpano

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
