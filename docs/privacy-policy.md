# ChenChen Privacy Policy

**Last Updated**: 2025-11-14
**Version**: 0.1.0

## Overview

ChenChen is committed to protecting your privacy. This document explains how we handle your data, what we collect (nothing), and what we don't collect (everything else).

---

## Data Handling Principles

### 1. Local-Only Operation (FR-012)

**What This Means**:
- All application logic runs locally on your computer
- No data is sent to ChenChen servers (we don't have any)
- No telemetry, analytics, or crash reporting
- No cloud storage of any kind

**Technical Implementation**:
- Desktop application built with Tauri (Rust + Svelte)
- All state management in-memory only
- No persistent storage of user prompts or responses
- Session data managed exclusively by platform webview

### 2. Session Cookies Only (FR-012)

**What We Store**:
- ✓ Platform webview session cookies (managed by your OS)
- ✓ Provider authentication sessions (ChatGPT, Gemini, Claude)
- ✓ Webview data directories (isolated per provider)

**What We DON'T Store**:
- ✗ User credentials (username, password, API keys)
- ✗ Prompt history
- ✗ LLM responses
- ✗ Usage analytics
- ✗ Personal information

**Location of Session Data**:
- **Windows**: `%APPDATA%\com.chenchen.app\webviews\{provider_name}\`
- **Linux**: `~/.local/share/com.chenchen.app/webviews/{provider_name}/`
- **macOS**: Platform-managed data store identifiers (UUID-based)

This data is managed entirely by your operating system's webview component and contains only session cookies necessary for authenticated access to LLM providers.

### 3. Network Communication (FR-013)

**What We Connect To**:
- ✓ `chat.openai.com` - ChatGPT provider
- ✓ `gemini.google.com` - Gemini provider
- ✓ `claude.ai` - Claude provider

**What We DON'T Connect To**:
- ✗ Analytics services
- ✗ Telemetry endpoints
- ✗ Update servers
- ✗ Crash reporting services
- ✗ Third-party tracking domains

**Verification**:
You can verify this yourself using network monitoring tools like Wireshark or Charles Proxy. See [testing-guide.md](./testing-guide.md) for detailed instructions.

---

## Privacy Guarantees

### No Credential Storage

**Guarantee**: ChenChen never stores your LLM provider credentials.

**How It Works**:
- You log in directly to provider websites through isolated webviews
- Authentication is handled by the providers themselves
- Session cookies are managed by your OS's webview component
- We never see, store, or transmit your passwords or API keys

**Verification**:
- Automated test: `test_no_credential_storage_in_app_data_directory`
- Manual test: Search app data directory for credential patterns

### No Prompt History

**Guarantee**: Your prompts and responses are never persisted to disk.

**How It Works**:
- All submission tracking is in-memory only
- When you close ChenChen, all prompt data is cleared
- No SQLite databases, JSON files, or logs containing your prompts

**Verification**:
- Automated test: `test_no_prompt_history_retained_after_restart`
- Manual test: Search app data directory after sending prompts

### No Telemetry

**Guarantee**: ChenChen collects zero usage data.

**How It Works**:
- No analytics libraries included
- No network requests to non-provider domains
- No error reporting or crash diagnostics sent externally
- Completely offline-capable (after provider authentication)

**Verification**:
- Manual test: Monitor network traffic during usage
- Source code: No analytics dependencies in `Cargo.toml` or `package.json`

---

## Your Rights

### Data Access

Since we don't collect or store any data, there is no data to access, export, or delete.

### Data Deletion

To completely remove all ChenChen-related data:

1. **Uninstall the application**
2. **Delete webview data directories**:
   - Windows: Delete `%APPDATA%\com.chenchen.app\`
   - Linux: Delete `~/.local/share/com.chenchen.app/`
   - macOS: Webview data is automatically cleared on uninstall

### Third-Party Privacy Policies

While ChenChen doesn't collect data, the LLM providers you interact with have their own privacy policies:

- [OpenAI Privacy Policy](https://openai.com/policies/privacy-policy) (ChatGPT)
- [Google Privacy Policy](https://policies.google.com/privacy) (Gemini)
- [Anthropic Privacy Policy](https://www.anthropic.com/privacy) (Claude)

When you send prompts through ChenChen, you are subject to those providers' privacy policies.

---

## Technical Audits

### Open Source Verification

ChenChen is open source. You can:
- Review all source code on GitHub
- Audit network requests in the codebase
- Verify no credential storage code exists
- Build from source to ensure binary integrity

### Automated Privacy Tests

We maintain automated tests to verify privacy guarantees:

```bash
# Run privacy test suite
cargo test privacy_test

# Expected tests:
# ✓ test_no_credential_storage_in_app_data_directory
# ✓ test_no_prompt_history_retained_after_restart
# ✓ test_webview_data_only_contains_session_cookies
```

### Manual Verification

Follow our [testing guide](./testing-guide.md) to manually verify:
- Network traffic analysis (Wireshark/Charles)
- File system inspection (app data directory)
- No unexpected third-party connections

---

## Privacy by Design

### Architecture Decisions

1. **No Backend**: Pure desktop application, no server component
2. **No Database**: In-memory state only, no persistent storage
3. **Isolated Webviews**: Each provider runs in a separate webview
4. **No Dependencies**: Minimal external libraries, no analytics SDKs
5. **Local-First**: All processing happens on your machine

### Security Practices

- All network communication uses HTTPS
- Provider credentials never leave webview sandboxes
- No logging of sensitive data
- No crash dumps or error reports transmitted

---

## Changes to This Policy

We will update this privacy policy only if our data handling practices change. Current policy reflects:
- **No data collection**: Will remain true forever
- **Local-only operation**: Core design principle
- **Zero telemetry**: Fundamental commitment

Any changes will be documented in git history and new versions will be published with the application.

---

## Contact

For privacy-related questions or concerns:
- **GitHub Issues**: [https://github.com/your-org/chenchen/issues](https://github.com/your-org/chenchen/issues)
- **Email**: privacy@chenchen.app

---

## Compliance

### GDPR Compliance

ChenChen complies with GDPR by not collecting personal data. Since we don't process, store, or transmit user data, GDPR obligations do not apply.

### CCPA Compliance

ChenChen does not "sell" user data (we don't have any). No opt-out mechanisms are necessary because no data collection occurs.

### Children's Privacy

ChenChen does not knowingly collect information from children. Since we don't collect any information at all, we are compliant with COPPA and similar regulations.

---

## Verification Checklist

Use this checklist to verify ChenChen's privacy guarantees:

- [ ] Run automated privacy tests: `cargo test privacy_test`
- [ ] Monitor network traffic with Wireshark/Charles
- [ ] Inspect app data directory for credentials
- [ ] Search for prompt history files
- [ ] Review source code for telemetry libraries
- [ ] Verify HTTPS-only connections
- [ ] Confirm provider domain exclusivity

**Expected Result**: All checks pass, confirming zero data collection.
