# Omigdex

Tauri, React, TypeScript ve Rust ile oluşturulmuş açık kaynaklı bir video indirme uygulaması. YouTube, Instagram, TikTok ve Pinterest'ten videoyu kolayca indirin.

## Özellikler

- **Çoklu platform desteği**: YouTube, Instagram, TikTok ve Pinterest'ten video indirin
- **Birden çok format**: MP4 (video) veya MP3 (ses) formatında indirin
- **Kalite seçimi**: En iyi, Yüksek (1080p), Orta (720p) veya Düşük kalite arasında seçim yapın
- **İndirme kuyruğu**: Otomatik olarak en fazla 3 eşzamanlı indirmeyi yönetin
- **İndirme geçmişi**: Kalıcı depolama ile tüm indirmelerinizi takip edin
- **Koyu/Açık tema**: Koyu ve açık mod arasında geçiş yapın
- **Toast bildirimleri**: İndirme durumunun gerçek zamanlı geri bildirimi
- **Minimalist UI**: Emoji olmadan temiz ve modern tasarım
- **Taşınabilir sürüm**: Kurulum olmadan çalıştırın (yt-dlp içerir)
- **Hızlı indirmeler**: Hız için optimize edilmiştir (15 dakikalık video ~10 saniyede)

## Teknoloji Yığını

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust ile Tauri
- **Styling**: Tailwind CSS + shadcn/ui
- **Video indirici**: yt-dlp (gömülü)
- **Derleme sistemi**: NSIS (yükleyici) + GitHub Actions

## Kurulum

### Release'dan

1. [GitHub Releases](https://github.com/Cr12dev/omigdex/releases) adresinden en son sürümü indirin
2. Şu seçeneklerden birini seçin:
   - **Yükleyici (NSIS)**: `.exe` yükleyicisini çalıştırın
   - **Taşınabilir**: `.zip` dosyasını çıkarın ve `omigdex-app.exe` çalıştırın

### Kaynak Koddan

**Önkoşullar**:
- Node.js (v18 veya daha yeni)
- pnpm
- Rust toolchain
- yt-dlp (geliştirme için)

**Adımlar**:

```bash
# Depoyu klonlayın
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Bağımlılıkları yükleyin
pnpm install

# Geliştirme modunda çalıştırın
pnpm tauri dev

# Üretim için derleyin
pnpm tauri build
```

## Kullanım

1. **Video indirin**:
   - Video URL'sini giriş alanına yapıştırın
   - Format seçin (MP4 veya MP3)
   - Kalite seçin (En iyi, Yüksek, Orta, Düşük)
   - "Download" düğmesine tıklayın

2. **İndirmeleri yönetin**:
   - "Queue" sekmesinde aktif indirmeleri görüntüleyin
   - Gerekirse indirmeleri iptal edin
   - Tamamlanan indirmeleri temizleyin

3. **Geçmişi görüntüleyin**:
   - "History" sekmesine geçin
   - Tüm geçmiş indirmeleri görün
   - Tek tek girişleri kaldırın veya tümünü temizleyin

4. **Tema değiştirin**:
   - Başlıktaki tema düğmesine tıklayın
   - Koyu ve açık mod arasında geçiş yapın

## Proje Yapısı

```
omigdex-app/
├── src/                    # React Frontend
│   ├── components/         # React bileşenleri
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/ui bileşenleri
│   ├── lib/               # Araçlar
│   │   └── utils.ts
│   ├── App.tsx            # Ana bileşen
│   └── main.tsx           # Giriş noktası
├── src-tauri/             # Rust Backend
│   ├── src/
│   │   ├── main.rs        # Giriş noktası
│   │   ├── lib.rs         # Tauri komutları
│   │   ├── downloader.rs  # yt-dlp entegrasyonu
│   │   ├── queue.rs       # Kuyruk yönetimi
│   │   ├── history.rs     - Kalıcı geçmiş
│   │   └── types.rs       # Veri yapıları
│   └── Cargo.toml         # Rust bağımlılıkları
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD pipeline
└── create-portable.bat    # Taşınabilir derleme betiği
```

## Rust Backend Mimarisi

Backend birkaç modülden oluşur:

- **`main.rs`**: Uygulamanın giriş noktası
- **`lib.rs`**: Tauri komut tanımları ve başlatma
- **`downloader.rs`**: Video indirme için yt-dlp entegrasyonu
- **`queue.rs`**: Eş zamanlı yönetim ile indirme kuyruğu
- **`history.rs`**: JSON'da kalıcı indirme geçmişi
- **`types.rs`**: Veri yapıları ve enumlar

## Geliştirme

**Geliştirme sunucusunu çalıştırın**:
```bash
pnpm tauri dev
```

**Sadece frontend çalıştırın** (backend olmadan):
```bash
npm run dev
```

**Yükleyiciyi derleyin**:
```bash
pnpm tauri build
```

**Taşınabilir sürüm oluşturun**:
```bash
create-portable.bat
```

## Sürümler Oluşturma

Bir tag gönderdiğinizde sürümler GitHub Actions üzerinden otomatik olarak oluşturulur:

```bash
git tag v1.0.0
git push origin v1.0.0
```

İş akışı şunları oluşturur:
- yt-dlp dahil edilmiş NSIS yükleyicisi
- yt-dlp dahil edilmiş taşınabilir ZIP

## Bilinen Sorunlar

- **Antivirüs uyarıları**: İmzasız yürütülebilir dosyalar Windows Defender veya diğer antivirüs yazılımlarını tetikleyebilir. Bu, imzasız uygulamalar için normaldir. Bir istisna eklemeyi veya taşınabilir sürümü kullanmayı düşünün.
- **Kod imzalama**: Antivirüs uyarılarını önlemek için uygulamanın dijital bir imzaya ihtiyacı vardır. Açık kaynak projeler için [SignPath.io](https://signpath.io/) ücretsiz kod imzalama sunar.

## Katkılar

Katkılar hoş geldir! Çekme isteği göndermekten çekinmeyin.

## Lisans

Bu proje açık kaynaklıdır ve GPL-3.0 lisansı altında kullanılabilir.

## Teşekkürler

- [Tauri](https://tauri.app/) - Çoklu platform masaüstü çerçevesi
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Video indirici
- [Tailwind CSS](https://tailwindcss.com/) - CSS çerçevesi
- [shadcn/ui](https://ui.shadcn.com/) - UI bileşenleri

## Bağlantılar

- [GitHub deposu](https://github.com/Cr12dev/omigdex)
- [Sürümler](https://github.com/Cr12dev/omigdex/releases)
- [Sorunlar](https://github.com/Cr12dev/omigdex/issues)

## Önerilen IDE Yapılandırması

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
