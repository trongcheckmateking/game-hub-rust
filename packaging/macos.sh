#!/usr/bin/env bash
set -eu
app='dist/GAME HUB.app'
mkdir -p "$app/Contents/MacOS"
case "$(uname -m)" in
  arm64) other_target=x86_64-apple-darwin ;;
  x86_64) other_target=aarch64-apple-darwin ;;
  *) echo "Unsupported macOS build host" >&2; exit 1 ;;
esac
rustup target add "$other_target"
cargo build --release --locked --target "$other_target"
lipo -create target/release/game-hub "target/$other_target/release/game-hub" -output "$app/Contents/MacOS/game-hub"
lipo "$app/Contents/MacOS/game-hub" -verify_arch arm64 x86_64
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
