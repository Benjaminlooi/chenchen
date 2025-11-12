# Research: Multi-LLM Prompt Desktop App Technical Decisions

**Branch**: `001-multi-llm-prompt` | **Date**: 2025-11-13
**Phase**: 0 (Outline & Research)

## Overview

This document captures all technical research and decisions made during planning for the multi-LLM prompt desktop application. All "NEEDS CLARIFICATION" items from Technical Context have been resolved.

---

## 1. Frontend Framework Selection

### Decision: **Svelte**

### Rationale

**Bundle Size (Critical Constraint):**
- Svelte 5: 3-10KB framework overhead
- React: 40-100KB minimum baseline
- Vue: 244KB recommended maximum
- With <15MB binary constraint, Svelte leaves 99%+ budget for application logic

**Performance:**
- Near-instant startup (critical for <3s startup goal)
- 30% faster load times vs React
- 50% lower memory usage vs React
- Compile-time optimization (no virtual DOM runtime overhead)
- More resources available for 1-3 simultaneous webview rendering

**Tauri 2.0 Integration:**
- Official template via `create-tauri-app`
- SvelteKit 2.20.4+ has official Tauri integration documentation (September 2025)
- Strong community momentum around "Tauri + Rust + Svelte" stack

**Testing Maturity:**
- Vitest recommended by both Tauri and Svelte communities
- `@testing-library/svelte` for component tests
- `@tauri-apps/api/mocks` works seamlessly
- Fast test execution supports TDD workflow

**Developer Experience for This Project:**
- Reactive `$:` statements perfect for real-time status updates
- `bind:group` for provider checkboxes cleaner than React controlled components
- Dynamic split-screen layout calculation straightforward with reactive bindings
- CSS scoping simplifies responsive grid/split layouts

### Alternatives Considered

**React** ❌
- Rejected: 40-100KB baseline violates binary size efficiency, virtual DOM overhead unnecessary, higher memory usage impacts multi-webview scenario

**Vue** ❌
- Rejected: Larger bundle than Svelte with no compelling advantages, slower startup than Svelte

**Vanilla JS** ❌
- Rejected: No component abstraction makes managing 4+ components verbose, manual DOM manipulation for status updates, harder testing, complex responsive grid logic

### Implementation

Use standard Svelte (not SvelteKit) since server-side rendering is unnecessary for desktop app.

**Recommended Stack:**
- Tauri 2.0 (Rust backend)
- Svelte 5 (frontend framework)
- Vite (build tool)
- TypeScript (type safety for Tauri commands)
- Vitest (testing)
- @testing-library/svelte (component tests)

**Expected Bundle:**
- Framework: 3-10KB
- UI components: 5-15KB
- Tauri API bindings: 2-5KB
- Total frontend: 10-30KB
- Final binary estimate: 4-6MB (well under 15MB constraint)

---

## 2. Testing Strategy

### Rust Backend Testing

**Primary Framework:**
- `cargo test` (built-in)
- `tauri::test` module for `mock_builder()`, `mock_app()`, `assert_ipc_response()`

**Additional Libraries:**
- **mockall**: Trait mocking with `#[automock]` attribute, expectation chaining
- **pretty_assertions**: Colorful diffs for `assert_eq!`
- **schemars**: JSON schema generation for contract tests

**Best Practices:**
- Enable test feature: `tauri = { version = "2.x", features = ["test"] }`
- Tests alongside implementation with `#[cfg(test)]` modules
- Integration tests in `/tests` directory
- Separate testable logic from Tauri commands for easier unit testing

### Frontend Testing

**Primary Framework:**
- **Vitest**: Native ESM support, blazing-fast performance, Vite integration, official Tauri examples

**Component Testing:**
- **@testing-library/svelte**: User-centric component testing

**E2E Testing:**
- **Playwright**: Cross-browser, auto-wait, network mocking (recommended for critical paths)

### Integration Testing: Tauri IPC

**Setup:**
```typescript
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks";
import { invoke } from "@tauri-apps/api/core";

afterEach(() => {
  clearMocks(); // CRITICAL between tests
});
```

**Mocking Commands:**
```typescript
mockIPC((cmd, args) => {
  if (cmd === "submit_prompt") {
    return { status: "success", provider: args.provider };
  }
});
```

