# Feature Specification: Multi-LLM Prompt Desktop App

**Feature Branch**: `001-multi-llm-prompt`
**Created**: 2025-11-12
**Status**: Draft
**Input**: User description: "A PC desktop app with a single prompt box where users pick ChatGPT/Gemini/Claude, use their existing logged-in sessions (no APIs/keys) to paste & send the prompt to each site, see per-site status, and keep everything local with no credential storage"

## Clarifications

### Session 2025-11-12

- Q: Which technical approach should be used for browser integration (automation, Electron webview, browser extension, or system browser)? → A: Tauri 2.0 with embedded webview
- Q: How should session/authentication work given Tauri webviews are isolated from system browsers? → A: Require users to log in within embedded webview, sessions persist per app
- Q: How should provider responses be displayed (multiple windows, tabbed interface, external browser, or split layout)? → A: Single window with grid/split layout showing all selected providers simultaneously, limited to max 3 providers
- Q: How should the app inject the user's prompt into each provider's chat interface within the embedded webview? → A: JavaScript injection to find textarea/input, set value, trigger submit button
- Q: What should be the timeout and retry behavior for failed prompt submissions? → A: 30 second timeout per provider, 1 automatic retry on timeout/network error, no retry on auth/rate-limit

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Send Same Prompt to Multiple LLMs (Priority: P1)

A user wants to compare responses from different LLM providers (ChatGPT, Gemini, Claude) for the same prompt without manually navigating to each website, copying the prompt, and pasting it multiple times.

**Why this priority**: This is the core value proposition - enabling efficient multi-LLM prompt submission. Without this, the app has no purpose.

**Independent Test**: Can be fully tested by typing a prompt, selecting one or more LLM providers, clicking send, and verifying the prompt appears in each selected provider's web interface. Delivers immediate value by eliminating manual copy-paste workflows.

**Acceptance Scenarios**:

1. **Given** the app is open and at least one LLM provider is selected, **When** the user types a prompt and clicks send, **Then** the prompt is automatically submitted to all selected LLM providers
2. **Given** the user has selected multiple LLM providers, **When** the send action completes, **Then** the app window displays a split-screen layout with one webview panel per selected provider showing their responses simultaneously
3. **Given** the user has selected 1 provider, **When** responses load, **Then** the single provider occupies the full response area
4. **Given** the user has selected 2 providers, **When** responses load, **Then** providers are arranged side by side in a vertical split
5. **Given** the user has selected 3 providers, **When** responses load, **Then** providers are arranged in a grid layout (e.g., 2 top, 1 bottom, or 3 columns)
6. **Given** a prompt submission is in progress, **When** the user views the status area, **Then** they see real-time status for each provider (e.g., "sending", "success", "failed")

---

### User Story 2 - Select Target LLM Providers (Priority: P1)

A user wants to choose which LLM providers should receive their prompt, allowing flexibility to query just one, two, or all three providers depending on their needs.

**Why this priority**: Provider selection is essential for the core workflow. Users must be able to control which LLMs receive prompts to avoid unnecessary operations or rate limiting.

**Independent Test**: Can be tested by toggling provider checkboxes/buttons and verifying only selected providers receive the prompt. Delivers value by giving users control over their workflow.

**Acceptance Scenarios**:

1. **Given** the app is open, **When** the user views the provider selection area, **Then** they see options to select ChatGPT, Gemini, and Claude
2. **Given** provider selection controls are visible, **When** the user toggles a provider on or off, **Then** the selection state is clearly indicated visually
3. **Given** no providers are selected, **When** the user attempts to send a prompt, **Then** the system prevents submission and displays a message indicating at least one provider must be selected

---

### User Story 3 - View Per-Provider Status (Priority: P1)

A user wants to see the current status of their prompt submission for each LLM provider to understand which operations succeeded, which are in progress, and which failed.

**Why this priority**: Status feedback is critical for user confidence and error recovery. Without it, users have no visibility into whether their prompts were successfully submitted.

**Independent Test**: Can be tested by sending a prompt and observing status indicators for each provider. Delivers value by providing transparency and enabling troubleshooting.

**Acceptance Scenarios**:

1. **Given** a prompt has been submitted to multiple providers, **When** each provider operation completes, **Then** the status area shows success or failure for each provider
2. **Given** a provider submission has failed, **When** the user views the status, **Then** they see a clear error message explaining the failure (e.g., "Not logged in", "Connection timeout")
3. **Given** submissions are in progress, **When** the user views the status area, **Then** they see which providers are still processing

---

### User Story 4 - Maintain Persistent Sessions Without API Keys (Priority: P2)

A user wants to log into ChatGPT, Gemini, and Claude once within the app and have those sessions persist across app restarts, without requiring API keys or storing credentials.

**Why this priority**: This is a key security and convenience feature - one-time login with persistent sessions eliminates repeated authentication while avoiding credential storage. However, the app could function requiring login each time if this feature is deferred.

**Independent Test**: Can be tested by logging into LLM providers within the app's embedded webview, closing and reopening the app, then verifying prompt submission works without re-authentication. Delivers value by streamlining the user experience while maintaining security.

**Acceptance Scenarios**:

1. **Given** the user is logged into a provider within the app's webview, **When** the app is closed and reopened, **Then** the provider session remains active without requiring re-login
2. **Given** the user is not logged into a selected provider, **When** the app attempts to send a prompt, **Then** the status indicates authentication is required and the app displays the provider's login page in the embedded webview
3. **Given** a provider session expires, **When** the app attempts to use that session, **Then** the status indicates session expiration and displays the provider's login page for re-authentication

---

### User Story 5 - Keep Everything Local (Priority: P2)

A privacy-conscious user wants assurance that their prompts, credentials, and usage data remain entirely on their local machine with no cloud storage or telemetry.

