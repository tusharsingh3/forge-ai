# Forge account API contract

The desktop app keeps account access optional and calls a separately deployed Forge account service only from the Account screen. Omitting `VITE_FORGE_API_URL` disables account actions without affecting Local Mode.

## Authentication endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| `POST` | `/v1/auth/register` | Create an account, send verification email, register the device, and return a session |
| `POST` | `/v1/auth/login` | Authenticate email/password and return a session |
| `POST` | `/v1/auth/refresh` | Rotate the refresh token and return a replacement session |
| `POST` | `/v1/auth/password/forgot` | Send a non-enumerating reset response |
| `POST` | `/v1/auth/email/resend` | Resend verification for the authenticated user |
| `POST` | `/v1/auth/oauth/{provider}/start` | Begin Google or GitHub device authorization |
| `POST` | `/v1/auth/oauth/poll` | Exchange an approved device code for a session |
| `POST` | `/v1/auth/logout` | Revoke the current refresh-token family |
| `POST` | `/v1/auth/logout-all` | Revoke every session for the account |

`register` and `login` receive `device_id`. Successful auth and refresh responses use this shape:

```json
{
  "session": {
    "access_token": "short-lived-token",
    "refresh_token": "rotating-token",
    "expires_at": 1784712300000,
    "user": {
      "id": "user-id",
      "email": "user@example.com",
      "display_name": "User",
      "email_verified": true
    }
  }
}
```

`expires_at` is Unix time in milliseconds. Error responses return `{ "message": "..." }`. While OAuth approval is pending, the poll endpoint returns a non-success response whose message contains `authorization_pending`.

## Account endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/v1/account/devices` | List current and other registered devices |
| `DELETE` | `/v1/account/devices/{id}` | Revoke one device |
| `DELETE` | `/v1/account` | Permanently delete the account and server-side account data |

All account endpoints use a Bearer access token. Device rows contain `id`, `name`, `platform`, `last_seen_at`, and `current`.

## Required server controls

- Hash passwords with Argon2id or delegate password handling to a mature identity provider.
- Use short-lived access tokens and single-use rotating refresh tokens with reuse detection.
- Rate-limit login, registration, reset, resend, OAuth polling, and refresh endpoints.
- Return the same forgot-password response whether or not the email exists.
- Require a recent authentication challenge before account deletion.
- Store hashed refresh tokens, device metadata, and append-only security audit events.
- Restrict CORS to packaged Forge AI origins and approved development origins.
- Never accept, request, or synchronize provider API keys or local chat content in this API.
