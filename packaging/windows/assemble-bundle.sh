#!/usr/bin/env bash
# Assemble a self-contained Windows folder: nextguide.exe + nextguide-cli.exe + GTK runtime.
# Run inside an MSYS2 UCRT64 shell from the repo root (or any cwd; script cds to root).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
cd "${REPO_ROOT}"

VERSION="${VERSION:-$(sed -n 's/^version = "\(.*\)"/\1/p' crates/nextguide/Cargo.toml | head -1)}"
MSYSTEM_PREFIX="${MSYSTEM_PREFIX:-/ucrt64}"
BIN="${MSYSTEM_PREFIX}/bin"
BUNDLE="${REPO_ROOT}/dist/windows-bundle"

if [[ ! -d "${BIN}" ]]; then
  echo "error: ${BIN} not found. Run this script in an MSYS2 UCRT64 shell." >&2
  exit 1
fi
if ! command -v pacman >/dev/null 2>&1; then
  echo "error: pacman not found — expected MSYS2 environment." >&2
  exit 1
fi

echo "==> Building release binaries (UCRT64)"
cargo build --release -p nextguide -p nextguide-cli

echo "==> Assembling ${BUNDLE}"
rm -rf "${BUNDLE}"
mkdir -p "${BUNDLE}"

cp target/release/nextguide.exe "${BUNDLE}/"
cp target/release/nextguide-cli.exe "${BUNDLE}/"

# gdk-pixbuf loaders + relative cache
PB_VER="$(ls "${MSYSTEM_PREFIX}/lib/gdk-pixbuf-2.0" | head -1)"
LOADERS="${BUNDLE}/lib/gdk-pixbuf-2.0/${PB_VER}/loaders"
mkdir -p "${LOADERS}"
cp "${MSYSTEM_PREFIX}/lib/gdk-pixbuf-2.0/${PB_VER}/loaders/"*.dll "${LOADERS}/" 2>/dev/null || true
if command -v gdk-pixbuf-query-loaders.exe >/dev/null 2>&1; then
  (
    cd "${LOADERS}"
    gdk-pixbuf-query-loaders.exe ./*.dll \
      | sed "s|$(pwd -W 2>/dev/null || pwd)/||g; s|\\\\|/|g" \
      > ../loaders.cache
  )
fi

copy_dll_closure() {
  local target="$1"
  local deps
  deps="$(ldd "${target}" 2>/dev/null | awk '{print $3}' | grep -iE "^${MSYSTEM_PREFIX}/bin/" | sort -u || true)"
  while IFS= read -r dll; do
    [[ -n "${dll}" && -f "${dll}" ]] || continue
    cp -n "${dll}" "${BUNDLE}/" || true
  done <<< "${deps}"
}

echo "==> Collecting DLL closure"
copy_dll_closure "${BUNDLE}/nextguide.exe"
for loader in "${LOADERS}"/*.dll; do
  [[ -f "${loader}" ]] || continue
  copy_dll_closure "${loader}"
done
# CLI is typically dependency-light; still collect anything it needs from UCRT64.
copy_dll_closure "${BUNDLE}/nextguide-cli.exe"

# Drop unused GStreamer media backend DLLs when GTK does not hard-link them.
if ! objdump -p "${BIN}/libgtk-4-1.dll" 2>/dev/null | awk '/DLL Name:/{print $3}' | grep -qi gst; then
  rm -f "${BUNDLE}"/libgst*.dll 2>/dev/null || true
fi

echo "==> GSettings schemas + icons"
mkdir -p "${BUNDLE}/share/glib-2.0/schemas" "${BUNDLE}/share/icons"
if [[ -f "${MSYSTEM_PREFIX}/share/glib-2.0/schemas/gschemas.compiled" ]]; then
  cp "${MSYSTEM_PREFIX}/share/glib-2.0/schemas/gschemas.compiled" \
    "${BUNDLE}/share/glib-2.0/schemas/"
else
  glib-compile-schemas "${MSYSTEM_PREFIX}/share/glib-2.0/schemas" \
    --targetdir "${BUNDLE}/share/glib-2.0/schemas"
fi
for theme in Adwaita hicolor; do
  if [[ -d "${MSYSTEM_PREFIX}/share/icons/${theme}" ]]; then
    cp -R "${MSYSTEM_PREFIX}/share/icons/${theme}" "${BUNDLE}/share/icons/"
  fi
done
gtk4-update-icon-cache.exe -q -t -f "${BUNDLE}/share/icons/Adwaita" 2>/dev/null || true

# Runtime env helper for unzipped smoke tests (installer sets working directory).
cat > "${BUNDLE}/run-nextguide.bat" <<'EOF'
@echo off
setlocal
set "ROOT=%~dp0"
set "XDG_DATA_DIRS=%ROOT%share"
set "GSETTINGS_SCHEMA_DIR=%ROOT%share\glib-2.0\schemas"
set "GDK_PIXBUF_MODULEDIR=%ROOT%lib\gdk-pixbuf-2.0\loaders"
for /d %%D in ("%ROOT%lib\gdk-pixbuf-2.0\*") do (
  if exist "%%D\loaders.cache" set "GDK_PIXBUF_MODULE_FILE=%%D\loaders.cache"
)
if exist "%ROOT%lib\gdk-pixbuf-2.0\loaders.cache" set "GDK_PIXBUF_MODULE_FILE=%ROOT%lib\gdk-pixbuf-2.0\loaders.cache"
start "" "%ROOT%nextguide.exe" %*
EOF

cat > "${BUNDLE}/VERSION.txt" <<EOF
NextGuide ${VERSION}
Contents: nextguide.exe (GTK4 bundled) + nextguide-cli.exe
Updates: manual (download a newer installer / bundle)
Signing: optional for early releases
EOF

DLL_COUNT="$(ls -1 "${BUNDLE}"/*.dll 2>/dev/null | wc -l | tr -d ' ')"
echo "Assembled ${BUNDLE} (${DLL_COUNT} DLLs), version ${VERSION}"
echo "Next: packaging/windows/build-installer.ps1  (or zip this folder)"
