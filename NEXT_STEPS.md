# Next Steps - ChenChen Implementation

**Status:** Phase 3 Backend + Frontend Complete (Untested due to system dependencies)

## ⚠️ Action Required: Install System Dependencies

The implementation cannot be tested or run until WebKitGTK dependencies are installed on your Linux system.

### Quick Install (Choose your distro):

**Fedora/RHEL:**
```bash
sudo dnf install webkit2gtk4.1-devel libsoup3-devel javascriptcoregtk4.1-devel
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libwebkit2gtk-4.1-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
```

**Arch Linux:**
```bash
sudo pacman -S webkit2gtk-4.1
```

### Verify Installation:
```bash
pkg-config --modversion webkit2gtk-4.1
# Should output version number (e.g., 2.42.0)
```

---

## 📦 What's Been Implemented

### ✅ Phase 1-2: Foundation (35+ tasks complete)

**Backend Infrastructure:**
- Complete type system (ProviderId, SubmissionStatus, SubmissionErrorType, CommandError)
- Module structure for all libraries (providers/, layout/, webview/, injection/, status/)
- AppState with thread-safe ProviderManager
- Logging configured with env_logger

**Frontend Infrastructure:**
- TypeScript types with Zod validation schemas
- Type-safe Tauri service wrapper
- Vitest test environment configured

### ✅ Phase 3: Provider Selection (13/13 tasks complete)

**Backend (Rust):**
- `Provider` struct with configuration fields
- `ProviderManager` with validation logic:
  - ✅ Prevents deselecting last provider (FR-004)
  - ✅ Prevents selecting more than 3 providers (TC-005)
  - ✅ Provides selected provider filtering
- `get_providers` Tauri command
- `update_provider_selection` Tauri command
- Unit tests (4 tests covering all validation rules)
- Contract tests (6 comprehensive tests)

**Frontend (Svelte):**
- `ProviderSelector.svelte` component (150 lines)
  - Loads providers on mount
  - Checkboxes with validation error handling
  - Visual feedback (icons, auth badges, selection count)
  - Accessible (proper labels, ARIA roles, keyboard support)
- Component tests (7 test cases covering all scenarios)

**Files Created:**
```
src-tauri/src/providers/
  ├── mod.rs                     (Provider struct)
  ├── manager.rs                 (ProviderManager + inline tests)
  └── config.rs                  (Config loader - stub for Phase 6)

src-tauri/tests/contract/
  └── provider_manager_test.rs   (Contract tests)

src/components/
  └── ProviderSelector.svelte    (UI component)

tests/components/
  └── ProviderSelector.test.ts   (Component tests)

src/services/
  └── tauri.ts                   (Type-safe API wrapper)

src/types.ts                     (TypeScript types + Zod schemas)
```

---

## 🧪 Testing After Dependencies Installed

### 1. Verify Rust Tests Pass

```bash
cd src-tauri

# Run inline unit tests
cargo test providers::manager::tests

# Run contract tests
cargo test --test provider_manager_test

# All tests should PASS:
# - test_new_returns_three_providers
# - test_cannot_deselect_last_provider
# - test_can_select_all_three_providers
# - test_can_toggle_selection
```

### 2. Verify Frontend Tests Pass

```bash
# From project root
npm test

# Should run 7 tests in ProviderSelector.test.ts
# All should PASS
```

### 3. Manual End-to-End Test

```bash
# Start dev server
npm run tauri dev

# Expected behavior:
# 1. App window opens with ProviderSelector component
# 2. See 3 provider checkboxes (ChatGPT, Gemini, Claude)
# 3. All checkboxes initially unchecked
# 4. Can check/uncheck boxes
# 5. Cannot uncheck the last checked box (validation error appears)
# 6. Selection count updates: "Selected: X / 3"
```

---

## 🚀 Next Implementation Phase: Phase 4 - Layout Configuration

Once tests pass, proceed with Phase 4 (Tasks T039-T052):

### Phase 4 Overview: Split-Screen Layout Calculator

**Goal:** Calculate panel dimensions based on number of selected providers

**Layout Rules:**
- 1 provider selected → Full window (100% width/height)
- 2 providers selected → Vertical split (50%/50%)
- 3 providers selected → Grid (2 top, 1 bottom spanning full width)

### Phase 4 Tasks (TDD - Tests First!):

1. **Write Tests (T039-T042):**
   ```bash
   # Create test file
   touch src-tauri/tests/unit/layout_calculator_test.rs

   # Write tests for calculate_layout():
   # - 1 provider: Full layout
   # - 2 providers: VerticalSplit layout
   # - 3 providers: Grid layout
   ```

2. **Verify Tests FAIL (RED):**
   ```bash
   cargo test layout_calculator_test
   # Should FAIL - implementation doesn't exist yet
   ```

