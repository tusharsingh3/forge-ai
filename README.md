# Forge AI 1.0.0

Forge AI is a cross-platform desktop control center for local, remote, and cloud AI providers.

## Native desktop outputs

Forge AI is packaged as a native Tauri application:

- macOS universal `.app` and `.dmg` for Apple Silicon and Intel Macs
- Windows x64 NSIS setup `.exe` and MSI installer `.msi`

Users install Forge AI normally and launch it from **Applications**, **Launchpad**, the **Start Menu**, or a desktop shortcut. They do not need to run the Vite frontend manually.

## Fastest way to obtain installers

1. Open this repository's **Actions** tab.
2. Run **Build Forge AI installers**.
3. Download these workflow artifacts:
   - `Forge-AI-1.0.0-macOS`
   - `Forge-AI-1.0.0-Windows`

The workflow compiles on the native operating system because Tauri's Windows and macOS bundles must be produced on their corresponding build hosts.

## Build on macOS

Requirements:

- Node.js 22+
- Rust stable installed through rustup
- Xcode Command Line Tools

Run:

```bash
./scripts/build-macos.sh
```

Outputs are created under:

```text
src-tauri/target/universal-apple-darwin/release/bundle/macos/
src-tauri/target/universal-apple-darwin/release/bundle/dmg/
```

The universal application supports both Apple Silicon and Intel Macs.

## Build on Windows

Requirements:

- Node.js 22+
- Rust stable with the MSVC toolchain
- Microsoft C++ Build Tools
- WebView2 development/runtime dependencies

Run in PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\build-windows.ps1
```

Outputs are created under:

```text
src-tauri\target\release\bundle\nsis\
src-tauri\target\release\bundle\msi\
```

## Development

```bash
npm install
npm run app:dev
```

## Release build

Pushing the tag `v1.0.0` runs the **Release Forge AI** workflow and creates a draft GitHub release containing the macOS and Windows installers.

## Signing status

Unsigned applications can be used for internal testing, but macOS Gatekeeper and Windows SmartScreen may warn users. Public distribution should add:

- Apple Developer ID signing and notarization for macOS
- Authenticode code signing for Windows

Signing credentials must be stored as encrypted GitHub Actions secrets and must never be committed to the repository.

## Version policy

Forge AI remains at **1.0.0** until Tushar explicitly requests a version change.
