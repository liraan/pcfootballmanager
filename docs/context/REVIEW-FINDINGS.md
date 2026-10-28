# AI handoff review brief for `liraan/pcfootballmanager`

Last reviewed: 2026-10-28
Scope: repository review of the `main` branch focusing on code correctness, architecture coherence, documentation integrity, and save/load safety.

## Executive summary

The repository is structurally coherent and the core L0 primitives I inspected appear intentionally designed around determinism and reproducibility. I did not find a critical runtime bug in the deterministic infrastructure I re-checked (`rng`, `ids`, `event_bus`, `save`, match engine).

The issues I found are real, but they are mostly not logic-breakers. They are concentrated in the following areas:

- broken internal documentation references
- drift between README/project documentation and the actual repository state
- a known save-format limitation caused by `serde_json` + floating-point values

This means the project is not fundamentally broken, but it is not yet fully review-safe for a new AI or contributor without some cleanup.

## Severity classification

- Blocking / critical: none found in the reviewed code paths
- Medium: broken documentation links, stale repo documentation, save-format precision caveat
- Low: minor doc drift and maintainability friction

## Findings

### F1 — Broken documentation reference
Severity: Medium
Status: Confirmed

Summary:
The repository references a documentation file that does not exist.

Evidence:
- `crates/pcfm-debug/src/main.rs`
```rust
//! See `docs/MODULE_DEVELOPER_GUIDE.md` for the design of the engine and
//! for the simulation / rendering split.
```

- `crates/pcfm-debug/Cargo.toml`
```toml
documentation = "https://github.com/liraan/pcfootballmanager/tree/main/docs/MODULE_DEVELOPER_GUIDE.md"
```

Reality in repo:
- `docs/PLUGIN_DEVELOPER_GUIDE.md` exists
- `docs/MODULE_DEVELOPER_GUIDE.md` does not exist

Why it matters:
- broken navigation and broken external docs links
- contributor confusion
- repository hygiene issue that makes the project appear less stable than it is

Recommended action:
- create the missing file and make it the canonical guide, or
- update all references to the file that actually exists

### F2 — Documentation drift between README and repo tree
Severity: Medium
Status: Confirmed

Summary:
The README does not match the actual repository state in some sections, especially around ADR history and project status descriptions.

Evidence:
README includes:
```md
├── docs/
│   ├── adr/          # architectural decisions, 000 → 015
```

Actual repo state includes later ADRs such as:
- `docs/adr/019-match-engine-contract.md`

This gap is not a code fault, but it creates a false impression that the architecture is more static and closed than it really is.

Why it matters:
- stale docs can mislead future AI reviewers and contributors
- review confidence drops because the project narrative and the actual file tree disagree
- it increases the chance of wrong assumptions about status and scope

Recommended action:
- synchronize README statements with the current docs tree
- keep ADR and module status numbers in sync with repo contents
- preferably add a simple docs consistency check to CI or contributor workflow

### F3 — Save/load precision caveat: `serde_json` + `f64`
Severity: Medium
Status: Confirmed and already documented in code

Summary:
The save format is functionally acceptable for replay and validation, but it is not strictly bit-stable for float-containing values serialized through JSON.

Evidence:
- `crates/pcfm-match/src/save.rs`
```rust
//! # Known limitation: `serde_json` and `f64`
//!
//! `MatchEvent.fields` is a `serde_json::Value`. When it contains an
//! `f64` derived from an `f32` (e.g. `xg_value`), the roundtrip
//! `serialize -> deserialize` may lose 1 ulp (last-bit) of precision.
//! This is a property of `serde_json`, not of the simulation.
//!
//! The identity of a saved match is verified via `state_hash`, not via
//! bit-for-bit equality of the events.
```

Why it matters:
- raw event payload equality after JSON roundtrip is unsafe
- if future logic compares serialized values directly, it can produce false negatives
- the project already compensates with `state_hash` as the identity check, which is the right approach

Recommended action:
- keep using `state_hash` as canonical verification for save identity
- avoid direct bitwise comparison on serialized event payloads
- plan a future migration to a more deterministic encoding (for example: compact typed format or string-based serialization)

## What I did not find

I did not find a severe logic bug in the core deterministic primitives reviewed so far:

- `pcfm-core::rng`
- `pcfm-core::ids`
- `pcfm-core::event_bus`
- `pcfm-match::engine`
- `pcfm-match::save`

The simulation design appears intentionally built around reproducibility and a phased architecture. The main issues are not “simulation is broken,” but instead:

- repository hygiene
- docs integrity
- serialization constraints

## Final assessment

Status: generally healthy architecture, but not yet fully polished for external review / contributor handoff.

The repo currently looks like a legitimate early-to-mid-stage deterministic simulation project, with coherent primitives and a clear design philosophy. The issues are real but manageable.

Main takeaway for another AI:
- do not conclude there is a critical deterministic bug
- do conclude there is documentation drift and at least one broken internal reference
- do treat the save precision caveat as a known design limitation, not as a sign of broken logic

## Suggested next actions

1. Fix all broken documentation references.
2. Align README and repo tree with actual ADR and module status.
3. Keep the JSON float caveat explicit during future save-format decisions.
4. Add a lightweight docs consistency check to CI or contribution policy if the repo is expected to grow.

## Handoff note

If another AI is asked to review this repository, the most important bias to avoid is overreacting to the repo’s ambitious design. The code I checked is not obviously broken. The bigger risk is documentation drift and maintenance debt, not a hidden algorithmic bug.
