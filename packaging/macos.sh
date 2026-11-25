#!/usr/bin/env bash
set -eu
app='dist/GAME HUB.app'
mkdir -p "$app/Contents/MacOS"
cp target/release/game-hub "$app/Contents/MacOS/game-hub"
cat > "$app/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>game-hub</string>
<key>CFBundleIdentifier</key><string>dev.trong.gamehub</string>
<key>CFBundleName</key><string>GAME HUB</string>
<key>CFBundleVersion</key><string>1</string>
<key>CFBundleShortVersionString</key><string>0.1.0</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
codesign --force --sign - "$app"
ditto -c -k --keepParent "$app" dist/GAME-HUB-macOS.zip
rm -r "$app"
