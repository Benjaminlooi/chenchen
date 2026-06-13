# Cloudflare CAPTCHA Evasion Fix Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the infinite/unsolvable Cloudflare CAPTCHA loop when rendering ChatGPT in the ChenChen desktop app.

**Architecture:** Update Rust WebviewBuilder to set platform-appropriate User-Agents (macOS, Windows, Linux) and modify embedded stealth JavaScript files to remove Canvas noise/fingerprint randomization and dynamically adapt Client Hints.

**Tech Stack:** Rust (Tauri 2.0 WebviewBuilder), JavaScript (Stealth Injector)

---

### Task 1: Update Rust WebviewBuilder User Agents

**Files:**
- Modify: `src-tauri/src/commands.rs`
- Test: Run `cargo check` in `src-tauri`

- [ ] **Step 1: Modify `commands.rs` to split user agents by target OS**
  Update [src-tauri/src/commands.rs](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/commands.rs) lines 360-369 to handle `"windows"`, `"macos"`, `"linux"`, and other operating systems individually.
  
  Replace:
  ```rust
        // T161: Platform-specific User-Agent corresponding to the stealth script
        #[cfg(target_os = "macos")]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15")
            .initialization_script(stealth_script);

        #[cfg(not(target_os = "macos"))]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .initialization_script(stealth_script);
  ```
  With:
  ```rust
        // T161: Platform-specific User-Agent corresponding to the stealth script
        #[cfg(target_os = "macos")]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15")
            .initialization_script(stealth_script);

        #[cfg(target_os = "windows")]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .initialization_script(stealth_script);

        #[cfg(target_os = "linux")]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .initialization_script(stealth_script);

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        let webview_builder = WebviewBuilder::new(&label, WebviewUrl::External(url.parse().unwrap()))
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .initialization_script(stealth_script);
  ```

- [ ] **Step 2: Run cargo check to verify compilation correctness**
  Run: `cargo check` in `src-tauri/`
  Expected: Builds successfully with no warnings or errors.

- [ ] **Step 3: Commit changes**
  Run:
  ```bash
  git add src-tauri/src/commands.rs
  git commit -m "feat: add platform-specific User-Agents to WebviewBuilder"
  ```

---

### Task 2: Modify `stealth.js` to Remove Canvas Noise and Set Dynamic Client Hints

**Files:**
- Modify: `src-tauri/src/injection/stealth.js`
- Modify: `src-tauri/src/injection/injector.rs:136-146` (update test assertions)

- [ ] **Step 1: Remove canvas noise code from `stealth.js`**
  Open [src-tauri/src/injection/stealth.js](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/injection/stealth.js) and delete lines 57-83 (the Canvas Fingerprinting Noise hook).
  
  Specifically, delete:
  ```javascript
      // 6. Canvas Fingerprinting Noise
      const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
      HTMLCanvasElement.prototype.toDataURL = function(type) {
          const context = this.getContext('2d');
          if (context) {
              const shift = {
                  'r': Math.floor(Math.random() * 10) - 5,
                  'g': Math.floor(Math.random() * 10) - 5,
                  'b': Math.floor(Math.random() * 10) - 5,
                  'a': Math.floor(Math.random() * 10) - 5
              };
              const width = this.width;
              const height = this.height;
              const imageData = context.getImageData(0, 0, width, height);
              for (let i = 0; i < height; i++) {
                  for (let j = 0; j < width; j++) {
                      const n = i * (width * 4) + j * 4;
                      imageData.data[n + 0] = imageData.data[n + 0] + shift.r;
                      imageData.data[n + 1] = imageData.data[n + 1] + shift.g;
                      imageData.data[n + 2] = imageData.data[n + 2] + shift.b;
                      imageData.data[n + 3] = imageData.data[n + 3] + shift.a;
                  }
              }
              context.putImageData(imageData, 0, 0);
          }
          return originalToDataURL.apply(this, arguments);
      };
  ```

