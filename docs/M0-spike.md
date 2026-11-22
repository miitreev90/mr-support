# M0 Spike — RustDesk Base Inspection

**Date:** 2026-11-21  
**Cloned:** `https://github.com/rustdesk/rustdesk.git` (shallow, `--depth 1`) into `~/workspace/rustdesk-spike`  
**Purpose:** Verify licensing, architecture, and platform APIs before building Mr. Support on top of RustDesk.

> **Scope note:** This document records only what was verified by reading the cloned source. Items marked **PENDING** were not verified and must be confirmed before design decisions depend on them.

---

## 1. License Verification

### LICENCE file (`~/workspace/rustdesk-spike/LICENCE`)

- Header (line 1): `GNU AFFERO GENERAL PUBLIC LICENSE Version 3, 19 November 2007`
- The "How to Apply These Terms to Your New Programs" section (end of file) states: *"either version 3 of the License, or (at your option) any later version"* — this is the standard AGPL-3.0-or-later template language.
- **VERIFIED 2026-11-22:** GitHub API `GET /repos/rustdesk/rustdesk` returns `license.spdx_id = "AGPL-3.0"` (key `agpl-3.0`, name "GNU Affero General Public License v3.0"). The LICENCE file's template text says "or (at your option) any later version", but the official SPDX identifier as published by GitHub is **AGPL-3.0** (not `-or-later`). Mr. Support fork must use AGPL-3.0 to match the upstream SPDX. `hbb_common/Cargo.toml` also has no `license` field (same as root); `libs/scrap` and `libs/enigo` are MIT.

### Sub-crate licenses (verified from Cargo.toml)

| Crate | License field | Notes |
|---|---|---|
| `libs/scrap` | `MIT` | Screen capture library |
| `libs/enigo` | `MIT` | Input control library |
| `libs/base` | *(not shown in Cargo.toml read)* | **PENDING** — verify |

**Key implication for Mr. Support:** RustDesk itself is AGPL (network copyleft). The `scrap` and `enigo` libraries are MIT-licensed and can be reused independently. The AGPL obligation applies to the RustDesk application code, not to these two libraries.

---

## 2. Repository Structure (verified)

```
rustdesk/
├── src/
│   ├── main.rs              — entry point; dispatches to core_main or UI
│   ├── core_main.rs         — CLI arg parsing; server vs client mode
│   ├── client.rs            — Controller logic (initiates connection)
│   ├── rendezvous_mediator.rs — Controlled Host logic (registers with hbbs, handles punch/relay)
│   ├── server/              — Controlled Host services
│   │   ├── video_service.rs — screen capture + encoding loop
│   │   ├── input_service.rs — mouse/keyboard event handling
│   │   ├── audio_service.rs, clipboard_service.rs, display_service.rs, etc.
│   ├── platform/            — platform-specific code (macos, windows, linux, android, ios)
│   ├── ui/                  — legacy Sciter UI (deprecated)
│   └── flutter/             — current UI (Flutter)
├── libs/
│   ├── hbb_common/          — ⚠️ EMPTY in this clone (git submodule, not initialized)
│   ├── base/                — client-only crate (config keys, message proto, file transfer)
│   ├── scrap/               — screen capture (MIT)
│   ├── enigo/               — input control (MIT)
│   ├── clipboard/           — clipboard sync
│   ├── virtual_display/     — virtual display support
│   ├── portable/            — portable service (Windows)
│   └── remote_printer/      — remote printer support
├── flutter/                 — Flutter UI (desktop, mobile, common, models)
├── Cargo.toml               — workspace root
├── LICENCE                  — AGPL-3.0
└── .gitmodules              — submodule definition for libs/hbb_common
```

### `libs/hbb_common` — VERIFIED 2026-11-22 (submodule initialized)

