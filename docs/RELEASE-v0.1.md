# M7 Release v0.1 — Public release prep

## Release notes draft
**Mr. Support v0.1.0 — AI support, human in control.**
Open-source Rust remote support with multi-agent swarm.

### Features
- Frame tap for AI vision (M1)
- Agent loop + swarm coordinator with 5 specialists (M2/M2b)
- Consent model: explicit approval, visible indicator, instant revocation (M3)
- Bounded autonomy: allowlist, rate limit, risk-based approval (M4)
- Multi-platform design: Windows, macOS, Linux, Android, iOS (M5)
- Audit log with JSON export

### Known limitations
- LLM decision is placeholder heuristic (M2) — real model integration pending
- RustDesk fork integration not yet wired (video_service/input_service)
- Packaging installers (.app, .exe, .deb, APK) pending
- Logo: Emma's final version pending

### License
AGPL-3.0 (matches RustDesk upstream SPDX).

## Release checklist
- [ ] All tests pass with zero warnings
- [ ] README updated with install instructions
- [ ] Logo finalized and added to assets/
- [ ] GitHub release v0.1.0 created
- [ ] Announce in community

## Status
PENDING — needs Miguel's approval to publish.
