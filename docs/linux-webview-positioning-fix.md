# Linux Webview Positioning Fix

## Problem Description

On Linux systems using GTK, Tauri applications with multiple webviews experience a critical positioning bug where webviews ignore their position and size settings. Instead of being positioned at specified coordinates, all webviews stack vertically in the default layout container.

### Symptoms

- Webviews ignore `set_webview_position()` and `set_webview_size()` calls
- Multiple webviews stack vertically instead of positioning absolutely
- Works correctly on Windows and macOS, but fails on Linux
- Issue affects all GTK-based Tauri applications using multiwebview feature

### Affected Versions

- Tauri: v2.x (all versions up to at least v2.9.x)
- tao: v0.33.x and later
- Platform: Linux (GTK-based systems)

### Related Issues

- https://github.com/tauri-apps/tauri/issues/10420
- https://github.com/tauri-apps/tauri/issues/13071

## Root Cause

The issue stems from GTK's container system and how Tauri manages child webviews:

1. **GTK Container Behavior**: Tauri on Linux uses `gtk::Box` (vertical box layout) as the default container for webviews
2. **Automatic Layout**: `gtk::Box` automatically manages child widget positioning, overriding any manual position settings
3. **Missing Absolute Positioning**: GTK requires `gtk::Fixed` container for absolute positioning of child widgets
4. **API Gap**: The `tao` windowing library didn't expose the `gtk::Fixed` container to Tauri

### Technical Details

```rust
// PROBLEM: Using gtk::Box (automatic layout)
let vbox = window.default_vbox().unwrap();
webview_builder.build_gtk(vbox)
// GTK's Box widget ignores absolute positioning

// SOLUTION: Using gtk::Fixed (absolute positioning)
let fixed = window.fixed().unwrap();
webview_builder.build_gtk(fixed)
// GTK's Fixed widget respects absolute positioning
```

## The Fix

The fix requires coordinated changes across two libraries:

### 1. tao (Windowing Library)

**Repository**: https://github.com/Benjaminlooi/tao

**Branch**: `multiwebview-fix`

**Changes**:
- Add `fixed: Option<gtk::Fixed>` field to `Window` struct
- Create `gtk::Fixed` container during window initialization
- Expose `fixed()` method in `WindowExtUnix` trait API

**Modified Files**:
- `src/platform/unix.rs` - Added public API
- `src/platform_impl/linux/window.rs` - Implementation

**Code Changes**:

```rust
// src/platform/unix.rs
pub trait WindowExtUnix {
    // ... existing methods ...
    fn fixed(&self) -> Option<&gtk::Fixed>;
}

impl WindowExtUnix for Window {
    fn fixed(&self) -> Option<&gtk::Fixed> {
        self.window.fixed.as_ref()
    }
}
```

```rust
// src/platform_impl/linux/window.rs
pub struct Window {
    pub(crate) window: gtk::ApplicationWindow,
    pub(crate) default_vbox: Option<gtk::Box>,
    pub(crate) fixed: Option<gtk::Fixed>,  // NEW FIELD
    // ... other fields ...
}

// During window creation:
let fixed = match default_vbox {
    Some(ref vbox) => {
        let fixed = gtk::Fixed::new();
        vbox.pack_start(&fixed, true, true, 0);
        fixed.show_all();
        Some(fixed)
    },
    None => None,
};
```

### 2. tauri-runtime-wry (Tauri Runtime)

**Repository**: https://github.com/Benjaminlooi/tauri

**Branch**: `multiwebview-fix`

**Changes**:
- Replace `window.default_vbox()` with `window.fixed()` in webview creation
- Add Cargo.toml patch to use patched tao

**Modified Files**:
- `crates/tauri-runtime-wry/src/lib.rs` (lines 4763, 4788)
- `crates/tauri-runtime-wry/Cargo.toml`

**Code Changes**:

```rust
// crates/tauri-runtime-wry/src/lib.rs

// BEFORE (line 4763):
let vbox = window.default_vbox().unwrap();
webview_builder.build_gtk(vbox)

// AFTER:
let fixed = window.fixed().unwrap();
webview_builder.build_gtk(fixed)

// Same change on line 4788
```

```toml
# crates/tauri-runtime-wry/Cargo.toml
[patch.crates-io]
tao = { git = "https://github.com/Benjaminlooi/tao.git", branch = "multiwebview-fix" }
```

## Implementation in Your Project

### Using the Patches

Add the following to your `Cargo.toml`:

```toml
# Git patches for Linux multiwebview positioning fix
# See: https://github.com/tauri-apps/tauri/issues/10420
[patch.crates-io]
tao = { git = "https://github.com/Benjaminlooi/tao.git" }
tauri-runtime = { git = "https://github.com/Benjaminlooi/tauri.git" }
tauri-runtime-wry = { git = "https://github.com/Benjaminlooi/tauri.git" }
tauri-utils = { git = "https://github.com/Benjaminlooi/tauri.git" }
```

**Note**: The patches track the default branch and will stay compatible with your Tauri version's dependency requirements.

### Update Dependencies

```bash
# Update to use patched versions
cargo update -p tao -p tauri-runtime -p tauri-runtime-wry -p tauri-utils

# Verify patches are applied
cargo tree -p tao --depth 0
cargo tree -p tauri-runtime-wry --depth 0
```