- `git submodule update --init libs/hbb_common` succeeded; checked out commit `fae47e024dd83cef6aaecde0aa7d11731df11649` from `https://github.com/rustdesk/hbb_common`.
- **Proto definitions:** `protos/rendezvous.proto` (proto3, package `hbb`, 30 messages). Key messages: `IdPk`, `RegisterPeer`, `PunchHoleRequest`, `PunchHole`, `RendezvousMessage` (oneof envelope), `KeyExchange`/`KxParams`, `RequestRelay`/`RelayResponse`, `PeerDiscovery`, `OnlineRequest`/`OnlineResponse`. Enums: `ConnType` (DEFAULT, FILE_TRANSFER, PORT_FORWARD, RDP, VIEW_CAMERA, TERMINAL), `NatType`. `ControlPermissions` bitmask includes keyboard, clipboard, file, audio, camera, terminal, tunnel, restart, recording, block_input, privacy_mode, usb. Generated via `protobuf-codegen` in `build.rs` (`pure()` mode, tokio_bytes).
- **Config core (`src/config.rs`):** `Config` (id, enc_id, password, salt, key_pair, key_confirmed, keys_confirmed), `Config2` (rendezvous_server, nat_type, serial, unlock_pin, trusted_devices, socks, options HashMap), `PeerConfig` (password, size, view_style, scroll_style, etc.), `Socks5Server`, `Resolution`. Globals via `lazy_static` RwLocks: `CONFIG`, `CONFIG2`, `LOCAL_CONFIG`, `STATUS`, `ONLINE`, `APP_NAME` (default "RustDesk"), `APP_DIR`. Constants: `RENDEZVOUS_SERVERS = ["rs-ny.rustdesk.com"]`, `RENDEZVOUS_PORT = 21116`, `RELAY_PORT = 21117`, `WS_RENDEZVOUS_PORT = 21118`, `WS_RELAY_PORT = 21119`, `RS_PUB_KEY`, timeouts 12s/18s. Config stored as TOML/JSON via `confy`-style load/store (`load_path`/`store_path`). Per AGENTS.md, client-only options belong in `libs/base`, not here (shared with server — changes cost a round-trip).
- **Socket layer:** `src/tcp.rs` (`FramedStream` over `DynTcpStream` + `BytesCodec`, sodiumoxide secretbox encryption, key exchange v0/v1 with transcript binding), `src/udp.rs` (`FramedSocket` Direct/ProxySocks), `src/socket_client.rs` (host:port helpers, server validation), `src/stream.rs` (`Stream` enum: WebRTC / WebSocket / Tcp), `src/websocket.rs`, `src/webrtc.rs` (feature-gated, pinned to 0.13 for Rust 1.75 compat), `src/tls.rs`, `src/proxy.rs`, `src/compress.rs` (zstd). `src/lib.rs` re-exports tokio, protobuf, futures, sodiumoxide, etc. and defines `SessionID = uuid::Uuid`.
- **Implication for Mr. Support:** Adding agent messages (e.g. `AgentActionRequest`) to `rendezvous.proto` would require modifying the submodule shared with the server — prefer client-only code in `libs/base` per AGENTS.md, or a separate sidecar protocol. Frame tap and input injection points remain in `src/server/video_service.rs` and `input_service.rs` (see §3).

### Workspace members (from root `Cargo.toml`)

```toml
members = [
    "libs/scrap",
    "libs/hbb_common",
    "libs/base",
    "libs/enigo",
    "libs/clipboard",
    "libs/virtual_display",
    "libs/virtual_display/dylib",
    "libs/portable",
    "libs/remote_printer"
]
exclude = ["vdi/host"]
```

### Root package

- Name: `rustdesk`, version `1.5.3`, edition `2021`, rust-version `1.75`
- Lib: `librustdesk`, crate-type `["cdylib", "staticlib", "rlib"]`
- Bins: `naming` (`src/naming.rs`), `service` (`src/service.rs`)
- Default features: `["use_dasp"]`
- Notable features: `flutter`, `hwcodec`, `vram`, `mediacodec`, `drm`, `screencapturekit`, `unix-file-copy-paste`

---

## 3. Controller / Controlled Host Separation

RustDesk is a **single binary** that acts as both Controller and Controlled Host, with the role determined by CLI arguments and runtime state.

### Entry point (`src/main.rs`)

- On Android/iOS/Flutter: calls `common::global_init`, `test_rendezvous_server`, `test_nat_type`.
- Otherwise: calls `core_main::core_main()`, then either `ui::start()` or exits.

### Role dispatch (`src/core_main.rs`)

| Argument | Behavior |
|---|---|
| `--server` | Starts Controlled Host server (`start_server(true, false)`); on macOS also spawns server thread + tray |
| `--tray` | Tray mode |
| `--service` | Service mode |
| `--install-service` | Install as system service |
| `--connect` / `--play` / `--file-transfer` | Initiates a new connection (Controller mode) |
| *(no args)* | Spawns `start_server(false, no_server)` in a thread, then starts UI |

