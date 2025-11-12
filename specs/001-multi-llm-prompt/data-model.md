# Data Model: Multi-LLM Prompt Desktop App

**Branch**: `001-multi-llm-prompt` | **Date**: 2025-11-13
**Phase**: 1 (Design & Contracts)

## Overview

This document defines the core entities, their fields, relationships, validation rules, and state transitions for the multi-LLM prompt desktop application.

---

## Entities

### 1. Prompt

**Description:** The text input provided by the user to be sent to LLM providers.

**Fields:**
| Field | Type | Required | Validation | Description |
|-------|------|----------|------------|-------------|
| `content` | `String` | Yes | Non-empty | The prompt text entered by user |
| `timestamp` | `DateTime` | Yes | Auto-generated | When prompt was created |

**Validation Rules:**
- `content` must be non-empty (minimum 1 character)
- `content` maximum length determined by provider limits (suggest 100,000 characters)

**Relationships:**
- Associated with multiple `Submission` entities (one per selected provider)

---

### 2. Provider

**Description:** An LLM service that can receive and respond to prompts.

**Fields:**
| Field | Type | Required | Validation | Description |
|-------|------|----------|------------|-------------|
| `id` | `enum(ChatGPT, Gemini, Claude)` | Yes | One of three values | Unique provider identifier |
| `name` | `String` | Yes | Non-empty | Display name ("ChatGPT", "Gemini", "Claude") |
| `url` | `URL` | Yes | Valid HTTPS URL | Provider website URL |
| `is_selected` | `bool` | Yes | Default: false | Whether user has selected this provider |
| `is_authenticated` | `bool` | Yes | Default: false | Whether user has valid session |
| `selector_config_id` | `String` | Yes | Valid config key | References ProviderSelectorConfig |

**Validation Rules:**
- `id` must be one of: `ChatGPT`, `Gemini`, `Claude`
- `url` must be valid HTTPS URL
- `name` must match `id` (enforced by enum mapping)

**State Transitions:**
- `is_selected`: User toggles via UI (false ↔ true)
- `is_authenticated`: Detected on webview load (false → true on successful session detection, true → false on auth failure)

**Invariants:**
- Exactly 3 Provider instances exist at all times (ChatGPT, Gemini, Claude)
- At least 1 provider must have `is_selected = true` before prompt submission (FR-004)
- Maximum 3 providers can have `is_selected = true` simultaneously (TC-005)

---

### 3. ProviderSelectorConfig

**Description:** Maintained configuration of CSS selectors and DOM identifiers for locating chat interface elements on each provider's website.

**Fields:**
| Field | Type | Required | Validation | Description |
|-------|------|----------|------------|-------------|
| `provider_id` | `enum(ChatGPT, Gemini, Claude)` | Yes | Valid Provider.id | Which provider this config applies to |
| `version` | `String` | Yes | Semver format | Config schema version (e.g., "1.0.0") |
| `input_selectors` | `Vec<String>` | Yes | Non-empty array | CSS selectors for chat input element (tried in order) |
| `submit_selectors` | `Vec<String>` | Yes | Non-empty array | CSS selectors for submit button (tried in order) |
| `auth_check_selectors` | `Vec<String>` | Yes | Non-empty array | CSS selectors indicating unauthenticated state |
| `last_updated` | `DateTime` | Yes | ISO 8601 format | When config was last modified |

**Validation Rules:**
- `version` must follow semver (MAJOR.MINOR.PATCH)
- All selector arrays must be non-empty
- CSS selectors must be valid syntax
- `provider_id` must correspond to an existing Provider

**Storage:**
- Persisted in `config/providers.json` file
- Versioned independently from app version (can be updated without recompiling)

**Example:**
```json
{
  "provider_id": "ChatGPT",
  "version": "1.0.0",
  "input_selectors": ["textarea[data-id='root']", "textarea[placeholder*='Message']"],
  "submit_selectors": ["button[data-testid='send-button']", "button[aria-label='Send']"],
  "auth_check_selectors": [".login-button", "#auth-required"],
  "last_updated": "2025-11-13T00:00:00Z"
}
```

---

### 4. Submission

**Description:** Tracks the state of a prompt submission to a specific provider.

**Fields:**
| Field | Type | Required | Validation | Description |
|-------|------|----------|------------|-------------|
| `id` | `UUID` | Yes | Auto-generated | Unique submission identifier |
| `provider_id` | `enum(ChatGPT, Gemini, Claude)` | Yes | Valid Provider.id | Which provider this submission targets |
| `prompt_content` | `String` | Yes | Non-empty | The prompt text being submitted |
| `status` | `enum(Pending, InProgress, Retrying, Success, Failed)` | Yes | Valid status | Current submission state |
| `attempt_count` | `u8` | Yes | 0-2 | Number of submission attempts (0 = not started, 1 = first attempt, 2 = retry) |
| `error_type` | `Option<enum>` | No | See error types | Classification of failure (if status = Failed) |
| `error_message` | `Option<String>` | No | N/A | Human-readable error description |
| `started_at` | `Option<DateTime>` | No | ISO 8601 | When submission attempt started |
| `completed_at` | `Option<DateTime>` | No | ISO 8601 | When submission finished (success or failure) |

