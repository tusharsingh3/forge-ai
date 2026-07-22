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

## Persistence

- Connections (without secrets), conversations, settings, and the last 1,000 usage records are stored in the platform application-data directory.
- API keys are stored through the operating system credential service (macOS Keychain or Windows Credential Manager). Legacy keys are migrated out of JSON when a connection is saved.
- Conversations retain messages and provider/model selection across restarts. Context sent to providers is bounded by user-configurable token and message limits.

## Desktop targets

- macOS universal application and DMG
- Windows NSIS executable and MSI

## Security direction

API keys use macOS Keychain and Windows Credential Manager through the Rust `keyring` adapter. Returned connection objects expose only whether a key exists, never the key itself.

## Optional Forge accounts

Forge AI starts in Local Mode and does not require registration. The desktop Account client is activated only when `VITE_FORGE_API_URL` is configured. It supports email/password, Google and GitHub device authorization, password reset, email verification, device revocation, logout-all, and account deletion through the versioned contract in [ACCOUNT_API.md](ACCOUNT_API.md).

Access and rotating refresh tokens are stored as one native keyring entry; they are not written to application JSON. A stable random installation identifier is stored locally and sent only to account endpoints for device registration. Local conversations and usage remain owned by the installation. Signing in does not upload or relabel historical data.

Forge account identity, provider connection identity, and provider/model usage identity remain independent:

| Scope | Key | Storage |
| --- | --- | --- |
| Forge account | Forge user and registered device | Account service; session in native keyring |
| Provider connection | Local connection ID | Local JSON; secret in native keyring |
| Provider/model usage | Connection ID + model ID | Local usage history |

Account-service failure, expiration, revocation, or offline startup must never block Local Mode or remove local application data.

The user-facing data boundary is documented in [PRIVACY.md](PRIVACY.md).

## Version policy

The application version remains 1.0.0 until Tushar explicitly requests a version bump.


## Provider-aware usage identity

Usage aggregation uses the composite identity `connection_id + model`. The provider connection is intentionally part of the key because two services can expose the same upstream model name while maintaining independent credentials, pricing, limits, and billing.

The Rust backend returns both connection totals and `by_model` totals. Each model row includes its provider display name and a nullable `remaining_tokens` field. The frontend never infers subscription quota: `null` is rendered as unavailable. The active conversation's remaining context budget is calculated locally from the configured context token budget.
