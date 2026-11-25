#!/usr/bin/env bash
set -eu
mkdir -p dist target/AppDir
cp target/release/game-hub target/AppDir/AppRun
cp assets/art/icon.png target/AppDir/game-hub.png
cat > target/AppDir/game-hub.desktop <<'DESKTOP'
[Desktop Entry]
Name=GAME HUB
Exec=AppRun
Type=Application
Icon=game-hub
Categories=Game;
Terminal=false
DESKTOP
tool="${APPIMAGETOOL:-target/appimagetool}"
if [[ ! -x "$tool" ]]; then
  curl --fail --location --retry 3 https://github.com/AppImage/appimagetool/releases/download/1.9.0/appimagetool-x86_64.AppImage -o "$tool"
  chmod +x "$tool"
fi
ARCH=x86_64 APPIMAGE_EXTRACT_AND_RUN=1 "$tool" target/AppDir dist/GAME-HUB-x86_64.AppImage