### Controller (`src/client.rs`)

- `Client::start()` initiates a peer connection.
- Handles rendezvous server lookup, NAT traversal (UDP punch, TCP punch, IPv6, WebRTC), relay fallback.
- Uses `scrap::codec::Decoder` for video decoding.
- **Insertion point for Mr. Support:** The Controller receives decoded video frames via `scrap`. A raw-frame tap before encoding/decoding is the natural place for AI vision input.

### Controlled Host (`src/rendezvous_mediator.rs` + `src/server/`)

- `RendezvousMediator::start_all()` registers the peer with hbbs, handles punch-hole and relay requests, and spawns the server.
- `src/server/video_service.rs` runs the capture loop: `Capturer::frame()` → encode → send.
- `src/server/input_service.rs` receives mouse/keyboard events from the Controller and applies them via `ENIGO`.
- **Insertion point for Mr. Support:** `input_service.rs` is where AI actions would be injected (after the consent/approval validator). `video_service.rs` is where the frame tap for AI vision would go.

---

## 4. Screen Capture APIs (`libs/scrap`)

### Core trait (`libs/scrap/src/common/mod.rs`)

```rust
pub trait TraitCapturer {
    fn frame<'a>(&'a mut self, timeout: Duration) -> io::Result<Frame<'a>>;
    // Windows-only:
    fn is_gdi(&self) -> bool;
    fn set_gdi(&mut self) -> bool;
}
```

- `Frame` enum: `PixelBuffer(PixelBuffer)` or `Texture((*mut c_void, usize))`
- `Frame::to(yuvfmt, yuv, mid_data)` converts to YUV for encoding.
- `Pixfmt` variants: `BGRA`, `RGBA`, `RGB565LE`, `I420`, `NV12`, `I444`
- `CodecFormat` variants: `VP8`, `VP9`, `AV1`, `H264`, `H265`

### Platform selection (`libs/scrap/src/common/mod.rs`, `cfg_if!` block)

The backend is chosen at compile time via `cfg` flags set in `libs/scrap/build.rs`:
- macOS → `quartz`
- Windows → `dxgi`
- Linux/Unix → `x11` (with optional `wayland` feature)
- Android → `android`

### macOS capture (`libs/scrap/src/quartz/`)

- **Backend:** `CGDisplayStreamCreateWithDispatchQueue` + `CGDisplayStreamStart` (legacy Quartz display stream API).
- **Files:** `capturer.rs`, `config.rs`, `display.rs`, `ffi.rs`, `frame.rs`, `mod.rs`
- `quartz/capturer.rs`: Creates a display stream with a dispatch queue; the handler receives `IOSurfaceRef` on `FrameComplete`.
- `quartz/ffi.rs`: Links `CoreGraphics`, `CoreFoundation`, `IOSurface`. Defines `CGDisplayStreamCreateWithDispatchQueue`, `CGDisplayStreamStart/Stop`, `CGMainDisplayID`, `IOSurfaceGetBaseAddress`, etc.
- **⚠️ Correction to plan (VERIFIED 2026-11-22):** Video capture on macOS uses **CGDisplayStream** (`CGDisplayStreamCreateWithDispatchQueue` in `libs/scrap/src/quartz/capturer.rs`), NOT ScreenCaptureKit. ScreenCaptureKit exists only as a Cargo feature flag (`screencapturekit = ["cpal/screencapturekit"]` in root `Cargo.toml:48`, with `cpal` from branch `osx-screencapturekit`) and is used **only for audio capture** via cpal (`src/server/audio_service.rs:185,215,378` — `cpal::HostId::ScreenCaptureKit`, `HOST_SCREEN_CAPTURE_KIT`). There is **no ScreenCaptureKit video backend** in `libs/scrap`. Error handling for ScreenCaptureKit audio stop is in `src/server/audio_service/audio_capture_error.rs` (error -3821). `src/flutter_ffi.rs:2750` gates a function on the feature. Conclusion: flag is real but audio-only; video migration to ScreenCaptureKit would be new work.
- **TCC permissions (VERIFIED 2026-11-22):** See §4b below. Screen Recording check uses `CGPreflightScreenCaptureAccess()` / `CGRequestScreenCaptureAccess()` via `src/platform/macos.mm` (`IsCanScreenRecording`), gated to macOS >= 11 via `CanUseNewApiForScreenCaptureCheck()`. Fallback heuristic via `CGWindowListCopyWindowInfo` in `src/platform/macos.rs:167-260` (`is_can_screen_recording`). Accessibility (input) uses `AXIsProcessTrustedWithOptions` with `kAXTrustedCheckOptionPrompt` in `src/platform/macos.rs:117-156` (`is_process_trusted`). Input Monitoring uses `IOHIDCheckAccess`/`IOHIDRequestAccess` in `macos.mm`. No TCC code in `libs/scrap/src/quartz/` or `libs/enigo/src/macos/` — checks live in `src/platform/macos.rs` + `macos.mm`.