3. **Implement Layout Library (T043-T046):**
   ```bash
   mkdir -p src-tauri/src/layout
   touch src-tauri/src/layout/mod.rs
   touch src-tauri/src/layout/calculator.rs

   # Create enums and structs:
   # - LayoutType enum (Full, VerticalSplit, Grid)
   # - PanelDimension struct (x, y, width, height)
   # - LayoutConfiguration struct
   # - calculate_layout() function
   ```

4. **Verify Tests PASS (GREEN):**
   ```bash
   cargo test layout_calculator_test
   # Should PASS
   ```

5. **Create Tauri Command (T047-T048):**
   ```rust
   // src-tauri/src/commands.rs
   #[tauri::command]
   pub fn get_layout_configuration(
       state: State<AppState>
   ) -> Result<LayoutConfiguration, CommandError> {
       // Get selected providers count
       // Calculate layout
       // Return configuration
   }
   ```

6. **Create ProviderPanel Component (T049-T052):**
   ```bash
   touch src/components/ProviderPanel.svelte
   touch tests/components/ProviderPanel.test.ts
   ```

### Estimated Time for Phase 4:
- Tests: 30 minutes
- Implementation: 1-2 hours
- Component: 1 hour
- **Total:** 2-3 hours

---

## 📊 Overall Progress

| Phase | Tasks | Status | Notes |
|-------|-------|--------|-------|
| **Phase 1: Setup** | 12 | ✅ Complete | Project initialized |
| **Phase 2: Foundation** | 9 | ✅ Complete | Types and infrastructure |
| **Phase 3: Provider Selection** | 17 | ✅ Complete | **Untested** (blocked) |
| **Phase 4: Layout** | 14 | ⏳ Next | Split-screen calculation |
| **Phase 5: Webview** | 12 | 📋 Pending | Session management |
| **Phase 6: Config** | 9 | 📋 Pending | Load provider configs |
| **Phase 7: Injection** | 12 | 📋 Pending | JavaScript injection |
| **Phase 8: Status** | 17 | 📋 Pending | Submission tracking |
| **Phase 9: Submission** | 22 | 📋 Pending | 🎯 **MVP** |
| **Phase 10: Auth** | 13 | 📋 Pending | Auth detection |
| **Phase 11: Privacy** | 8 | 📋 Pending | Privacy validation |
| **Phase 12: Polish** | 16 | 📋 Pending | Final touches |

**Total:** ~160 tasks
**Completed:** 38 tasks (24%)
**To MVP:** 90 more tasks

---

## 🐛 Known Issues

1. **Cannot build/test on Linux without WebKitGTK** (blocking)
   - Solution: Install system dependencies (see top of this doc)

2. **config/providers.json doesn't exist yet**
   - Will be created in Phase 6 (T005, T065-T073)
   - Contains CSS selectors for ChatGPT/Gemini/Claude

3. **ProviderSelector not integrated into main app**
   - Need to add to `src/routes/+page.svelte` or main App component
   - Will be done when wiring up end-to-end flow

---

## 📚 Documentation References

- **Current Status:** `IMPLEMENTATION_STATUS.md` (detailed progress)
- **Setup Instructions:** `SETUP_NOTES.md` (dependencies, commands)
- **Task Breakdown:** `specs/001-multi-llm-prompt/tasks.md` (all 160+ tasks)
- **Requirements:** `specs/001-multi-llm-prompt/spec.md` (user stories)
- **Architecture:** `specs/001-multi-llm-prompt/plan.md` (technical decisions)

---

## ✅ Checklist Before Continuing

- [ ] Install WebKitGTK dependencies
- [ ] Run `cargo test` and verify all pass
- [ ] Run `npm test` and verify all pass
- [ ] Run `npm run tauri dev` and manually test provider selection
- [ ] Review Phase 4 tasks in `tasks.md`
- [ ] Write Phase 4 tests (TDD RED phase)
- [ ] Implement Phase 4 (TDD GREEN phase)
- [ ] Continue through Phases 5-9 to reach MVP

---

## 💡 Tips for Continuing

1. **Always TDD:** Write tests first, verify they fail, then implement
2. **One phase at a time:** Complete each phase's checkpoint before moving on
3. **Run tests frequently:** After every task or small change
4. **Follow tasks.md:** Tasks are ordered by dependencies
5. **Mark tasks complete:** Update tasks.md by changing `[ ]` to `[X]` as you complete tasks
6. **Commit often:** Small, focused commits after each completed task/group

---

## 🎯 Critical Path to MVP (Phase 9)

```
Phase 3 (Provider Selection) ✅ COMPLETE
    ↓
Phase 4 (Layout) → Calculate split-screen dimensions
    ↓
Phase 5 (Webview) → Create persistent provider webviews
    ↓
Phase 6 (Config) → Load CSS selectors from JSON
    ↓
Phase 7 (Injection) → Generate + execute JavaScript
    ↓
Phase 8 (Status) → Track submission state machine
    ↓
Phase 9 (Submission) → 🎯 FULL PROMPT SUBMISSION FLOW
    ↓
🎉 MVP COMPLETE - Users can send prompts to 3 LLMs!
```

**Good luck with the implementation!** 🚀
