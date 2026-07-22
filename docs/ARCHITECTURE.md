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

The UI talks to one provider-neutral command layer. Provider adapters are responsible for authentication, model discovery, request formatting, response parsing, token accounting, and capability reporting. The command layer applies a configurable context token/message budget and retries enabled providers in the configured fallback order.

Usage identity is the composite key `(connection_id, model)`. This keeps identically named models from different providers separate. Optional token allowances are stored in settings under the same composite identity; remaining tokens are derived locally as `allowance - recorded usage` and are never presented as provider-reported quota.

## Persistence

- Connections (without secrets), conversations, settings (including per-model token allowances), and the last 1,000 usage records are stored in the platform application-data directory.
- API keys are stored through the operating system credential service (macOS Keychain or Windows Credential Manager). Legacy keys are migrated out of JSON when a connection is saved.
- Conversations retain messages and provider/model selection across restarts. Context sent to providers is bounded by user-configurable token and message limits.

## Desktop targets

- macOS universal application and DMG
- Windows NSIS executable and MSI

## Security direction

API keys use macOS Keychain and Windows Credential Manager through the Rust `keyring` adapter. Returned connection objects expose only whether a key exists, never the key itself.

## Version policy

The application version remains 1.0.0 until Tushar explicitly requests a version bump.
