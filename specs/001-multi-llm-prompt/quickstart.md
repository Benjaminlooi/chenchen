# Quickstart Guide: Multi-LLM Prompt Desktop App

**Branch**: `001-multi-llm-prompt` | **Date**: 2025-11-13
**Phase**: 1 (Design & Contracts)

## Overview

This guide provides a quickstart for developers implementing the multi-LLM prompt desktop app using Tauri 2.0. Follow these steps to set up your development environment, understand the architecture, and begin TDD implementation.

---

## Prerequisites

### System Requirements

**Operating System:**
- Windows 10+ (for WebView2 testing)
- macOS 10.15+ (macOS 14+ recommended for persistent sessions)
- Linux (with WebKitGTK)

**Required Tools:**
- **Rust**: Latest stable (1.75+)
- **Node.js**: 18+ (for Svelte frontend)
- **npm**: 9+
- **Tauri CLI**: 2.x

### Check Prerequisites

Run the prerequisite check script:
```bash
.specify/scripts/bash/check-prerequisites.sh
```

This verifies:
- Rust toolchain installed (`cargo --version`)
- Node.js and npm versions
- Platform-specific webview dependencies (WebView2 Runtime on Windows, WebKitGTK on Linux)

---

## Project Initialization

### 1. Create Tauri App with Svelte Template

```bash
npm create tauri-app@latest

# Prompts:
# Project name: chenchen
# Choose template: Svelte
# Use TypeScript: Yes
# Package manager: npm
```

This creates:
```
chenchen/
├── src/              # Svelte frontend
├── src-tauri/        # Rust backend
├── package.json
└── ...
```

### 2. Install Dependencies

```bash
cd chenchen

# Frontend dependencies
npm install

# Backend dependencies
cd src-tauri
cargo build
```

### 3. Verify Setup

```bash
# Run dev server (hot reload enabled)
npm run tauri dev
```

You should see a Tauri window open with the default Svelte template.

---

## Architecture Overview

### Component Structure

```
┌─────────────────────────────────────────────┐
│          Svelte Frontend (src/)             │
│  ┌──────────────────────────────────────┐   │
│  │ Components:                          │   │
│  │ - PromptInput.svelte                 │   │
│  │ - ProviderSelector.svelte            │   │
│  │ - StatusDisplay.svelte               │   │
│  │ - ProviderPanel.svelte               │   │
│  └──────────────────────────────────────┘   │
│  ┌──────────────────────────────────────┐   │
│  │ Services:                            │   │
│  │ - tauri.ts (command invocation)      │   │
│  │ - state.ts (reactive state)          │   │
│  └──────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
                     ↕ IPC (Tauri Commands)
┌─────────────────────────────────────────────┐
│       Rust Backend (src-tauri/src/)         │
│  ┌──────────────────────────────────────┐   │
│  │ Libraries:                           │   │
│  │ - webview/    (session management)   │   │
│  │ - injection/  (JS injection)         │   │
│  │ - providers/  (selector configs)     │   │
│  │ - layout/     (split-screen calc)    │   │
│  │ - status/     (submission tracking)  │   │
│  └──────────────────────────────────────┘   │
│  ┌──────────────────────────────────────┐   │
│  │ commands.rs (Tauri IPC interface)    │   │
│  └──────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
                     ↕
┌─────────────────────────────────────────────┐
│    Platform Webviews (Provider Sessions)    │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ ChatGPT  │  │  Gemini  │  │  Claude  │  │
│  │ WebView  │  │ WebView  │  │ WebView  │  │
│  └──────────┘  └──────────┘  └──────────┘  │
└─────────────────────────────────────────────┘
```

### Key Architectural Principles (Constitution Alignment)

1. **Library-First**: Each Rust module (`webview/`, `injection/`, etc.) is an independent library with clear boundaries
2. **Test-First**: All libraries have tests written BEFORE implementation (Red-Green-Refactor)
3. **Simplicity**: No complex frameworks (direct JS injection, simple CSS selectors, maximum 3 providers)
4. **Observability**: Structured logging, text-based configs (JSON), status events via IPC
5. **Versioning**: App semver (starting 0.1.0), provider configs independently versioned

---

## TDD Workflow

### Red-Green-Refactor Cycle

**MANDATORY:** Tests must be written first for all code. No exceptions.

#### Example: Implementing Provider Selection (User Story 2)

**Step 1: Write Failing Test (RED)**

```rust
// src-tauri/tests/contract/provider_selection_test.rs
use chenchen::providers::ProviderManager;

#[test]
fn test_get_all_providers_returns_three_providers() {
    let manager = ProviderManager::new();
    let providers = manager.get_all_providers();

    assert_eq!(providers.len(), 3);
    assert!(providers.iter().any(|p| p.id == "ChatGPT"));
    assert!(providers.iter().any(|p| p.id == "Gemini"));
    assert!(providers.iter().any(|p| p.id == "Claude"));
}

#[test]
fn test_update_provider_selection_toggles_state() {
    let mut manager = ProviderManager::new();

    let result = manager.update_provider_selection("ChatGPT", true);
    assert!(result.is_ok());

    let provider = result.unwrap();
    assert_eq!(provider.is_selected, true);
}

#[test]
fn test_cannot_deselect_last_provider() {
    let mut manager = ProviderManager::new();
    manager.update_provider_selection("ChatGPT", true).unwrap();

    let result = manager.update_provider_selection("ChatGPT", false);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, "ValidationError");
}
```

