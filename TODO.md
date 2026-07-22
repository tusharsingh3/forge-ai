# Test checklist

All previously tracked implementation items are complete. Before release, validate packaged builds with your own credentials/endpoints:

- [ ] macOS: Keychain save/update/delete and app restart persistence
- [ ] Windows: Credential Manager save/update/delete and app restart persistence
- [ ] Ollama: local and remote model discovery, test, chat, fallback
- [ ] OpenAI and OpenAI-compatible: discovery, test, chat, usage metadata
- [ ] Anthropic: discovery/test/chat with an enabled API account
- [ ] Gemini: default URL, discovery/test/chat with an enabled API key
- [ ] Automatic fallback using one intentionally unavailable primary connection
- [ ] Dark, light, and system themes
- [ ] Conversation restore, new chat, usage table, and clear history
