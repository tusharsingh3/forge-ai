# Forge AI

Forge AI is a cross-platform desktop control center for local, remote, and cloud AI providers. It lets users connect their own model endpoints, track token usage and cost, and continue a conversation while switching between providers such as OpenAI, Anthropic, Gemini, local Ollama, or a remote Ollama/Linux server.

## How the app works

Forge AI has three main responsibilities:

1. **Provider connections**: Users add one or more AI providers with a base URL, API key, default model, and optional token pricing.
2. **Unified chat**: The Playground tab keeps the conversation inside Forge AI. When a user switches providers, the app sends the recent transcript with the next prompt so the new model can continue with context.
3. **Usage tracking**: Each request records input tokens, output tokens, total tokens, estimated cost, latency, provider, model, and timestamp.

Consumer subscriptions such as ChatGPT Plus, Claude Pro, or Gemini Advanced are not the same as API access. To use cloud models in Forge AI, each user needs API access from the provider they want to use.

## Supported connection types

| Connection type | Use case | Base URL example |
| --- | --- | --- |
| Local Ollama | Run models on the same computer | `http://localhost:11434` |
| Remote Ollama / Linux server | Run models on a rented server | `http://your-server-ip:11434` |
| OpenAI API | Use OpenAI models with an API key | `https://api.openai.com/v1` |
| Anthropic API | Use Claude models with an API key | `https://api.anthropic.com` |
| Google Gemini API | Use Gemini models with an API key | Gemini API endpoint configured by the app/provider adapter |
| OpenAI-compatible API | Use providers that expose OpenAI-style APIs | Provider-specific URL |

## Using Forge AI with API keys

1. Install and open Forge AI.
2. Go to **Connections**.
3. Click **Add**.
4. Select the provider type.
5. Enter:
   - **Name**: A friendly label, for example `OpenAI GPT-5` or `Local Qwen`.
   - **Base URL**: The provider endpoint.
   - **API key**: The key generated from the provider console.
   - **Default model**: The model ID to use by default.
   - **Input/output cost per million tokens**: Optional, used for cost estimates.
6. Save the connection.
7. Open **Playground**.
8. Select the provider/model and start chatting.

The Playground keeps the current conversation in the app. If one provider reaches quota, becomes unavailable, or is too expensive, select another provider in the model control panel and continue from the same chat.

## Generating API keys

Keep API keys private. Do not commit them to git, paste them into public issues, or share them in screenshots.

### OpenAI

1. Sign in to the OpenAI Platform.
2. Open the API keys page: `https://platform.openai.com/settings/organization/api-keys`
3. Click **Create new secret key**.
4. Copy the key immediately and store it securely.
5. Add it to Forge AI using the **OpenAI API** connection type.

Recommended base URL:

```text
https://api.openai.com/v1
```

Example model IDs depend on the models enabled for your account.

### Anthropic Claude

1. Sign in to the Anthropic Console: `https://platform.claude.com/`
2. Open **Settings** or **API Keys**.
3. Create a new API key.
4. Copy the key immediately and store it securely.
5. Add it to Forge AI using the **Anthropic API** connection type.

Recommended base URL:

```text
https://api.anthropic.com
```

Example model IDs depend on the Claude models enabled for your account.

### Google Gemini

1. Open Google AI Studio API keys: `https://aistudio.google.com/api-keys`
2. Sign in with your Google account.
3. Click **Create API key**.
4. Copy the generated key and store it securely.
5. Add it to Forge AI using the **Google Gemini API** connection type.

Model IDs depend on the Gemini models available for your account and region.

### OpenAI-compatible providers

Some providers expose OpenAI-compatible APIs. Use the **OpenAI-compatible API** connection type when a provider documents a `/v1/chat/completions` style API.

You usually need:

- Provider base URL
- API key
- Model ID
- Optional input/output pricing

Check the provider's documentation for exact model IDs and endpoint URLs.

## Local models with Ollama

Install Ollama, pull a model, and keep Ollama running.

```bash
ollama pull qwen2.5:7b
ollama serve
```

Then add a Forge AI connection:

```text
Type: Local Ollama
Base URL: http://localhost:11434
Default model: qwen2.5:7b
```

Local models are useful for private work, lower cost, and code-editing workflows where response quality is acceptable for the task.

## Remote Linux server models

Forge AI can also connect to Ollama running on a remote Linux server.

Typical setup:

1. Rent or prepare a Linux server.
2. Install Ollama on the server.
3. Pull the required model.
4. Expose Ollama safely through a private network, VPN, reverse proxy, or firewall-restricted port.
5. Add the server URL in Forge AI as a **Remote Ollama / Linux server** connection.

Avoid exposing an unauthenticated Ollama endpoint publicly.

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

## Development

Requirements:

- Node.js 22+
- Rust stable installed through rustup
- Tauri system dependencies for your operating system

Run:

```bash
npm install
npm run app:dev
```

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

## Release build

Pushing the tag `v1.0.0` runs the **Release Forge AI** workflow and creates a draft GitHub release containing the macOS and Windows installers.

## Signing status

Unsigned applications can be used for internal testing, but macOS Gatekeeper and Windows SmartScreen may warn users. Public distribution should add:

- Apple Developer ID signing and notarization for macOS
- Authenticode code signing for Windows

Signing credentials must be stored as encrypted GitHub Actions secrets and must never be committed to the repository.
