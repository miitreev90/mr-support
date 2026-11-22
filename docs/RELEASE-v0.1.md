# M7 Release v0.1 — Public release prep

## Release notes draft
**Mr. Support v0.1.0 — AI support, human in control.**
Open-source Rust remote support with multi-agent swarm and MCP server for external AI assistants.

### Features
- Frame tap for AI vision (M1)
- Agent loop + swarm coordinator with 5 specialists (M2/M2b)
- Consent model: explicit approval, visible indicator, instant revocation (M3)
- Bounded autonomy: allowlist, rate limit, risk-based approval (M4)
- Multi-platform design: Windows, macOS, Linux, Android, iOS (M5)
- MCP server: external assistants (Muse, OpenCode, Codex, Claude Code) connect with their own LLM, no API key in Mr. Support (M2c)
- Audit log with JSON export
- CI perfect: fmt/check/clippy/test zero-warnings on ubuntu/macos/windows

### Known limitations
- LLM decision is placeholder heuristic (M2) — external assistant decides via MCP
- RustDesk fork integration not yet wired (video_service/input_service)
- Packaging installers (.app, .exe, .deb, APK) pending
- Logo: assets/logo.jpg uploaded (Emma's final version), SVG removed

### License
AGPL-3.0 (matches RustDesk upstream SPDX).

## Release checklist
- [x] All tests pass with zero warnings (17 tests, cargo check/clippy/fmt clean)
- [x] Logo finalized and added to assets/
- [x] CI passing on all platforms (run 44996135044, 58s)
- [ ] README updated with install instructions
- [ ] GitHub release v0.1.0 created
- [ ] Announce in community

## Status
READY for Miguel's approval to publish v0.1.0.
