# M6 Pilot — Supervised pilot with real client

## Goal
Validate Mr. Support end-to-end with one real client under full supervision.

## Prerequisites (from M0-M5)
- [x] Frame tap working (M1)
- [x] Agent loop + swarm (M2/M2b)
- [x] Consent with revocation + audit log (M3)
- [x] Bounded autonomy policy (M4)
- [x] Multi-platform packaging plan (M5)
- [ ] RustDesk fork integration (video_service.rs tap + input_service.rs injection)
- [ ] TCC / UAC / uinput permissions verified on target OS
- [ ] Emma's final logo in assets/

## Pilot checklist
1. Install on client machine (macOS first)
2. Grant Screen Recording + Accessibility (TCC)
3. Start session: client approves, indicator visible
4. Agent diagnoses (Diagnostics specialist)
5. Agent proposes action, client approves (HITL)
6. Agent executes via single input queue
7. Audit log records before/after captures
8. Client revokes mid-session → agent stops instantly
9. Post-mortem: review audit log, fix issues

## Success criteria
- Zero warnings/errors during session
- Client can revoke at any time
- Audit log complete and exportable
- No data leaves the machine without consent

## Status
PENDING — needs a real client, date and target OS from Miguel.
