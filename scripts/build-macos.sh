#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script must run on macOS."
  exit 1
fi

command -v node >/dev/null || { echo "Node.js is required."; exit 1; }
command -v cargo >/dev/null || { echo "Rust is required: https://rustup.rs"; exit 1; }

rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm install
npm run app:build:mac

echo ""
echo "Forge AI artifacts:"
find src-tauri/target/universal-apple-darwin/release/bundle -maxdepth 3 \( -name '*.dmg' -o -name '*.app' \) -print
