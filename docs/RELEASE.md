# Forge AI release process

This document is the source of truth for publishing Forge AI installers.

## Release levels

- **Internal build:** unsigned artifact produced by CI for maintainers.
- **Release candidate:** versioned draft release used for macOS and Windows acceptance testing.
- **Public release:** signed, notarized, tested installers published from an approved release candidate.

## Required automated gates

A release commit must pass all required checks on `master`:

1. `npm ci`
2. `npm test`
3. `npm run build`
4. production dependency audit with no high or critical findings
5. `cargo fmt --check`
6. `cargo clippy -- -D warnings`
7. Rust tests
8. universal macOS application and DMG build
9. Windows NSIS and MSI build

The `Forge AI quality gate` and `Build Forge AI installers` workflows implement these checks. Branch protection should require both workflows before merge.

## Manual acceptance matrix

Run the following against the exact release-candidate artifacts, not a developer build.

### macOS

- Apple Silicon: clean install, upgrade, offline launch, uninstall and reinstall.
- Intel: clean install and first launch.
- Confirm API keys are stored in Keychain and absent from JSON files.
- Confirm the signed application passes `spctl --assess --type execute --verbose`.
- Confirm the DMG and application are notarized and stapled.

### Windows

- Windows 10 and Windows 11: clean NSIS and MSI installs.
- Test upgrade, uninstall, offline launch and retained application data.
- Confirm API keys are stored in Credential Manager and absent from JSON files.
- Confirm Authenticode signature verification succeeds and SmartScreen does not report an unknown publisher.

### Provider and local-mode testing

Using tester-owned credentials, test Ollama, OpenAI, Anthropic, Gemini and one OpenAI-compatible provider. Verify model discovery, connection testing, chat, context trimming, usage accounting, automatic fallback and restart persistence.

Launch without `VITE_FORGE_API_URL` and verify Local Mode remains fully functional. When an account service is configured, run registration, verification, login, reset, refresh rotation, device revocation, logout and account-deletion tests against staging before production.

## Signing configuration

Public releases require credentials that cannot be stored in the repository.

### Apple

Provide the Apple Developer ID certificate, certificate password, Apple ID, app-specific password and team ID as encrypted GitHub Actions secrets. The release workflow must sign, notarize and staple the generated application and DMG.

### Windows

Provide an Authenticode code-signing certificate through a secure signing service or encrypted CI secret. Timestamp every executable and MSI package.

Unsigned artifacts are acceptable only for internal testing and must not be described as public production releases.

## Release sequence

1. Merge only after required checks pass.
2. Create `vX.Y.Z-rc.N` and let the release workflow produce a draft release.
3. Complete and record the acceptance matrix for the generated artifacts.
4. Promote the tested commit to `vX.Y.Z`.
5. Verify checksums, signatures, release notes and downloadable assets.
6. Publish the GitHub release.

## Evidence

For each release, attach or link:

- workflow run URLs
- installer artifact names and checksums
- macOS and Windows test results
- provider/local-mode test results
- signing and notarization verification output
- known limitations

A release is not production-ready until this evidence exists for the exact published commit.