You should see output like:
```
tao v0.33.0 (https://github.com/Benjaminlooi/tao.git#...)
tauri-runtime-wry v2.6.0 (https://github.com/Benjaminlooi/tauri.git#...)
```

### Build and Test

```bash
# Clean build to ensure patches are applied
cargo clean
cargo build

# Or run development server
npm run tauri dev
```

## Verification

### Testing the Fix

1. **Create Multiple Webviews**: Create 2-3 webviews with different positions
2. **Set Explicit Positions**: Use `set_webview_position()` with distinct coordinates
3. **Verify Layout**: Webviews should appear at specified positions, not stacked vertically

### Expected Behavior

**Before Fix**:
```
┌─────────────────┐
│   Webview 1     │
├─────────────────┤
│   Webview 2     │  <- All stacked vertically
├─────────────────┤
│   Webview 3     │
└─────────────────┘
```

**After Fix**:
```
┌─────────────────────────┐
│  ┌────┐        ┌────┐   │
│  │ W1 │        │ W2 │   │  <- Positioned as specified
│  └────┘        └────┘   │
│          ┌────┐         │
│          │ W3 │         │
│          └────┘         │
└─────────────────────────┘
```

## Version Compatibility

### Tested Versions

| Component | Version | Status |
|-----------|---------|--------|
| tauri | 2.5.1 | ✅ Working |
| tao | 0.33.0 | ✅ Patched |
| tauri-runtime-wry | 2.6.0 | ✅ Patched |
| GTK | 3.24+ | ✅ Compatible |

### Future Versions

The patches are maintained on GitHub and should work with future Tauri versions until the fix is merged upstream. Monitor the issue tracker for official releases that include this fix.

## Troubleshooting

### Patches Not Applied

**Symptom**: Cargo warnings about unused patches

```
warning: Patch `tao v0.33.0` was not used in the crate graph
```

**Solution**: Version mismatch. Check your Tauri version's dependency requirements:

```bash
cargo tree -i tao
cargo tree -i tauri-runtime-wry
```

Update the patch git URLs to match the required versions, or pin your Tauri version.

### Build Errors

**Symptom**: Trait bound errors or missing methods

**Solution**:
1. Clean build directory: `cargo clean`
2. Update dependencies: `cargo update`
3. Verify patches: `cargo tree -p tao`

### Webviews Still Stacking

**Symptom**: Webviews still appear vertically stacked after applying patches

**Possible Causes**:
1. Old build artifacts - Run `cargo clean`
2. Patches not applied - Verify with `cargo tree`
3. Platform-specific code path - Check that your code runs the GTK path
4. Window created with `with_default_vbox(false)` - Fix requires default vbox enabled

### Webviews Lose Position on Window Resize

**Symptom**: Webviews move to the top-left corner when the main window is resized

**Root Cause**: Using separate `set_position()` and `set_size()` calls doesn't properly trigger GTK Fixed container repositioning

**Solution**: Use `set_bounds()` instead, which properly handles GTK Fixed container updates:

```rust
// BEFORE (incorrect for GTK Fixed)
webview.set_position(Position::Logical(position))?;
webview.set_size(Size::Logical(size))?;

// AFTER (correct for GTK Fixed)
let bounds = Rect {
    position: Position::Logical(position),
    size: Size::Logical(size),
};
webview.set_bounds(bounds)?;
```

When the window is resized, the frontend must recalculate webview positions and call `set_bounds()` to update all webviews. The `set_bounds()` method internally calls `gtk_fixed_move()` and properly updates child widget positions within the Fixed container.

**Implementation in ChenChen**:
- Frontend listens for window resize events (src/routes/+page.svelte:38)
- Debounced handler recalculates panel bounds (src/routes/+page.svelte:86-95)
- Backend command uses `set_bounds()` for updates (src-tauri/src/commands.rs:289-291)

## Performance Impact

The fix has minimal performance impact:

- **Memory**: One additional `gtk::Fixed` widget per window (~negligible)
- **CPU**: No measurable overhead
- **Compatibility**: No breaking changes to existing API

## Contributing

If you encounter issues with these patches:

1. **Bug Reports**: Open issues on the respective GitHub repositories
2. **Improvements**: Submit PRs to the patch branches
3. **Upstream**: Encourage Tauri maintainers to merge the fix

## Credits

- **Original Issue**: Reported by community members on GitHub issues #10420, #13071
- **Fix Implementation**: Based on community investigation and GTK documentation
- **Patches Maintained By**: Benjaminlooi (https://github.com/Benjaminlooi)

## References

- [Tauri Issue #10420](https://github.com/tauri-apps/tauri/issues/10420)
- [Tauri Issue #13071](https://github.com/tauri-apps/tauri/issues/13071)
- [GTK Fixed Container Documentation](https://docs.gtk.org/gtk3/class.Fixed.html)
- [tao Windowing Library](https://github.com/tauri-apps/tao)
- [Patched tao Repository](https://github.com/Benjaminlooi/tao)
- [Patched tauri Repository](https://github.com/Benjaminlooi/tauri)

## License

These patches maintain the same license as the original Tauri and tao projects (Apache-2.0/MIT).

---

**Last Updated**: 2025-11-19
**Tauri Version**: 2.9.3
**Status**: Active patch with resize fix, awaiting upstream merge