**Tracking Calls:**
```typescript
const spy = vi.spyOn(window.__TAURI_INTERNALS__, "invoke");
await invoke("submit_prompt", { provider: "chatgpt", prompt: "test" });
expect(spy).toHaveBeenCalledWith("submit_prompt", ...);
```

### Webview JavaScript Injection Testing

**Approach: Mock Runtime (Unit/Integration Tests)**

**Strategy:** Test injection logic WITHOUT real browser/webview

1. **Unit Test Script Generation (Rust):**
   ```rust
   #[test]
   fn test_injection_script_generation() {
       let config = ProviderConfig { /* ... */ };
       let script = generate_injection_script(&config);
       assert!(script.contains("function submitPrompt()"));
   }
   ```

2. **Mock Execution (Frontend):**
   ```typescript
   test("injected script behavior", () => {
       const injectedCode = `window.submitPrompt = function(text) { return "submitted"; };`;
       eval(injectedCode);
       expect(window.submitPrompt("test")).toBe("submitted");
   });
   ```

3. **E2E with WebDriver (Real Webview):**
   ```typescript
   it("should execute injected functions", async () => {
       const result = await browser.execute(() => {
           return window.submitPrompt("test prompt");
       });
       expect(result).toBe("submitted");
   });
   ```

**Best Practice:**
- Unit tests: Script generation logic (Rust)
- Integration tests: Script behavior in jsdom (frontend)
- Contract tests: Validate script interface
- E2E tests: Verify actual injection in real webview (critical paths only)

### Contract Testing

**Rust Side:**
```rust
use schemars::{JsonSchema, schema_for};

#[derive(Serialize, Deserialize, JsonSchema)]
struct ProviderConfig {
    id: String,
    enabled: bool,
}

#[test]
fn test_command_output_contract() {
    let output = StatusResponse { status: "ready".to_string() };
    let json = serde_json::to_string(&output).unwrap();
    assert!(json.contains("\"status\""));
}
```

**TypeScript Side:**
```typescript
import { z } from "zod";

const ProviderConfigSchema = z.object({
  id: z.string(),
  enabled: z.boolean(),
});

test("command response matches contract", async () => {
  const result = await invoke("get_config");
  expect(() => ProviderConfigSchema.parse(result)).not.toThrow();
});
```

### Test Pyramid

```
        /\
       /  \  E2E (Playwright/WebDriver) - Critical paths only
      /    \
     /------\ Integration Tests
    /        \ - Tauri IPC mocking
   /          \ - Contract tests (schemars + zod)
  /            \
 /--------------\
   Unit Tests
   - Rust: cargo test + mockall
   - Frontend: Vitest + mockIPC
```

### TDD Workflow

**Backend (Rust):**
1. Write failing unit test for command logic
2. Implement command handler
3. Refactor with passing tests
4. Add integration test using `tauri::test`

**Frontend:**
1. Write test with mocked IPC using `mockIPC()`
2. Implement UI logic calling `invoke()`
3. Verify mock interactions with spies
4. Clear mocks with `clearMocks()`

**Integration:**
1. Define contract (JSON Schema + Zod)
2. Write contract tests (both sides)
3. Test IPC flow with mock runtime
4. Add E2E test with WebDriver for critical paths

---

## 3. Persistent Webview Sessions

### Decision: Use `data_directory` (Windows/Linux) and `data_store_identifier` (macOS) for isolated, persistent sessions per provider

### Rationale

**Platform-native session management:**
- **Windows (WebView2)**: Stores cookies in SQLite in user data folder
- **macOS (WKWebKit)**: Stores cookies in `/Users/<user>/Library/WebKit/WebsiteDataStore/<UUID>`
- **Linux (WebKitGTK)**: Stores cookies as plain text in data directory

**No credential storage required:** Webview's native cookie manager handles all persistence. App only configures where webview stores session data (FR-012).

**Survives app restarts:** Cookies persist to disk automatically and reload when webview recreates with same data directory/identifier (FR-011).

**Session isolation per provider:** Separate data directories/identifiers create completely isolated cookie jars for ChatGPT, Gemini, Claude.

### Platform Considerations

**Windows (WebView2):**
- Storage location: User data folder with SQLite cookies
- Isolation: Different user data folders = separate sessions
- Path resolution: `app.path().app_local_data_dir()` + provider subdirectories

