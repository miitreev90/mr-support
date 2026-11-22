# UI design — Mr. Support (not RustDesk Flutter clone)

## Decision
Do NOT copy RustDesk's Flutter UI (heavy, AGPL, hard to maintain, not AI-focused).
Mr. Support keeps RustDesk transport/protocol (hbb_common, rendezvous, relay) but ships a **lightweight native Rust UI** designed for AI supervision.

## Stack
- Phase 1 desktop (Windows/macOS/Linux): **egui** (immediate-mode, pure Rust, single binary, no webview) or **Iced** (Elm-style). egui preferred for simplicity and audit-log tables.
- Mobile (Android/iOS): later, via same core + platform UI (not Flutter clone).
- No Electron/Tauri for v0.1 (keep binary small, fast startup).

## Layout (AI-supervision first)
```
+--------------------------------------------------+
| Mr. Support  [ID: 123456789]  [AI: OFF/ON]       |  <- header: ID always visible
+--------------------------------------------------+
| Remote ID: [________]  [Connect]                 |  <- connect like AnyDesk
| Password / Consent: [Approve] [Revoke]           |  <- revoke ALWAYS visible
+--------------------------------------------------+
| AI indicator: "AI operating — revoke anytime"    |  <- banner when active
| Cursor: ai-cursor (differentiated)               |
+--------------------------------------------------+
| Swarm: Coordinator + 5 specialists [status]      |
| MCP: Connected assistant: Muse (OpenCode...)   |
+--------------------------------------------------+
| Audit log: [time] [agent] [action] [before/after]|
| [Export JSON]                                    |
+--------------------------------------------------+
```

## Key UX rules (consent model)
- Revoke button is never hidden, never requires scrolling.
- AI indicator + differentiated cursor are mandatory when consent is Active.
- Every AI action appears in audit log with before/after captures.
- User input always preempts AI (single input queue, human priority).

## Implementation plan
1. `src/ui/` skeleton: `UiState`, `ConsentDialog`, `AuditViewer`, `McpPanel` — pure Rust structs, testable, no GUI dep yet.
2. Wire to `ConsentSession`, `AuditLog`, `McpServer`, `DeviceId`.
3. Add egui feature flag later (`--features gui`) so core stays headless for CI.
4. Mobile UI in phase 2.
