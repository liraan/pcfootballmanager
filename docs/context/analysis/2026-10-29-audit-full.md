---
title: "Full repository audit — PC Football Manager"
version: "1.0"
status: "ratified"
last_updated: "2026-10-29"
supersedes: null
depends_on: ["00-foundations", "STRUCTURE", "ARCHITECTURE"]
referenced_by: []
conforms_to: "00-foundations (v2.0)"
---

# Repository audit — `liraan/pcfootballmanager`

**Branch:** `main`  
**Last commit audited:** `72398d2` (Create GitHub agent review report after aligning file naming convention, 2026-10-29)  
**Auditor:** Hermes Agent (static analysis, local clone + build + test)  
**Scope:** All 46 Rust source files, all 11 L0 module specs, `STRUCTURE.md`, `ARCHITECTURE.md`, `CODE_TOUR.md`, `CHANGELOG-SIM.md`, `PROJECT-STATUS.md`, ADR-003, ADR-019, `.githooks/pre-commit`, `.gitignore`, `Cargo.toml`, `Cargo.lock`

---

## 1. Executive summary

The repository is **coherent, well architected, and clean from evident static vulnerabilities**. The Three Planes doctrine is enforced by the compiler (ADR-003 Option B), not merely by convention: `pcfm-hidden` (Plane A) exposes only `resolve_*` / `drain_observations`, has no public fields, no `Serialize`, and a `Debug` that does not leak latents. `pcfm-stats` (Plane B) does not depend on `pcfm-hidden`. `pcfm-match` is the sole orchestrator touching both planes.

**Verification results (real tool output):**

| Check | Result |
|---|---|
| `cargo build --release` | OK — 27s, zero errors |
| `cargo test --workspace` | **103 passed, 0 failed** |
| `cargo clippy --workspace -- -D warnings` | OK — zero warnings |
| Pre-commit hook (`.githooks/pre-commit`) | Present: `fmt --check`, `clippy -D warnings`, `test` on staged Rust files |

**Security static scan:** no hardcoded secrets, no `unsafe`, no network (`std::net`), no shell/process execution (`std::process::Command`), no SQL injection / eval / exec / unsafe deserialization patterns. One low-risk observation: `pcfm-demo/src/render.rs:159` accepts a raw `--html` path from the CLI and passes it to `std::fs::write` without sanitization — path traversal via `../../evil.html` is possible for a local CLI user.

**Dependency note:** `cargo-audit` / `cargo-deny` are not installed in this environment, so known-CVE scanning of transitive dependencies was not performed. Recommended before any release.

---

## 2. Coherence: spec ↔ code

The L0 specification is **faithfully implemented**:

| Spec element | Code location | Status |
|---|---|---|
| Three Planes isolation | `pcfm-hidden`, `pcfm-stats`, `pcfm-match` | ✅ Implemented, compiler-enforced |
| Deterministic named RNG streams | `crates/pcfm-core/src/rng.rs` | ✅ `BTreeMap<StreamName, ChaCha20Rng>`, `blake3` derivation |
| Append-only EventChain with hash chain | `crates/pcfm-stats/src/chain.rs` | ✅ No `update`/`delete`, BLAKE3 chain, verified on load |
| Save/load without Plane A | `crates/pcfm-match/src/save.rs` | ✅ `GroundTruth` reconstructed from `(world_seed, match_id)` |
| Narrative as data (template_key + params) | `crates/pcfm-match/src/narrative.rs` | ✅ i18n-ready, English default rendering |
| Validation on construction | `pcfm-model` (Player, Squad, Pitch, Referee) | ✅ `try_new`, range checks, NaN rejection |
| Pre-commit quality gate | `.githooks/pre-commit` | ✅ `fmt`, `clippy -D warnings`, `test` |

### Incongruences spec → code (all documented as work-in-progress)

