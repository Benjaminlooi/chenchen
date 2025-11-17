# Setup Notes - ChenChen Multi-LLM Prompt App

## System Dependencies (Linux)

The project requires WebKitGTK system libraries to build Tauri applications on Linux.

### Required Packages

#### Fedora/RHEL
```bash
sudo dnf install webkit2gtk4.1-devel libsoup3-devel javascriptcoregtk4.1-devel
```

#### Ubuntu/Debian
```bash
sudo apt-get install libwebkit2gtk-4.1-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
```

#### Arch Linux
```bash
sudo pacman -S webkit2gtk-4.1
```

### Verification

After installing dependencies, verify with:
```bash
pkg-config --modversion webkit2gtk-4.1
pkg-config --modversion libsoup-3.0
pkg-config --modversion javascriptcoregtk-4.1
```

## Implementation Progress

### ✅ Completed (Phase 1-2: Foundation)

**Project Structure:**
- ✅ Tauri + Svelte project initialized
- ✅ Test directories created (`src-tauri/tests/`, `tests/`)
- ✅ Dependencies configured in Cargo.toml and package.json

**Backend (Rust):**
- ✅ Core types defined (`src-tauri/src/types.rs`):
  - ProviderId enum (ChatGPT, Gemini, Claude)
  - SubmissionStatus enum (Pending, InProgress, Retrying, Success, Failed)
  - SubmissionErrorType enum (Timeout, NetworkError, etc.)
  - CommandError struct with helpers
- ✅ Provider module (`src-tauri/src/providers/`):
  - Provider struct with configuration
  - ProviderManager with selection logic
  - ProviderSelectorConfig loading from JSON
- ✅ AppState structure for shared state
- ✅ Module structure for all libraries (webview/, injection/, layout/, status/)

**Frontend (TypeScript):**
- ✅ TypeScript type definitions (`src/types.ts`) mirroring Rust types
- ✅ Zod schemas for runtime validation
- ✅ Tauri service wrapper (`src/services/tauri.ts`) for type-safe IPC

**Tests:**
- ✅ Vitest configured with jsdom environment
- ✅ Test setup file with IPC mock clearing
- ✅ Provider manager unit tests written (inline in manager.rs)
- ✅ Contract test structure created (src-tauri/tests/contract/)

### 🚧 In Progress (Phase 3: User Story 2 - Provider Selection)

**Blocked by system dependencies:**
- Cannot run `cargo test` until WebKitGTK libraries installed
- Provider manager logic implemented but not yet verified

**Next Steps:**
1. Install system dependencies (see above)
2. Run tests to verify provider manager: `cargo test providers::manager`
3. Implement Tauri commands (`get_providers`, `update_provider_selection`)
4. Create ProviderSelector.svelte component
5. Wire up frontend to backend via Tauri IPC

### 📋 Remaining (Phases 4-12)

See `/home/ben/dev/ben/chenchen/specs/001-multi-llm-prompt/tasks.md` for complete task breakdown.

**Key Milestones:**
- Phase 4: Layout configuration (split-screen calculation)
- Phase 5: Webview session management (persistent logins)
- Phase 6-7: Provider configuration loading and JavaScript injection
- Phase 8: Submission status tracking
- Phase 9: **MVP COMPLETE** - Full prompt submission flow
- Phase 10-11: Authentication detection and privacy validation
- Phase 12: Polish, logging, final validation

## Running Tests

### Backend (Rust)
```bash
cd src-tauri
cargo test              # All tests
cargo test --lib        # Library tests only
cargo test providers    # Provider module tests only
```

### Frontend (TypeScript)
```bash
npm test                # Run Vitest
npm run test:watch      # Watch mode
npm run test:coverage   # Coverage report
```

### Dev Server
```bash
npm run tauri dev       # Start Tauri app with hot reload
```

### Build
```bash
npm run tauri build     # Create production binary
```

## Architecture Overview

```
┌─────────────────────────────────────┐
│     Frontend (Svelte + TypeScript)  │
│  - PromptInput.svelte              │
│  - ProviderSelector.svelte         │
│  - StatusDisplay.svelte            │
│  - ProviderPanel.svelte            │
└──────────────┬──────────────────────┘
               │ IPC (Tauri Commands)
┌──────────────┴──────────────────────┐
│     Backend (Rust)                  │
│  - providers/ (config, selection)   │
│  - layout/ (split-screen calc)      │
│  - webview/ (session management)    │
│  - injection/ (JS injection)        │
│  - status/ (submission tracking)    │
└──────────────┬──────────────────────┘
               │
┌──────────────┴──────────────────────┐
│  Platform Webviews (ChatGPT, etc.)  │
└─────────────────────────────────────┘
```

## TDD Workflow

**RED-GREEN-REFACTOR cycle is mandatory:**
1. **RED**: Write failing test
2. **GREEN**: Implement minimal code to pass
3. **REFACTOR**: Improve code while keeping tests green

All user stories follow this pattern. No implementation without tests first.

## Documentation

- **Spec**: `specs/001-multi-llm-prompt/spec.md`
- **Plan**: `specs/001-multi-llm-prompt/plan.md`
- **Tasks**: `specs/001-multi-llm-prompt/tasks.md`
- **Data Model**: `specs/001-multi-llm-prompt/data-model.md`
- **Contracts**: `specs/001-multi-llm-prompt/contracts/`
- **Research**: `specs/001-multi-llm-prompt/research.md`
- **Quickstart**: `specs/001-multi-llm-prompt/quickstart.md`
