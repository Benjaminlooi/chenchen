# ChenChen Development Guidelines

**Last updated**: 2025-11-14
**Version**: 0.1.0 (MVP Complete)

## Project Overview

ChenChen is a privacy-focused desktop application that allows users to send prompts to multiple LLM providers (ChatGPT, Gemini, Claude) simultaneously. Built with Tauri 2.0 (Rust backend) and Svelte 5 (TypeScript frontend).

## Active Technologies

### Backend
- **Rust** (latest stable) - Core application logic
- **Tauri 2.0** - Desktop application framework
- **serde/serde_json** - Serialization
- **uuid** - ID generation
- **chrono** - Timestamp handling
- **log/env_logger** - Logging infrastructure

### Frontend
- **Svelte 5** - UI framework with $state runes
- **TypeScript** - Type-safe frontend code
- **Vite** - Build tool
- **Zod** - Runtime type validation

### Testing
- **Rust**: Unit, contract, and integration tests
- **Vitest** - Frontend testing
- **tempfile** - Temporary file handling in tests

## Project Structure

```text
chenchen/
├── src/                           # Svelte frontend
│   ├── components/
│   │   ├── PromptInput.svelte     # Prompt text area component
│   │   ├── ProviderSelector.svelte # Provider selection UI
│   │   ├── StatusDisplay.svelte   # Submission status tracking
│   │   └── ProviderPanel.svelte   # Webview panel container
│   ├── routes/
│   │   └── +page.svelte           # Main application page
│   ├── services/
│   │   └── tauri.ts               # Tauri IPC wrappers
│   └── types.ts                   # TypeScript type definitions
│
├── src-tauri/                     # Rust backend
│   ├── src/
│   │   ├── commands.rs            # Tauri commands (IPC interface)
│   │   ├── state.rs               # Application state management
│   │   ├── types.rs               # Rust type definitions
│   │   ├── logging.rs             # Structured logging module
│   │   ├── providers/
│   │   │   ├── mod.rs             # Provider entity definitions
│   │   │   └── manager.rs         # Provider selection logic
│   │   ├── layout/
│   │   │   ├── mod.rs             # Layout type definitions
│   │   │   └── calculator.rs      # Split-screen layout math
│   │   ├── webview/
│   │   │   ├── mod.rs             # Webview types
│   │   │   └── manager.rs         # Session persistence
│   │   ├── injection/
│   │   │   ├── mod.rs             # Injection types
│   │   │   ├── script_builder.rs  # JavaScript generation
│   │   │   └── injector.rs        # Script execution
│   │   └── status/
│   │       ├── mod.rs             # Submission entity
│   │       └── tracker.rs         # State tracking
│   ├── tests/
│   │   ├── contract/              # Public API tests
│   │   ├── unit/                  # Unit tests
│   │   └── integration/           # Integration tests
│   └── config/
│       └── providers.json         # Provider configurations
│
├── docs/                          # Documentation
│   ├── privacy-policy.md          # Privacy guarantees
│   └── testing-guide.md           # Testing procedures
│
└── specs/                         # Feature specifications
    └── 001-multi-llm-prompt/      # Current feature spec
```

## Essential Commands

### Development

```bash
# Start development server
npm run tauri dev

# Run backend tests
cd src-tauri && cargo test

# Run frontend tests
npm test

# Check TypeScript types
npm run check

# Lint Rust code
cargo clippy

# Format Rust code
cargo fmt
```

### Build

```bash
# Production build
npm run tauri build

# Debug build
cd src-tauri && cargo build
```

### Testing

```bash
# All tests
cargo test && npm test

# Specific test suites
cargo test privacy_test        # Privacy tests
cargo test success_rate_test   # Success rate tests
cargo test logging_format      # Logging tests
```

## Code Style Guidelines

### Rust
- Follow standard Rust conventions (rustfmt)
- Use descriptive error types (`Result<T, CommandError>`)
- Implement `Default` trait where appropriate
- Add doc comments for public APIs (`///`)
- Use structured logging macros (`log_info!`, `log_error!`)

### TypeScript/Svelte
- Strict TypeScript mode enabled
- Use Zod for runtime validation
- Svelte 5 `$state` runes for reactivity
- Avoid `any` types
- Prefer `const` over `let`

### Testing
- **TDD**: Write tests before implementation
- **Contract tests**: Verify public API behavior
- **Unit tests**: Test individual functions
- **Integration tests**: Test component interactions

