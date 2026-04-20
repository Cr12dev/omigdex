# Omigdex

Une application de téléchargement de vidéos open source construite avec Tauri, React, TypeScript et Rust. Téléchargez des vidéos depuis YouTube, Instagram, TikTok et Pinterest facilement.

## Fonctionnalités

- **Support multiplateforme**: Téléchargez des vidéos depuis YouTube, Instagram, TikTok et Pinterest
- **Formats multiples**: Téléchargez en formats MP4 (vidéo) ou MP3 (audio)
- **Sélection de la qualité**: Choisissez entre Meilleure, Haute (1080p), Moyenne (720p) ou Basse qualité
- **File de téléchargement**: Gérez jusqu'à 3 téléchargements simultanés automatiquement
- **Historique des téléchargements**: Suivez tous vos téléchargements avec stockage persistant
- **Thème clair/sombre**: Basculez entre les modes clair et sombre
- **Notifications toast**: Retour en temps réel sur l'état du téléchargement
- **Interface minimaliste**: Design propre et moderne sans emojis
- **Version portable**: Exécutez sans installation (inclut yt-dlp)
- **Téléchargements rapides**: Optimisé pour la vitesse (vidéo de 15 minutes en ~10 secondes)

## Stack Technologique

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust avec Tauri
- **Styles**: Tailwind CSS + shadcn/ui
- **Téléchargeur de vidéos**: yt-dlp (intégré)
- **Système de build**: NSIS (installateur) + GitHub Actions

## Installation

### Depuis la Release

1. Téléchargez la dernière release depuis [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. Choisissez entre:
   - **Installateur (NSIS)**: Exécutez l'installateur `.exe`
   - **Portable**: Extrayez le `.zip` et exécutez `omigdex-app.exe`

### Depuis le Code Source

**Prérequis**:
- Node.js (v18 ou supérieur)
- pnpm
- Toolchain Rust
- yt-dlp (pour le développement)

**Étapes**:

```bash
# Clonez le repository
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# Installez les dépendances
pnpm install

# Exécutez en mode développement
pnpm tauri dev

# Construisez pour la production
pnpm tauri build
```

## Utilisation

1. **Télécharger une vidéo**:
   - Collez l'URL de la vidéo dans le champ de saisie
   - Sélectionnez le format (MP4 ou MP3)
   - Sélectionnez la qualité (Meilleure, Haute, Moyenne, Basse)
   - Cliquez sur "Download"

2. **Gérer les téléchargements**:
   - Visualisez les téléchargements actifs dans l'onglet "Queue"
   - Annulez les téléchargements si nécessaire
   - Nettoyez les téléchargements terminés

3. **Voir l'historique**:
   - Passez à l'onglet "History"
   - Visualisez tous les téléchargements passés
   - Supprimez des entrées individuelles ou nettoyez tout

4. **Basculer le thème**:
   - Cliquez sur le bouton de thème dans l'en-tête
   - Basculez entre les modes clair et sombre

## Structure du Projet

```
omigdex-app/
├── src/                    # Frontend React
│   ├── components/         # Composants React
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # Composants shadcn/ui
│   ├── lib/               # Utilitaires
│   │   └── utils.ts
│   ├── App.tsx            # Composant principal
│   └── main.tsx           # Point d'entrée
├── src-tauri/             # Backend Rust
│   ├── src/
│   │   ├── main.rs        # Point d'entrée
│   │   ├── lib.rs         # Commandes Tauri
│   │   ├── downloader.rs  # Intégration yt-dlp
│   │   ├── queue.rs       # Gestion de la file
│   │   ├── history.rs     # Historique persistant
│   │   └── types.rs       # Structures de données
│   └── Cargo.toml         # Dépendances Rust
├── .github/
│   └── workflows/
│       └── build.yml      # Pipeline CI/CD
└── create-portable.bat    # Script de build portable
```

## Architecture Backend Rust

Le backend se compose de plusieurs modules:

- **`main.rs`**: Point d'entrée de l'application
- **`lib.rs`**: Définitions des commandes Tauri et initialisation
- **`downloader.rs`**: Intégration avec yt-dlp pour le téléchargement de vidéos
- **`queue.rs`**: File de téléchargement avec gestion simultanée
- **`history.rs`**: Historique de téléchargement persistant en JSON
- **`types.rs`**: Structures de données et enums

## Développement

**Exécuter le serveur de développement**:
```bash
pnpm tauri dev
```

**Exécuter uniquement le frontend** (sans backend):
```bash
npm run dev
```

**Construire l'installateur**:
```bash
pnpm tauri build
```

**Créer la version portable**:
```bash
create-portable.bat
```

## Créer des Releases

Les releases sont créées automatiquement via GitHub Actions lorsque vous poussez un tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Le workflow crée:
- Installateur NSIS avec yt-dlp inclus
- ZIP portable avec yt-dlp inclus

## Problèmes Connus

- **Avertissements antivirus**: Les exécutables non signés peuvent déclencher Windows Defender ou autre logiciel antivirus. C'est normal pour les applications non signées. Envisagez d'ajouter une exception ou d'utiliser la version portable.
- **Signature de code**: Pour éviter les avertissements antivirus, l'application a besoin d'une signature numérique. Pour les projets open source, [SignPath.io](https://signpath.io/) offre une signature de code gratuite.

## Contributions

Les contributions sont les bienvenues! N'hésitez pas à envoyer une Pull Request.

## Licence

Ce projet est open source et est disponible sous la Licence MIT.

## Remerciements

- [Tauri](https://tauri.app/) - Framework desktop multiplateforme
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - Téléchargeur de vidéos
- [Tailwind CSS](https://tailwindcss.com/) - Framework CSS
- [shadcn/ui](https://ui.shadcn.com/) - Composants UI

## Liens

- [Repository GitHub](https://github.com/Cr12dev/omigdex)
- [Releases](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## Configuration IDE Recommandée

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
