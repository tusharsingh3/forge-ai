# TODO

This file tracks UI or backend pieces that exist but are not fully functional yet.

## Product behavior

- **Automatic fallback is not implemented**: the Chat side panel displays a fallback order, but requests do not automatically retry on the next provider when quota, network, or provider errors occur.
- **Conversation persistence is not implemented**: the current chat transcript is stored only in React state and resets when the app reloads or restarts.
- **Context management is basic**: provider switching sends the recent transcript with the next prompt, but there is no summarization, token budgeting, or per-project memory yet.
- **Settings screen is placeholder-only**: it explains the next hardening task, but does not expose configurable app settings.

## Connection management

- **Delete connection is not exposed in the UI**: the backend has `delete_connection`, but the Connections screen only supports add/edit.
- **Test connection is not exposed in the UI**: the backend has `test_connection`, but there is no Test button or status display on the Connections screen.
- **Enabled/disabled state is not editable**: connections have an `enabled` field, but the modal does not expose a toggle.
- **Token pricing fields are not editable**: connections support input/output cost per million tokens, but the modal does not expose those fields.

## Usage tracking

- **Clear usage history is not exposed in the UI**: the backend has `clear_usage_history`, but the Usage screen has no reset action.
- **Usage screen is summary-only**: recent requests are stored by the backend, but the UI does not yet show a request history table.

## Provider details

- **Gemini setup needs a clearer default base URL**: the UI asks for a base URL, but the app does not prefill or explain the expected Gemini API base URL.
- **Provider-specific model discovery is not implemented**: users must manually type model IDs instead of selecting from available models.
