@echo off
echo Creating Omigdex Portable Package...
echo.

set "SOURCE_DIR=%~dp0src-tauri\target\release"
set "PORTABLE_DIR=%~dp0omigdex-portable"
set "EXE_NAME=omigdex-app.exe"

echo Step 1: Building frontend...
call pnpm build
if errorlevel 1 (
    echo ERROR: Failed to build frontend.
    pause
    exit /b 1
)

echo Step 2: Creating portable directory...
if exist "%PORTABLE_DIR%" rmdir /s /q "%PORTABLE_DIR%"
mkdir "%PORTABLE_DIR%"
mkdir "%PORTABLE_DIR%\downloads"
mkdir "%PORTABLE_DIR%\resources"

echo Step 3: Copying application executable...
copy "%SOURCE_DIR%\%EXE_NAME%" "%PORTABLE_DIR%\" >nul
if errorlevel 1 (
    echo ERROR: Could not find executable at %SOURCE_DIR%\%EXE_NAME%
    echo Please run 'pnpm tauri build' first to build the release executable.
    pause
    exit /b 1
)

echo Step 4: Copying frontend assets...
xcopy "%~dp0dist" "%PORTABLE_DIR%\dist\" /E /I /Y >nul

echo Step 5: Downloading yt-dlp...
powershell -Command "Invoke-WebRequest -Uri 'https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe' -OutFile '%PORTABLE_DIR%\yt-dlp.exe'"
if errorlevel 1 (
    echo WARNING: Failed to download yt-dlp automatically.
    echo You can download it manually from https://github.com/yt-dlp/yt-dlp/releases
) else (
    echo yt-dlp downloaded successfully.
)

echo Step 6: Creating README...
(
echo Omigdex Portable - Video Downloader
echo ====================================
echo.
echo Features:
echo - Download videos from YouTube, Instagram, TikTok, Pinterest
echo - MP4 and MP3 format support
echo - Quality selection (Best, High, Medium, Low)
echo - Download queue with up to 3 concurrent downloads
echo - Download history
echo - Dark/Light theme
echo - Minimalist UI without emojis
echo - Toast notifications for feedback
echo.
echo Requirements:
echo - yt-dlp is included in this package
echo - No additional dependencies required
echo.
echo Usage:
echo 1. Double-click omigdex-app.exe to run the application
echo 2. Paste a video URL and click Download
echo 3. Select format (MP4/MP3) and quality
echo 4. Monitor downloads in the Queue tab
echo 5. View download history in the History tab
echo.
echo Downloads are saved in the 'downloads' folder.
echo.
echo Note: This is a portable version. You can move the entire
echo 'omigdex-portable' folder to any location on your computer.
) > "%PORTABLE_DIR%\README.txt"

echo Step 5: Creating launch script...
(
echo @echo off
echo cd /d "%~dp0"
echo start "" "%EXE_NAME%"
) > "%PORTABLE_DIR%\Omigdex.lnk"

echo.
echo ====================================
echo Portable package created successfully!
echo Location: %PORTABLE_DIR%
echo.
echo You can now:
echo 1. Move the 'omigdex-portable' folder anywhere
echo 2. Run omigdex-app.exe to start the application
echo 3. The application will use the included yt-dlp
echo.
pause
