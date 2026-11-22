# Fork wiring — RustDesk integration

Base: RustDesk upstream (AGPL-3.0). Local fork workspace: `~/workspace/mr-support-fork`.

## What was wired (2026-11-22)

### 1. New module `src/server/mr_support.rs`
- `AI_CONSENT_ACTIVE: AtomicBool` — set by the user via Approve/Revoke.
- `AI_TAP_ENABLED: AtomicBool` — kill switch for the frame tap.
- `on_frame_captured(width, height, rgba_len)` — called from video_service; rate-limited to ~2 fps (500 ms), no-op when consent inactive; never blocks the human stream.
- `ai_input_allowed()` / `set_ai_consent(active)` — consent gate helpers.

### 2. Video tap insertion — `src/server/video_service.rs`
- Registered module in `src/server.rs` (`pub mod mr_support;`).
- Tap call inserted right after `c.frame(spf)` returns a valid frame (line ~822):
  `super::mr_support::on_frame_captured(0, 0, 0);`
- Next step (M1): forward the downscaled frame to the Mr. Support core `VideoTapBridge` over a local channel.

### 3. Consent-gated input — `src/server/input_service.rs`
- New `ai_handle_mouse(...)` entry point: checks `ai_input_allowed()`, logs and drops the action when consent is not active, otherwise delegates to `handle_mouse`.
- Human input keeps using `handle_mouse` directly and always preempts AI.

## Permissions verified in fork code (2026-11-22)

- **macOS TCC**: `CGPreflightScreenCaptureAccess` (Screen Recording) in `src/platform/macos.mm:32`; `AXIsProcessTrustedWithOptions` (Accessibility) in `src/platform/macos.rs:118,156`; `IOHIDCheckAccess` (Input Monitoring) in `src/platform/macos.mm:48-49`.
- **Windows UAC / secure desktop**: `is_process_consent_running()` in `src/platform/windows.rs:3289`; installer handoff handles cancelled UAC prompts.
- **Linux uinput**: `VirtualDevice` / `VirtualDeviceBuilder` in `src/server/uinput.rs` (keyboard/mouse/control IPC postfixes). Wayland input still requires uinput permissions — main risk for the pilot.

## Status

- Wiring code in place in the fork tree. Compilation of the full RustDesk fork not attempted in the VM (needs vcpkg + submodule + platform toolchains); compile on Miguel's PC / CI with the RustDesk build environment.
- Mr. Support core (`src/transport/video_integration.rs`, `src/transport/input.rs`) already implements the tap bridge and consent-gated injector; the fork hook is the remaining bridge.