### Windows capture (`libs/scrap/src/dxgi/`)

- **Primary:** DXGI Desktop Duplication via `D3D11CreateDevice` + `IDXGIOutputDuplication` (`DuplicateOutput` / `DuplicateOutput1` for HDR float format).
- **Flow:** `AcquireNextFrame` → `MapDesktopSurface` / staging texture copy → rotation via D3D11 video processor.
- **Fallback:** GDI (`CapturerGDI` in `src/dxgi/gdi.rs`) if DXGI fails.
- **Files:** `mod.rs`, `gdi.rs`, `hdr.rs`, `mag.rs` (magnification API for privacy mode)
- **Recovery:** `video_service.rs` implements DXGI recovery (up to 3 restarts in 10s, then falls back to GDI).

### Linux capture (`libs/scrap/src/common/`)

- **X11:** `x11.rs` — standard X11 screen capture.
- **Wayland:** `wayland.rs` + `wayland/` (PipeWire) — requires the `wayland` feature.
- **DRM:** `drm_reader.rs`, `drm_render.rs`, `drmtap_dl.rs` — runtime-dlopen of `libdrmtap.so.0`, requires `drm` feature (which requires `wayland`).
- **Selection:** `is_x11()` in `common/mod.rs` delegates to `base::platform::linux::is_x11_or_headless()`.
- **Wayland limitation:** Only one video capturer supported; multi-screen capture not supported on Wayland (noted in `video_service.rs` comments).

### Android capture (`libs/scrap/src/common/android.rs`)

- Uses Android MediaProjection via JNI. **PENDING:** detailed API not inspected in this pass.

### Build requirements (`libs/scrap/build.rs`)

- Emits `cargo:rustc-cfg=dxgi` on Windows, `quartz` on macOS, `x11` on Unix.
- Requires vcpkg libraries: `libyuv`, `libvpx`, `aom` (bindgen).

---

### 4b. macOS TCC Permissions (VERIFIED 2026-11-22)

Three distinct TCC checks, all in `src/platform/macos.rs` + `src/platform/macos.mm` (none in `libs/scrap/src/quartz/` or `libs/enigo/src/macos/`):

| Permission | API | Location | Prompt behavior |
|---|---|---|---|
| Screen Recording | `CGPreflightScreenCaptureAccess()` / `CGRequestScreenCaptureAccess()` | `macos.mm:32-34` (`IsCanScreenRecording`), exposed via `IsCanScreenRecording()` FFI | `prompt=true` calls `CGRequestScreenCaptureAccess()` (shows system prompt); macOS >= 11 only (`CanUseNewApiForScreenCaptureCheck`, `macos.mm:14-21`). Fallback: `CGWindowListCopyWindowInfo` heuristic in `macos.rs:167-260` for older macOS. |
| Accessibility (input injection) | `AXIsProcessTrustedWithOptions` + `kAXTrustedCheckOptionPrompt` | `macos.rs:117-156` (`is_process_trusted(prompt)`) | `prompt=true` sets `kAXTrustedCheckOptionPrompt = YES`, which prompts user to open System Settings → Privacy & Security → Accessibility. |
| Input Monitoring | `IOHIDCheckAccess(kIOHIDRequestTypeListenEvent)` / `IOHIDRequestAccess` | `macos.mm` (`InputMonitoringAuthStatus`) | On denied, opens `x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent` URL. |

**Implication for Mr. Support:** Onboarding must check all three before starting capture/input, and guide the user to grant them in System Settings. The Screen Recording check should use the new API (`CGPreflightScreenCaptureAccess`) on macOS >= 11, with the window-list heuristic as fallback. Note the comment in `macos.rs`: `tccutil reset All com.carriez.rustdesk` resets all permissions for testing. Bundle ID matters: RustDesk uses `com.carriez.rustdesk` / ORG `com.carriez` (config.rs) — Mr. Support will need its own bundle ID, so TCC grants will not carry over.