**Why this priority**: Privacy and security are important but don't block basic functionality. The app can work without explicit privacy features, but they add significant value for trust and compliance.

**Independent Test**: Can be tested by monitoring network traffic and file system access to verify no data is sent to third-party servers (except the LLM providers themselves). Delivers value by ensuring user privacy and data sovereignty.

**Acceptance Scenarios**:

1. **Given** the app is in use, **When** network traffic is monitored, **Then** no data is sent to servers other than the selected LLM provider websites
2. **Given** the user has submitted prompts, **When** the app is closed and reopened, **Then** no prompt history is retained unless explicitly saved by the user
3. **Given** the app accesses browser sessions, **When** checking stored data, **Then** no credentials or session tokens are stored by the app

---

### Edge Cases

- What happens when the user's browser session expires while a prompt is being sent? (Status shows auth failure, no automatic retry per FR-009)
- What happens if a user selects a provider they are not logged into? (Status indicates auth required, displays login page per User Story 4)
- How does the app behave if the user modifies the prompt while a submission is in progress?
- What happens when an LLM provider's website DOM structure changes (breaking the JavaScript selectors used for prompt injection)? (Selector config needs update per TC-007)
- How should the app detect JavaScript injection failures (element not found, submit failed)? (Timeout triggers after 30s, one retry per FR-007/FR-008)
- What happens if the user closes the app while submissions are in progress?
- How does the app handle screen size limitations when displaying split layouts (minimum window size)?
- What happens if a user sends a new prompt while previous provider responses are still visible in the split layout?
- How does the split layout reconfigure if user changes provider selection mid-session?
- How should the status UI differentiate between first attempt vs retry attempt?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a single text input area for users to compose prompts
- **FR-002**: System MUST provide selection controls for ChatGPT, Gemini, and Claude
- **FR-003**: System MUST allow users to select between 1 and 3 LLM providers before sending
- **FR-004**: System MUST prevent prompt submission if no providers are selected
- **FR-005**: System MUST inject and submit the prompt to all selected LLM providers via JavaScript DOM manipulation when the user initiates send
- **FR-006**: System MUST display real-time status for each provider (pending, in progress, success, failed)
- **FR-007**: System MUST apply a 30-second timeout to each provider submission attempt
- **FR-008**: System MUST automatically retry once on timeout or network errors, updating status to show retry attempt
- **FR-009**: System MUST NOT automatically retry on authentication failures or rate-limiting errors
- **FR-010**: System MUST provide error messages when a submission fails after all retry attempts
- **FR-011**: System MUST maintain persistent webview sessions within the app that survive app restarts, allowing users to log in once per provider
- **FR-012**: System MUST NOT store user credentials (passwords, API keys) locally; only session cookies managed by the webview
- **FR-013**: System MUST NOT send user data to any servers except the selected LLM provider websites
- **FR-014**: System MUST handle authentication failures gracefully by notifying the user
- **FR-015**: System MUST run as a standalone desktop application on Windows, macOS, and Linux
- **FR-016**: System MUST display selected provider responses in a split-screen layout within a single app window
- **FR-017**: System MUST arrange provider webview panels according to selection count: 1 provider (full width), 2 providers (vertical split), 3 providers (grid layout)
- **FR-018**: System MUST label each provider panel clearly to identify which LLM is being displayed
- **FR-019**: System MUST detect when browser sessions are unavailable or expired
- **FR-020**: System MUST operate entirely offline except for communication with LLM provider websites

### Key Entities

- **Prompt**: The text input provided by the user to be sent to LLM providers
- **Provider**: An LLM service (ChatGPT, Gemini, or Claude) that can receive and respond to prompts
- **Provider Selection**: The user's choice of which providers should receive the prompt
- **Submission Status**: The current state of a prompt submission to a specific provider (pending, in progress, retrying, success, failed)
- **Browser Session**: An authenticated session within the app's embedded webview used to access an LLM provider
- **Provider Selector Configuration**: A maintained set of CSS selectors and aria-labels for locating chat input elements and submit buttons on each provider's website

### Technical Constraints

- **TC-001**: Application MUST be built using Tauri 2.0 framework with Rust backend
- **TC-002**: Browser integration MUST use Tauri's embedded webview capabilities (WebView2 on Windows, WebKit on macOS, WebKitGTK on Linux)
- **TC-003**: Session handling MUST leverage native webview session storage with persistent cookies in Tauri's app data directory
- **TC-004**: Application binary size SHOULD remain under 15MB (excluding system webview dependencies)
- **TC-005**: Provider selection MUST be limited to a maximum of 3 simultaneous providers to ensure usable split-screen comparison on standard displays
- **TC-006**: Prompt injection MUST use JavaScript injection via Tauri's webview eval API to locate chat input elements (textarea/input) using provider-specific CSS selectors or aria-labels, set values, and programmatically trigger submit actions
- **TC-007**: Each provider MUST have a maintained selector configuration to handle provider website updates without requiring app recompilation

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can send a prompt to three LLM providers in under 10 seconds, compared to 2+ minutes required for manual copy-paste workflow
- **SC-002**: 95% of prompt submissions succeed when the user has valid active sessions with the selected providers
- **SC-003**: Users can view status updates for all providers within 2 seconds of submission
- **SC-004**: Zero user credentials are stored by the application at any time
- **SC-005**: The application completes startup and is ready for use within 3 seconds of launch
- **SC-006**: Users can successfully switch between different provider combinations without restarting the app
- **SC-007**: All selected provider responses (up to 3) are visible simultaneously in an automatic split-screen layout without requiring manual window management
- **SC-008**: Each provider panel is clearly labeled, allowing users to identify which LLM response they are viewing within 1 second
