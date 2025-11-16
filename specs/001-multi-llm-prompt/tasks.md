# Tasks: Multi-LLM Prompt Desktop App

**Input**: Design documents from `/specs/001-multi-llm-prompt/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/tauri-commands.md

**Tests**: Tests are MANDATORY per Constitution Principle II (Test-First). All tasks follow TDD: Write test → Watch it fail → Implement → Watch it pass.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

Based on plan.md structure:
- **Rust backend**: `src-tauri/src/`
- **Rust tests**: `src-tauri/tests/`
- **Frontend**: `src/`
- **Frontend tests**: `tests/`
- **Config**: `config/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create Tauri + Svelte project using `npm create tauri-app@latest` with TypeScript template
- [X] T002 Configure Cargo.toml with dependencies: tauri 2.x, serde, uuid, chrono, log, schemars
- [X] T003 [P] Configure package.json with dependencies: vite, typescript, vitest, @testing-library/svelte, zod
- [X] T004 [P] Configure Cargo.toml dev-dependencies: mockall, pretty_assertions, tauri (test feature)
- [X] T005 [P] Create config/providers.json with provider selector configurations (ChatGPT, Gemini, Claude)
- [X] T006 Update tauri.conf.json with app identifier "com.chenchen.app", version 0.1.0, window dimensions
- [X] T007 [P] Create src-tauri/tests/contract/ directory for library API tests
- [X] T008 [P] Create src-tauri/tests/integration/ directory for cross-library integration tests
- [X] T009 [P] Create src-tauri/tests/unit/ directory for complex logic unit tests
- [ ] T010 [P] Create tests/components/ directory for Svelte component tests
- [X] T011 [P] Setup Rust logging with env_logger in src-tauri/src/lib.rs
- [ ] T012 [P] Configure Vitest in vite.config.ts with jsdom environment

**Checkpoint**: Project structure initialized, dependencies configured, test infrastructure ready

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T013 Define ProviderId enum in src-tauri/src/types.rs (ChatGPT, Gemini, Claude)
- [X] T014 [P] Define SubmissionStatus enum in src-tauri/src/types.rs (Pending, InProgress, Retrying, Success, Failed)
- [X] T015 [P] Define SubmissionErrorType enum in src-tauri/src/types.rs (Timeout, NetworkError, AuthenticationError, etc.)
- [X] T016 [P] Define CommandError struct in src-tauri/src/types.rs with code and message fields
- [X] T017 Create AppState struct in src-tauri/src/state.rs to hold shared application state
- [X] T018 Register AppState in src-tauri/src/lib.rs with tauri::Builder::manage()
- [X] T019 Create src-tauri/src/commands.rs with empty module for Tauri command registration
- [X] T020 [P] Create TypeScript type definitions in src/types.ts mirroring Rust types using zod
- [X] T021 [P] Create src/services/tauri.ts wrapper for invoke() with type safety

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 2 - Select Target LLM Providers (Priority: P1) 🎯 MVP Foundation

**Goal**: Users can select 1-3 LLM providers (ChatGPT, Gemini, Claude) to receive prompts

**Independent Test**: Toggle provider checkboxes and verify selection state persists, cannot deselect all providers

### Tests for User Story 2 (TDD - Write FIRST)

- [X] T022 [P] [US2] Contract test for ProviderManager::new() returns 3 providers in src-tauri/tests/contract/provider_manager_test.rs
- [X] T023 [P] [US2] Contract test for ProviderManager::get_all_providers() in src-tauri/tests/contract/provider_manager_test.rs
- [X] T024 [P] [US2] Contract test for ProviderManager::update_selection() validates minimum 1 selected in src-tauri/tests/contract/provider_manager_test.rs
- [X] T025 [P] [US2] Contract test for ProviderManager::update_selection() validates maximum 3 selected in src-tauri/tests/contract/provider_manager_test.rs
- [ ] T026 [P] [US2] Component test for ProviderSelector.svelte in tests/components/ProviderSelector.test.ts

**⚠️ VERIFY**: Run `cargo test provider_manager_test` - all tests should FAIL before proceeding