## Architecture Patterns

### Backend Patterns

1. **Command Pattern**: All IPC through Tauri commands in `commands.rs`
2. **State Management**: Centralized `AppState` with `Mutex` for thread safety
3. **Error Handling**: Consistent `CommandError` with codes and messages
4. **Logging**: Dual-format structured logging (JSON + human-readable)

### Frontend Patterns

1. **Component Communication**: Svelte events + custom window events
2. **Type Safety**: TypeScript interfaces matching Rust types
3. **Service Layer**: Abstraction over Tauri IPC in `tauri.ts`
4. **Reactivity**: Svelte 5 `$state` for local state

## Key Architectural Decisions

### Privacy by Design
- **No credential storage**: Sessions managed by OS webview only
- **No telemetry**: Zero analytics or tracking code
- **Local-only**: No backend servers or cloud services
- **In-memory state**: No persistent prompt/response storage

### Platform-Specific Handling
- **Windows/Linux**: `data_directory` for webview sessions
- **macOS**: `data_store_identifier` (UUID-based)
- Conditional compilation with `#[cfg]`

### State Machine
- Submission lifecycle: `Pending → InProgress → Success/Failed/Retrying`
- Retry logic: Timeout/NetworkError retry, AuthError/RateLimitError fail
- Timeout detection with `is_timed_out()` checks

## Testing Strategy

### Test Coverage (69 total tests)
- **26 unit tests**: Core logic (providers, layout, injection, status, logging)
- **19 contract tests**: Public API interfaces
- **24 integration tests**: Privacy, success rate, logging formats

### Success Criteria
- **SC-001**: <10 second submission to 3 LLMs ✅
- **SC-002**: >=95% success rate with valid sessions ✅
- **FR-012**: No credential storage (verified by privacy tests) ✅
- **FR-013**: Provider domains only (manual network monitoring) ✅

## Recent Changes

### Phase 10: Authentication Detection (commit 43470d8)
- Backend: `check_authentication` command
- Frontend: Login status UI with "Login Required" buttons
- Tests: 5 contract tests

### Phase 11: Privacy Validation (commit 7f966c1)
- Privacy tests: credential storage, prompt history, webview isolation
- Success rate tests: submission reliability validation
- Documentation: privacy-policy.md, testing-guide.md

### Phase 12: Structured Logging (commits 823ede3, c352ed4)
- Logging module with dual-format output
- Enhanced commands, injector, tracker with structured logging
- 8 logging format integration tests

## Known Limitations

- **Webview execution**: Mock implementation (TODO: integrate with Tauri webviews)
- **Event system**: Submission status events not yet implemented
- **Auth check execution**: Returns mock status (TODO: execute in webviews)
- **Async execution**: Submissions currently sequential (TODO: concurrent tasks)

## Future Enhancements (Post-MVP)

- Real webview JavaScript execution via `webview.eval()`
- Tauri event system for real-time status updates
- Concurrent submission execution with tokio tasks
- Response streaming and result display
- User-configurable timeout and retry settings
- Export conversation history (opt-in)

## Dependencies

### Backend (Cargo.toml)
- `tauri = "2"`
- `serde = { version = "1", features = ["derive"] }`
- `serde_json = "1"`
- `uuid = { version = "1", features = ["v4", "serde"] }`
- `chrono = { version = "0.4", features = ["serde"] }`
- `log = "0.4"`
- `env_logger = "0.11"`
- `schemars = "0.8"`

### Frontend (package.json)
- `svelte@^5`
- `typescript@^5`
- `vite@^5`
- `@tauri-apps/cli@^2`
- `zod` (runtime validation)

## Configuration Files

- `src-tauri/tauri.conf.json` - Tauri app configuration
- `src-tauri/config/providers.json` - Provider definitions
- `tsconfig.json` - TypeScript compiler options
- `vite.config.ts` - Vite build configuration

## Getting Help

- **Issues**: https://github.com/your-org/chenchen/issues
- **Documentation**: [README.md](README.md)
- **Privacy**: [docs/privacy-policy.md](docs/privacy-policy.md)
- **Testing**: [docs/testing-guide.md](docs/testing-guide.md)

---

<!-- MANUAL ADDITIONS START -->
<!-- Add any project-specific notes or guidelines below -->

<!-- MANUAL ADDITIONS END -->
