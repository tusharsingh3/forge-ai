# Forge AI architecture

Forge AI 1.0.0 is a Tauri 2 desktop application with a React/TypeScript frontend and Rust native backend.

## Provider model

All execution targets are represented as connections:

- Local Ollama
- Remote Ollama / Linux server
- OpenAI
- Anthropic
- Google Gemini
- Generic OpenAI-compatible servers such as vLLM

The UI talks to one provider-neutral command layer. Provider adapters are responsible for authentication, request formatting, response parsing, token accounting, and capability reporting.

## Desktop targets

- macOS universal application and DMG
- Windows NSIS executable and MSI

## Security direction

API keys must be moved from application JSON storage to macOS Keychain and Windows Credential Manager before public distribution.

## Version policy

The application version remains 1.0.0 until Tushar explicitly requests a version bump.
