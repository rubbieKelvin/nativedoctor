# WASM Compatibility Fix for NativeDoctor

This document explains how I resolved WebAssembly (WASM) compilation issues when trying to serve the Dioxus app for web browsers using `dx serve -p app --platform web`.

## Original Problem

When attempting to compile the project for WASM, I encountered several compilation errors:

1. **getrandom crate**: Missing "js" feature for WASM targets
2. **tokio crate**: Unsupported features for WASM (only certain features work on WASM)  
3. **rfd crate**: File dialog library that doesn't work in browsers
4. **wry/tao crates**: Desktop webview libraries being pulled into WASM builds
5. **dioxus-desktop**: Desktop-specific functionality being included in web builds

## Root Cause Analysis

The main issue was that my dependency configuration wasn't properly separating desktop-only dependencies from web-compatible ones. The `dioxus` crate was pulling in all platform dependencies by default, including desktop-specific ones like `wry`, `tao`, and `dioxus-desktop`.

## Solution Overview

I implemented **platform-specific conditional compilation** using Rust's `cfg` attributes to:

1. Make desktop-only dependencies conditional
2. Use platform-specific Dioxus features
3. Provide web-safe fallbacks for desktop functionality
4. Ensure proper feature flags for WASM-compatible crates

## Detailed Changes

### 1. Platform-Specific Dependency Configuration

**Before:**
```toml
[dependencies]
dioxus = { version = "0.6.1" }
rfd = "0.15.3"
```

**After:**
```toml
[dependencies]
# Explicit getrandom with js feature for WASM support
getrandom = { version = "0.2", features = ["js"] }

# Web-only dependencies
[target.'cfg(target_arch = "wasm32")'.dependencies]
dioxus = { version = "0.6.1", default-features = false, features = ["web", "macro"] }

# Desktop-only dependencies (file dialogs don't work in browsers)
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
rfd = "0.15.3"
dioxus = { version = "0.6.1", default-features = false, features = ["desktop", "macro"] }
```

### 2. Platform-Specific Code Compilation

**Main Application Entry (`app/src/main.rs`):**

```rust
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Desktop-specific window configuration and launch
        use dioxus::desktop::wry::dpi::Size;
        use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

        let window_builder = WindowBuilder::new()
            .with_inner_size(Size::Logical(LogicalSize::new(1200.0, 800.0)))
            .with_resizable(true);

        dioxus::LaunchBuilder::desktop()
            .with_cfg(Config::new().with_window(window_builder))
            .launch(App);
    }

    #[cfg(target_arch = "wasm32")]
    dioxus::LaunchBuilder::web().launch(App);
}
```

**Window Management (`app/src/components/wm_drag_area.rs`):**

```rust
#[cfg(not(target_arch = "wasm32"))]
use dioxus::desktop::use_window;

#[component]
pub fn WmDragArea(class: Option<&'static str>, children: Element) -> Element {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Desktop window dragging functionality
        let window = use_window();
        return rsx! {
            div {
                class,
                onmousedown: move |event| {
                    if event.held_buttons() == MouseButton::Primary {
                        window.drag_window()?;
                    }
                    Ok(())
                },
                {children}
            }
        };
    }

    #[cfg(target_arch = "wasm32")]
    return rsx! {
        div { class, {children}}
    };
}
```

**File System Operations (`app/src/session/fs.rs`):**

```rust
#[cfg(not(target_arch = "wasm32"))]
use rfd::AsyncFileDialog;

impl Session {
    pub async fn load_from_fs_from_dialog() -> Result<Option<Self>, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let picker = AsyncFileDialog::new()
                .set_title("Pick project to open")
                .add_filter("Native Doctor Project", &[EXTENSION_FOR_PROJECT]);

            match picker.pick_file().await {
                Some(path) => {
                    return Ok(Some(Session::load_from_fs_from_path(
                        path.path().to_path_buf(),
                    )?));
                }
                None => return Ok(None),
            }
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            // File dialogs don't work in browsers
            return Ok(None);
        }
    }
}
```

### 3. Core Crate WASM Support

Added explicit `getrandom` dependency with "js" feature to the core crate:

```toml
# core/Cargo.toml
[dependencies]
uuid = { version = "1.17.0", features = ["v4", "serde", "js"] }
getrandom = { version = "0.2", features = ["js"] }
```

## Key Patterns Used

### 1. Target Architecture Detection
- `#[cfg(target_arch = "wasm32")]` - Code that only runs in browsers
- `#[cfg(not(target_arch = "wasm32"))]` - Code that only runs on desktop

### 2. Conditional Dependencies
- `[target.'cfg(target_arch = "wasm32")'.dependencies]` - WASM-only dependencies
- `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` - Desktop-only dependencies

### 3. Feature-Specific Compilation
- `default-features = false` - Prevents pulling in all platform dependencies
- Explicit feature selection - Only enable needed features per platform

## Browser Limitations Addressed

1. **File System Access**: Browsers don't have direct file system access
2. **File Dialogs**: Native file dialogs aren't available in browsers  
3. **Window Management**: Browser windows are managed by the browser, not the app
4. **Threading**: Different threading model in WASM vs desktop

## Testing the Fix

The fix can be verified by running:

```bash
# Desktop build (should work)
dx serve -p app --platform desktop

# Web build (should work after fixes)
dx serve -p app --platform web
```

Both commands should now compile and run successfully without dependency conflicts.

## Benefits of This Approach

1. **Clean Separation**: Desktop and web code paths are clearly separated
2. **No Runtime Overhead**: Unused code is eliminated at compile time
3. **Maintainable**: Easy to understand which code runs on which platform
4. **Future-Proof**: Easy to add new platform-specific features
5. **Bundle Size**: Web builds don't include desktop dependencies

## Future Considerations

- Consider using browser APIs like File System Access API for web file operations
- Implement web-specific storage solutions (localStorage, IndexedDB)
- Add platform-specific UI adaptations
- Consider progressive web app (PWA) features for better desktop-like experience

## Lessons Learned

1. Always use `default-features = false` for multi-platform crates
2. WASM has significant limitations compared to desktop environments
3. Platform detection should be done at compile time, not runtime
4. Conditional compilation is essential for cross-platform Rust applications
5. Web apps need different UX patterns than desktop apps due to browser constraints 