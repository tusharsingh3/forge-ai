# Pre-deployment verification checklist

All product implementation TODOs are complete. Public release is gated by the automated workflows and the platform acceptance evidence described in [docs/RELEASE.md](docs/RELEASE.md).

## Automated gates

- [x] Run deterministic dependency installation with `npm ci`.
- [x] Run frontend tests and the production TypeScript/Vite build on every pull request.
- [x] Reject high or critical production dependency vulnerabilities.
- [x] Run Rust formatting, Clippy with warnings denied, and Rust tests.
- [x] Build universal macOS app/DMG and Windows NSIS/MSI packages in native GitHub runners.
- [x] Upload installer artifacts and fail when expected packages are missing.

## Functional release-candidate testing

These checks require the exact generated installer artifacts and tester-owned credentials; record evidence for each release candidate rather than marking them complete globally.

- [ ] Add, edit, enable/disable, test, discover models for, and delete each connection type.
- [ ] Confirm API keys appear in macOS Keychain or Windows Credential Manager and never in `connections.json`.
- [ ] Confirm legacy JSON API keys migrate to credential storage after the connection is next saved.
- [ ] Send requests through Ollama, OpenAI, Anthropic, Gemini, and an OpenAI-compatible endpoint using tester-owned credentials.
- [ ] Stop or invalidate the active provider and confirm automatic fallback uses the configured order.
- [ ] Restart Forge AI and confirm conversations, settings, usage, and the selected theme persist.
- [ ] Exceed configured context limits and confirm older messages are trimmed while newest context is retained.
- [ ] Verify light, dark, and system themes on macOS and Windows.
- [ ] Verify the usage request table and Clear history confirmation.
- [ ] Launch without `VITE_FORGE_API_URL`; confirm Local Mode remains functional while account actions are disabled.
- [ ] Against staging, test email registration, verification, login, reset, rotating refresh, restart persistence, Google/GitHub authorization, and non-enumerating errors.
- [ ] Revoke another device, logout current/all devices, and delete an account; confirm local chats, usage, settings, and provider credentials remain untouched.
- [ ] Start offline and with expired/revoked sessions; confirm Local Mode remains available and no local data is erased.

## Packaging and trust

- [ ] Install the universal macOS DMG on Apple Silicon and Intel hardware.
- [ ] Install both Windows NSIS and MSI packages on Windows 10 and Windows 11.
- [ ] Verify credential prompts, upgrades, uninstall behavior, offline launch, and application-data retention.
- [ ] Configure Apple Developer ID signing, notarization, and stapling through encrypted CI secrets.
- [ ] Configure Windows Authenticode signing and timestamping through encrypted CI secrets or a secure signing service.
- [ ] Deploy the account API contract and production OAuth/email/rate-limit/retention configuration after security review.

## Provider usage limitations

- [x] Track token usage independently by provider connection and model.
- [x] Show provider-qualified model labels in Chat and Usage.
- [x] Show the active conversation's calculated remaining context budget.
- [ ] Add provider-specific quota adapters only when cloud APIs expose authoritative allowance endpoints. Until then, display remaining allowance as unavailable.

> The unchecked items cannot be completed safely inside source control because they require external hardware, credentials, certificates, production infrastructure, or live-provider accounts. They are release evidence requirements, not missing application code.
