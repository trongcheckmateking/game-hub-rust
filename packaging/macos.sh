#!/usr/bin/env bash
set -eu
app='dist/GAME HUB.app'
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources" target/Hub.iconset
for size in 16 32 128; do
  sips -z "$size" "$size" assets/art/icon.png --out "target/Hub.iconset/icon_${size}x${size}.png" >/dev/null
  double=$((size*2))
  sips -z "$double" "$double" assets/art/icon.png --out "target/Hub.iconset/icon_${size}x${size}@2x.png" >/dev/null
done
cp assets/art/icon.png target/Hub.iconset/icon_256x256.png
iconutil -c icns target/Hub.iconset -o "$app/Contents/Resources/Hub.icns"
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
<key>CFBundleIconFile</key><string>Hub.icns</string>
<key>CFBundleName</key><string>GAME HUB</string>
<key>CFBundleVersion</key><string>3</string>
<key>CFBundleShortVersionString</key><string>0.3.0</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
codesign --force --sign - "$app"
ditto -c -k --keepParent "$app" dist/GAME-HUB-macOS.zip
rm -r "$app"
