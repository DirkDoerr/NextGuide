#!/bin/bash
# Launcher for NextGuide.app — points GTK at bundled runtime data, then execs the real binary.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export DYLD_LIBRARY_PATH="${ROOT}/Frameworks${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"
export GTK_EXE_PREFIX="${ROOT}/Resources"
export GTK_DATA_PREFIX="${ROOT}/Resources"
export XDG_DATA_DIRS="${ROOT}/Resources/share${XDG_DATA_DIRS:+:$XDG_DATA_DIRS}"
export GSETTINGS_SCHEMA_DIR="${ROOT}/Resources/share/glib-2.0/schemas"
# Prefer versioned loader dir if present; fall back to flat cache path.
if [[ -d "${ROOT}/Resources/lib/gdk-pixbuf-2.0" ]]; then
  for d in "${ROOT}/Resources/lib/gdk-pixbuf-2.0"/*/loaders; do
    if [[ -d "$d" ]]; then
      export GDK_PIXBUF_MODULEDIR="$d"
      break
    fi
  done
fi
if [[ -f "${ROOT}/Resources/lib/gdk-pixbuf-2.0/loaders.cache" ]]; then
  export GDK_PIXBUF_MODULE_FILE="${ROOT}/Resources/lib/gdk-pixbuf-2.0/loaders.cache"
elif [[ -n "${GDK_PIXBUF_MODULEDIR:-}" && -f "${GDK_PIXBUF_MODULEDIR}/../loaders.cache" ]]; then
  export GDK_PIXBUF_MODULE_FILE="${GDK_PIXBUF_MODULEDIR}/../loaders.cache"
fi
export GTK_PATH="${ROOT}/Resources"

exec "${ROOT}/MacOS/nextguide-bin" "$@"
