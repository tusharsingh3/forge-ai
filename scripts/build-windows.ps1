$ErrorActionPreference = "Stop"

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
  throw "Node.js is required."
}
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  throw "Rust is required. Install it from https://rustup.rs"
}

npm install
npm run app:build:windows

Write-Host "`nForge AI artifacts:"
Get-ChildItem -Recurse src-tauri\target\release\bundle -Include *.exe,*.msi | Select-Object -ExpandProperty FullName