---

## 5. Input Control APIs (`libs/enigo`)

### Core traits (`libs/enigo/src/lib.rs`)

```rust
pub trait MouseControllable {
    fn mouse_move_to(&mut self, x: i32, y: i32);
    fn mouse_move_relative(&mut self, x: i32, y: i32);
    fn mouse_down(&mut self, button: MouseButton);
    fn mouse_up(&mut self, button: MouseButton);
    fn mouse_click(&mut self, button: MouseButton);
    fn mouse_scroll_x(&mut self, length: i32);
    fn mouse_scroll_y(&mut self, length: i32);
}

pub trait KeyboardControllable {
    fn key_sequence(&mut self, sequence: &str);
    fn key_down(&mut self, key: Key);
    fn key_up(&mut self, key: Key);
    fn key_click(&mut self, key: Key);
    fn get_key_state(&mut self, key: Key) -> bool;
}
```

- `MouseButton`: `Left`, `Middle`, `Right`, `Back`, `Forward`, `Scroll*`
- `Key`: `Alt`, `Backspace`, `CapsLock`, `Command`, `Control`, `Delete`, …, `Layout(char)`, `Raw(u16)`
- Platform modules gated by `target_os`: `src/win/`, `src/macos/`, `src/linux/`. On Android/iOS a stub `Enigo` struct exists.
- Crate version: `0.0.14`, license `MIT`.

### macOS input (`libs/enigo/src/macos/macos_impl.rs`)

- **Backend:** CoreGraphics `CGEvent` (`CGEvent::new_mouse_event`, `new_keyboard_event`, `CGEventPost` with `CGEventTapLocation::HID`, `CGEventSource` CombinedSessionState).
- `ENIGO_INPUT_EXTRA_VALUE = 100` (used to distinguish synthetic events).
- Scroll: `CGEventCreateScrollWheelEvent`.
- Keyboard layout: `UCKeyTranslate` + `TISCopyCurrentKeyboardInputSource`.
- Double-click interval from `NSEvent`.
- **Accessibility permission (VERIFIED 2026-11-22):** `CGEventPost` requires Accessibility trust. RustDesk checks via `AXIsProcessTrustedWithOptions` + `kAXTrustedCheckOptionPrompt` in `src/platform/macos.rs:117-156` (`is_process_trusted(prompt)`), which can prompt the user to open System Settings. Input Monitoring (`IOHIDCheckAccess`) is checked separately in `macos.mm`. Mr. Support must replicate this check before injecting input on macOS.

### Windows input (`libs/enigo/src/win/win_impl.rs`)

- **Backend:** `SendInput` with `INPUT_MOUSE` / `INPUT_KEYBOARD`.
- Absolute mouse moves: `MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK`, clamped to 0..65535.
- Keyboard layout: `MapVirtualKeyExW` / `VkKeyScanExW` for layout-dependent keys.
- `ENIGO_INPUT_EXTRA_VALUE = 100` as `dwExtraInfo`.
- Unicode input via `KEYEVENTF_UNICODE` with UTF-16 surrogate handling.
- **PENDING:** Secure desktop (UAC) limitation — not verified in this read. `SendInput` cannot inject into secure desktop prompts; RustDesk handles this via the portable service / consent.exe (seen in `video_service.rs` references to `is_process_consent_running`).

### Linux input (`libs/enigo/src/linux/`)

- **X11:** Routes to `xdo` (libxdo-sys, patched to a stub that dynamically loads libxdo; null xdo = silent no-ops). Uses `xdo_move_mouse`, `xdo_click_window`, `xdo_enter_text_window`, delay 12000 µs.
- **Wayland:** No direct XTest. Uses uinput (`src/server/uinput.rs`) or RDP input (`src/server/rdp_input.rs`) via `setup_uinput` / `setup_rdp_input` in `src/server/input_service.rs`. Custom keyboard/mouse devices set via `set_custom_keyboard` / `set_custom_mouse`.
- **⚠️ Major risk:** Wayland input is the highest-risk platform path. It requires uinput device creation (permissions) or an RDP bridge. **PENDING:** verify uinput permission requirements and Wayland portal behavior.

### Server-side input handling (`src/server/input_service.rs`)