1. **`CHANGELOG-SIM.md` §19** states `SIM_VERSION` is defined in `pcfm-core` (planned). In code it is `pcfm-match/src/save.rs:76`. Documentation or code should be aligned.
2. **`modules/L0-match/04-match-engine.md` §2.1** describes `MatchContext` with `bus` and `modifiers` fields, and `MatchResult.events` as `Vec<MatchEvent>`. Code (`context.rs`, `result.rs`) has `match_`, `hidden`, `rng`, `chain`, `config` and `events: EventChain`. The spec is ahead of the current prototype implementation.
3. **`modules/L0-match/05-match-events.md`** describes attempt/outcome event types (`PassAttempted`, `ShotAttempted`, …) and `position_snapshot` / `attempt_event_id` fields (ADR-019 D11). Code `event.rs` does not yet contain them. ADR-019 is `accepted` but implementation is declared future.
4. **`modules/L0-match/07-save-minimal.md`** describes `MatchSave` with `command_log: Vec<Command>` and `postcard` serialization. Code has `config_versions: BTreeMap<String, String>` (no `command_log`) and uses `serde_json`. Documented as planned / known limitation.
5. **`00-foundations.md` §5** bans hardcoded rules; `pcfm-match/src/state.rs` has hardcoded thresholds (0.60/0.85/0.97 action split, `action_probability = 0.0005`). L0.04 flags the `balance/match_engine.toml` as planned. Accepted work-in-progress.
6. **ADR-003 §"Exception: pcfm-debug"** references a `pcfm-debug` crate that no longer exists in the workspace (replaced by `pcfm-demo` and `pcfm-test-runner` per ADR-020). Documentation should be updated.

---

## 3. Security findings

### ✅ No findings

- Hardcoded API keys, passwords, tokens, or credentials: **0 matches**
- `unsafe` blocks: **0** (workspace lint `unsafe_code = "deny"`)
- Network access (`std::net`): **absent**
- Shell/command injection (`std::process::Command`, `os.system`-style): **absent**
- SQL injection, `eval()`/`exec()`, unsafe deserialization (`pickle`): **N/A** (Rust, `serde_json` is safe)

### ⚠️ Low: unsanitized CLI output path

`pcfm-demo/src/render.rs:159` — `std::fs::write(path, html)` where `path` comes from `--html <PATH>` on the command line. A local user could write outside the intended directory (e.g. `--html ../../evil.html`). Low risk (offline CLI tool), but should be sanitized (canonicalize + confine to a safe directory).

### ⚠️ Informational: dependency CVE scan not performed

`cargo-audit` / `cargo-deny` are not installed in this environment. Transitive dependencies (blake3 1.8.7, rand_chacha 0.9.0, serde_json 1.0.154, zmij 1.0.23, zerocopy 0.8.63, etc.) are pinned and mature, but a formal CVE scan should be run before release.

---

## 4. Correctness observations (not bugs, design notes)

1. **`pcfm-match/src/state.rs:222-226` (`pick_players`)**: the same RNG roll is used to pick both attacker and defender indices, so both teams pick the same relative slot. Not a security issue, but introduces an artificial correlation between the two teams. Worth revising when the positional agent-first model (ADR-019) is implemented.
2. **`pcfm-match/src/state.rs`**: current model is "probability-per-tick" (pass/shot/duel/foul 60/25/12/3), declared a placeholder in `04-match-engine.md` and `CODE_TOUR.md`. Deterministic and tested, but not a realistic football model.
3. **`pcfm-hidden/src/lib.rs:71`**: `GroundTruth::new` uses `.expect("stream name is always non-empty")` — documented as unreachable, acceptable.
4. **`pcfm-demo/src/render.rs`**: HTML output is self-contained (inline CSS, inline JSON data, no external assets). Consistent with offline-first design.
5. **Project maturity**: no releases published, 1 contributor. L0 (match engine) complete; L1–L3 planned. v1.0 target Q2 2028. Code should be considered work-in-progress.

---

## 5. Verdict

**Pass.** The repository is secure-by-design for its current scope, internally consistent, and fully verified by build + test + clippy. The remaining gaps are documentation alignment and planned features (postcard saves, command log, attempt/outcome events, `balance/*.toml`), not defects.

### Recommended actions (priority order)

1. Run `cargo-audit` / `cargo-deny` on a machine with the full toolchain to scan transitive dependencies for known CVEs.
2. Sanitize the `--html` path argument in `pcfm-demo/src/render.rs` to prevent path traversal.
3. Align `CHANGELOG-SIM.md` §19 (`SIM_VERSION` location: `pcfm-match`, not `pcfm-core`).
4. Update ADR-003 §"Exception: pcfm-debug" — the crate no longer exists (replaced by `pcfm-demo` / `pcfm-test-runner`).
5. When implementing ADR-019 (positional agent-first engine), revise `pick_players` to remove attacker/defender slot correlation and add attempt/outcome event types per `05-match-events.md` §2.6.