**macOS (WKWebKit):**
- **Key difference:** `data_directory` unsupported - MUST use `data_store_identifier`
- **Requires macOS 14+:** dataStoreIdentifier API needs recent macOS/iOS versions
- Storage location: Automatically managed at `/Users/<user>/Library/WebKit/WebsiteDataStore/<UUID>`
- UUID persistence: Generate and store UUID per provider in app config, reuse same UUID to maintain sessions
- Non-persistent fallback: Before macOS 14, sessions lost on restart

**Linux (WebKitGTK):**
- Storage format: Cookies as plain text files
- Explicit persistence: Must call `webkit_cookie_manager_set_persistent_storage()` (Tauri handles this)
- Path resolution: Similar to Windows

### Implementation Pattern

```rust
use tauri::{WebviewWindowBuilder, WebviewUrl, Manager};

#[cfg(not(target_os = "macos"))]
fn create_provider_window(
    app: &tauri::AppHandle,
    provider: &str,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = app.path()
        .app_local_data_dir()?
        .join("webviews")
        .join(provider);

    std::fs::create_dir_all(&data_dir)?;

    WebviewWindowBuilder::new(app, provider, WebviewUrl::External(url.parse()?))
        .data_directory(data_dir)
        .title(format!("{} - ChenChen", provider))
        .build()?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn create_provider_window(
    app: &tauri::AppHandle,
    provider: &str,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider_uuid = get_or_create_provider_uuid(provider)?;

    WebviewWindowBuilder::new(app, provider, WebviewUrl::External(url.parse()?))
        .data_store_identifier(provider_uuid)
        .title(format!("{} - ChenChen", provider))
        .build()?;

    Ok(())
}
```

### Directory Structure

```
# Windows
C:\Users\<user>\AppData\Local\com.chenchen.app\webviews\
├── chatgpt/     # WebView2 user data folder
├── gemini/      # Separate WebView2 instance
└── claude/      # Third isolated instance

# macOS (managed by WebKit)
/Users/<user>/Library/WebKit/WebsiteDataStore/
├── <uuid-for-chatgpt>/
├── <uuid-for-gemini>/
└── <uuid-for-claude>/

# Linux
/home/<user>/.local/share/com.chenchen.app/webviews/
├── chatgpt/
├── gemini/
└── claude/
```

### Configuration

**Tauri Config (`tauri.conf.json`):**
```json
{
  "app": {
    "withGlobalTauri": true,
    "security": {
      "dangerousUseHttpScheme": false  // Keep using https://tauri.localhost
    }
  },
  "bundle": {
    "identifier": "com.chenchen.app"
  }
}
```

**Provider URLs:**
- ChatGPT: `https://chat.openai.com/`
- Gemini: `https://gemini.google.com/`
- Claude: `https://claude.ai/`

### Key Implementation Notes

1. **Session detection (FR-019):** Check if login required by attempting navigation or checking for auth-required DOM elements after page load

2. **First-time setup:** Create webview with persistent storage, display provider's login page, cookies persist automatically after successful login

3. **macOS compatibility:** Consider minimum OS version requirement (macOS 14+) or implement fallback with warning about session loss on restart

4. **Testing:** Close app, verify data directories contain cookies/storage files, reopen app, verify login state maintained

---

## Summary of Resolved Clarifications

| Technical Context Item | Resolution |
|------------------------|------------|
| Frontend framework | Svelte 5 with Vite and TypeScript |
| Frontend test framework | Vitest + @testing-library/svelte |
| Persistent session approach | `data_directory` (Windows/Linux), `data_store_identifier` (macOS) with platform-specific cookie storage |
| Session isolation strategy | Separate data directories/UUIDs per provider (ChatGPT, Gemini, Claude) |
| Testing webview injection | Mock runtime for unit/integration tests, WebDriver for E2E critical paths |

All architectural decisions align with constitutional principles:
- **Library-First:** Five independent libraries identified (webview, injection, providers, layout, status)
- **Test-First:** TDD workflow defined with cargo test, Vitest, mockIPC, contract tests
- **Simplicity:** Direct JavaScript injection, no complex automation frameworks, maximum 3 providers
- **Observability:** Structured logging for injection attempts, text-based status events, JSON provider configs
- **Versioning:** App semver starting 0.1.0, provider config schema independently versioned
