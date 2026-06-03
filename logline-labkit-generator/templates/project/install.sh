#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: ./install.sh [--prefix <path>] [--dev] [--help]

Build and install the local logline-lab CLI without sudo or external services.

Options:
  --prefix <path>  Install to <path>/bin (default: $HOME/.local)
  --dev            Build a debug binary and install that copy
  --help           Show this help

Examples:
  ./install.sh
  ./install.sh --prefix "$HOME/.local"
  ./install.sh --dev --prefix "$PWD/.local"
USAGE
}

PREFIX="${PREFIX:-$HOME/.local}"
PROFILE="release"
BUILD_ARGS=(--release)

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prefix)
      if [[ $# -lt 2 ]]; then
        echo "install: missing value for --prefix" >&2
        exit 2
      fi
      PREFIX="$2"
      shift 2
      ;;
    --dev)
      PROFILE="debug"
      BUILD_ARGS=()
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "install: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if ! command -v cargo >/dev/null 2>&1; then
  cat >&2 <<'ERROR'
install: Rust/Cargo is required but cargo was not found on PATH.
Install Rust from https://www.rust-lang.org/tools/install, then rerun ./install.sh.
No Supabase or external service is required for the local-offline profile.
ERROR
  exit 1
fi

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

VERSION="$(tr -d '[:space:]' < VERSION)"
BIN_DIR="$PREFIX/bin"
SOURCE_BIN="target/$PROFILE/logline-lab"
TARGET_BIN="$BIN_DIR/logline-lab"

echo "Installing logline-lab $VERSION"
echo "Project: $PROJECT_ROOT"
echo "Profile: $PROFILE"
echo "Prefix:  $PREFIX"
echo

cargo build "${BUILD_ARGS[@]}" -p logline-lab-cli
mkdir -p "$BIN_DIR"
cp "$SOURCE_BIN" "$TARGET_BIN"
chmod 0755 "$TARGET_BIN"

cat <<NEXT

Installed: $TARGET_BIN

Next steps:
  export PATH="$BIN_DIR:\$PATH"
  logline-lab --version
  logline-lab serve
  # then open http://127.0.0.1:8787

Authority boundary:
  local-offline uses local workspace state only; it is not an official spine,
  not a receipt store, and does not require Supabase or external services.

Supabase profile:
  apply supabase/migrations/0001_ops_logline_acts.sql with your reviewed
  Supabase migration flow, then set DATABASE_URL. REST env is supported only
  for compatible RPC wrappers: SUPABASE_URL plus SUPABASE_SERVICE_ROLE_KEY or
  SUPABASE_SECRET_KEY.
  Use: logline-lab supabase check
NEXT
