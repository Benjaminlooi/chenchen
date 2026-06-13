# Cloudflare CAPTCHA Evasion Fix Design Spec

## Objective
Resolve the unsolvable/infinite-loop Cloudflare CAPTCHA (Turnstile) issues when loading ChatGPT and other LLM provider panels in the ChenChen desktop app.

## Root Causes Identified
1. **Canvas Fingerprinting Noise:** The stealth scripts hook `HTMLCanvasElement.prototype.toDataURL` to inject random noise. Cloudflare Turnstile validates the canvas engine's output hash to detect browser fingerprint spoofing. Since the hash is randomized on every request, Cloudflare identifies the WebView as a bot and loops the challenge indefinitely.
2. **User-Agent & Platform Mismatches:** Both Windows and Linux clients currently receive a Linux Chrome User-Agent. However, Windows WebViews return `"Win32"` for `navigator.platform` and contain other Windows-specific API indicators. This mismatch between the spoofed Linux User-Agent header and actual Windows platform APIs triggers Cloudflare's bot-detection heuristics.

## Proposed Changes

### 1. Platform-Specific User-Agent Configuration
Update [commands.rs](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/commands.rs) to set User-Agents corresponding to the client's actual compilation platform:

- **macOS:** Safari 18 on macOS 14
  `Mozilla/5.0 (Macintosh; Intel Mac OS X 14_5) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.4 Safari/605.1.15`
- **Windows:** Google Chrome 131 on Windows 10/11
  `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36`
- **Linux:** Google Chrome 131 on Linux
  `Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36`
- **Fallback (Other OS):** Google Chrome 131 on Linux (default)

### 2. Stealth Script Modifications
Modify the embedded JavaScript files to eliminate canvas tampering and dynamically adjust client hints at runtime:

#### A. Remove Canvas Fingerprinting Hooks
Remove the `HTMLCanvasElement.prototype.toDataURL` hook from:
- [stealth.js](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/injection/stealth.js)
- [stealth_macos.js](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/injection/stealth_macos.js)

#### B. Dynamic Client Hints in `stealth.js`
In [stealth.js](file:///home/ben/ghq/github.com/Benjaminlooi/chenchen/src-tauri/src/injection/stealth.js), dynamically detect the platform via the injected User-Agent at runtime to set matching Client Hints (`navigator.userAgentData`):
```javascript
const isWindows = navigator.userAgent.includes("Windows NT");
const platform = isWindows ? "Windows" : "Linux";
const platformVersion = isWindows ? "10.0.0" : "6.5.0";
```

## Verification Plan
1. Compile and run the Tauri application.
2. Verify that the WebView loads ChatGPT without displaying an infinite Cloudflare CAPTCHA loop.
3. Verify that the User-Agent and `navigator.userAgentData` match the client's operating system.
4. Ensure existing backend and frontend tests continue to pass (`cargo test` and `npm test`).