Run tests (they should FAIL):
```bash
cd src-tauri
cargo test
```

**Step 2: Implement Minimal Code (GREEN)**

```rust
// src-tauri/src/providers/manager.rs
pub struct ProviderManager {
    providers: Vec<Provider>,
}

impl ProviderManager {
    pub fn new() -> Self {
        Self {
            providers: vec![
                Provider::new("ChatGPT"),
                Provider::new("Gemini"),
                Provider::new("Claude"),
            ],
        }
    }

    pub fn get_all_providers(&self) -> &[Provider] {
        &self.providers
    }

    pub fn update_provider_selection(
        &mut self,
        provider_id: &str,
        is_selected: bool
    ) -> Result<Provider, CommandError> {
        // Validation: Cannot deselect last provider
        if !is_selected {
            let selected_count = self.providers.iter()
                .filter(|p| p.is_selected)
                .count();

            if selected_count == 1 {
                return Err(CommandError {
                    code: "ValidationError".to_string(),
                    message: "At least one provider must be selected".to_string(),
                });
            }
        }

        let provider = self.providers.iter_mut()
            .find(|p| p.id == provider_id)
            .ok_or_else(|| CommandError {
                code: "NotFound".to_string(),
                message: format!("Provider {} not found", provider_id),
            })?;

        provider.is_selected = is_selected;
        Ok(provider.clone())
    }
}
```

Run tests (they should PASS):
```bash
cargo test
```

**Step 3: Refactor (REFACTOR)**

- Extract validation logic
- Improve error messages
- Add documentation
- Keep tests green

```rust
// Refactored with extracted validation
impl ProviderManager {
    fn validate_selection_change(&self, is_selected: bool) -> Result<(), CommandError> {
        if !is_selected && self.selected_count() == 1 {
            return Err(CommandError::validation(
                "At least one provider must be selected"
            ));
        }
        Ok(())
    }

    fn selected_count(&self) -> usize {
        self.providers.iter().filter(|p| p.is_selected).count()
    }
}
```

Run tests again to verify refactoring didn't break anything:
```bash
cargo test
```

---

## Development Sequence (By User Story)

Implementation follows spec.md user story priorities:

### Phase 1: P1 User Stories (MVP)

**User Story 2: Select Target LLM Providers (P1)**
- Libraries: `providers/` (config, manager)
- Tests: Contract tests for ProviderManager API
- Frontend: ProviderSelector.svelte component
- Commands: `get_providers`, `update_provider_selection`

**User Story 1: Send Same Prompt to Multiple LLMs (P1)**
- Libraries: `injection/` (script generation, execution), `status/` (submission tracking)
- Tests: Integration tests for JS injection, unit tests for timeout logic
- Frontend: PromptInput.svelte, StatusDisplay.svelte
- Commands: `submit_prompt`, `get_submission_status`
- Events: `submission_status_changed`

**User Story 3: View Per-Provider Status (P1)**
- Libraries: `status/` (state transitions, error classification)
- Tests: Unit tests for status state machine
- Frontend: StatusDisplay.svelte (real-time updates)
- Commands: Event listeners for `submission_status_changed`

**Split-Screen Layout (FR-016, FR-017)**
- Libraries: `layout/` (dimension calculation)
- Tests: Unit tests for layout logic (1/2/3 provider scenarios)
- Frontend: CSS grid/flexbox for provider panels
- Commands: `get_layout_configuration`

### Phase 2: P2 User Stories (Post-MVP)

**User Story 4: Maintain Persistent Sessions (P2)**
- Libraries: `webview/` (session management, data directory config)
- Tests: Integration tests for session persistence (require app restart)
- Frontend: Login prompts when not authenticated
- Commands: `create_provider_webview`, `check_authentication`

**User Story 5: Keep Everything Local (P2)**
- Validation: Network traffic monitoring tests
- Tests: Verify no telemetry/cloud calls
- Documentation: Privacy policy, data handling disclosure

---

## Testing Strategy

### Test Organization

```
src-tauri/tests/
├── contract/              # Library public API tests
│   ├── provider_manager_test.rs
│   ├── injection_test.rs
│   ├── layout_calculator_test.rs
│   └── status_tracker_test.rs
├── integration/           # Cross-library integration tests
│   ├── submission_flow_test.rs
│   ├── webview_session_test.rs
│   └── ipc_commands_test.rs
└── unit/                  # Complex logic unit tests
    ├── timeout_logic_test.rs
    └── selector_matching_test.rs
```

### Frontend Tests

```
tests/
├── components/
│   ├── PromptInput.test.ts
│   ├── ProviderSelector.test.ts
│   ├── StatusDisplay.test.ts
│   └── ProviderPanel.test.ts
└── integration/
    ├── submission_flow.test.ts
    └── layout_responsiveness.test.ts
```