### Implementation for User Story 2

- [X] T027 [US2] Create Provider struct in src-tauri/src/providers/mod.rs with id, name, url, is_selected, is_authenticated, selector_config_id
- [X] T028 [US2] Implement ProviderManager struct in src-tauri/src/providers/manager.rs with new(), get_all_providers(), update_selection()
- [X] T029 [US2] Add validation logic: prevent deselecting last provider (FR-004) in src-tauri/src/providers/manager.rs
- [X] T030 [US2] Add validation logic: prevent selecting more than 3 providers (TC-005) in src-tauri/src/providers/manager.rs
- [X] T031 [US2] Implement get_providers Tauri command in src-tauri/src/commands.rs
- [X] T032 [US2] Implement update_provider_selection Tauri command in src-tauri/src/commands.rs
- [X] T033 [US2] Register get_providers and update_provider_selection commands in src-tauri/src/lib.rs
- [X] T034 [US2] Create ProviderSelector.svelte component in src/components/ProviderSelector.svelte
- [X] T035 [US2] Add provider selection UI with checkboxes bound to state in ProviderSelector.svelte
- [X] T036 [US2] Wire up invoke("get_providers") on component mount in ProviderSelector.svelte
- [X] T037 [US2] Wire up invoke("update_provider_selection") on checkbox change in ProviderSelector.svelte
- [X] T038 [US2] Add error handling for validation errors in ProviderSelector.svelte

**⚠️ VERIFY**: Run `cargo test provider_manager_test` - all tests should PASS

**Checkpoint**: Provider selection working independently - users can toggle providers with validation

---

## Phase 4: User Story 1 (Part 1) - Layout Configuration (Priority: P1)

**Goal**: Display selected provider webviews in split-screen layout (1=full, 2=vertical split, 3=grid)

**Independent Test**: Select different provider counts and verify layout dimensions calculate correctly

### Tests for Layout Configuration (TDD - Write FIRST)

- [X] T039 [P] [US1] Unit test for calculate_layout() with 1 provider (Full) in src-tauri/tests/unit/layout_calculator_test.rs
- [X] T040 [P] [US1] Unit test for calculate_layout() with 2 providers (VerticalSplit) in src-tauri/tests/unit/layout_calculator_test.rs
- [X] T041 [P] [US1] Unit test for calculate_layout() with 3 providers (Grid) in src-tauri/tests/unit/layout_calculator_test.rs
- [ ] T042 [P] [US1] Component test for provider panel layout rendering in tests/components/ProviderPanel.test.ts

**⚠️ VERIFY**: Run `cargo test layout_calculator_test` - all tests should FAIL before proceeding

### Implementation for Layout Configuration

- [X] T043 [US1] Create LayoutType enum in src-tauri/src/layout/mod.rs (Full, VerticalSplit, Grid)
- [X] T044 [US1] Create PanelDimension struct in src-tauri/src/layout/mod.rs (provider_id, x, y, width, height)
- [X] T045 [US1] Create LayoutConfiguration struct in src-tauri/src/layout/mod.rs (provider_count, layout_type, panel_dimensions)
- [X] T046 [US1] Implement calculate_layout() in src-tauri/src/layout/calculator.rs following data-model.md specifications
- [X] T047 [US1] Implement get_layout_configuration Tauri command in src-tauri/src/commands.rs
- [X] T048 [US1] Register get_layout_configuration command in src-tauri/src/lib.rs
- [X] T049 [US1] Create ProviderPanel.svelte component in src/components/ProviderPanel.svelte
- [X] T050 [US1] Implement CSS grid/absolute positioning for panel dimensions in ProviderPanel.svelte
- [X] T051 [US1] Wire up invoke("get_layout_configuration") when provider selection changes in App.svelte
- [X] T052 [US1] Apply panel dimensions to provider panel styles in App.svelte

**⚠️ VERIFY**: Run `cargo test layout_calculator_test` - all tests should PASS

**Checkpoint**: Layout calculation working - changing provider selection updates split-screen layout