- Global: `static ref ENIGO: Arc<Mutex<Enigo>>`
- Functions: `handle_mouse`, `handle_mouse_simulation_`, `handle_pointer_`
- macOS: mouse handling runs on main dispatch queue (`QUEUE.exec_async`).
- Windows: via portable_service client.
- Linux: remaps Wayland uinput coords.
- **This is the insertion point for the AI approval validator:** every peer action goes through `ENIGO.lock()` here. An AI action layer would call the same `ENIGO` instance after passing consent checks.

---

## 6. `libs/base` Crate (verified)

- **Purpose:** Client-only code (per AGENTS.md: "Code that only RustDesk itself uses. `hbb_common` stays the crate shared with the server, so anything the server never touches belongs here instead.")
- **Contents:** `protos/message.proto`, `src/config/`, `src/fs.rs` (file transfer), `src/keyboard.rs`, `src/lib.rs`, `src/platform/`
- **Key rule (from AGENTS.md):** `base::config::keys` is the single import path for all options. Put client-only code here, not in `hbb_common` (which is a submodule shared with the server — changing it costs a round-trip).

---

## 7. Risks & Open Questions

| # | Risk / Question | Severity | Status |
|---|---|---|---|
| 1 | `hbb_common` submodule not initialized — cannot build or fully inspect without it | **Blocker** for M0 build validation | **RESOLVED 2026-11-22** — submodule initialized at `fae47e0`, inspected (proto/config/socket, see §2) |
| 2 | Exact SPDX license identifier (AGPL-3.0 vs AGPL-3.0-or-later) not confirmed from Cargo.toml | Medium — affects fork licensing | **RESOLVED 2026-11-22** — GitHub API returns `AGPL-3.0`; use AGPL-3.0 for fork |
| 3 | macOS capture uses CGDisplayStream, not ScreenCaptureKit (plan correction) | Medium — CGDisplayStream is deprecated on newer macOS; ScreenCaptureKit may be required for future compatibility | **VERIFIED 2026-11-22** — ScreenCaptureKit is audio-only (cpal); no video backend exists. Migration would be new work. |
| 4 | macOS TCC permissions (Screen Recording + Accessibility) — permission flow not inspected | High — affects UX and onboarding design | **VERIFIED 2026-11-22** — `CGPreflightScreenCaptureAccess` + `AXIsProcessTrustedWithOptions` in `src/platform/macos.rs`/`macos.mm` (see §4b) |
| 5 | Windows secure desktop (UAC) — `SendInput` cannot inject into secure prompts; consent.exe mechanism not fully traced | Medium — AI cannot act on UAC prompts without elevation | **PENDING** |
| 6 | Linux Wayland input — requires uinput or RDP bridge; permission model unclear | **High** — may block Linux support without root/uinput setup | **PENDING** |
| 7 | Android/iOS capture and input — stub `Enigo` on mobile; MediaProjection flow not inspected | Medium — mobile support scope unclear | **PENDING** |
| 8 | No compilation attempted — `cargo check` not run (blocked by security policy on cloned content; also requires vcpkg + submodule) | Info — M0 is inspection-only | Documented; build validation deferred to M1 |

---

## 8. Next Steps

### Immediate (M0 completion)

1. ✅ **DONE 2026-11-22** — `hbb_common` submodule initialized (`fae47e0`), proto/config/socket inspected (see §2).
2. ✅ **DONE 2026-11-22** — GitHub API confirms SPDX `AGPL-3.0` (see §1).
3. ✅ **DONE 2026-11-22** — ScreenCaptureKit is audio-only via cpal; video remains CGDisplayStream (see §4).
4. ✅ **DONE 2026-11-22** — TCC code in `src/platform/macos.rs` + `macos.mm` (CGPreflight, AXIsProcessTrusted, IOHIDCheckAccess); none in scrap quartz/enigo (see §4b).
5. **Inspect Windows consent/UAC handling** (`src/platform/windows/`, `consent.exe` references).
6. **Inspect Linux Wayland input path** (`src/server/uinput.rs`, `src/server/rdp_input.rs`, `src/server/input_service.rs` Wayland sections).

### M1 (Frame tap prototype)

7. Build RustDesk from source in a safe environment (requires vcpkg: libvpx, libyuv, opus, aom; plus Flutter for UI — or build headless with `--no-default-features` if possible).
8. Prototype the frame tap: intercept `TraitCapturer::frame()` output in `video_service.rs` before encoding, write a single frame to disk, verify format.
9. Prototype the input injection point: add a test action in `input_service.rs` that calls `ENIGO` directly, verify it moves the mouse on macOS.

