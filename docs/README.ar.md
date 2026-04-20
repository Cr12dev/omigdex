# Omigdex

تطبيق مفتوح المصدر لتنزيل الفيديوهات مبني باستخدام Tauri و React و TypeScript و Rust. قم بتنزيل مقاطع الفيديو من YouTube و Instagram و TikTok و Pinterest بسهولة.

## الميزات

- **دعم منصات متعددة**: قم بتنزيل الفيديوهات من YouTube و Instagram و TikTok و Pinterest
- **تنسيقات متعددة**: قم بالتنزيل بتنسيق MP4 (فيديو) أو MP3 (صوت)
- **اختيار الجودة**: اختر بين الأفضل، العالي (1080p)، المتوسط (720p)، أو الجودة المنخفضة
- **قائمة الانتظار**: قم بإدارة ما يصل إلى 3 تنزيلات متزامنة تلقائياً
- **سجل التنزيل**: تتبع جميع تنزيلاتك مع تخزين دائم
- **الوضع الداكن/الفاتح**: التبديل بين الوضع الداكن والفاتح
- **إشعارات Toast**: رد فعل فوري على حالة التنزيل
- **واجهة بسيطة**: تصميم نظيف وعصري بدون رموز تعبيرية
- **الإصدار المحمول**: تشغيل بدون تثبيت (يتضمن yt-dlp)
- **تنزيلات سريعة**: محسّن للسرعة (فيديو 15 دقيقة في ~10 ثواني)

## مجموعة التقنيات

- **الواجهة الأمامية**: React + TypeScript + Vite
- **الواجهة الخلفية**: Rust مع Tauri
- **التنسيق**: Tailwind CSS + shadcn/ui
- **محمّل الفيديو**: yt-dlp (مدمج)
- **نظام البناء**: NSIS (مثبت) + GitHub Actions

## التثبيت

### من الإصدار

1. قم بتنزيل أحدث إصدار من [GitHub Releases](https://github.com/Cr12dev/omigdex/releases)
2. اختر بين:
   - **المثبت (NSIS)**: قم بتشغيل المثبت `.exe`
   - **المحمول**: قم باستخراج `.zip` وتشغيل `omigdex-app.exe`

### من الكود المصدري

**المتطلبات المسبقة**:
- Node.js (v18 أو أحدث)
- pnpm
- Rust toolchain
- yt-dlp (للتطوير)

**الخطوات**:

```bash
# استنسخ المستودع
git clone https://github.com/Cr12dev/omigdex.git
cd omigdex

# قم بتثبيت التبعيات
pnpm install

# قم بالتشغيل في وضع التطوير
pnpm tauri dev

# قم بالبناء للإنتاج
pnpm tauri build
```

## الاستخدام

1. **تنزيل فيديو**:
   - الصق رابط الفيديو في حقل الإدخال
   - اختر التنسيق (MP4 أو MP3)
   - اختر الجودة (الأفضل، العالي، المتوسط، المنخفض)
   - انقر على "Download"

2. **إدارة التنزيلات**:
   - عرض التنزيلات النشطة في علامة التبويب "Queue"
   - إلغاء التنزيلات إذا لزم الأمر
   - مسح التنزيلات المكتملة

3. **عرض السجل**:
   - التبديل إلى علامة التبويب "History"
   - عرض جميع التنزيلات السابقة
   - إزالة الإدخالات الفردية أو مسح الكل

4. **تبديل السمة**:
   - انقر على زر السمة في الرأس
   - التبديل بين الوضع الداكن والفاتح

## هيكل المشروع

```
omigdex-app/
├── src/                    # React Frontend
│   ├── components/         # مكونات React
│   │   ├── DownloadForm.tsx
│   │   ├── DownloadQueue.tsx
│   │   ├── DownloadHistory.tsx
│   │   ├── PlatformIcon.tsx
│   │   ├── ThemeToggle.tsx
│   │   ├── Toast.tsx
│   │   └── ui/             # مكونات shadcn/ui
│   ├── lib/               # أدوات مساعدة
│   │   └── utils.ts
│   ├── App.tsx            # المكون الرئيسي
│   └── main.tsx           # نقطة الدخول
├── src-tauri/             # Rust Backend
│   ├── src/
│   │   ├── main.rs        # نقطة الدخول
│   │   ├── lib.rs         # أوامر Tauri
│   │   ├── downloader.rs  # تكامل yt-dlp
│   │   ├── queue.rs       # إدارة القائمة
│   │   ├── history.rs     # سجل دائم
│   │   └── types.rs       # هياكل البيانات
│   └── Cargo.toml         # تبعيات Rust
├── .github/
│   └── workflows/
│       └── build.yml      # CI/CD pipeline
└── create-portable.bat    # سكريبت بناء المحمول
```

## بنية Rust Backend

يتكون الخلفي من عدة وحدات:

- **`main.rs`**: نقطة دخول التطبيق
- **`lib.rs`**: تعريفات أوامر Tauri والتهيئة
- **`downloader.rs`**: التكامل مع yt-dlp لتنزيل الفيديوهات
- **`queue.rs`**: قائمة التنزيل مع إدارة متزامنة
- **`history.rs`**: سجل تنزيل دائم في JSON
- **`types.rs`**: هياكل البيانات والتعدادات

## التطوير

**تشغيل خادم التطوير**:
```bash
pnpm tauri dev
```

**تشغيل الواجهة الأمامية فقط** (بدون الخلفية):
```bash
npm run dev
```

**بناء المثبت**:
```bash
pnpm tauri build
```

**إنشاء الإصدار المحمول**:
```bash
create-portable.bat
```

## إنشاء الإصدارات

يتم إنشاء الإصدارات تلقائياً عبر GitHub Actions عند إرسال علامة:

```bash
git tag v1.0.0
git push origin v1.0.0
```
يقوم سير العمل بإنشاء:
- مثبت NSIS مع yt-dlp مضمن
- ZIP محمول مع yt-dlp مضمن

## المشاكل المعروفة

- **تحذيرات مكافحة الفيروسات**: قد تؤدي الملفات التنفيذية غير الموقعة إلى تشغيل Windows Defender أو برامج مكافحة الفيروسات الأخرى. هذا أمر طبيعي للتطبيقات غير الموقعة. فكر في إضافة استثناء أو استخدام الإصدار المحمول.
- **توقيع الكود**: لتجنب تحذيرات مكافحة الفيروسات، يحتاج التطبيق إلى توقيع رقمي. للمشاريع مفتوحة المصدر، يقدم [SignPath.io](https://signpath.io/) توقيع كود مجاني.

## المساهمات

المساهمات مرحب بها! لا تتردد في إرسال Pull Request.

## الترخيص

هذا المشروع مفتوح المصدر ومتاح بموجب ترخيص GPL-3.0.

## الشكر

- [Tauri](https://tauri.app/) - إطار عمل سطح المكتب متعدد المنصات
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - محمّل الفيديو
- [Tailwind CSS](https://tailwindcss.com/) - إطار عمل CSS
- [shadcn/ui](https://ui.shadcn.com/) - مكونات UI

## الروابط

- [مستودع GitHub](https://github.com/Cr12dev/omigdex)
- [الإصدارات](https://github.com/Cr12dev/omigdex/releases)
- [المشاكل](https://github.com/Cr12dev/omigdex/issues)

## تكوين IDE الموصى به

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
