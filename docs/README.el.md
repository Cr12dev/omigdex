# Omigdex

Μια εφαρμογή λήψης βίντεο ανοιχτού κώδικα που δημιουργήθηκε με Tauri, React, TypeScript και Rust. Κατεβάστε βίντεο από YouTube, Instagram, TikTok και Pinterest εύκολα.

## Χαρακτηριστικά

- **Υποστήριξη πολλαπλών πλατφορμών**: Κατεβάστε βίντεο από YouTube, Instagram, TikTok και Pinterest
- **Πολλαπλές μορφές**: Κατεβάστε σε μορφή MP4 (βίντεο) ή MP3 (ήχος)
- **Επιλογή ποιότητας**: Επιλέξτε μεταξύ Καλύτερη, Υψηλή (1080p), Μεσαία (720p) ή Χαμηλή ποιότητα
- **Ουρά λήψεων**: Διαχειριστείτε έως 3 ταυτόχρονες λήψεις αυτόματα
- **Ιστορικό λήψεων**: Παρακολουθήστε όλες τις λήψεις σας με μόνιμη αποθήκευση
- **Σκοτεινό/Φωτεινό θέμα**: Εναλλαγή μεταξύ σκοτεινής και φωτεινής λειτουργίας
- **Ειδοποιήσεις Toast**: Άμεση ανατροφοδότηση κατάστασης λήψης
- **Ελαχιστικό UI**: Καθαρό και μοντέρνο σχεδιασμό χωρίς emoji
- **Φορητή έκδοση**: Εκτελέστε χωρίς εγκατάσταση (περιλαμβάνει yt-dlp)
- **Γρήγορες λήψεις**: Βελτιστοποιημένο για ταχύτητα (βίντεο 15 λεπτών σε ~10 δευτερόλεπτα)

## Τεχνολογική Στοίβα

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust με Tauri
- **Στυλ**: Tailwind CSS + shadcn/ui
- **Λήψη βίντεο**: yt-dlp (ενσωματωμένο)
- **Σύστημα κατασκευής**: NSIS (εγκαταστάτης) + GitHub Actions

## Εγκατάσταση

### Από Release

