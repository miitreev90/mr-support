# M5 Packaging — Multi-platform

Mr. Support targets Windows, macOS, Linux, Android and iOS (like AnyDesk/TeamViewer).

## Platform abstraction
- `src/platform/mod.rs` detects current OS via `cfg!`.
- `src/platform/{macos,windows,linux,android,ios}/mod.rs` are placeholders for capture/input backends.
- Capture: RustDesk `scrap` (CGDisplayStream macOS, DXGI Windows, X11/Wayland Linux).
- Input: RustDesk `enigo` (CGEvent macOS, SendInput Windows, X11/uinput Linux).

## Build commands (zero warnings policy)
```bash
cargo check -D warnings
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo test
```

## CI
GitHub Actions workflow `.github/workflows/ci.yml` runs the four commands above on ubuntu, macos and windows.

## Packaging (future M5b)
- macOS: `.app` bundle with TCC entitlements (Screen Recording + Accessibility).
- Windows: installer with UAC manifest.
- Linux: AppImage / deb.
- Android: APK with MediaProjection permission.
- iOS: TestFlight build with ReplayKit.

Current status: code compiles and tests pass on Linux. Cross-compile targets to be added in M5b.