### Running Tests

**Rust Backend:**
```bash
cd src-tauri
cargo test                      # All tests
cargo test --test contract      # Contract tests only
cargo test providers::          # Specific module
```

**Frontend:**
```bash
npm run test                    # Vitest
npm run test:watch              # Watch mode
npm run test:coverage           # Coverage report
```

**E2E (Critical Paths Only):**
```bash
npm run test:e2e                # Playwright
```

---

## Configuration Management

### Provider Selector Configuration

Create `config/providers.json` in repo root:

```json
{
  "version": "1.0.0",
  "providers": {
    "ChatGPT": {
      "provider_id": "ChatGPT",
      "config_version": "1.0.0",
      "input_selectors": ["textarea[data-id='root']"],
      "submit_selectors": ["button[data-testid='send-button']"],
      "auth_check_selectors": [".login-button"],
      "last_updated": "2025-11-13T00:00:00Z"
    },
    "Gemini": { /* ... */ },
    "Claude": { /* ... */ }
  }
}
```

Load in Rust:
```rust
// src-tauri/src/providers/config.rs
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderConfigs {
    pub version: String,
    pub providers: HashMap<String, ProviderConfig>,
}

impl ProviderConfigs {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("config/providers.json")?;
        let configs: ProviderConfigs = serde_json::from_str(&content)?;
        Ok(configs)
    }
}
```

### Tauri Configuration

Update `src-tauri/tauri.conf.json`:

```json
{
  "productName": "ChenChen",
  "identifier": "com.chenchen.app",
  "version": "0.1.0",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "ChenChen",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600
      }
    ],
    "withGlobalTauri": true,
    "security": {
      "dangerousUseHttpScheme": false
    }
  },
  "bundle": {
    "identifier": "com.chenchen.app",
    "targets": ["nsis", "dmg", "appimage"]
  }
}
```

---

## Debugging Tips

### Rust Backend Logs

Enable Tauri logging:
```bash
RUST_LOG=debug npm run tauri dev
```

Add structured logging in code:
```rust
use log::{info, warn, error};

info!("Submitting prompt to provider: {}", provider_id);
warn!("Timeout after 30s for provider: {}", provider_id);
error!("JavaScript injection failed: {}", error_message);
```

### Frontend DevTools

Open Chrome DevTools in Tauri window:
```javascript
// src/main.ts (development only)
if (import.meta.env.DEV) {
  document.addEventListener('contextmenu', e => e.preventDefault());
}
```

Or programmatically:
```rust
#[cfg(debug_assertions)]
window.open_devtools();
```

### WebView Debugging

**Windows (WebView2):**
- Navigate to `edge://inspect` in Microsoft Edge
- See "Remote Targets" for Tauri webviews

**macOS (WebKit):**
- Enable Safari Developer Menu
- Develop > Show Web Inspector (select Tauri window)

**Linux (WebKitGTK):**
```bash
WEBKIT_WEBGL_DEBUG=1 npm run tauri dev
```

---

## Common Pitfalls

### 1. Forgetting to Clear IPC Mocks

**Problem:** Frontend tests fail intermittently

**Solution:**
```typescript
import { clearMocks } from "@tauri-apps/api/mocks";

afterEach(() => {
  clearMocks();  // CRITICAL
});
```

### 2. Not Awaiting Tauri Commands

**Problem:** Commands return undefined or race conditions

**Solution:**
```typescript
// WRONG
invoke("get_providers");

// CORRECT
const providers = await invoke<Provider[]>("get_providers");
```

### 3. Missing Test Feature Flag

**Problem:** `tauri::test` module not found

**Solution:**
```toml
# Cargo.toml
[dev-dependencies]
tauri = { version = "2.x", features = ["test"] }
```

### 4. macOS Data Store Identifier Issues

**Problem:** Sessions not persisting on macOS

**Solution:**
- Verify macOS version >= 14
- Check UUID is persisted in config file
- Test with fallback for older macOS versions

---

## Next Steps

1. **Set up project** following initialization steps above
2. **Write first test** for ProviderManager (User Story 2)
3. **Implement library** to pass test (Red-Green-Refactor)
4. **Create Tauri command** exposing library via IPC
5. **Build frontend component** calling command
6. **Verify integration** with E2E test
7. **Repeat** for next user story

**Remember:** Tests first, always. Constitution Principle II is NON-NEGOTIABLE.

---

## Resources

- **Tauri Docs**: https://v2.tauri.app/
- **Svelte Docs**: https://svelte.dev/docs
- **Vitest Docs**: https://vitest.dev/
- **Mockall Docs**: https://docs.rs/mockall/
- **Schemars Docs**: https://graham.cool/schemars/

For questions, refer to:
- [spec.md](./spec.md) - Feature requirements and acceptance criteria
- [data-model.md](./data-model.md) - Entity definitions and state machines
- [contracts/tauri-commands.md](./contracts/tauri-commands.md) - IPC API contract
- [research.md](./research.md) - Technical decisions and rationale
