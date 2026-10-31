#!/usr/bin/env bash
# Build NextGuide.app (GTK4 runtime bundled) + nextguide-cli into a DMG.
# Requires: brew install gtk4 dylibbundler; Rust toolchain; run from anywhere.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

VERSION="${VERSION:-$(sed -n 's/^version = "\(.*\)"/\1/p' crates/nextguide/Cargo.toml | head -1)}"
ARCH="$(uname -m)"
DIST="${REPO_ROOT}/dist"
APP_NAME="NextGuide"
APP_ROOT="${DIST}/${APP_NAME}.app"
DMG_STAGING="${DIST}/dmg-staging"
DMG_PATH="${DIST}/${APP_NAME}-${VERSION}-macos-${ARCH}.dmg"
GTK_PREFIX="${GTK_PREFIX:-$(pkg-config --variable=prefix gtk4 2>/dev/null || brew --prefix gtk4)}"

if ! command -v dylibbundler >/dev/null 2>&1; then
  echo "error: dylibbundler not found. Install with: brew install dylibbundler" >&2
  exit 1
fi
if ! pkg-config --exists gtk4; then
  echo "error: gtk4 pkg-config not found. Install with: brew install gtk4" >&2
  exit 1
fi

echo "==> Building release binaries"
cargo build --release -p nextguide -p nextguide-cli

echo "==> Assembling ${APP_NAME}.app"
rm -rf "${APP_ROOT}" "${DMG_STAGING}"
mkdir -p \
  "${APP_ROOT}/Contents/MacOS" \
  "${APP_ROOT}/Contents/Frameworks" \
  "${APP_ROOT}/Contents/Resources/share/glib-2.0/schemas" \
  "${APP_ROOT}/Contents/Resources/share/icons" \
  "${APP_ROOT}/Contents/Resources/lib/gdk-pixbuf-2.0"

cp "${SCRIPT_DIR}/Info.plist" "${APP_ROOT}/Contents/Info.plist"
# Patch version into Info.plist (portable sed).
if [[ "$(uname)" == "Darwin" ]]; then
  sed -i '' "s|<string>0.1.0</string>|<string>${VERSION}</string>|g" "${APP_ROOT}/Contents/Info.plist"
else
  sed -i "s|<string>0.1.0</string>|<string>${VERSION}</string>|g" "${APP_ROOT}/Contents/Info.plist"
fi

cp target/release/nextguide "${APP_ROOT}/Contents/MacOS/nextguide-bin"
cp "${SCRIPT_DIR}/launcher.sh" "${APP_ROOT}/Contents/MacOS/NextGuide"
chmod +x "${APP_ROOT}/Contents/MacOS/NextGuide" "${APP_ROOT}/Contents/MacOS/nextguide-bin"

# CLI ships beside the .app on the DMG (same release artifact set).
# nextguide-cli has no GTK linkage — copy the release binary as-is.
CLI_STAGING="${DIST}/nextguide-cli-macos"
rm -rf "${CLI_STAGING}"
mkdir -p "${CLI_STAGING}"
cp target/release/nextguide-cli "${CLI_STAGING}/nextguide-cli"
chmod +x "${CLI_STAGING}/nextguide-cli"

echo "==> Bundling GTK dylibs into Frameworks"
dylibbundler -od -b \
  -x "${APP_ROOT}/Contents/MacOS/nextguide-bin" \
  -d "${APP_ROOT}/Contents/Frameworks" \
  -p "@executable_path/../Frameworks/"

echo "==> Copying GTK runtime data (schemas, pixbuf loaders, icons)"
# Compiled GSettings schemas
if [[ -f "${GTK_PREFIX}/share/glib-2.0/schemas/gschemas.compiled" ]]; then
  cp "${GTK_PREFIX}/share/glib-2.0/schemas/gschemas.compiled" \
    "${APP_ROOT}/Contents/Resources/share/glib-2.0/schemas/"
else
  glib-compile-schemas "${GTK_PREFIX}/share/glib-2.0/schemas" \
    --targetdir "${APP_ROOT}/Contents/Resources/share/glib-2.0/schemas"
fi

