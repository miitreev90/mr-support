# Uncle Bob strict harness

Inspired by Robert C. Martin (Uncle Bob) — Clean Code, Clean Architecture, TDD.

## Rules enforced in CI (fitness functions)

1. **No `unwrap()` / `expect()` in production code** — use `?`, `match`, or `unwrap_or`. Tests may use unwrap (allow-unwrap-in-tests).
2. **No `panic!` / `unimplemented!` / `todo!` in production** — handle errors explicitly.
3. **Small functions** — clippy `too-many-lines-threshold = 50`, `cognitive-complexity-threshold = 15`.
4. **Zero warnings** — `RUSTFLAGS="-Dwarnings"`, `cargo clippy -- -D warnings` (standard lints as errors; pedantic/nursery tracked as future tightening — 83 pedantic findings today, fix gradually with `#[allow]` where intentional).
5. **TDD / tests first** — every module ships with unit tests; CI requires `cargo test` pass on ubuntu/macos/windows.
6. **Single Responsibility** — modules: `agent` (vision/action), `swarm` (coordinator/specialists), `consent` (session/policy), `audit` (log), `transport` (frame tap/input), `connectivity` (ID/rendezvous/relay), `mcp` (external assistants), `platform` (OS backends). No cross-layer violations.
7. **Dependency inversion** — high-level modules (agent, swarm) depend on abstractions (traits like `InputInjector`), not concrete platform code.

## CI steps
- `cargo fmt --all -- --check`
- `RUSTFLAGS="-Dwarnings" cargo check --all-targets --all-features`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features` (includes architecture fitness tests: no unwrap/panic in production)

## Why
Clean code is not optional: Mr. Support controls real desktops with AI. Strict harness prevents regressions, keeps the codebase maintainable for contributors, and enforces the consent/safety model at the architecture level.