- [ ] **Step 2: Update client hints code in `stealth.js` to detect Windows vs Linux**
  Modify Client Hints (lines 13-41) to detect platform from `navigator.userAgent`:
  
  Replace:
  ```javascript
      // 1. Client Hints (Chrome 131 on Linux)
      if (!navigator.userAgentData) {
          const userAgentData = {
              brands: [
                  { brand: "Chromium", version: "131" },
                  { brand: "Google Chrome", version: "131" },
                  { brand: "Not_A Brand", version: "24" }
              ],
              mobile: false,
              platform: "Linux",
              getHighEntropyValues: function(hints) {
                  return Promise.resolve({
                      architecture: "x86",
                      bitness: "64",
                      brands: this.brands,
                      mobile: this.mobile,
                      model: "",
                      platform: this.platform,
                      platformVersion: "6.5.0", // Example Linux kernel version
                      uaFullVersion: "131.0.6778.85",
                      fullVersionList: [
                          { brand: "Chromium", version: "131.0.6778.85" },
                          { brand: "Google Chrome", version: "131.0.6778.85" },
                          { brand: "Not_A Brand", version: "24.0.0.0" }
                      ]
                  });
              }
          };
          mockReadonly(navigator, 'userAgentData', userAgentData);
      }
  ```
  With:
  ```javascript
      // 1. Client Hints (Chrome 131 on Windows/Linux)
      if (!navigator.userAgentData) {
          const isWindows = navigator.userAgent.includes("Windows NT");
          const platform = isWindows ? "Windows" : "Linux";
          const platformVersion = isWindows ? "10.0.0" : "6.5.0";

          const userAgentData = {
              brands: [
                  { brand: "Chromium", version: "131" },
                  { brand: "Google Chrome", version: "131" },
                  { brand: "Not_A Brand", version: "24" }
              ],
              mobile: false,
              platform: platform,
              getHighEntropyValues: function(hints) {
                  return Promise.resolve({
                      architecture: "x86",
                      bitness: "64",
                      brands: this.brands,
                      mobile: this.mobile,
                      model: "",
                      platform: this.platform,
                      platformVersion: platformVersion,
                      uaFullVersion: "131.0.6778.85",
                      fullVersionList: [
                          { brand: "Chromium", version: "131.0.6778.85" },
                          { brand: "Google Chrome", version: "131.0.6778.85" },
                          { brand: "Not_A Brand", version: "24.0.0.0" }
                      ]
                  });
              }
          };
          mockReadonly(navigator, 'userAgentData', userAgentData);
      }
  ```

- [ ] **Step 3: Update `injector.rs` unit tests to remove `toDataURL` assertion**
  Open [src-tauri/src/injection/injector.rs](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/injection/injector.rs) and modify lines 141-142 to assert `userAgent` check rather than `toDataURL`.
  
  Replace:
  ```rust
          assert!(script.contains("hardwareConcurrency")); // HardwareConcurrency spoofing
          assert!(script.contains("toDataURL")); // Canvas noise
          assert!(script.contains("plugins"));
  ```
  With:
  ```rust
          assert!(script.contains("hardwareConcurrency")); // HardwareConcurrency spoofing
          assert!(script.contains("userAgent")); // Client Hints dynamic check
          assert!(script.contains("plugins"));
  ```

- [ ] **Step 4: Run cargo test to verify backend tests pass**
  Run: `cargo test` in `src-tauri/`
  Expected: All tests pass.

- [ ] **Step 5: Commit changes**
  Run:
  ```bash
  git add src-tauri/src/injection/stealth.js src-tauri/src/injection/injector.rs
  git commit -m "feat: remove canvas noise and implement dynamic Client Hints in stealth.js"
  ```

---

### Task 3: Modify `stealth_macos.js` to Remove Canvas Noise

**Files:**
- Modify: `src-tauri/src/injection/stealth_macos.js`
- Test: Run `cargo test` in `src-tauri`

- [ ] **Step 1: Remove canvas noise code from `stealth_macos.js`**
  Open [src-tauri/src/injection/stealth_macos.js](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/injection/stealth_macos.js) and delete lines 26-52 (the Canvas Fingerprinting Noise hook).
  
  Specifically, delete:
  ```javascript
      // 5. Canvas Fingerprinting Noise
      const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
      HTMLCanvasElement.prototype.toDataURL = function(type) {
          const context = this.getContext('2d');
          if (context) {
              const shift = {
                  'r': Math.floor(Math.random() * 10) - 5,
                  'g': Math.floor(Math.random() * 10) - 5,
                  'b': Math.floor(Math.random() * 10) - 5,
                  'a': Math.floor(Math.random() * 10) - 5
              };
              const width = this.width;
              const height = this.height;
              const imageData = context.getImageData(0, 0, width, height);
              for (let i = 0; i < height; i++) {
                  for (let j = 0; j < width; j++) {
                      const n = i * (width * 4) + j * 4;
                      imageData.data[n + 0] = imageData.data[n + 0] + shift.r;
                      imageData.data[n + 1] = imageData.data[n + 1] + shift.g;
                      imageData.data[n + 2] = imageData.data[n + 2] + shift.b;
                      imageData.data[n + 3] = imageData.data[n + 3] + shift.a;
                  }
              }
              context.putImageData(imageData, 0, 0);
          }
          return originalToDataURL.apply(this, arguments);
      };
  ```

- [ ] **Step 2: Run cargo test to verify backend tests pass**
  Run: `cargo test` in `src-tauri/`
  Expected: All tests pass.

- [ ] **Step 3: Commit changes**
  Run:
  ```bash
  git add src-tauri/src/injection/stealth_macos.js
  git commit -m "feat: remove canvas noise from stealth_macos.js"
  ```
