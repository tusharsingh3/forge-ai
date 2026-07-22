# Account and local-data privacy

Forge AI is local-first. Registration is optional and Local Mode remains available when the account service is offline, unconfigured, or the user signs out.

| Data | Default location | Sent to Forge account service? |
| --- | --- | --- |
| Provider API keys | macOS Keychain or Windows Credential Manager | No |
| Forge access and refresh tokens | macOS Keychain or Windows Credential Manager | Only tokens are presented to authenticated account endpoints |
| Conversations | Local application-data directory | No |
| Usage and cost history | Local application-data directory | No |
| Application settings and connections | Local application-data directory | No |
| Forge profile and verification status | Forge account service | Yes, only after registration/sign-in |
| Installation ID and device metadata | Local application data; account service after sign-in | Only for account device/session management |

Signing out, revoking a session, or deleting a Forge account does not delete local chats, provider connections, credentials, settings, or usage. Account deletion removes server-side Forge account data according to the deployed service's retention policy. The production retention period and support contact must be published before account-enabled builds are distributed.

Future synchronization must be separately enabled, documented, and end-to-end encrypted before any local conversation, setting, usage record, or provider credential can leave the device.