### Design decisions informed by this spike

- **AI vision tap:** Best placed in `video_service.rs` after `c.frame(spf)` and before `frame.to(encoder.yuvfmt(), ...)`. The `Frame::PixelBuffer` variant gives raw pixels; `Frame::Texture` gives a GPU texture pointer (harder to consume).
- **AI action injection:** Best placed in `input_service.rs` at the `ENIGO.lock()` call sites, behind a consent validator. The swarm coordinator would serialize actions through a single queue to avoid conflicts.
- **Licensing:** Fork must be AGPL-3.0 (VERIFIED via GitHub API 2026-11-22; LICENCE template mentions or-later but SPDX is AGPL-3.0). The AI agent layer can be a separate process communicating via IPC to remain Apache-2.0/MIT, **but** this needs legal review of the AGPL network clause (section 13) — **PENDING legal review**.
- **macOS MVP:** CGDisplayStream works today but is deprecated. Plan for ScreenCaptureKit migration in M5 (macOS packaging) at the latest.

---

## 9. Files Inspected (for reference)

| File | What was verified |
|---|---|
| `LICENCE` | AGPL-3.0 header; "or later" template language |
| `Cargo.toml` (root) | Package name, version, workspace members, features |
| `.gitmodules` | `libs/hbb_common` submodule URL |
| `AGENTS.md` | Project layout, Rust/Tokio rules, editing hygiene |
| `src/main.rs` | Entry point dispatch |
| `src/core_main.rs` | CLI arg → role mapping |
| `src/client.rs` | Controller logic, NAT traversal, WebRTC |
| `src/rendezvous_mediator.rs` | Controlled Host registration, punch/relay |
| `src/server/video_service.rs` | Capture loop, DXGI recovery, encoder setup |
| `src/server/input_service.rs` | ENIGO usage, platform dispatch (partial read) |
| `libs/scrap/Cargo.toml` | Version 0.5.0, MIT, features |
| `libs/scrap/src/lib.rs` | Module gating by cfg |
| `libs/scrap/src/common/mod.rs` | `TraitCapturer`, `Frame`, `Pixfmt`, `CodecFormat` |
| `libs/scrap/src/quartz/capturer.rs` | CGDisplayStream usage |
| `libs/scrap/src/quartz/ffi.rs` | CoreGraphics/IOSurface FFI |
| `libs/scrap/src/dxgi/mod.rs` | DXGI Desktop Duplication |
| `libs/enigo/Cargo.toml` | Version 0.0.14, MIT |
| `libs/enigo/src/lib.rs` | `MouseControllable`, `KeyboardControllable` traits |
| `libs/enigo/src/macos/macos_impl.rs` | CGEvent backend |
| `libs/enigo/src/win/win_impl.rs` | SendInput backend |
| `libs/enigo/src/linux/nix_impl.rs` | X11/Wayland routing |
| `libs/base/Cargo.toml` | Client-only crate, dependencies |
| `libs/hbb_common/Cargo.toml` | Shared crate deps (tokio, protobuf, sodiumoxide, webrtc pinned 0.13) |
| `libs/hbb_common/protos/rendezvous.proto` | 30 messages, ConnType, ControlPermissions, rendezvous protocol |
| `libs/hbb_common/src/config.rs` | Config/Config2/PeerConfig, rendezvous ports, RS_PUB_KEY |
| `libs/hbb_common/src/lib.rs` | Module exports, SessionID, Stream re-exports |
| `libs/hbb_common/src/tcp.rs` / `udp.rs` / `stream.rs` / `socket_client.rs` | Socket layer (FramedStream, FramedSocket, Stream enum) |
| `src/platform/macos.rs` (TCC section) | `is_process_trusted`, `is_can_screen_recording`, `is_can_input_monitoring` |
| `src/platform/macos.mm` | `IsCanScreenRecording` (CGPreflight), `InputMonitoringAuthStatus` (IOHID) |

**Not inspected (PENDING):** `src/platform/*` beyond macOS TCC (windows/linux/android/ios), `flutter/*`, `libs/clipboard/*`, `libs/virtual_display/*`, Android JNI code, Web TS client, build scripts beyond `libs/scrap/build.rs` and `libs/hbb_common/build.rs`.

---

*Generated by M0 spike inspection, 2026-11-21. No code was compiled or executed from the cloned RustDesk source.*