**Error Types (for `error_type` field):**
```rust
enum SubmissionErrorType {
    Timeout,           // 30s timeout exceeded
    NetworkError,      // Connection/network failure
    AuthenticationError, // Not logged in / session expired
    RateLimitError,    // Provider rate limiting
    ElementNotFound,   // JavaScript injection couldn't locate input/submit elements
    InjectionFailed,   // JavaScript execution failed
}
```

**Validation Rules:**
- `attempt_count` must be 0-2 (max 2 attempts: initial + 1 retry per FR-008)
- `status = Pending` → `attempt_count = 0`
- `status = InProgress` or `Retrying` → `started_at` must be set
- `status = Success` or `Failed` → `completed_at` must be set
- `status = Failed` → `error_type` and `error_message` must be set

**State Transitions:**
```
Pending → InProgress (attempt_count = 1, started_at set)
    ↓
InProgress → Success (completed_at set)
    ↓
InProgress → Retrying (on Timeout/NetworkError, attempt_count = 2)
    ↓
Retrying → Success (completed_at set)
    ↓
Retrying → Failed (completed_at set, error_type set, error_message set)
    ↓
InProgress → Failed (on AuthError/RateLimitError/ElementNotFound, no retry per FR-009)
```

**Retry Logic (FR-007, FR-008):**
- Timeout or NetworkError → automatic retry once (attempt_count increments)
- AuthenticationError, RateLimitError, ElementNotFound, InjectionFailed → fail immediately (no retry per FR-009)
- After 2 attempts (initial + 1 retry), status becomes `Failed`

**Timeout Handling (FR-007):**
- Each attempt has 30-second timeout
- Timeout detection: `(now() - started_at) > 30s`
- On timeout: transition to `Retrying` if `attempt_count = 1`, else `Failed`

---

### 5. LayoutConfiguration

**Description:** Calculates and stores the split-screen arrangement based on number of selected providers.

**Fields:**
| Field | Type | Required | Validation | Description |
|-------|------|----------|------------|-------------|
| `provider_count` | `u8` | Yes | 1-3 | Number of selected providers |
| `layout_type` | `enum(Full, VerticalSplit, Grid)` | Yes | Derived from count | How to arrange provider panels |
| `panel_dimensions` | `Vec<PanelDimension>` | Yes | Length = provider_count | Position/size for each panel |

**Layout Types:**
```rust
enum LayoutType {
    Full,          // 1 provider: full window width/height
    VerticalSplit, // 2 providers: side-by-side vertical split
    Grid,          // 3 providers: 2 top, 1 bottom (or 3 columns)
}
```

**PanelDimension:**
```rust
struct PanelDimension {
    x: f32,      // X position (0.0 - 1.0 as percentage of window width)
    y: f32,      // Y position (0.0 - 1.0 as percentage of window height)
    width: f32,  // Width (0.0 - 1.0 as percentage)
    height: f32, // Height (0.0 - 1.0 as percentage)
}
```

**Validation Rules:**
- `provider_count` must be 1-3 (enforced by Provider.is_selected invariant)
- `panel_dimensions.len() == provider_count`
- All dimension values must be 0.0-1.0

**Layout Calculation (FR-017):**
```
provider_count = 1:
  layout_type = Full
  panel_dimensions = [{x: 0.0, y: 0.0, width: 1.0, height: 1.0}]

provider_count = 2:
  layout_type = VerticalSplit
  panel_dimensions = [
    {x: 0.0, y: 0.0, width: 0.5, height: 1.0},  // Left panel
    {x: 0.5, y: 0.0, width: 0.5, height: 1.0}   // Right panel
  ]

provider_count = 3:
  layout_type = Grid
  panel_dimensions = [
    {x: 0.0, y: 0.0, width: 0.5, height: 0.5},    // Top-left
    {x: 0.5, y: 0.0, width: 0.5, height: 0.5},    // Top-right
    {x: 0.0, y: 0.5, width: 1.0, height: 0.5}     // Bottom (full width)
  ]
```

**State Transitions:**
- Recalculated whenever user changes provider selection
- Reactive to window resize events (dimensions remain percentages, actual pixels recalculated)

---

### 6. WebviewSession

**Description:** Represents an authenticated browser session within an embedded webview.

**Fields:**
| Field | Type | Required | Validation | Description |
|-------|------|----------|------------|-------------|
| `provider_id` | `enum(ChatGPT, Gemini, Claude)` | Yes | Valid Provider.id | Which provider this session belongs to |
| `data_directory` | `PathBuf` | Yes (Windows/Linux) | Valid path | Persistent storage path for cookies/localStorage |
| `data_store_identifier` | `[u8; 16]` | Yes (macOS) | 16-byte UUID | WebKit data store identifier |
| `is_persistent` | `bool` | Yes | Always true | Whether session survives app restart |
| `last_activity` | `DateTime` | Yes | ISO 8601 | Last successful interaction with provider |

