# Omigdex

使用 Tauri、React、TypeScript 和 Rust 构建的开源视频下载应用程序。轻松从 YouTube、Instagram、TikTok 和 Pinterest 下载视频。

## 功能

- **多平台支持**：从 YouTube、Instagram、TikTok 和 Pinterest 下载视频
- **多种格式**：下载 MP4（视频）或 MP3（音频）格式
- **质量选择**：选择最佳、高（1080p）、中（720p）或低质量
- **下载队列**：自动管理最多 3 个并发下载
- **下载历史**：使用持久存储跟踪所有下载
- **明暗主题**：在亮色和暗色模式之间切换
- **Toast 通知**：下载状态的实时反馈
- **极简界面**：无表情符号的简洁现代设计
- **便携版本**：无需安装即可运行（包含 yt-dlp）
- **快速下载**：针对速度进行了优化（15 分钟视频约 10 秒）

## 技术栈

- **前端**：React + TypeScript + Vite
- **后端**：Rust 与 Tauri
- **样式**：Tailwind CSS + shadcn/ui
- **视频下载器**：yt-dlp（内置）
- **构建系统**：NSIS（安装程序）+ GitHub Actions

## 安装

### 从 Release 安装

1. 从 [GitHub Releases](https://github.com/Cr12dev/omigdex/releases) 下载最新版本
2. 选择：
   - **安装程序 (NSIS)**：运行 `.exe` 安装程序
   - **便携版**：解压 `.zip` 并运行 `omigdex-app.exe`

### 从源码安装

**先决条件**：
- Node.js（v18 或更高版本）
- pnpm
- Rust 工具链
- yt-dlp（用于开发）

**步骤**：

```bash
# 克隆仓库
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# 安装依赖
pnpm install

# 在开发模式下运行
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

## 使用方法

1. **下载视频**：
   - 在输入字段中粘贴视频 URL
   - 选择格式（MP4 或 MP3）
   - 选择质量（最佳、高、中、低）
   - 点击 "Download"

2. **管理下载**：
   - 在 "Queue" 标签页中查看活动下载
   - 如有必要取消下载
   - 清除已完成的下载

3. **查看历史**：
   - 切换到 "History" 标签页
   - 查看所有过去的下载
   - 删除单个条目或清除全部

4. **切换主题**：
   - 点击标题中的主题按钮
   - 在亮色和暗色模式之间切换

## 项目结构

```
omigdex-app/
├── src/                    # React 前端
│   ├── components/         # React 组件
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # shadcn/ui 组件
│   ├── lib/               # 工具
│   │   └── utils.ts
│   ├── App.tsx            # 主组件
│   └── main.tsx           # 入口点
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── main.rs        # 入口点
│   │   ├── lib.rs         # Tauri 命令
│   │   ├── downloader.rs  # yt-dlp 集成
│   │   ├── queue.rs       # 队列管理
│   │   ├── history.rs     # 持久历史
│   │   └── types.rs       # 数据结构
│   └── Cargo.toml         # Rust 依赖
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD 管道
└── create-portable.bat    # 便携版构建脚本
```

## Rust 后端架构

后端由多个模块组成：

- **`main.rs`**：应用程序入口点
- **`lib.rs`**：Tauri 命令定义和初始化
- **`downloader.rs`**：yt-dlp 集成用于视频下载
- **`queue.rs`**：具有并发管理的下载队列
- **`history.rs`：JSON 中的持久下载历史
- **`types.rs`**：数据结构和枚举

## 开发

**运行开发服务器**：
```bash
pnpm tauri dev
```

**仅运行前端**（无后端）：
```bash
npm run dev
```

**构建安装程序**：
```bash
pnpm tauri build
```

**创建便携版**：
```bash
create-portable.bat
```

## 创建 Release

当您推送标签时，Release 会通过 GitHub Actions 自动创建：

```bash
git tag v1.0.0
git push origin v1.0.0
```

工作流程创建：
- 包含 yt-dlp 的 NSIS 安装程序
- 包含 yt-dlp 的便携版 ZIP

## 已知问题

- **防病毒警告**：未签名的可执行文件可能会触发 Windows Defender 或其他防病毒软件。这对于未签名的应用程序是正常的。考虑添加例外或使用便携版。
- **代码签名**：为了避免防病毒警告，应用程序需要数字签名。对于开源项目，[SignPath.io](https://signpath.io/) 提供免费代码签名。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 许可证

本项目是开源的，可在 MIT 许可证下使用。

## 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面框架
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - 视频下载器
- [Tailwind CSS](https://tailwindcss.com/) - CSS 框架
- [shadcn/ui](https://ui.shadcn.com/) - UI 组件

## 链接

- [GitHub 仓库](https://github.com/Cr12dev/omigdex)
- [Release](https://github.com/Cr12dev/omigdex/releases)
- [Issues](https://github.com/Cr12dev/omigdex/issues)

## 推荐的 IDE 配置

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
