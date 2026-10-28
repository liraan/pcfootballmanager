# ADR-020 — Split the test runner from the demo tool

- **Status:** proposed
- **Date:** 2026-10-28
- **Supersedes:** —
- **Superseded by:** —
- **Blocks:** IDEA-INFRA-004 (`pcfm-debug` covers installed modules and plugins), IDEA-INFRA-005 (separate diagnostics and benchmark from the test runner), and any future rename of `pcfm-debug`.

## Context

`pcfm-debug` today does **three** things, and its name describes **none** of them precisely:

1. **Simulation demo** — `main.rs` builds two demo squads, simulates a match with `TextMatchEngine`, prints text/JSON, optionally writes HTML (`demo.rs`, `render.rs`).
2. **Test runner** — `--check` runs scenarios, `--list` lists them, `--coverage` reports feature coverage (`check.rs`, `commands.rs`, `features/`).
3. **Nothing named "debug"** — there is no state inspection, no replay, no trace. The name suggests a debugger; the function is demo + runner.

The crate's own documentation acknowledges the ambiguity: `pcfm-testkit/src/lib.rs` says *"the runner (`pcfm-debug`) aggregates them"* and *"The runner (e.g. `pcfm-debug`) creates one registry"*. The code calls itself a runner while being named a debugger.

The criterion applied in this decision is the one established in conversation on 2026-10-28: **the name must describe the function**. `pcfm-debug` fails it. `pcfm-debug` also fails IDEA-INFRA-004 and IDEA-INFRA-005, which cannot be designed coherently until the crate's scope is settled.

Cost of not deciding: IDEA-INFRA-004 (plugin discovery in the runner) and IDEA-INFRA-005 (diagnostics and benchmark) remain ambiguous; any rename later costs more, because more code and docs will reference the current name.

## Options considered

### Option A — Rename only (`pcfm-debug` → `pcfm-test-runner`)

- **Pro:** minimal cost (~53 live references, 1 commit); fast.
- **Con:** does not solve the problem. The crate still does two things (demo + runner) under a name that describes one. The ambiguity moves, it does not disappear. Rejected by the "name = function" criterion.

### Option B — Split into two crates (`pcfm-test-runner` + `pcfm-demo`)

- **Pro:** each crate does one thing; the name describes it. Future-proof: diagnostics and benchmark will be separate crates, not subcommands of a catch-all.
- **Con:** higher cost — new crate, workspace `Cargo.toml`, ~450 lines moved, ~53 references updated, the demo's 3 tests migrated.

### Option B′ — Split, with the demo as a workspace example (`examples/demo/`)

- **Pro:** satisfies the "name = function" criterion with lower cost than B: no new crate, no new `Cargo.toml`, tests migrate to `cargo test`. The demo is an internal smoke test, not a distributed binary.
- **Con:** `cargo install` does not install examples (irrelevant today: nothing is distributed); feasibility with the demo's real dependencies (`pcfm-match`, `pcfm-hidden`, `pcfm-stats`) must be verified before committing.

### Option C — Merge under a single CLI (`pcfm-tools run|list|coverage|demo`)

- **Pro:** convenient; one binary, one `--help`, unified UX (the `git`/`cargo` model).
- **Con:** repeats the error. `tools` is generic like `debug`. Every future function (diagnostics, benchmark) becomes a subcommand of an ever-growing drawer. Rejected by the "name = function" criterion.

## Decision

**Option B′ is proposed, subject to ratification.** Motivation:

- It satisfies the criterion: `pcfm-test-runner` runs scenarios; `examples/demo/` simulates a match for manual inspection. One function per name.
- It costs less than B (no new crate) and is more correct than A and C (which keep or hide the ambiguity).
- **B′ is conditional:** if the demo's dependencies cannot live cleanly in `examples/demo/`, the fallback is **B** (a `pcfm-demo` crate). The feasibility check comes before implementation.

This ADR is **proposed**, not accepted. No code changes until it is ratified.

## Consequences

### Easier
- IDEA-INFRA-004 and IDEA-INFRA-005 get a stable home.
- A future `pcfm-diagnostics` (or `pcfm-log`/`pcfm-trace`) does not have to fight a catch-all name.
- The "name = function" criterion is applied once, visibly, and can be cited later.

### Harder
- ~53 live references must be updated in one coordinated change.
- `args.rs` (233 lines) must be reviewed: its flags mix runner flags and demo flags, and the split forces a decision on each.
- The workspace `Cargo.toml` and `STRUCTURE.md` §3 (crate list) must be updated.

### Locked in
- `pcfm-demo` as an example is **not** installable via `cargo install`. Acceptable while no binary is distributed. If a distributed demo binary is needed later, B becomes the right choice and this ADR is superseded.

## Implementation notes

1. **Feasibility check for B′ first:** create `examples/demo/main.rs` with the current `demo.rs` + `render.rs` logic and the real dependencies (`pcfm-match`, `pcfm-hidden`, `pcfm-stats`). Run `cargo test` and `cargo run --example demo -- --seed 42`. If it works, B′ holds; if not, fall back to B.
2. Rename `pcfm-debug` to `pcfm-test-runner` (`Cargo.toml`, workspace members, `src/`).
3. Keep in `pcfm-test-runner`: `check.rs`, `commands.rs`, `features/`, and the runner-only part of `args.rs`. Move to `examples/demo/`: `demo.rs`, `render.rs`, and the simulation part of `main.rs`.
4. Update the ~53 live references: `ARCHITECTURE.md`, `README.md`, `PLUGIN_DEVELOPER_GUIDE.md`, `CODE_TOUR.md`, `STRUCTURE.md`, `PROJECT-STATUS.md`, `CONTRIBUTING.md`, `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `REPO-SNAPSHOT.md`.
5. **Do not touch:** `docs/adr/003-plane-a-enforcement.md` (immutable ADR), `IDEAS.md` / `PROJECT-IDEA-LOG.md` / `2026-10-28-09-42-review.md` (historical), except for an append that cites this ADR.
6. `modules/00-foundations.md` (1 reference): verify whether it is doctrine or an example. If doctrine, update it with this ADR as the reference.
7. Full verification cascade after the change: `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin pcfm-test-runner -- --check --seed 42`.

## References

- `IDEA-INFRA-004` — `pcfm-debug` covers installed modules and plugins (`docs/context/IDEAS.md`).
- `IDEA-INFRA-005` — separate diagnostics and benchmark from the test runner (`docs/context/IDEAS.md`).
- ADR-012 (plugins), ADR-013 (worlds) — the runtime contracts the runner will eventually load.
- Code read for this ADR: `crates/pcfm-debug/src/main.rs`, `commands.rs`, `check.rs`; `crates/pcfm-testkit/src/lib.rs`, `scenario.rs`, `registry.rs`, `feature.rs`.
- Not read in detail: `crates/pcfm-debug/src/args.rs` (233 lines), `demo.rs`, `render.rs`, `error.rs`. Their split between runner and demo is part of the implementation, not the decision.