**Platform-Specific Fields:**
- Windows/Linux: Use `data_directory` field
- macOS: Use `data_store_identifier` field

**Validation Rules:**
- `provider_id` must be unique (only one session per provider)
- `data_directory` path must exist (created if missing)
- `data_store_identifier` must be valid UUID bytes

**Relationships:**
- One WebviewSession per Provider
- WebviewSession persists independently of app lifecycle (FR-011)

**State Transitions:**
- Created: When user first selects a provider (display login page)
- Authenticated: After successful login (detected via absence of auth_check_selectors)
- Expired: When submission fails with AuthenticationError (display login page again)

---

## Entity Relationships

```
Prompt (1) ----< Submission (N)
  │
  └─ timestamp

Provider (1) ----< Submission (N)
  │
  ├─ is_selected (drives LayoutConfiguration)
  ├─ is_authenticated (drives WebviewSession state)
  └─ selector_config_id ----< ProviderSelectorConfig (1)

ProviderSelectorConfig (3 total)
  │
  └─ Used by JavaScript injection logic

Submission
  │
  ├─ status (enum with state machine)
  ├─ attempt_count (retry logic)
  └─ error_type (error classification)

LayoutConfiguration (derived)
  │
  └─ Calculated from Provider.is_selected count

WebviewSession (3 total)
  │
  └─ Persistent across app restarts (FR-011)
```

---

## Aggregate Roots

**Primary Aggregate:** `Prompt` + `Submission[]`
- User creates one Prompt
- Prompt spawns N Submissions (one per selected Provider)
- Submissions tracked independently (different states, timings, errors)
- Prompt lifecycle: created → submitted → (monitor submissions until all complete)

**Configuration Aggregate:** `ProviderSelectorConfig[]`
- Managed independently via JSON file
- Updated without app recompilation (TC-007)
- Versioned for backward compatibility

**Session Aggregate:** `WebviewSession[]`
- Managed by Tauri webview lifecycle
- App configures data directory/identifier, platform handles persistence
- No direct manipulation of cookies/tokens (FR-012)

---

## Validation Summary

**Pre-Submission Validation:**
1. At least one Provider has `is_selected = true` (FR-004)
2. Prompt `content` is non-empty
3. All selected Providers have valid `ProviderSelectorConfig`

**Runtime Validation:**
1. Submission `attempt_count` never exceeds 2
2. Submission timeout enforced at 30 seconds (FR-007)
3. Retry logic follows error type rules (FR-008, FR-009)
4. LayoutConfiguration `provider_count` matches selected count

**Persistence Validation:**
1. WebviewSession `data_directory` paths exist and are writable
2. ProviderSelectorConfig JSON is valid and schema-compliant
3. No credentials stored in app data (FR-012)

---

## State Machine: Submission Lifecycle

```
┌─────────┐
│ Pending │  attempt_count = 0
└────┬────┘
     │ User clicks Send
     ▼
┌──────────────┐
│  InProgress  │  attempt_count = 1, started_at set, timeout timer starts
└──┬───────┬───┘
   │       │
   │       │ (30s timeout OR NetworkError)
   │       ▼
   │  ┌──────────┐
   │  │ Retrying │  attempt_count = 2, started_at reset, timeout timer restarts
   │  └────┬─────┘
   │       │
   │       │ (30s timeout OR final error)
   │       │
   ▼       ▼
┌─────────────┐           ┌────────┐
│   Failed    │  ◄─────── │ Success│
│ error_type  │           │        │
│ set         │           │        │
└─────────────┘           └────────┘
   ▲
   │ (AuthError, RateLimitError, ElementNotFound - no retry)
   │
   └─ Direct transition from InProgress
```

**State Invariants:**
- `Pending`: attempt_count = 0, started_at = None
- `InProgress` or `Retrying`: started_at = Some, completed_at = None
- `Success` or `Failed`: completed_at = Some
- `Failed`: error_type = Some, error_message = Some
- `Retrying`: Only reachable from InProgress with Timeout/NetworkError
- Maximum 2 state transitions from InProgress before terminal state

---

## Performance Considerations

**Memory:**
- Maximum 3 Submission entities active simultaneously (1 per selected provider)
- ProviderSelectorConfig loaded once at startup, cached in memory
- WebviewSession lightweight (path reference only, actual storage managed by OS)

**Storage:**
- ProviderSelectorConfig: ~1-2KB per provider (JSON file)
- WebviewSession cookies: Managed by platform webview (varies by site, typically 10-100KB per provider)
- No prompt history retained (FR-013, SC-004)

**Concurrency:**
- Submissions run concurrently (independent status tracking)
- Timeout timers run per-submission (not shared)
- Layout recalculation triggered on provider selection change (debounced if user rapidly toggles)