---

## Phase 5: User Story 4 (Part 1) - Webview Session Management (Priority: P2)

**Goal**: Create persistent webviews for each provider with isolated session storage

**Independent Test**: Create webview, close app, reopen app, verify session data directory persists

### Tests for Webview Sessions (TDD - Write FIRST)

- [X] T053 [P] [US4] Contract test for WebviewManager::create_webview() on Windows/Linux in src-tauri/tests/contract/webview_manager_test.rs
- [X] T054 [P] [US4] Contract test for WebviewManager::create_webview() on macOS in src-tauri/tests/contract/webview_manager_test.rs
- [ ] T055 [P] [US4] Integration test for session persistence after app restart in src-tauri/tests/integration/webview_session_test.rs

**⚠️ VERIFY**: Run `cargo test webview_manager_test` - all tests should FAIL before proceeding

### Implementation for Webview Sessions

- [X] T056 [US4] Create WebviewSession struct in src-tauri/src/webview/mod.rs (provider_id, data_directory, data_store_identifier, is_persistent, last_activity)
- [X] T057 [US4] Implement WebviewManager struct in src-tauri/src/webview/manager.rs with create_webview() method
- [X] T058 [US4] Implement data_directory configuration for Windows/Linux in src-tauri/src/webview/manager.rs using app_local_data_dir()
- [X] T059 [US4] Implement data_store_identifier configuration for macOS in src-tauri/src/webview/manager.rs with UUID persistence
- [X] T060 [US4] Add platform-specific compilation flags (#[cfg(target_os)]) in src-tauri/src/webview/manager.rs
- [X] T061 [US4] Create WebviewInfo struct in src-tauri/src/webview/mod.rs (provider_id, label, url, is_persistent, data_path, data_store_id)
- [ ] T062 [US4] Implement create_provider_webview Tauri command in src-tauri/src/commands.rs
- [ ] T063 [US4] Register create_provider_webview command in src-tauri/src/lib.rs
- [ ] T064 [US4] Wire up invoke("create_provider_webview") when user selects provider in ProviderSelector.svelte

**⚠️ VERIFY**: Run `cargo test webview_manager_test` - all tests should PASS

**Checkpoint**: Webviews create with persistent sessions - session data survives app restarts

---

## Phase 6: User Story 1 (Part 2) - Provider Configuration Loading (Priority: P1)

**Goal**: Load CSS selectors and DOM identifiers from config/providers.json for JavaScript injection

**Independent Test**: Load provider config and verify selectors are available for all 3 providers

### Tests for Provider Configuration (TDD - Write FIRST)

- [ ] T065 [P] [US1] Contract test for ProviderConfig::load() parses JSON successfully in src-tauri/tests/contract/provider_config_test.rs
- [ ] T066 [P] [US1] Contract test for ProviderConfig::get_input_selectors() returns non-empty array in src-tauri/tests/contract/provider_config_test.rs
- [ ] T067 [P] [US1] Unit test for config validation (version semver format) in src-tauri/tests/unit/config_validation_test.rs

**⚠️ VERIFY**: Run `cargo test provider_config_test` - all tests should FAIL before proceeding

### Implementation for Provider Configuration

- [X] T068 [US1] Create ProviderSelectorConfig struct in src-tauri/src/providers/config.rs (provider_id, version, input_selectors, submit_selectors, auth_check_selectors, last_updated)
- [X] T069 [US1] Implement load() method to read config/providers.json in src-tauri/src/providers/config.rs
- [X] T070 [US1] Add validation for semver version format in src-tauri/src/providers/config.rs
- [X] T071 [US1] Add validation for non-empty selector arrays in src-tauri/src/providers/config.rs
- [X] T072 [US1] Cache loaded configs in AppState in src-tauri/src/state.rs
- [X] T073 [US1] Load configs during app initialization in src-tauri/src/lib.rs

**⚠️ VERIFY**: Run `cargo test provider_config_test` - all tests should PASS

**Checkpoint**: Provider configs load from JSON - selectors available for injection logic

---

## Phase 7: User Story 1 (Part 3) - JavaScript Injection Engine (Priority: P1)

**Goal**: Generate and execute JavaScript to find input elements, set prompt value, trigger submit

**Independent Test**: Generate injection script and verify it contains correct selector logic and functions

### Tests for JavaScript Injection (TDD - Write FIRST)

- [X] T074 [P] [US1] Unit test for generate_injection_script() includes input selector logic in src-tauri/tests/unit/script_generation_test.rs
- [X] T075 [P] [US1] Unit test for generate_injection_script() includes submit button trigger in src-tauri/tests/unit/script_generation_test.rs
- [ ] T076 [P] [US1] Integration test for script execution in jsdom mock environment in tests/integration/injection_execution.test.ts
- [X] T077 [P] [US1] Contract test for Injector::execute() returns success/failure status in src-tauri/tests/contract/injector_test.rs

**⚠️ VERIFY**: Run `cargo test script_generation_test && npm test injection_execution` - all tests should FAIL

### Implementation for JavaScript Injection

- [X] T078 [US1] Create Injector struct in src-tauri/src/injection/injector.rs with execute() method
- [X] T079 [US1] Implement generate_injection_script() in src-tauri/src/injection/script_builder.rs
- [X] T080 [US1] Add selector iteration logic (try each selector in array until element found) in src-tauri/src/injection/script_builder.rs
- [X] T081 [US1] Add input value setting logic in src-tauri/src/injection/script_builder.rs
- [X] T082 [US1] Add submit button click trigger logic in src-tauri/src/injection/script_builder.rs
- [X] T083 [US1] Add element-not-found error reporting in src-tauri/src/injection/script_builder.rs
- [ ] T084 [US1] Implement execute() to call webview.eval() with generated script in src-tauri/src/injection/injector.rs
- [ ] T085 [US1] Add timeout handling (30 seconds per FR-007) in src-tauri/src/injection/injector.rs

**⚠️ VERIFY**: Run `cargo test script_generation_test` - all tests should PASS

**Checkpoint**: JavaScript injection working - scripts generate correctly with selector logic

---

## Phase 8: User Story 3 - Submission Status Tracking (Priority: P1)

**Goal**: Track submission state per provider (Pending → InProgress → Success/Failed/Retrying)

**Independent Test**: Create submission and verify state transitions follow state machine rules from data-model.md

### Tests for Status Tracking (TDD - Write FIRST)

- [X] T086 [P] [US3] Unit test for Submission state machine: Pending → InProgress in src-tauri/tests/unit/status_transitions_test.rs
- [X] T087 [P] [US3] Unit test for Submission state machine: InProgress → Success in src-tauri/tests/unit/status_transitions_test.rs
- [X] T088 [P] [US3] Unit test for Submission state machine: InProgress → Retrying (on Timeout) in src-tauri/tests/unit/status_transitions_test.rs
- [X] T089 [P] [US3] Unit test for Submission state machine: Retrying → Failed in src-tauri/tests/unit/status_transitions_test.rs
- [X] T090 [P] [US3] Unit test for Submission state machine: InProgress → Failed (on AuthError, no retry) in src-tauri/tests/unit/status_transitions_test.rs
- [X] T091 [P] [US3] Unit test for timeout detection (now() - started_at > 30s) in src-tauri/tests/unit/timeout_logic_test.rs
- [ ] T092 [P] [US3] Contract test for StatusTracker::create_submission() in src-tauri/tests/contract/status_tracker_test.rs
- [ ] T093 [P] [US3] Contract test for StatusTracker::update_status() emits events in src-tauri/tests/contract/status_tracker_test.rs

**⚠️ VERIFY**: Run `cargo test status_transitions_test && cargo test timeout_logic_test` - all tests should FAIL

### Implementation for Status Tracking

- [X] T094 [US3] Create Submission struct in src-tauri/src/status/mod.rs (id, provider_id, prompt_content, status, attempt_count, error_type, error_message, started_at, completed_at)
- [X] T095 [US3] Implement StatusTracker struct in src-tauri/src/status/tracker.rs with create_submission(), get_status(), update_status()
- [X] T096 [US3] Implement state transition validation in src-tauri/src/status/tracker.rs per data-model.md state machine
- [X] T097 [US3] Add timeout detection logic (30-second check) in src-tauri/src/status/tracker.rs
- [X] T098 [US3] Add retry logic: Timeout/NetworkError → increment attempt_count, transition to Retrying in src-tauri/src/status/tracker.rs
- [X] T099 [US3] Add no-retry logic: AuthError/RateLimitError → transition directly to Failed in src-tauri/src/status/tracker.rs
- [ ] T100 [US3] Implement event emission (submission_status_changed) in src-tauri/src/status/tracker.rs
- [X] T101 [US3] Implement get_submission_status Tauri command in src-tauri/src/commands.rs
- [X] T102 [US3] Register get_submission_status command in src-tauri/src/lib.rs

**⚠️ VERIFY**: Run `cargo test status_transitions_test && cargo test timeout_logic_test` - all tests should PASS

**Checkpoint**: Status tracking working - submissions transition through states correctly with timeout and retry logic

---

## Phase 9: User Story 1 (Part 4) - Prompt Submission Integration (Priority: P1) 🎯 MVP Core

**Goal**: Submit prompt to all selected providers, trigger injection, track status, display results

**Independent Test**: Type prompt, select providers, click send, verify injection executes and status updates display

### Tests for Prompt Submission (TDD - Write FIRST)

- [ ] T103 [P] [US1] Integration test for full submission flow (validation → injection → status) in src-tauri/tests/integration/submission_flow_test.rs
- [ ] T104 [P] [US1] Contract test for submit_prompt command creates N submissions (N = selected providers) in src-tauri/tests/contract/submit_prompt_test.rs
- [ ] T105 [P] [US1] Component test for PromptInput.svelte validates non-empty prompt in tests/components/PromptInput.test.ts
- [ ] T106 [P] [US1] Component test for StatusDisplay.svelte renders real-time status updates in tests/components/StatusDisplay.test.ts

**⚠️ VERIFY**: Run `cargo test submission_flow_test && npm test PromptInput` - all tests should FAIL

### Implementation for Prompt Submission

- [X] T107 [US1] Create Prompt struct in src-tauri/src/status/mod.rs (content, timestamp)
- [X] T108 [US1] Implement submit_prompt Tauri command in src-tauri/src/commands.rs
- [X] T109 [US1] Add validation: non-empty prompt and at least 1 provider selected in src-tauri/src/commands.rs
- [X] T110 [US1] Create Submission entities for each selected provider in submit_prompt command
- [ ] T111 [US1] Spawn async tasks for each submission (concurrent execution) in submit_prompt command
- [ ] T112 [US1] Call Injector::execute() for each provider in async task
- [ ] T113 [US1] Update Submission status based on injection result in async task
- [ ] T114 [US1] Emit submission_status_changed event after each status update
- [X] T115 [US1] Register submit_prompt command in src-tauri/src/lib.rs
- [X] T116 [US1] Create PromptInput.svelte component in src/components/PromptInput.svelte
- [X] T117 [US1] Add textarea with prompt binding in PromptInput.svelte
- [X] T118 [US1] Add send button with invoke("submit_prompt") in PromptInput.svelte
- [X] T119 [US1] Add validation error display in PromptInput.svelte
- [X] T120 [US1] Create StatusDisplay.svelte component in src/components/StatusDisplay.svelte
- [ ] T121 [US1] Wire up listen("submission_status_changed") event listener in StatusDisplay.svelte
- [X] T122 [US1] Display status for each provider (Pending, InProgress, Retrying, Success, Failed) in StatusDisplay.svelte
- [X] T123 [US1] Display error messages for failed submissions in StatusDisplay.svelte
- [X] T124 [US1] Integrate PromptInput, ProviderSelector, ProviderPanel, StatusDisplay in src/App.svelte
- [ ] T124a [US1] Integration test for end-to-end submission timing: prompt to all 3 providers completes <10s in src-tauri/tests/integration/submission_timing_test.rs

**⚠️ VERIFY**: Run `cargo test submission_flow_test && npm test` - all tests should PASS

**Checkpoint**: MVP COMPLETE - Users can send prompts to multiple LLMs and see responses in split-screen layout

---

## Phase 10: User Story 4 (Part 2) - Authentication Detection (Priority: P2)

**Goal**: Detect when provider session is unauthenticated and prompt user to log in

**Independent Test**: Create webview, check authentication status, verify auth_check_selectors detect login requirement

### Tests for Authentication Detection (TDD - Write FIRST)

- [X] T125 [P] [US4] Contract test for check_authentication() returns AuthenticationStatus in src-tauri/tests/contract/auth_check_test.rs
- [ ] T126 [P] [US4] Integration test for auth_check_selectors detection (mock DOM with login button) in src-tauri/tests/integration/auth_detection_test.rs

**⚠️ VERIFY**: Run `cargo test auth_check_test` - all tests should FAIL

### Implementation for Authentication Detection

- [X] T127 [US4] Create AuthenticationStatus struct in src-tauri/src/webview/mod.rs (provider_id, is_authenticated, last_checked, requires_login)
- [X] T128 [US4] Implement check_authentication() in src-tauri/src/webview/manager.rs
- [ ] T129 [US4] Inject JavaScript to check for auth_check_selectors presence in check_authentication()
- [ ] T130 [US4] Return is_authenticated=false if auth selectors found
- [ ] T131 [US4] Return is_authenticated=true if auth selectors not found
- [ ] T132 [US4] Implement check_authentication Tauri command in src-tauri/src/commands.rs
- [ ] T133 [US4] Register check_authentication command in src-tauri/src/lib.rs
- [ ] T134 [US4] Call check_authentication before prompt submission in ProviderSelector.svelte
- [ ] T135 [US4] Display "Login Required" message if requires_login=true in ProviderSelector.svelte
- [ ] T136 [US4] Show provider webview for manual login in ProviderSelector.svelte

**⚠️ VERIFY**: Run `cargo test auth_check_test` - all tests should PASS

**Checkpoint**: Authentication detection working - app detects when login required and prompts user

---

## Phase 11: User Story 5 - Privacy and Local Data (Priority: P2)

**Goal**: Ensure no credentials stored, no telemetry, all data stays local

**Independent Test**: Monitor network traffic and verify no data sent to non-provider servers, verify no credentials in app storage

### Tests for Privacy (TDD - Write FIRST)

- [X] T137 [P] [US5] Integration test verifies no credential storage in app data directory in src-tauri/tests/integration/privacy_test.rs
- [X] T138 [P] [US5] Integration test verifies no prompt history retained after app restart in src-tauri/tests/integration/privacy_test.rs
- [ ] T139 [P] [US5] Manual test: Monitor network traffic with Wireshark/Charles during prompt submission in docs/testing-guide.md

**⚠️ VERIFY**: Run `cargo test privacy_test` - tests should FAIL until privacy guarantees implemented

### Implementation for Privacy Guarantees

- [X] T140 [US5] Audit all file I/O operations to verify no credential writes in src-tauri/src/
- [X] T141 [US5] Audit all network operations to verify only provider URLs contacted in src-tauri/src/
- [X] T142 [US5] Remove any prompt history/logging code in src-tauri/src/
- [X] T143 [US5] Document data handling: only session cookies managed by platform webview (FR-012) in docs/privacy-policy.md
- [X] T144 [US5] Document network usage: only LLM provider websites contacted (FR-013) in docs/privacy-policy.md
- [X] T144a [P] [US5] Create integration test to measure submission success rate over 20 attempts in src-tauri/tests/integration/success_rate_test.rs

**⚠️ VERIFY**: Run `cargo test privacy_test` - all tests should PASS

**Checkpoint**: Privacy guarantees validated - no credentials stored, no telemetry, local-only operation

---

## Phase 12: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T145 [P] Add structured logging for all Tauri commands in src-tauri/src/commands.rs
- [X] T146 [P] Add structured logging for injection attempts (element found/not found) in src-tauri/src/injection/injector.rs
- [X] T147 [P] Add structured logging for timeout events with timestamps in src-tauri/src/status/tracker.rs
- [X] T147a [P] Verify structured logging outputs both JSON (machine-readable) and human-readable formats per Constitution Principle IV in src-tauri/tests/integration/logging_format_test.rs
- [X] T148 [P] Refactor error handling: consistent CommandError usage across all commands in src-tauri/src/commands.rs
- [X] T149 [P] Add CSS styling for PromptInput, ProviderSelector, StatusDisplay components in src/components/
- [X] T150 [P] Add responsive design for split-screen layout enforcing TC-008 minimum window size in src/App.svelte
- [X] T151 [P] Add loading spinners for in-progress submissions in StatusDisplay.svelte
- [ ] T152 [P] Performance: Optimize layout recalculation with debouncing in App.svelte
- [ ] T153 [P] Documentation: Update README.md with quickstart instructions
- [X] T154 [P] Documentation: Create CLAUDE.md with active technologies and project structure
- [ ] T155 Run quickstart.md validation: cargo test && npm test
- [ ] T156 Run quickstart.md validation: npm run tauri dev (verify app starts <3s)
- [X] T157 [P] Security: Audit for XSS vulnerabilities in JavaScript injection code
- [X] T158 [P] Security: Audit for command injection vulnerabilities
- [ ] T159 Build production binary: npm run tauri build
- [ ] T160 Verify binary size <15MB (TC-004)
- [ ] T160a Validate SC-002: Run success_rate_test and verify >=95% success rate with valid sessions
- [ ] T160b Validate SC-001: Run submission_timing_test and verify <10 second submission to 3 LLMs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - US2 (Provider Selection) → No dependencies after Foundational
  - US1 Part 1 (Layout) → Depends on US2 (needs selected providers)
  - US4 Part 1 (Webview) → No dependencies after Foundational (parallel with US2)
  - US1 Part 2 (Config) → No dependencies after Foundational (parallel with US2, US4)
  - US1 Part 3 (Injection) → Depends on US1 Part 2 (needs configs)
  - US3 (Status) → No dependencies after Foundational (parallel with US1 Part 3)
  - US1 Part 4 (Submission) → Depends on US2, US1 Parts 1-3, US3, US4 Part 1 (MVP integration)
  - US4 Part 2 (Auth Detection) → Depends on US4 Part 1 and US1 Part 3
  - US5 (Privacy) → No dependencies (can be done in parallel, but verify at end)
- **Polish (Phase 12)**: Depends on all desired user stories being complete

### Critical Path to MVP

1. Phase 1: Setup → Phase 2: Foundational
2. Phase 3: US2 (Provider Selection) ← **MUST COMPLETE FIRST**
3. Phase 4: US1 Part 1 (Layout)
4. Phase 5: US4 Part 1 (Webview Sessions)
5. Phase 6: US1 Part 2 (Provider Config)
6. Phase 7: US1 Part 3 (JavaScript Injection)
7. Phase 8: US3 (Status Tracking)
8. Phase 9: US1 Part 4 (Prompt Submission) ← **MVP COMPLETE HERE**

### Parallel Opportunities

**After Foundational (Phase 2):**
- US2 (Provider Selection) + US4 Part 1 (Webview) + US1 Part 2 (Config) can run in parallel

**After Provider Selection (Phase 3):**
- US1 Part 1 (Layout) + US1 Part 3 (Injection after Part 2) + US3 (Status) can run in parallel

**Within Each Phase:**
- All tasks marked [P] can run in parallel
- All test tasks within a story can run in parallel

---

## Parallel Example: Phase 2 (Foundational)

```bash
# Launch all type definition tasks together:
Task: "Define ProviderId enum in src-tauri/src/types.rs"
Task: "Define SubmissionStatus enum in src-tauri/src/types.rs"
Task: "Define SubmissionErrorType enum in src-tauri/src/types.rs"
Task: "Define CommandError struct in src-tauri/src/types.rs"
Task: "Create TypeScript type definitions in src/types.ts"
Task: "Create src/services/tauri.ts wrapper"
```

---

## Implementation Strategy

### MVP First (User Stories 1, 2, 3 - P1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: US2 Provider Selection
4. Complete Phase 4: US1 Part 1 Layout
5. Complete Phase 5: US4 Part 1 Webview (for manual testing)
6. Complete Phase 6: US1 Part 2 Config
7. Complete Phase 7: US1 Part 3 Injection
8. Complete Phase 8: US3 Status Tracking
9. Complete Phase 9: US1 Part 4 Submission Integration
10. **STOP and VALIDATE**: Test full prompt submission flow
11. Demo MVP: Send prompt to 3 LLMs, see responses in split-screen

### Incremental Delivery

1. Foundation (Phase 1-2) → Structure ready
2. Add US2 → Test provider selection independently
3. Add US1 Layout → Test split-screen arrangement
4. Add US4 Webviews → Test persistent sessions
5. Add US1 Config + Injection + US3 Status → Test injection and tracking
6. Add US1 Submission → **MVP READY** (send prompts, see results)
7. Add US4 Auth Detection → Enhanced login UX
8. Add US5 Privacy Validation → Trust and compliance
9. Polish → Production-ready

### Parallel Team Strategy

With 3 developers after Foundational phase:

- **Developer A**: US2 (Provider Selection) → US1 Part 1 (Layout) → US1 Part 4 (Integration)
- **Developer B**: US4 Part 1 (Webview) → US1 Part 2 (Config) → US1 Part 3 (Injection)
- **Developer C**: US3 (Status Tracking) → US4 Part 2 (Auth) → US5 (Privacy)

Synchronization points:
- After US2: Developer A provides provider selection for layout
- Before US1 Part 4: All developers sync for integration testing

---

## TDD Enforcement

**Constitution Principle II (NON-NEGOTIABLE)**: Tests MUST be written first.

### Red-Green-Refactor Checklist Per Task

1. **RED**: Write test for task, run test suite, verify test FAILS
2. **GREEN**: Implement minimal code to pass test, run test suite, verify test PASSES
3. **REFACTOR**: Improve code quality while keeping tests green

### Test Coverage Requirements

- All Rust libraries: Contract tests + unit tests for complex logic
- All Tauri commands: Contract tests validating IPC interface
- All Svelte components: Component tests with @testing-library/svelte
- Critical paths: Integration tests (submission flow, session persistence)

### Test Execution Frequency

- Run unit tests: After every task implementation
- Run integration tests: After each phase completion
- Run E2E tests: Before MVP validation checkpoint
- Run all tests: Before commit/push

---

## Notes

- [P] tasks = different files, no dependencies → maximize parallel execution
- [Story] label maps task to specific user story for traceability
- Each phase has a checkpoint: validate independently before proceeding
- TDD is mandatory: RED (failing test) → GREEN (passing test) → REFACTOR
- Tests are not optional: Constitution Principle II is NON-NEGOTIABLE
- Commit after passing tests for each task or logical group
- MVP stops at Phase 9: full prompt submission with status tracking
- Post-MVP adds authentication detection (US4 Part 2) and privacy validation (US5)
- Avoid: skipping tests, implementing before tests written, cross-story dependencies that break independence

---

## Success Criteria (from spec.md)

Upon completion of all tasks:

- ✅ SC-001: Send prompt to 3 LLMs in <10 seconds (vs 2+ minutes manual)
- ✅ SC-002: 95% submission success rate with valid sessions
- ✅ SC-003: Status updates visible within 2 seconds
- ✅ SC-004: Zero credentials stored by app
- ✅ SC-005: App startup <3 seconds
- ✅ SC-006: Switch provider combinations without restart
- ✅ SC-007: All 3 provider responses visible simultaneously in split-screen
- ✅ SC-008: Provider panels clearly labeled for identification <1 second

**Validation**: Run quickstart.md scenarios to verify all success criteria met.
