; NSIS installer hooks for yt-dlp installation

!macro customInstall
  ; Download and install yt-dlp
  DetailPrint "Installing yt-dlp..."
  
  ; Create a temporary directory for the download
  CreateDirectory "$TEMP\omigdex"
  
  ; Download yt-dlp executable
  DetailPrint "Downloading yt-dlp..."
  inetc::get /silent "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe" "$TEMP\omigdex\yt-dlp.exe" /END
  
  ; Check if download was successful
  IfErrors download_failed download_success
  
download_failed:
  DetailPrint "Failed to download yt-dlp. Please install it manually from https://github.com/yt-dlp/yt-dlp"
  Goto yt_dlp_done
  
download_success:
  ; Copy yt-dlp to the application directory
  DetailPrint "Installing yt-dlp to application directory..."
  CopyFiles "$TEMP\omigdex\yt-dlp.exe" "$INSTDIR\yt-dlp.exe"
  
  ; Add yt-dlp to PATH (optional - for system-wide access)
  ; CreateDirectory "$APPDATA\omigdex"
  ; CopyFiles "$TEMP\omigdex\yt-dlp.exe" "$APPDATA\omigdex\yt-dlp.exe"
  
  DetailPrint "yt-dlp installed successfully"
  
yt_dlp_done:
  ; Clean up temporary files
  Delete "$TEMP\omigdex\yt-dlp.exe"
  RMDir "$TEMP\omigdex"
!macroend

!macro customUnInstall
  ; Optionally remove yt-dlp during uninstallation
  ; Uncomment the line below if you want to remove yt-dlp when uninstalling
  ; Delete "$INSTDIR\yt-dlp.exe"
!macroend
