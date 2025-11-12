# Implementation Plan: Multi-LLM Prompt Desktop App

**Branch**: `001-multi-llm-prompt` | **Date**: 2025-11-13 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-multi-llm-prompt/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a Tauri 2.0-based desktop application that enables users to send a single prompt to multiple LLM providers (ChatGPT, Gemini, Claude) simultaneously. The app uses embedded webviews for provider authentication (no API keys), JavaScript injection for prompt submission, and displays responses in a split-screen layout (1-3 providers max). All data remains local with persistent sessions.

## Technical Context

**Language/Version**: Rust (latest stable for Tauri 2.0), TypeScript/JavaScript for frontend
**Primary Dependencies**: Tauri 2.0, NEEDS CLARIFICATION (frontend framework - React/Vue/Svelte/Vanilla)
**Storage**: Tauri app data directory for webview session cookies (persistent), no database required
**Testing**: cargo test (Rust backend), NEEDS CLARIFICATION (frontend test framework)
**Target Platform**: Cross-platform desktop (Windows 10+/WebView2, macOS 10.15+/WebKit, Linux/WebKitGTK)
**Project Type**: Desktop application (Tauri hybrid architecture - Rust backend + web frontend)
**Performance Goals**: App startup <3s, prompt submission initiation <10s for 3 providers, status updates <2s
**Constraints**: Binary size <15MB, max 3 simultaneous providers, 30s timeout per provider, offline-capable except provider communication
**Scale/Scope**: Single-user desktop app, 3 LLM providers, ~5-10 screens/views (main prompt UI, provider webviews, settings)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Library-First ✅

**Status**: PASS

**Analysis**: The architecture naturally decomposes into standalone libraries:
- Webview management library (create, configure, persist sessions)
- JavaScript injection library (locate elements, set values, trigger submit)
- Provider configuration library (manage selectors for ChatGPT/Gemini/Claude)
- Layout manager library (calculate split-screen arrangements based on provider count)
- Status tracker library (manage submission states, timeouts, retries)

Each library has clear boundaries and can be tested independently without the full Tauri app.

### II. Test-First (NON-NEGOTIABLE) ⚠️

**Status**: CONDITIONAL PASS - TDD will be enforced during implementation

**Analysis**: This planning phase defines requirements and acceptance criteria. Tests will be written first during `/speckit.implement` phase. All libraries identified above must have tests written before implementation begins.

**Commitment**:
- Contract tests for all library public APIs
- Integration tests for webview-to-provider communication
- Integration tests for JavaScript injection with mock DOM structures
- Unit tests for layout calculation, status state transitions

### III. Simplicity & YAGNI ✅

**Status**: PASS

**Analysis**:
- Direct JavaScript injection (no complex browser automation frameworks)
- Maximum 3 providers (prevents overengineering for arbitrary scaling)
- No credential storage (leverages webview native session management)
- No database (sessions managed by platform webview)
- Simple CSS selector configuration (no ML-based element detection)

**Complexity Avoided**:
- ❌ Full browser automation (Selenium/Playwright) - unnecessary overhead
- ❌ API key management - not required per user preference
- ❌ Cloud sync - explicitly rejected for privacy
- ❌ Complex state management - simple status enums sufficient

### IV. Observability ✅

**Status**: PASS

**Analysis**:
- Tauri commands expose text-based IPC between frontend and Rust backend
- Status updates for each provider (pending, in_progress, success, failed, retrying)
- Error messages include provider name, error type, and actionable guidance
- Structured logging for JavaScript injection attempts (element found/not found, submit triggered/failed)
- Timeout events logged with timestamps

**Text-based interfaces**:
- Provider configuration as JSON files (CSS selectors, timeouts)
- Status updates as structured events
- Error logs to stderr with correlation IDs

### V. Versioning & Breaking Changes ✅

**Status**: PASS

**Analysis**:
- Initial version 0.1.0 (pre-stable, breaking changes permitted)
- Provider selector configurations versioned independently (providers update selectors without app recompilation per TC-007)
- Tauri commands form public API contract (will follow semver on 1.0.0+ releases)
- Breaking changes to Tauri command signatures will require MAJOR version bump

**Version Strategy**:
- App version: semver starting at 0.1.0
- Provider config schema version: independent semver for backwards compatibility checks

### Gate Decision: ✅ PROCEED TO PHASE 0

All constitutional principles satisfied. No violations requiring justification in Complexity Tracking table.

---

## Post-Phase 1 Constitution Re-Check

*Re-evaluation after Phase 1 design (data model, contracts, architecture)*

### I. Library-First ✅

**Status**: PASS

