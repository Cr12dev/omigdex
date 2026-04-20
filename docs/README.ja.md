# Omigdex

Tauri、React、TypeScript、Rustを使用して構築されたオープンソースのビデオダウンローダーアプリケーション。YouTube、Instagram、TikTok、Pinterestからビデオを簡単にダウンロードできます。

## 機能

- **マルチプラットフォーム対応**: YouTube、Instagram、TikTok、Pinterestからビデオをダウンロード
- **複数のフォーマット**: MP4（ビデオ）またはMP3（オーディオ）形式でダウンロード
- **品質選択**: 最高、高（1080p）、中（720p）、低品質から選択
- **ダウンロードキュー**: 最大3つの同時ダウンロードを自動的に管理
- **ダウンロード履歴**: 永続ストレージですべてのダウンロードを追跡
- **ダーク/ライトテーマ**: ダークモードとライトモードを切り替え
- **トースト通知**: ダウンロード状態のリアルタイムフィードバック
- **ミニマルUI**: 絵文字のないシンプルでモダンなデザイン
- **ポータブル版**: インストールなしで実行（yt-dlpを含む）
- **高速ダウンロード**: 速度に最適化（15分のビデオを約10秒でダウンロード）

## 技術スタック

- **フロントエンド**: React + TypeScript + Vite
- **バックエンド**: RustとTauri
- **スタイリング**: Tailwind CSS + shadcn/ui
- **ビデオダウンローダー**: yt-dlp（組み込み）
- **ビルドシステム**: NSIS（インストーラー）+ GitHub Actions

## インストール

### リリースから

1. [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)から最新版をダウンロード
2. 次から選択:
   - **インストーラー（NSIS）**: `.exe`インストーラーを実行
   - **ポータブル**: `.zip`を展開して`omigdex-app.exe`を実行

### ソースから

**前提条件**:
- Node.js（v18以降）
- pnpm
- Rust toolchain
- yt-dlp（開発用）

**手順**:

```bash
# リポジトリをクローン
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# 依存関係をインストール
pnpm install

# 開発モードで実行
pnpm tauri dev

# 本番用にビルド
pnpm tauri build
```

## 使用方法

1. **ビデオをダウンロード**:
   - 入力フィールドにビデオURLを貼り付け
   - フォーマットを選択（MP4またはMP3）
   - 品質を選択（最高、高、中、低）
   - "Download"をクリック

2. **ダウンロードを管理**:
   - "Queue"タブでアクティブなダウンロードを表示
   - 必要に応じてダウンロードをキャンセル
   - 完了したダウンロードをクリア

3. **履歴を表示**:
   - "History"タブに切り替え
   - 過去のすべてのダウンロードを表示
   - 個別のエントリを削除またはすべてをクリア

4. **テーマを切り替え**:
   - ヘッダーのテーマボタンをクリック
   - ダークモードとライトモードを切り替え

## プロジェクト構造

```
omigdex-app/
├── src/                    # React Frontend
│   ├── components/         # Reactコンポーネント
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/uiコンポーネント
│   ├── lib/               # ユーティリティ
│   │   └── utils.ts
│   ├── App.tsx            # メインコンポーネント
│   └── main.tsx           # エントリーポイント
├── src-tauri/             # Rust Backend
│   ├── src/
│   │   ├── main.rs        # エントリーポイント
│   │   ├── lib.rs         # Tauriコマンド
│   │   ├── downloader.rs  # yt-dlp統合
│   │   ├── queue.rs       # キュー管理
│   │   ├── history.rs     # 永続履歴
│   │   └── types.rs       # データ構造
│   └── Cargo.toml         # Rust依存関係
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CDパイプライン
└── create-portable.bat    # ポータブルビルドスクリプト
```

## Rustバックエンドアーキテクチャ

バックエンドは複数のモジュールで構成されています:

- **`main.rs`**: アプリケーションのエントリーポイント
- **`lib.rs`**: Tauriコマンド定義と初期化
- **`downloader.rs`**: ビデオダウンロード用のyt-dlp統合
- **`queue.rs`**: 並行管理付きダウンロードキュー
- **`history.rs`**: JSON内の永続ダウンロード履歴
- **`types.rs`**: データ構造と列挙型

## 開発

**開発サーバーを実行**:
```bash
pnpm tauri dev
```

**フロントエンドのみを実行**（バックエンドなし）:
```bash
npm run dev
```

**インストーラーをビルド**:
```bash
pnpm tauri build
```

**ポータブル版を作成**:
```bash
create-portable.bat
```

## リリースの作成

タグをプッシュすると、GitHub Actionsを通じてリリースが自動的に作成されます:

```bash
git tag v1.0.0
git push origin v1.0.0
```

ワークフローは以下を作成します:
- yt-dlpを含むNSISインストーラー
- yt-dlpを含むポータブルZIP

## 既知の問題

- **アンチウイルス警告**: 署名なしの実行可能ファイルはWindows Defenderまたは他のアンチウイルスソフトウェアをトリガーする可能性があります。これは署名なしアプリケーションでは正常です。例外を追加するか、ポータブル版を使用することを検討してください。
- **コード署名**: アンチウイルス警告を回避するには、デジタル署名が必要です。オープンソースプロジェクトの場合、[SignPath.io](https://signpath.io/)は無料のコード署名を提供しています。

## 貢献

貢献は歓迎します！お気軽にプルリクエストを送信してください。

## ライセンス

このプロジェクトはオープンソースであり、MITライセンスの下で利用可能です。

## 謝辞

- [Tauri](https://tauri.app/) - マルチプラットフォームデスクトップフレームワーク
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - ビデオダウンローダー
- [Tailwind CSS](https://tailwindcss.com/) - CSSフレームワーク
- [shadcn/ui](https://ui.shadcn.com/) - UIコンポーネント

## リンク

- [GitHubリポジトリ](https://github.com/Cr12dev/omigdex)
- [リリース](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## 推奨IDE設定

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