1. Κατεβάστε την τελευταία έκδοση από [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. Επιλέξτε μεταξύ:
   - **Εγκαταστάτης (NSIS)**: Εκτελέστε τον εγκαταστάτη `.exe`
   - **Φορητό**: Εξάγετε το `.zip` και εκτελέστε `omigdex-app.exe`

### Από Πηγαίο Κώδικα

**Προαπαιτούμενα**:
- Node.js (v18 ή νεότερο)
- pnpm
- Rust toolchain
- yt-dlp (για ανάπτυξη)

**Βήματα**:

```bash
# Κλωνοποιήστε το αποθετήριο
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Εγκαταστήστε τις εξαρτήσεις
pnpm install

# Εκτελέστε σε λειτουργία ανάπτυξης
pnpm tauri dev

# Κατασκευάστε για παραγωγή
pnpm tauri build
```

## Χρήση

1. **Λήψη βίντεο**:
   - Επικολλήστε το URL του βίντεο στο πεδίο εισαγωγής
   - Επιλέξτε τη μορφή (MP4 ή MP3)
   - Επιλέξτε την ποιότητα (Καλύτερη, Υψηλή, Μεσαία, Χαμηλή)
   - Κάντε κλικ στο "Download"

2. **Διαχείριση λήψεων**:
   - Προβολή ενεργών λήψεων στην καρτέλα "Queue"
   - Ακύρωση λήψεων αν χρειάζεται
   - Καθαρισμός ολοκληρωμένων λήψεων

3. **Προβολή ιστορικού**:
   - Μετάβαση στην καρτέλα "History"
   - Προβολή όλων των προηγούμενων λήψεων
   - Αφαίρεση μεμονωμένων καταχωρήσεων ή καθαρισμός όλων

4. **Εναλλαγή θέματος**:
   - Κάντε κλικ στο κουμπί θέματος στην επικεφαλίδα
   - Εναλλαγή μεταξύ σκοτεινής και φωτεινής λειτουργίας

## Δομή Έργου

```
omigdex-app/
├── src/                    # React Frontend
│   ├── components/         # React συστατικά
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/ui συστατικά
│   ├── lib/               # Εργαλεία
│   │   └── utils.ts
│   ├── App.tsx            # Κύριο συστατικό
│   └── main.tsx           # Σημείο εισόδου
├── src-tauri/             # Rust Backend
│   ├── src/
│   │   ├── main.rs        # Σημείο εισόδου
│   │   ├── lib.rs         # Εντολές Tauri
│   │   ├── downloader.rs  # Ενοποίηση yt-dlp
│   │   ├── queue.rs       # Διαχείριση ουράς
│   │   ├── history.rs     # Μόνιμο ιστορικό
│   │   └── types.rs       # Δομές δεδομένων
│   └── Cargo.toml         # Εξαρτήσεις Rust
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD pipeline
└── create-portable.bat    # Script κατασκευής φορητού
```

## Αρχιτεκτονική Rust Backend

Το backend αποτελείται από πολλα αρθρωτά:

- **`main.rs`**: Σημείο εισόδου της εφαρμογής
- **`lib.rs`**: Ορισμοί εντολών Tauri και αρχικοποίηση
- **`downloader.rs`**: Ενοποίηση με yt-dlp για λήψη βίντεο
- **`queue.rs`**: Ουρά λήψεων με ταυτόχρονη διαχείριση
- **`history.rs`**: Μόνιμο ιστορικό λήψεων σε JSON
- **`types.rs`**: Δομές δεδομένων και enums

## Ανάπτυξη

**Εκτέλεση διακομιστή ανάπτυξης**:
```bash
pnpm tauri dev
```

**Εκτέλεση μόνο frontend** (χωρίς backend):
```bash
npm run dev
```

**Κατασκευή εγκαταστάτη**:
```bash
pnpm tauri build
```

**Δημιουργία φορητής έκδοσης**:
```bash
create-portable.bat
```

## Δημιουργία Releases

Τα releases δημιουργούνται αυτόματα μέσω GitHub Actions όταν στέλνετε ένα tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Η ροή εργασίας δημιουργεί:
- Εγκαταστάτη NSIS με yt-dlp ενσωματωμένο
- Φορητό ZIP με yt-dlp ενσωματωμένο

## Γνωστά Προβλήματα

- **Προειδοποιήσεις antivirus**: Μη υπογεγραμμένα εκτελέσιμα αρχεία μπορούν να ενεργοποιήσουν το Windows Defender ή άλλο λογισμικό antivirus. Αυτό είναι φυσιο για μη υπογεγραμμένες εφαρμογές. Σκεφτείτε να προσθέσετε μια εξαίρεση ή να χρησιμοποιήσετε τη φορητή έκδοση.
- **Υπογραφή κώδικα**: Για την αποφυγή προειδοποιήσεων antivirus, η εφαρμογή χρειάζεται ψηφιακή υπογραφή. Για έργα ανοιχτού κώδικα, το [SignPath.io](https://signpath.io/) προσφέρει δωρεάν υπογραφή κώδικα.

## Συνεισφορές

Οι συνεισφορές είναι ευπρόσδεκτες! Μην διστάσετε να στείλετε Pull Request.

## Άδεια

Αυτό το έργο είναι ανοιχτού κώδικα και είναι διαθέσιμο υπό την άδεια MIT.

## Αναγνωρίσεις

- [Tauri](https://tauri.app/) - Πλατφόρμα desktop πολλαπλών πλατφορμών
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Λήψη βίντεο
- [Tailwind CSS](https://tailwindcss.com/) - Πλαίσιο CSS
- [shadcn/ui](https://ui.shadcn.com/) - Συστατικά UI

## Σύνδεσμοι

- [Αποθετήριο GitHub](https://github.com/Cr12dev/omigdex)
- [Releases](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Προτεινόμενη Διαμόρφωση IDE

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