**Re-evaluation**: Design confirms library-first architecture with clear module boundaries:
- **webview/** library: Session management, data directory configuration (contract: `create_provider_webview`)
- **injection/** library: JavaScript generation and execution (contract: used internally by `submit_prompt`)
- **providers/** library: Configuration loading and validation (contract: `get_providers`, `update_provider_selection`)
- **layout/** library: Dimension calculation (contract: `get_layout_configuration`)
- **status/** library: Submission state machine (contract: `get_submission_status`, `submit_prompt`)

Each library has defined types in data-model.md and exposed via Tauri commands. Contract tests planned for all library APIs.

### II. Test-First (NON-NEGOTIABLE) ✅

**Status**: PASS

**Re-evaluation**: TDD workflow explicitly defined in quickstart.md:
- Contract test structure defined (`tests/contract/`)
- Integration test structure defined (`tests/integration/`)
- Unit test structure defined (`tests/unit/`)
- Red-Green-Refactor example provided (ProviderManager)
- Frontend test structure defined (Vitest + @testing-library/svelte)
- Contract validation with schemars (Rust) + zod (TypeScript)

Commitment reaffirmed: No implementation code written until tests exist and fail.

### III. Simplicity & YAGNI ✅

**Status**: PASS

**Re-evaluation**: Design remains simple and avoids premature complexity:
- **Data model**: 6 entities with clear responsibilities, no over-engineering
- **API surface**: 7 Tauri commands (not 20+), focused on user stories
- **State machine**: Simple 5-state submission lifecycle (Pending → InProgress → Retrying → Success/Failed)
- **Layout calculation**: Direct percentage-based calculations, no complex responsive framework
- **Provider configs**: JSON files with CSS selectors, no ML-based element detection

**Complexity avoided in design:**
- ❌ Repository pattern - direct state management sufficient
- ❌ Event sourcing - simple status updates sufficient
- ❌ Microservices - single Tauri app with libraries sufficient
- ❌ Advanced error recovery - simple retry logic (1 retry on timeout) sufficient

### IV. Observability ✅

**Status**: PASS

**Re-evaluation**: Observability embedded in design:
- **Status events**: `submission_status_changed` event emitted for all state transitions
- **Structured errors**: `CommandError` type with `code` and `message` fields
- **Text-based configs**: Provider selector configs as JSON (provider-config-example.json)
- **Logging points identified**: JavaScript injection attempts, timeout triggers, authentication checks
- **Diagnostic data in errors**: `SubmissionErrorType` enum classifies failures (Timeout, NetworkError, AuthenticationError, etc.)

Contract definitions include ISO 8601 timestamps for `started_at`, `completed_at`, `last_checked` fields.

### V. Versioning & Breaking Changes ✅

**Status**: PASS

**Re-evaluation**: Versioning strategy formalized:
- **App version**: 0.1.0 (pre-stable, per tauri.conf.json)
- **Contract version**: 1.0.0 (defined in contracts/tauri-commands.md)
- **Provider config schema version**: 1.0.0 (defined in contracts/provider-config-schema.json)
- **Per-provider config version**: Independent semver per provider (allows selective updates)

**Breaking change policy documented:**
- Tauri command signature changes → MAJOR version bump
- New commands → MINOR version bump
- Provider config schema changes tracked independently (backward compatibility checks)

**Migration path**: Provider configs can be updated without recompiling app (TC-007 satisfied).

### Gate Decision: ✅ PROCEED TO PHASE 2 (Task Generation)

All constitutional principles remain satisfied after Phase 1 design. Architecture successfully decomposes into libraries, TDD workflow established, simplicity maintained, observability designed in, versioning strategy defined.

**Ready for `/speckit.tasks` command.**

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── lib.rs                    # Tauri app entry point, command registration
│   ├── webview/                  # Webview management library
│   │   ├── mod.rs
│   │   ├── manager.rs            # Create, configure, lifecycle
│   │   └── session.rs            # Persistent session handling
│   ├── injection/                # JavaScript injection library
│   │   ├── mod.rs
│   │   ├── injector.rs           # Element location, value setting, submit trigger
│   │   └── script_builder.rs    # JS code generation
│   ├── providers/                # Provider configuration library
│   │   ├── mod.rs
│   │   ├── config.rs             # Load/validate selector configs
│   │   ├── chatgpt.rs            # ChatGPT-specific selectors
│   │   ├── gemini.rs             # Gemini-specific selectors
│   │   └── claude.rs             # Claude-specific selectors
│   ├── layout/                   # Layout manager library
│   │   ├── mod.rs
│   │   └── calculator.rs         # Split-screen arrangement logic
│   ├── status/                   # Status tracker library
│   │   ├── mod.rs
│   │   ├── tracker.rs            # State transitions, timeout handling
│   │   └── types.rs              # Status enums, event types
│   └── commands.rs               # Tauri command definitions (public API)
├── tests/
│   ├── contract/                 # Library API contract tests
│   ├── integration/              # Cross-library integration tests
│   └── unit/                     # Complex logic unit tests
├── Cargo.toml
└── tauri.conf.json               # Tauri configuration

src/                              # Frontend (framework TBD in research phase)
├── components/
│   ├── PromptInput.{tsx,vue,svelte}
│   ├── ProviderSelector.{tsx,vue,svelte}
│   ├── StatusDisplay.{tsx,vue,svelte}
│   └── ProviderPanel.{tsx,vue,svelte}
├── services/
│   ├── tauri.{ts,js}             # Tauri command invocation wrapper
│   └── state.{ts,js}             # Frontend state management
├── App.{tsx,vue,svelte}
└── main.{ts,js}

tests/                            # Frontend tests (framework-specific)
├── components/
└── integration/

config/
└── providers.json                # Provider selector configurations (versioned)
```

**Structure Decision**: Tauri 2.0 hybrid desktop architecture selected. Rust backend (`src-tauri/`) contains all core libraries as independent modules for testability. Frontend (`src/`) uses a web framework (TBD in Phase 0 research - likely React, Vue, or Svelte) compiled to web assets served by Tauri. This structure enforces Library-First principle with clear boundaries between webview management, injection logic, provider configs, layout calculation, and status tracking.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
