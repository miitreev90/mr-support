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
- [ ] RustDesk fork integration (video_service.rs tap + input_service.rs injection)
- [ ] TCC / UAC / uinput permissions verified on target OS

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

## Status
READY for pilot definition — needs from Miguel: real client name, date, target OS.
