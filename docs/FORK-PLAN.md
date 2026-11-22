# Fork plan — RustDesk → Mr. Support

## Base
- Upstream: https://github.com/rustdesk/rustdesk (AGPL-3.0, SPDX verified)
- Local full clone: `~/workspace/rustdesk-spike` (99 MB, full history, hbb_common fae47e0)
- Fork workspace: `~/workspace/mr-support-fork` (copy of spike, working tree for integration)
- Mr. Support core (agent/swarm/consent/mcp): `~/workspace/mr-support-initial` (public repo miitreev90/mr-support)

## Integration points (from M0 spike)
1. **Frame tap (AI vision):** `src/server/video_service.rs` — after `c.frame()`, before encode. Tap via `std::sync::mpsc` channel to Mr. Support `FrameTap` (M1 done, 7 tests). Human stream unaffected.
2. **Action injection:** `src/server/input_service.rs` — at `ENIGO.lock()` sites. AI actions go through single input queue + consent/policy gate (M2/M3/M4 done).
3. **Connectivity (ID + rendezvous + relay):** RustDesk `src/rendezvous_mediator.rs` + `libs/hbb_common` (proto, socket layer). Mr. Support adds `src/connectivity/` skeleton: device ID, rendezvous client, relay client. No public IP needed; NAT traversal via outbound connection + relay fallback. Cloudflare Tunnel / Ngrok only for dev, not production video.
4. **MCP server:** `src/mcp/` (M2c done, 17 tests) — external assistants (Muse, OpenCode, Codex, Claude Code) connect with their own LLM, no API key in Mr. Support. Chat BYO optional future.

## License
- Fork must be AGPL-3.0 (matches upstream SPDX). AI agent layer as sidecar via IPC (Apache-2.0 possible) if no internal crate import; otherwise AGPL-3.0 all-in. Legal review of AGPL section 13 pending.

## Next steps
- [ ] Wire frame tap in video_service.rs (M1 integration)
- [ ] Wire input injection in input_service.rs with consent gate
- [ ] Implement connectivity: ID generation, rendezvous register/lookup, relay fallback
- [ ] Verify TCC (macOS), UAC (Windows), uinput (Linux Wayland) on target OS
- [ ] Pilot with real client (M6)
