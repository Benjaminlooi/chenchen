# Release Workflow Prevention Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the release workflow so Windows, Linux, and unsigned macOS builds run in parallel, then one final job auto-publishes a single release containing all platform assets.

**Architecture:** The existing `.github/workflows/release.yml` matrix job currently builds and creates releases in each platform job, which caused duplicate `v0.5.1` release objects. Replace it with a `build` matrix job that only builds and uploads Actions artifacts, plus a single tag-only `publish` job that downloads those artifacts, verifies expected assets, and creates one GitHub release.

**Tech Stack:** GitHub Actions, pnpm 11.1.2, Node.js 22, Rust stable, Tauri CLI, GitHub CLI in Actions, `actions/upload-artifact`, `actions/download-artifact`.

---

## File Structure

- Modify: `.github/workflows/release.yml`
  - Owns release build and publish automation.
  - Defines the parallel platform matrix.
  - Verifies generated bundle files per platform.
  - Uploads build outputs as Actions artifacts.
  - Publishes one release from a single final job.
- No production source files change.
- No one-time cleanup for `v0.5.1` is included.

## Task 1: Replace Release Workflow With Build And Publish Jobs

**Files:**
- Modify: `.github/workflows/release.yml`

- [ ] **Step 1: Replace the workflow contents**

Edit `.github/workflows/release.yml` so the full file is:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build:
    name: build (${{ matrix.platform }})
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: windows-latest
            artifact-name: release-windows
            args: ''
            rust-targets: ''
            bundle-globs: |
              src-tauri/target/release/bundle/msi/*.msi
              src-tauri/target/release/bundle/nsis/*setup.exe
          - platform: ubuntu-22.04
            artifact-name: release-linux
            args: ''
            rust-targets: ''
            bundle-globs: |
              src-tauri/target/release/bundle/deb/*.deb
              src-tauri/target/release/bundle/rpm/*.rpm
              src-tauri/target/release/bundle/appimage/*.AppImage
          - platform: macos-latest
            artifact-name: release-macos
            args: ''
            rust-targets: ''
            bundle-globs: |
              src-tauri/target/release/bundle/dmg/*.dmg

    runs-on: ${{ matrix.platform }}

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup pnpm
        uses: pnpm/action-setup@v4
        with:
          version: 11.1.2

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.rust-targets }}

      - name: Install Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: './src-tauri -> target'

      - name: Install Linux dependencies
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

      - name: Install frontend dependencies
        run: pnpm install --frozen-lockfile

      - name: Build Tauri app
        run: pnpm tauri build ${{ matrix.args }}

      - name: Verify bundle outputs
        shell: bash
        run: |
          set -euo pipefail
          shopt -s nullglob

          files=()
          while IFS= read -r pattern; do
            [[ -z "$pattern" ]] && continue
            for file in $pattern; do
              files+=("$file")
            done
          done <<'EOF'
          ${{ matrix.bundle-globs }}
          EOF

          if [[ ${#files[@]} -eq 0 ]]; then
            echo "No bundle outputs found for ${{ matrix.platform }}" >&2
            echo "Expected patterns:" >&2
            cat <<'EOF' >&2
          ${{ matrix.bundle-globs }}
          EOF
            exit 1
          fi

          printf '%s\n' "${files[@]}" > release-files.txt
          echo "Found bundle outputs:"
          cat release-files.txt

      - name: Upload release artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact-name }}
          path: ${{ matrix.bundle-globs }}
          if-no-files-found: error
          retention-days: 7

  publish:
    name: publish release
    needs: build
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-22.04
    permissions:
      contents: write

    steps:
      - name: Download release artifacts
        uses: actions/download-artifact@v4
        with:
          path: release-assets

      - name: Verify release assets
        shell: bash
        run: |
          set -euo pipefail
          shopt -s nullglob globstar

          windows_assets=(release-assets/release-windows/**/*.msi release-assets/release-windows/**/*setup.exe)
          linux_assets=(release-assets/release-linux/**/*.deb release-assets/release-linux/**/*.rpm release-assets/release-linux/**/*.AppImage)
          macos_assets=(release-assets/release-macos/**/*.dmg)
          mapfile -d '' all_assets < <(find release-assets -type f -print0 | sort -z)

          if [[ ${#windows_assets[@]} -lt 2 ]]; then
            echo "Missing Windows release assets. Expected .msi and setup .exe." >&2
            find release-assets -maxdepth 3 -type f -print >&2
            exit 1
          fi

          if [[ ${#linux_assets[@]} -lt 3 ]]; then
            echo "Missing Linux release assets. Expected .deb, .rpm, and .AppImage." >&2
            find release-assets -maxdepth 3 -type f -print >&2
            exit 1
          fi

          if [[ ${#macos_assets[@]} -lt 1 ]]; then
            echo "Missing macOS release asset. Expected unsigned .dmg." >&2
            find release-assets -maxdepth 3 -type f -print >&2
            exit 1
          fi

          if [[ ${#all_assets[@]} -eq 0 ]]; then
            echo "No release assets downloaded." >&2
            exit 1
          fi

          echo "Release assets ready:"
          find release-assets -type f -print | sort

      - name: Create GitHub release
        shell: bash
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          set -euo pipefail
          mapfile -d '' release_assets < <(find release-assets -type f -print0 | sort -z)

          gh release create "${{ github.ref_name }}" \
            --repo "${{ github.repository }}" \
            --title "ChenChen ${{ github.ref_name }}" \
            --notes-file - \
            "${release_assets[@]}" <<'EOF'
          See the assets below to download the app for your platform.

          ## What's Changed
          - See the [changelog](https://github.com/${{ github.repository }}/blob/${{ github.ref_name }}/CHANGELOG.md) for details

          ## Installation
          - **Windows**: Download and run the `.msi` or `.exe` installer
          - **Linux**: Download the `.AppImage` and make it executable
          - **macOS**: Download the unsigned `.dmg`. macOS Gatekeeper may warn because this build is not signed or notarized.
          EOF
```

- [ ] **Step 2: Check that release creation was removed from platform jobs**

Run:

```bash
rg -n "tauri-apps/tauri-action|releaseDraft|releaseName|tagName|GITHUB_TOKEN" .github/workflows/release.yml
```

Expected: no output. Exit code `1` is acceptable because `rg` returns `1` when there are no matches.

- [ ] **Step 3: Check that there is exactly one release creation command**

Run:

```bash
rg -n "gh release create" .github/workflows/release.yml
```

Expected output includes exactly one match in the `publish` job.

- [ ] **Step 4: Commit the workflow refactor**

Run:

```bash
git add .github/workflows/release.yml
git commit -m "fix(ci): publish releases from single job"
```

Expected: commit succeeds with only `.github/workflows/release.yml` changed.

## Task 2: Validate Workflow Structure And Local Build Checks

**Files:**
- Verify: `.github/workflows/release.yml`
- Verify: `package.json`
- Verify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Parse the workflow as YAML**

Run:

```bash
ruby -e "require 'yaml'; YAML.load_file('.github/workflows/release.yml'); puts 'workflow yaml parsed'"
```

Expected output:

```text
workflow yaml parsed
```

- [ ] **Step 2: Confirm tag-only publish guard exists**

Run:

```bash
rg -n "if: startsWith\\(github\\.ref, 'refs/tags/v'\\)" .github/workflows/release.yml
```

Expected output includes the `publish` job guard.

- [ ] **Step 3: Confirm all three platform artifacts are referenced**

Run:

```bash
rg -n "release-windows|release-linux|release-macos|windows-latest|ubuntu-22\\.04|macos-latest" .github/workflows/release.yml
```

Expected output includes all three runner labels and all three artifact names.

- [ ] **Step 4: Run Svelte and TypeScript checks**

Run:

```bash
pnpm check
```

Expected: command exits `0`. Existing warnings are acceptable only if the command still passes.

- [ ] **Step 5: Run frontend build**

Run:

```bash
pnpm build
```

Expected: command exits `0` and produces the static frontend build used by Tauri.

- [ ] **Step 6: Commit validation follow-up if needed**

If validation required edits, run:

```bash
git add .github/workflows/release.yml
git commit -m "fix(ci): validate release workflow assets"
```

Expected: commit succeeds only if edits were made. If no edits were needed, skip this step.

## Task 3: Optional CI Trial Without Publishing

**Files:**
- Verify: `.github/workflows/release.yml`

- [ ] **Step 1: Push the branch with the workflow changes**

Run:

```bash
git status --short
git log --oneline -3
```

Expected: working tree is clean and the release workflow commit is present.

- [ ] **Step 2: Trigger manual workflow dispatch from a branch**

Run:

```bash
gh workflow run release.yml --ref master
```

Expected: GitHub accepts the dispatch.

- [ ] **Step 3: Watch the workflow run**

Run:

```bash
gh run list --workflow release.yml --limit 1
```

Expected: the newest run is a `workflow_dispatch` run for `master`.

- [ ] **Step 4: Confirm the branch run does not publish**

Open the run in GitHub or inspect jobs with:

```bash
gh run view --json jobs --jq '.jobs[] | {name, conclusion}'
```

Expected: `build` jobs run, while the `publish release` job is skipped because the ref is not `refs/tags/v*`.

## Task 4: Next Tag Release Verification

**Files:**
- Verify: GitHub release page for the next version tag.

- [ ] **Step 1: Create the next version tag using the project release process**

Use the existing version bump and tag process for this repository. Do not reuse `v0.5.1`.

- [ ] **Step 2: Watch the tag-triggered release workflow**

Run:

```bash
gh run list --workflow release.yml --limit 1
```

Expected: the newest run is a `push` run for the new `v*` tag.

- [ ] **Step 3: Confirm release asset completeness**

After the run passes, run this command after replacing `v0.5.2` if the next release tag is different:

```bash
NEW_TAG=v0.5.2
gh release view "$NEW_TAG" --json assets --jq '.assets[].name'
```

Expected asset names include:

```text
*.msi
*setup.exe
*.deb
*.rpm
*.AppImage
*.dmg
```

- [ ] **Step 4: Confirm there is only one release object for the tag**

Run this command after replacing `v0.5.2` if the next release tag is different:

```bash
NEW_TAG=v0.5.2
gh api repos/Benjaminlooi/chenchen/releases --paginate --jq '.[] | select(.tag_name==env.NEW_TAG) | {id, tag_name, draft, assets: [.assets[].name]}'
```

Expected: exactly one JSON object is printed for the new tag, `draft` is `false`, and all platform assets are on that release.
