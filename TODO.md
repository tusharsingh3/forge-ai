# Pre-deployment verification checklist

All previously tracked product TODOs are implemented. Use this checklist on both macOS and Windows before public deployment.

## Functional testing

- [ ] Add, edit, enable/disable, test, discover models for, and delete each connection type.
- [ ] Confirm API keys appear in macOS Keychain or Windows Credential Manager and never in `connections.json`.
- [ ] Confirm legacy JSON API keys migrate to credential storage after the connection is next saved.
- [ ] Send requests through Ollama, OpenAI, Anthropic, Gemini, and an OpenAI-compatible endpoint using tester-owned credentials.
- [ ] Stop or invalidate the active provider and confirm automatic fallback uses the configured order.
- [ ] Restart Forge AI and confirm conversations, settings, usage, and the selected theme persist.
- [ ] Exceed the configured context limits and confirm older messages are trimmed while the newest context is retained.
- [ ] Verify light, dark, and system themes on both operating systems.
- [ ] Verify the usage request table and Clear history confirmation.

## Packaging testing

- [ ] Build and install the universal macOS DMG on Apple Silicon and Intel hardware.
- [ ] Build and install both Windows NSIS and MSI packages on Windows 10 and Windows 11.
- [ ] Verify credential prompts, upgrades, uninstall behavior, offline launch, and application data retention.
- [ ] Add Apple notarization and Windows Authenticode signing before public distribution.
