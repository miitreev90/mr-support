# M6 Pilot — Supervised pilot with real client

## Goal
Validate Mr. Support end-to-end with one real client under full supervision.

## Prerequisites (from M0-M5 + M2c MCP)
- [x] Frame tap working (M1)
- [x] Agent loop + swarm (M2/M2b)
- [x] Consent with revocation + audit log (M3)
- [x] Bounded autonomy policy (M4)
- [x] Multi-platform packaging plan (M5)
- [x] MCP server for external AI assistants (M2c, commit f348576, 17 tests)
- [x] CI perfect: RUSTFLAGS, rust-cache, fmt/check/clippy/test on ubuntu/macos/windows (commit c8b44d9, run passed 58s)
- [x] Emma's final logo in repo (assets/logo.jpg, SVG removed)
- [x] RustDesk fork integration wired (mr_support.rs + video_service tap + ai_handle_mouse, commit 6660bcb)
- [x] TCC / UAC / uinput verified in fork code (docs/FORK-WIRING.md)
- [ ] Compile full RustDesk fork on Miguel's PC (needs vcpkg + platform toolchains)

## Pilot checklist
1. Install on client machine (macOS first)
2. Grant Screen Recording + Accessibility (TCC)
3. Start session: client approves, indicator visible
4. Connect external assistant via MCP (Muse, OpenCode, Codex, Claude Code) — no API key in Mr. Support
5. Agent diagnoses (Diagnostics specialist)
6. Agent proposes action, client approves (HITL)
7. Agent executes via single input queue
8. Audit log records before/after captures
9. Client revokes mid-session → agent stops instantly
10. Post-mortem: review audit log, fix issues

## Success criteria
- Zero warnings/errors during session
- Client can revoke at any time
- Audit log complete and exportable
- No data leaves the machine without consent

## Pilot on Miguel's PC (ThinkPad T14 Gen 4, Windows)
1. Clone fork: `git clone https://github.com/rustdesk/rustdesk` then apply the mr_support wiring (or use `~/workspace/mr-support-fork` tree).
2. Build with RustDesk's documented environment (vcpkg, VS Build Tools on Windows).
3. Install, note the 9-digit DeviceId from the UI.
4. From a second machine (or phone), connect by ID; approve consent; verify the green "AI operating" banner and audit log entries.
5. Test revoke: press Revoke mid-session; confirm AI actions stop instantly.
6. Export audit log JSON and review.

## Status
Fork wiring done and pushed (commit 6660bcb). Pending: compile on Miguel's PC, then run the pilot checklist above. Needs from Miguel: date for the pilot session.
