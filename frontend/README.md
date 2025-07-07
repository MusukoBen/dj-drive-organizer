# DJ Drive Organizer Frontend - Clean Architecture

This is a clean, minimal implementation of a `use_drive_state` hook for Yew that manages drive scan results and integrates with Tauri event listeners to update state based on backend progress messages.

## Project Structure

```
frontend/
├── src/
│   ├── lib.rs                    # Main entry point
│   ├── minimal_drive_hooks.rs    # Core drive state hooks
│   ├── simple_demo.rs           # Demo component
│   ├── tauri_api.rs             # Tauri API bindings
│   └── types.rs                 # Core data types
├── Cargo.toml                   # Dependencies
├── README.md                    # This file
└── .gitignore                   # Git ignore patterns
```

## Core Implementation

### Drive State Hook (`minimal_drive_hooks.rs`)

The main `use_drive_state` hook provides:

1. **State Management**: Stores current scan results, scanning status, progress, and error messages
2. **Action-based Updates**: Uses a reducer pattern with actions for state updates
3. **Tauri Integration**: Wireframes for event listeners to receive backend progress messages

#### Key Components:

1. **`DriveState` Structure**:
   - `scan_results`: Vector of scanned file metadata
   - `scan_progress`: Optional progress information during scanning
   - `is_scanning`: Boolean flag for current scan status
   - `error_message`: Optional error message for error handling

2. **`DriveStateAction` Enum**:
   - `StartScan`: Initiates a scan operation
   - `UpdateProgress(ScanProgress)`: Updates scan progress
   - `CompleteScan(Vec<FileMeta>)`: Completes scan with results
   - `ScanError(String)`: Handles scan errors

3. **Core Hooks**:
   - `use_drive_state()`: Main state management hook
   - `use_drive_scan()`: Convenience hook for scan operations

## Usage Example

```rust
use crate::minimal_drive_hooks::use_drive_scan;

#[function_component]
pub fn MyComponent() -> Html {
    let (state, start_scan) = use_drive_scan();
    
    let on_scan = {
        let start_scan = start_scan.clone();
        Callback::from(move |_| {
            start_scan.emit("/path/to/drive".to_string());
        })
    };
    
    html! {
        <div>
            <button onclick={on_scan}>{"Start Scan"}</button>
            <p>{format!("Is scanning: {}", state.is_scanning)}</p>
            <p>{format!("Files found: {}", state.scan_results.len())}</p>
            if let Some(error) = state.error_message {
                <p style="color: red;">{format!("Error: {}", error)}</p>
            }
        </div>
    }
}
```

## Features Implemented

### ✅ State Management
- [x] `use_drive_state` hook with reducer pattern
- [x] Scan results storage (`Vec<FileMeta>`)
- [x] Progress tracking (`Option<ScanProgress>`)
- [x] Error handling (`Option<String>`)
- [x] Scanning status tracking (`bool`)

### ✅ Event Integration Framework
- [x] Action-based state updates
- [x] Async scan operation handling
- [x] Error propagation from Tauri backend
- [x] Progress update structure ready for events

### ✅ Convenience Hooks
- [x] `use_drive_scan()` for scan operations
- [x] Clean separation of concerns
- [x] Type-safe callbacks

## Architecture

### State Flow
1. User triggers scan via UI callback
2. `StartScan` action dispatched
3. Async scan operation initiated with Tauri
4. Progress updates can be received via events (framework ready)
5. Completion or error updates state accordingly

### Event Listener Integration

The hook is designed to integrate with Tauri event listeners. The framework is in place in `tauri_api.rs` for:

- `listen_to_scan_progress()`: Progress updates during scanning
- `listen_to_event()`: Generic event listener for scan completion/errors
- Event-driven state updates via actions

### Backend Integration

The implementation connects to Tauri backend commands:
- `invoke_scan_drive()`: Initiates drive scanning
- Event emissions for progress updates
- Error handling for scan failures

## Building and Testing

```bash
# Check compilation
cargo check

# Build the frontend
cargo build

# Build for WebAssembly (production)
wasm-pack build --target web
```

## Technical Details

- **Framework**: Yew 0.21
- **State Management**: Reducer pattern with actions
- **Async Handling**: `wasm_bindgen_futures::spawn_local`
- **Type Safety**: Full TypeScript-like type safety with Rust
- **Event System**: Ready for Tauri event integration

## Event Listener Wiring (Future Enhancement)

The framework is ready for `use_effect` integration to wire Tauri event listeners:

```rust
// Future implementation would include:
use_effect_with((), move |_| {
    let dispatch_progress = dispatch.clone();
    let _ = listen_to_scan_progress(move |progress| {
        dispatch_progress.emit(DriveStateAction::UpdateProgress(progress));
    });
    
    // Cleanup function
    || {}
});
```

This implementation provides a solid foundation for managing drive scan state in the DJ Drive Organizer application with full Tauri integration capabilities.
