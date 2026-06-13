# Release Workflow Prevention Design

## Context

The `v0.5.1` release was split across two GitHub release objects because the release workflow let parallel matrix jobs create and upload to releases independently. The Windows job built `.msi` and `.exe` installers, but uploaded them to a duplicate draft release while the Linux assets ended up on the published release.

This design only covers preventing the issue in future releases. It does not include one-time cleanup for the existing `v0.5.1` release.

## Goals

- Keep platform builds parallel.
- Produce release assets for Windows, Linux, and macOS.
- Publish releases automatically after all platform assets are ready.
- Ensure only one workflow job creates or publishes a GitHub release.
- Fail visibly instead of publishing a release with missing platform assets.
- Keep manual workflow runs useful for build validation without accidentally publishing branch builds.

## Non-Goals

- Cleaning up or merging assets for `v0.5.1`.
- Signing or notarizing macOS builds.
- Adding auto-update signatures.
- Reworking version bumping or changelog generation.

## Recommended Approach

Use a two-stage workflow:

1. A parallel `build` matrix job builds platform bundles and uploads them as GitHub Actions artifacts.
2. A single `publish` job waits for all builds, downloads all artifacts, creates one GitHub release, uploads every bundle file, and publishes the release.

This removes the release creation race because only the `publish` job talks to the GitHub Releases API.

## Workflow Architecture

### Build Job

The `build` job remains a matrix with these platforms:

- `windows-latest`
- `ubuntu-22.04`
- `macos-latest`

Each matrix entry should define:

- `platform`: the GitHub runner label.
- `artifact-name`: stable artifact name, such as `release-windows`, `release-linux`, or `release-macos`.
- `bundle-globs`: the generated bundle files expected for that platform.
- `args`: optional Tauri build arguments.

The build job is responsible for:

- Checking out the repository.
- Installing pnpm, Node.js, Rust, and platform-specific system dependencies.
- Running the Tauri build.
- Verifying that expected bundle files exist.
- Uploading the bundle files as a GitHub Actions artifact.

It must not create, update, publish, or upload assets to a GitHub release.

### Publish Job

The `publish` job depends on the full `build` matrix with `needs: build`.

The publish job is responsible for:

- Running only for version tag refs matching `refs/tags/v*`.
- Downloading all platform artifacts.
- Verifying that the downloaded release asset directory contains expected assets from Windows, Linux, and macOS.
- Creating one GitHub release for `github.ref_name`.
- Uploading all bundle files to that release.
- Publishing the release immediately.

If a release for the tag already exists, the publish job should fail rather than mutating an existing published release.

## Platform Outputs

Expected bundle outputs:

- Windows:
  - `src-tauri/target/release/bundle/msi/*.msi`
  - `src-tauri/target/release/bundle/nsis/*setup.exe`
- Linux:
  - `src-tauri/target/release/bundle/deb/*.deb`
  - `src-tauri/target/release/bundle/rpm/*.rpm`
  - `src-tauri/target/release/bundle/appimage/*.AppImage`
- macOS unsigned:
  - `src-tauri/target/release/bundle/dmg/*.dmg`

The first macOS implementation should produce unsigned artifacts only. The release notes should state that macOS builds are unsigned, so users understand that Gatekeeper may warn when opening the app.

## Release Semantics

Releases should auto-publish after all platform builds succeed.

Manual `workflow_dispatch` runs can remain available for build testing, but publishing must be guarded with `startsWith(github.ref, 'refs/tags/v')`. This prevents a manual branch run from publishing a release with a branch name.

The release name should stay consistent with the current workflow:

```text
ChenChen ${{ github.ref_name }}
```

The release body should stay close to the current wording, with updated macOS installation text noting that the macOS artifact is unsigned.

## Error Handling

The workflow should fail before publishing when required assets are missing.

Required behavior:

- Keep `strategy.fail-fast: false` so all platform jobs report independently.
- Use per-platform verification steps after Tauri build.
- Use `actions/upload-artifact` with `if-no-files-found: error`.
- Make `publish` depend on all build jobs.
- In `publish`, fail if downloaded assets are missing for any platform.
- Do not publish drafts.
- Do not update an existing release for the same tag by default.

## Validation Plan

Local and structural validation:

- Parse or inspect the workflow for valid YAML and expression structure.
- Run existing checks that are relevant and available, such as `pnpm check`.
- Run `pnpm build` if dependencies and environment allow it.

CI validation:

- Use `workflow_dispatch` to validate builds without publishing from a branch.
- Confirm the next tag run creates exactly one release.
- Confirm the release contains Windows, Linux, and unsigned macOS assets.
- Confirm the publish job runs only after all platform build jobs complete.

## Alternatives Considered

### Serialize The Matrix

Set `strategy.max-parallel: 1` and keep `tauri-apps/tauri-action` release uploads in each platform job.

This is smaller, but it makes releases slower and keeps release ownership spread across platform jobs.

### Pre-Create The Release

Create the release in a setup job, then allow platform jobs to upload to it.

This avoids duplicate release creation, but release asset uploads are still spread across concurrent jobs. Publishing after all uploads still requires a final release-update job, so it is less clean than centralizing publication.

## Open Decisions

No open decisions remain for this design.
