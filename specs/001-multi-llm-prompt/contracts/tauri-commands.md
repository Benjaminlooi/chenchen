# Tauri Commands API Contract

**Branch**: `001-multi-llm-prompt` | **Date**: 2025-11-13
**Phase**: 1 (Design & Contracts)

## Overview

This document defines the Tauri IPC command interface between the Rust backend and frontend. All commands follow Tauri 2.0 conventions and use JSON serialization via serde.

---

## Command List

1. [get_providers](#1-get_providers) - Retrieve available LLM providers
2. [update_provider_selection](#2-update_provider_selection) - Toggle provider selection
3. [submit_prompt](#3-submit_prompt) - Submit prompt to selected providers
4. [get_submission_status](#4-get_submission_status) - Query submission status
5. [create_provider_webview](#5-create_provider_webview) - Create authenticated webview for provider
6. [check_authentication](#6-check_authentication) - Check if provider session is authenticated
7. [get_layout_configuration](#7-get_layout_configuration) - Calculate split-screen layout

---

## 1. get_providers

**Description:** Retrieve all available LLM providers and their current state.

**Command:**
```rust
#[tauri::command]
async fn get_providers(state: State<'_, AppState>) -> Result<Vec<Provider>, CommandError>
```

**Request:**
- No parameters

**Response:**
```typescript
interface Provider {
  id: "ChatGPT" | "Gemini" | "Claude";
  name: string;
  url: string;
  is_selected: boolean;
  is_authenticated: boolean;
  selector_config_id: string;
}
```

**Example Response:**
```json
[
  {
    "id": "ChatGPT",
    "name": "ChatGPT",
    "url": "https://chat.openai.com/",
    "is_selected": false,
    "is_authenticated": false,
    "selector_config_id": "chatgpt-v1"
  },
  {
    "id": "Gemini",
    "name": "Gemini",
    "url": "https://gemini.google.com/",
    "is_selected": false,
    "is_authenticated": false,
    "selector_config_id": "gemini-v1"
  },
  {
    "id": "Claude",
    "name": "Claude",
    "url": "https://claude.ai/",
    "is_selected": false,
    "is_authenticated": false,
    "selector_config_id": "claude-v1"
  }
]
```

**Errors:**
- `InternalError`: Failed to read provider state

**Frontend Usage:**
```typescript
import { invoke } from "@tauri-apps/api/core";

const providers = await invoke<Provider[]>("get_providers");
```

---

## 2. update_provider_selection

**Description:** Toggle provider selection state. Validates that at least one provider remains selected (FR-004).

**Command:**
```rust
#[tauri::command]
async fn update_provider_selection(
    provider_id: String,
    is_selected: bool,
    state: State<'_, AppState>
) -> Result<Provider, CommandError>
```

**Request:**
```typescript
interface UpdateProviderSelectionRequest {
  provider_id: "ChatGPT" | "Gemini" | "Claude";
  is_selected: boolean;
}
```

**Response:**
```typescript
interface Provider {
  // Same as get_providers
}
```

**Example Request:**
```json
{
  "provider_id": "ChatGPT",
  "is_selected": true
}
```

**Example Response:**
```json
{
  "id": "ChatGPT",
  "name": "ChatGPT",
  "url": "https://chat.openai.com/",
  "is_selected": true,
  "is_authenticated": false,
  "selector_config_id": "chatgpt-v1"
}
```

**Errors:**
- `ValidationError`: Cannot deselect last provider (must have at least one selected)
- `NotFound`: Invalid provider_id
- `MaxProvidersExceeded`: Cannot select more than 3 providers (TC-005)

**Frontend Usage:**
```typescript
try {
  const provider = await invoke<Provider>("update_provider_selection", {
    provider_id: "ChatGPT",
    is_selected: true
  });
} catch (error) {
  if (error === "ValidationError") {
    alert("At least one provider must be selected");
  }
}
```

---

## 3. submit_prompt

**Description:** Submit prompt to all selected providers. Creates Submission entities and initiates JavaScript injection.

**Command:**
```rust
#[tauri::command]
async fn submit_prompt(
    prompt: String,
    app_handle: AppHandle,
    state: State<'_, AppState>
) -> Result<SubmitPromptResponse, CommandError>
```

**Request:**
```typescript
interface SubmitPromptRequest {
  prompt: string;
}
```

**Response:**
```typescript
interface SubmitPromptResponse {
  prompt_id: string;  // UUID
  submissions: Submission[];
}

interface Submission {
  id: string;  // UUID
  provider_id: "ChatGPT" | "Gemini" | "Claude";
  prompt_content: string;
  status: "Pending" | "InProgress" | "Retrying" | "Success" | "Failed";
  attempt_count: number;
  error_type?: "Timeout" | "NetworkError" | "AuthenticationError" | "RateLimitError" | "ElementNotFound" | "InjectionFailed";
  error_message?: string;
  started_at?: string;  // ISO 8601
  completed_at?: string;  // ISO 8601
}
```

**Example Request:**
```json
{
  "prompt": "Explain quantum computing in simple terms"
}
```

**Example Response:**
```json
{
  "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
  "submissions": [
    {
      "id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
      "provider_id": "ChatGPT",
      "prompt_content": "Explain quantum computing in simple terms",
      "status": "Pending",
      "attempt_count": 0
    },
    {
      "id": "7ca8c920-9dad-11d1-80b4-00c04fd430c9",
      "provider_id": "Gemini",
      "prompt_content": "Explain quantum computing in simple terms",
      "status": "Pending",
      "attempt_count": 0
    }
  ]
}
```

**Errors:**
- `ValidationError`: Empty prompt or no providers selected
- `InternalError`: Failed to create submissions

**Side Effects:**
- Emits `submission_status_changed` events as submissions progress
- Triggers JavaScript injection into provider webviews
- Starts 30-second timeout timers per submission (FR-007)

**Frontend Usage:**
```typescript
const response = await invoke<SubmitPromptResponse>("submit_prompt", {
  prompt: promptText
});

// Listen for status updates
await listen<Submission>("submission_status_changed", (event) => {
  console.log(`Submission ${event.payload.id} status: ${event.payload.status}`);
});
```

---

## 4. get_submission_status

**Description:** Query current status of a specific submission.

**Command:**
```rust
#[tauri::command]
async fn get_submission_status(
    submission_id: String,
    state: State<'_, AppState>
) -> Result<Submission, CommandError>
```

**Request:**
```typescript
interface GetSubmissionStatusRequest {
  submission_id: string;  // UUID
}
```

**Response:**
```typescript
interface Submission {
  // Same as submit_prompt response
}
```

**Example Request:**
```json
{
  "submission_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
}
```

**Example Response:**
```json
{
  "id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "provider_id": "ChatGPT",
  "prompt_content": "Explain quantum computing in simple terms",
  "status": "Success",
  "attempt_count": 1,
  "started_at": "2025-11-13T10:30:00Z",
  "completed_at": "2025-11-13T10:30:15Z"
}
```

**Errors:**
- `NotFound`: Invalid submission_id

**Frontend Usage:**
```typescript
const submission = await invoke<Submission>("get_submission_status", {
  submission_id: submissionId
});
```

---

## 5. create_provider_webview

**Description:** Create or show the webview window for a specific provider with persistent session configuration.

**Command:**
```rust
#[tauri::command]
async fn create_provider_webview(
    provider_id: String,
    app_handle: AppHandle,
    state: State<'_, AppState>
) -> Result<WebviewInfo, CommandError>
```

**Request:**
```typescript
interface CreateProviderWebviewRequest {
  provider_id: "ChatGPT" | "Gemini" | "Claude";
}
```

**Response:**
```typescript
interface WebviewInfo {
  provider_id: string;
  label: string;  // Webview window label
  url: string;
  is_persistent: boolean;
  data_path?: string;  // Windows/Linux only
  data_store_id?: string;  // macOS only (hex UUID)
}
```

**Example Request:**
```json
{
  "provider_id": "ChatGPT"
}
```

**Example Response (Windows/Linux):**
```json
{
  "provider_id": "ChatGPT",
  "label": "chatgpt",
  "url": "https://chat.openai.com/",
  "is_persistent": true,
  "data_path": "/home/user/.local/share/com.chenchen.app/webviews/chatgpt"
}
```

**Example Response (macOS):**
```json
{
  "provider_id": "ChatGPT",
  "label": "chatgpt",
  "url": "https://chat.openai.com/",
  "is_persistent": true,
  "data_store_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Errors:**
- `NotFound`: Invalid provider_id
- `WebviewCreationFailed`: Failed to create webview window
- `PlatformNotSupported`: macOS version < 14 (data_store_identifier unavailable)

**Side Effects:**
- Creates webview window if it doesn't exist
- Shows/focuses window if it already exists
- Configures persistent session storage

**Frontend Usage:**
```typescript
const webviewInfo = await invoke<WebviewInfo>("create_provider_webview", {
  provider_id: "ChatGPT"
});
```

---

## 6. check_authentication

**Description:** Check if a provider's webview session is authenticated by evaluating auth check selectors.

**Command:**
```rust
#[tauri::command]
async fn check_authentication(
    provider_id: String,
    app_handle: AppHandle,
    state: State<'_, AppState>
) -> Result<AuthenticationStatus, CommandError>
```

**Request:**
```typescript
interface CheckAuthenticationRequest {
  provider_id: "ChatGPT" | "Gemini" | "Claude";
}
```

**Response:**
```typescript
interface AuthenticationStatus {
  provider_id: string;
  is_authenticated: boolean;
  last_checked: string;  // ISO 8601
  requires_login: boolean;
}
```

**Example Request:**
```json
{
  "provider_id": "ChatGPT"
}
```

**Example Response:**
```json
{
  "provider_id": "ChatGPT",
  "is_authenticated": true,
  "last_checked": "2025-11-13T10:30:00Z",
  "requires_login": false
}
```

**Errors:**
- `NotFound`: Invalid provider_id or webview not created
- `WebviewNotReady`: Webview still loading

**Implementation:**
- Injects JavaScript to check for presence of `auth_check_selectors` from ProviderSelectorConfig
- If auth selectors found → `is_authenticated = false, requires_login = true`
- If auth selectors NOT found → `is_authenticated = true, requires_login = false`

**Frontend Usage:**
```typescript
const authStatus = await invoke<AuthenticationStatus>("check_authentication", {
  provider_id: "ChatGPT"
});

if (authStatus.requires_login) {
  alert("Please log into ChatGPT");
}
```

---

## 7. get_layout_configuration

**Description:** Calculate split-screen layout configuration based on currently selected providers.

**Command:**
```rust
#[tauri::command]
async fn get_layout_configuration(
    state: State<'_, AppState>
) -> Result<LayoutConfiguration, CommandError>
```

**Request:**
- No parameters

**Response:**
```typescript
interface LayoutConfiguration {
  provider_count: number;  // 1-3
  layout_type: "Full" | "VerticalSplit" | "Grid";
  panel_dimensions: PanelDimension[];
}

interface PanelDimension {
  provider_id: string;
  x: number;       // 0.0 - 1.0 (percentage)
  y: number;       // 0.0 - 1.0 (percentage)
  width: number;   // 0.0 - 1.0 (percentage)
  height: number;  // 0.0 - 1.0 (percentage)
}
```

**Example Response (2 providers):**
```json
{
  "provider_count": 2,
  "layout_type": "VerticalSplit",
  "panel_dimensions": [
    {
      "provider_id": "ChatGPT",
      "x": 0.0,
      "y": 0.0,
      "width": 0.5,
      "height": 1.0
    },
    {
      "provider_id": "Gemini",
      "x": 0.5,
      "y": 0.0,
      "width": 0.5,
      "height": 1.0
    }
  ]
}
```

**Example Response (3 providers):**
```json
{
  "provider_count": 3,
  "layout_type": "Grid",
  "panel_dimensions": [
    {
      "provider_id": "ChatGPT",
      "x": 0.0,
      "y": 0.0,
      "width": 0.5,
      "height": 0.5
    },
    {
      "provider_id": "Gemini",
      "x": 0.5,
      "y": 0.0,
      "width": 0.5,
      "height": 0.5
    },
    {
      "provider_id": "Claude",
      "x": 0.0,
      "y": 0.5,
      "width": 1.0,
      "height": 0.5
    }
  ]
}
```

**Errors:**
- `ValidationError`: No providers selected

**Frontend Usage:**
```typescript
const layout = await invoke<LayoutConfiguration>("get_layout_configuration");

// Apply layout to webview containers
layout.panel_dimensions.forEach(panel => {
  const element = document.getElementById(`webview-${panel.provider_id}`);
  element.style.left = `${panel.x * 100}%`;
  element.style.top = `${panel.y * 100}%`;
  element.style.width = `${panel.width * 100}%`;
  element.style.height = `${panel.height * 100}%`;
});
```

---

## Events

### submission_status_changed

**Description:** Emitted when a submission's status changes (Pending → InProgress → Success/Failed/Retrying).

**Payload:**
```typescript
interface Submission {
  // Same as Submission type from submit_prompt
}
```

**Example:**
```json
{
  "id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "provider_id": "ChatGPT",
  "prompt_content": "Explain quantum computing in simple terms",
  "status": "Success",
  "attempt_count": 1,
  "started_at": "2025-11-13T10:30:00Z",
  "completed_at": "2025-11-13T10:30:15Z"
}
```

**Frontend Listener:**
```typescript
import { listen } from "@tauri-apps/api/event";

await listen<Submission>("submission_status_changed", (event) => {
  updateStatusUI(event.payload);
});
```

---

## Error Handling

**Error Type:**
```typescript
interface CommandError {
  code: string;
  message: string;
}
```

**Error Codes:**
- `ValidationError`: Request validation failed (e.g., empty prompt, no providers selected)
- `NotFound`: Requested resource not found (e.g., invalid provider_id)
- `InternalError`: Unexpected internal error
- `WebviewCreationFailed`: Failed to create webview window
- `WebviewNotReady`: Webview still loading
- `MaxProvidersExceeded`: More than 3 providers selected
- `PlatformNotSupported`: Platform-specific feature unavailable

**Example Error Response:**
```json
{
  "code": "ValidationError",
  "message": "At least one provider must be selected"
}
```

**Frontend Error Handling:**
```typescript
try {
  await invoke("submit_prompt", { prompt: "" });
} catch (error) {
  const err = error as CommandError;
  console.error(`${err.code}: ${err.message}`);
}
```

---

## Type Definitions (TypeScript)

Complete TypeScript definitions for frontend usage:

```typescript
// Provider types
type ProviderId = "ChatGPT" | "Gemini" | "Claude";

interface Provider {
  id: ProviderId;
  name: string;
  url: string;
  is_selected: boolean;
  is_authenticated: boolean;
  selector_config_id: string;
}

// Submission types
type SubmissionStatus = "Pending" | "InProgress" | "Retrying" | "Success" | "Failed";

type SubmissionErrorType =
  | "Timeout"
  | "NetworkError"
  | "AuthenticationError"
  | "RateLimitError"
  | "ElementNotFound"
  | "InjectionFailed";

interface Submission {
  id: string;
  provider_id: ProviderId;
  prompt_content: string;
  status: SubmissionStatus;
  attempt_count: number;
  error_type?: SubmissionErrorType;
  error_message?: string;
  started_at?: string;
  completed_at?: string;
}

interface SubmitPromptResponse {
  prompt_id: string;
  submissions: Submission[];
}

// Layout types
type LayoutType = "Full" | "VerticalSplit" | "Grid";

interface PanelDimension {
  provider_id: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

interface LayoutConfiguration {
  provider_count: number;
  layout_type: LayoutType;
  panel_dimensions: PanelDimension[];
}

// Authentication types
interface AuthenticationStatus {
  provider_id: string;
  is_authenticated: boolean;
  last_checked: string;
  requires_login: boolean;
}

// Webview types
interface WebviewInfo {
  provider_id: string;
  label: string;
  url: string;
  is_persistent: boolean;
  data_path?: string;
  data_store_id?: string;
}

// Error types
interface CommandError {
  code: string;
  message: string;
}
```

---

## Rust Type Definitions (Serde)

Corresponding Rust types with serde annotations:

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderId {
    ChatGPT,
    Gemini,
    Claude,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub url: String,
    pub is_selected: bool,
    pub is_authenticated: bool,
    pub selector_config_id: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubmissionStatus {
    Pending,
    InProgress,
    Retrying,
    Success,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubmissionErrorType {
    Timeout,
    NetworkError,
    AuthenticationError,
    RateLimitError,
    ElementNotFound,
    InjectionFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: Uuid,
    pub provider_id: ProviderId,
    pub prompt_content: String,
    pub status: SubmissionStatus,
    pub attempt_count: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<SubmissionErrorType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitPromptResponse {
    pub prompt_id: Uuid,
    pub submissions: Vec<Submission>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LayoutType {
    Full,
    VerticalSplit,
    Grid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelDimension {
    pub provider_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfiguration {
    pub provider_count: u8,
    pub layout_type: LayoutType,
    pub panel_dimensions: Vec<PanelDimension>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationStatus {
    pub provider_id: String,
    pub is_authenticated: bool,
    pub last_checked: chrono::DateTime<chrono::Utc>,
    pub requires_login: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebviewInfo {
    pub provider_id: String,
    pub label: String,
    pub url: String,
    pub is_persistent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_store_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    pub code: String,
    pub message: String,
}
```

---

## Contract Versioning

**Initial Version:** 1.0.0

**Versioning Rules:**
- MAJOR: Breaking changes to command signatures or response structures
- MINOR: New commands added (backward compatible)
- PATCH: Bug fixes, documentation updates

**Breaking Change Examples:**
- Renaming a command
- Removing a field from response
- Changing field types
- Changing error codes

**Non-Breaking Change Examples:**
- Adding new optional fields to response
- Adding new commands
- Adding new error codes
- Documentation clarifications
