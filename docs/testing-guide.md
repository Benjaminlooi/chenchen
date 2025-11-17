# ChenChen Testing Guide

## Manual Privacy Testing

### Network Traffic Monitoring (T139)

**Objective**: Verify that ChenChen only contacts LLM provider websites and does not send data to any other servers.

**Requirements**:
- Wireshark, Charles Proxy, or similar network monitoring tool
- ChenChen application running
- Active internet connection

**Test Procedure**:

1. **Start Network Capture**:
   - Launch Wireshark or Charles Proxy
   - Start capturing network traffic for your network interface
   - Apply filter for HTTP/HTTPS traffic: `tcp.port == 443 || tcp.port == 80`

2. **Launch ChenChen**:
   ```bash
   npm run tauri dev
   ```

3. **Perform Test Actions**:
   - Select all three providers (ChatGPT, Gemini, Claude)
   - Enter a test prompt: "What is 2+2?"
   - Click "Send Prompt" button
   - Wait for submissions to complete

4. **Analyze Network Traffic**:

   **Expected Behavior** ✓:
   - Connections ONLY to these domains:
     - `chat.openai.com` (ChatGPT)
     - `gemini.google.com` (Gemini)
     - `claude.ai` (Claude)
   - All requests should be HTTPS (encrypted)
   - No DNS queries for tracking domains
   - No connections to analytics services

   **Failure Indicators** ✗:
   - Connections to unknown domains
   - HTTP requests to telemetry/analytics services
   - Unencrypted data transmission
   - Connections to update servers or crash reporting services

5. **Stop Capture and Review**:
   - Stop the network capture
   - Export captured packets for review
   - Document any unexpected network connections

**Pass Criteria**:
- ✓ All network traffic goes exclusively to LLM provider domains
- ✓ No telemetry or analytics requests detected
- ✓ No credentials transmitted outside of provider authentication flows
- ✓ All connections use HTTPS encryption

**Wireshark Filter Examples**:

```wireshark
# Show only HTTP/HTTPS traffic
http || ssl

# Exclude provider domains to see unexpected traffic
!(ip.dst_host == "chat.openai.com" || ip.dst_host == "gemini.google.com" || ip.dst_host == "claude.ai")

# Show DNS queries (to detect tracking domains)
dns
```

**Expected Results**:
- Network traffic limited to provider domains only
- No data leakage to third-party services
- No credential storage outside platform webview cookies

---

## Automated Test Execution

### Privacy Tests

```bash
# Run all privacy integration tests
cargo test privacy_test

# Expected: All tests PASS
# - test_no_credential_storage_in_app_data_directory
# - test_no_prompt_history_retained_after_restart
# - test_webview_data_only_contains_session_cookies
```

### All Tests

```bash
# Backend tests
cd src-tauri
cargo test

# Frontend tests
cd ..
npm test
```

---

## Test Coverage

### Privacy Guarantees (FR-012, FR-013)

| Test | Type | Coverage |
|------|------|----------|
| No credential storage | Integration | File system audit |
| No prompt history | Integration | Persistence check |
| Webview data isolation | Integration | Cookie-only verification |
| Network traffic monitoring | Manual | Third-party communication |

### Success Rate (SC-002)

```bash
# Run success rate test (T144a)
cargo test success_rate_test

# Expected: >=95% success rate over 20 attempts
```

---

## Troubleshooting

### Privacy Test Failures

**If credential storage test fails**:
- Check for leaked API keys or passwords in app data directory
- Audit all file I/O operations in codebase
- Verify no logging of sensitive data

**If prompt history test fails**:
- Check for SQLite databases or JSON files storing prompts
- Audit StatusTracker and Submission entities for persistence
- Verify in-memory only operation

**If network monitoring detects unexpected traffic**:
- Review dependency list for telemetry libraries
- Check for auto-update mechanisms
- Audit all HTTP client usage in codebase

---

## Continuous Verification

### Pre-Release Checklist

- [ ] Run all automated privacy tests
- [ ] Perform manual network traffic analysis
- [ ] Review source code for new file I/O operations
- [ ] Verify no new dependencies with telemetry
- [ ] Confirm privacy policy documentation is up-to-date
