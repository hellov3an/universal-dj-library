# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| Latest release | ✅ |
| Previous minor | ⚠️ Critical fixes only |
| Older versions | ❌ |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them through [GitHub Security Advisories](https://github.com/user/universal-dj-library/security/advisories/new).

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **Acknowledgement**: Within 48 hours
- **Assessment**: Within 7 days
- **Fix (critical)**: Within 30 days
- **Fix (non-critical)**: Next release cycle

### Process

1. You report the vulnerability privately via GitHub Security Advisories
2. We acknowledge receipt and begin investigation
3. We develop a fix in a private branch
4. We release the fix and publish a security advisory
5. We credit you in the advisory (unless you prefer anonymity)

## Security Design Principles

Universal DJ Library follows these security principles:

### Local-First

- All data is stored locally by default
- No mandatory cloud services
- No mandatory internet connection
- No mandatory user accounts

### Minimal Permissions

- Tauri capability-based permission system
- Frontend cannot access arbitrary filesystem paths
- All filesystem access requires user interaction (native dialogs)
- IPC inputs are validated in Rust before processing

### No Data Collection

- No telemetry by default
- Optional telemetry is opt-in, documented, and open source
- No analytics
- No tracking

### Dependency Security

- Regular dependency audits via `cargo audit` and `npm audit`
- Dependabot enabled for automated security updates
- CI checks for known vulnerabilities

## Security Features

### File Integrity

- SHA-256 verification for all file transfers
- Database integrity checks on startup
- Backup system for data recovery

### Code Signing

- macOS: Developer ID + notarization (when available)
- Windows: Authenticode signing (when available)
- All release checksums published with each release

## Contact

For security concerns, use [GitHub Security Advisories](https://github.com/user/universal-dj-library/security/advisories/new).