# gdk-pixbuf loaders + relative cache
PB_VER="$(ls "${GTK_PREFIX}/lib/gdk-pixbuf-2.0" | head -1)"
LOADERS_SRC="${GTK_PREFIX}/lib/gdk-pixbuf-2.0/${PB_VER}/loaders"
LOADERS_DST="${APP_ROOT}/Contents/Resources/lib/gdk-pixbuf-2.0/${PB_VER}/loaders"
mkdir -p "${LOADERS_DST}"
if [[ -d "${LOADERS_SRC}" ]]; then
  cp "${LOADERS_SRC}"/*.so "${LOADERS_DST}/" 2>/dev/null || true
  # Also rewrite loader deps into Frameworks
  for so in "${LOADERS_DST}"/*.so; do
    [[ -f "$so" ]] || continue
    dylibbundler -od -b \
      -x "$so" \
      -d "${APP_ROOT}/Contents/Frameworks" \
      -p "@loader_path/../../../../Frameworks/" \
      || true
  done
  # Relative loaders.cache (paths relative to module dir)
  if command -v gdk-pixbuf-query-loaders >/dev/null 2>&1; then
    (
      cd "${LOADERS_DST}"
      gdk-pixbuf-query-loaders ./*.so \
        | sed "s|${LOADERS_DST}/||g" \
        > ../loaders.cache
    )
    # launcher expects loaders.cache at lib/gdk-pixbuf-2.0/loaders.cache
    cp "${APP_ROOT}/Contents/Resources/lib/gdk-pixbuf-2.0/${PB_VER}/loaders.cache" \
      "${APP_ROOT}/Contents/Resources/lib/gdk-pixbuf-2.0/loaders.cache" 2>/dev/null \
      || mv "${APP_ROOT}/Contents/Resources/lib/gdk-pixbuf-2.0/${PB_VER}/loaders.cache" \
        "${APP_ROOT}/Contents/Resources/lib/gdk-pixbuf-2.0/loaders.cache"
  fi
fi

# Icons (Adwaita + hicolor fallback)
for theme in Adwaita hicolor; do
  if [[ -d "${GTK_PREFIX}/share/icons/${theme}" ]]; then
    cp -R "${GTK_PREFIX}/share/icons/${theme}" \
      "${APP_ROOT}/Contents/Resources/share/icons/"
  fi
done
# Homebrew sometimes puts Adwaita in a separate formula prefix
ADWAITA_PREFIX="$(brew --prefix adwaita-icon-theme 2>/dev/null || true)"
if [[ -n "${ADWAITA_PREFIX}" && -d "${ADWAITA_PREFIX}/share/icons/Adwaita" ]]; then
  mkdir -p "${APP_ROOT}/Contents/Resources/share/icons"
  cp -R "${ADWAITA_PREFIX}/share/icons/Adwaita" \
    "${APP_ROOT}/Contents/Resources/share/icons/" 2>/dev/null || true
fi

echo "==> Code signing (optional identity; default ad-hoc)"
CODESIGN_IDENTITY="${CODESIGN_IDENTITY:--}"
codesign --force --deep --sign "${CODESIGN_IDENTITY}" "${APP_ROOT}"
# CLI binary
codesign --force --sign "${CODESIGN_IDENTITY}" "${CLI_STAGING}/nextguide-cli" || true

if [[ "${NOTARIZE:-}" == "1" ]]; then
  echo "==> Notarizing (requires Apple credentials in env)"
  : "${APPLE_ID:?set APPLE_ID}"
  : "${APPLE_TEAM_ID:?set APPLE_TEAM_ID}"
  : "${APPLE_APP_SPECIFIC_PASSWORD:?set APPLE_APP_SPECIFIC_PASSWORD}"
  ZIP_FOR_NOTARY="${DIST}/${APP_NAME}-for-notary.zip"
  ditto -c -k --keepParent "${APP_ROOT}" "${ZIP_FOR_NOTARY}"
  xcrun notarytool submit "${ZIP_FOR_NOTARY}" \
    --apple-id "${APPLE_ID}" \
    --team-id "${APPLE_TEAM_ID}" \
    --password "${APPLE_APP_SPECIFIC_PASSWORD}" \
    --wait
  xcrun stapler staple "${APP_ROOT}"
fi

echo "==> Creating DMG"
mkdir -p "${DMG_STAGING}"
cp -R "${APP_ROOT}" "${DMG_STAGING}/"
cp "${CLI_STAGING}/nextguide-cli" "${DMG_STAGING}/nextguide-cli"
ln -sf /Applications "${DMG_STAGING}/Applications"
rm -f "${DMG_PATH}"
hdiutil create \
  -volname "${APP_NAME}" \
  -srcfolder "${DMG_STAGING}" \
  -ov -format UDZO \
  "${DMG_PATH}"

echo "Built ${DMG_PATH}"
echo "Contents: ${APP_NAME}.app (GTK bundled) + nextguide-cli"
echo "Signing: identity='${CODESIGN_IDENTITY}' (set CODESIGN_IDENTITY / NOTARIZE=1 for release signing)"
