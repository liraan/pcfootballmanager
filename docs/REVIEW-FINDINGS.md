# Review findings for PC Football Manager

This document summarizes the issues found during a review of the `main` branch of `liraan/pcfootballmanager`. It is written to be easy to hand to another AI or human reviewer so the current state and the main risks are clear.

## Scope

The review focused on the repository structure, documentation consistency, and the core L0 implementation around deterministic simulation primitives and save/load behavior.

## Executive summary

Overall, the project is coherent and has a sensible architecture. I did not find a critical deterministic bug in the primitive L0 code I inspected (`rng`, `ids`, `event_bus`, `save`, `match` surfaces).

However, there are a few real issues worth fixing:

- a broken documentation link pointing to a non-existent file
- documentation drift from the actual repository structure
- a known serialization limitation around `serde_json` + `f64`, which is not a logic bug but a real engineering constraint

These are important for maintainability and trust in the project, even if they are not runtime blockers.

## 1) Broken documentation reference

### Problem

The code and package metadata reference a documentation file that does not exist.

### Evidence

Source file:
- `crates/pcfm-debug/src/main.rs`

Relevant comment:
```rust
//! `pcfm-debug` — simulate a PCFM match and print the result.
//!
//! See `docs/MODULE_DEVELOPER_GUIDE.md` for the design of the engine and
//! for the simulation / rendering split.
```

Package metadata:
- `crates/pcfm-debug/Cargo.toml`

Relevant entry:
```toml
documentation = "https://github.com/liraan/pcfootballmanager/tree/main/docs/MODULE_DEVELOPER_GUIDE.md"
```

### Why this matters

- The repository contains `docs/PLUGIN_DEVELOPER_GUIDE.md` but not `docs/MODULE_DEVELOPER_GUIDE.md`.
- This creates broken navigation, broken external links, and confusion for contributors.
- It suggests the doc was renamed or moved without updating all references.

### Recommended fix

Choose one of the following and apply consistently:

- create `docs/MODULE_DEVELOPER_GUIDE.md` and make it the canonical developer guide, or
- replace all references to `docs/MODULE_DEVELOPER_GUIDE.md` with the actual file path that exists

## 2) Documentation drift: README and repo structure are not aligned

### Problem

The project documentation describes a repository state that does not match the content currently present on `main`.

### Evidence

README says:
```md
├── docs/
│   ├── adr/          # architectural decisions, 000 → 015
```

But the repo includes files like:
- `docs/adr/019-match-engine-contract.md`
- and likely later ADRs beyond the README range

This is not a code bug, but it is a maintenance issue: the documentation is stale and gives a misleading picture of the actual state.

### Why this matters

- Contributors may assume the ADR timeline is closed when it is not.
- A reviewer may think some decisions are settled, while others exist later in the repo.
- The “project status” story becomes harder to trust because the docs and file tree disagree.

### Recommended fix

- Update the README’s architecture and ADR range to reflect the actual tree
- or regenerate the docs from the repository state so they are synchronized
- keep project status numbers in sync with the actual modules/ADR files present

## 3) Save/load limitation: `serde_json` + `f64` is a real constraint

### Problem

The save system is not fully “bit-for-bit” stable for floating point values serialized through JSON.

### Evidence

Source file:
- `crates/pcfm-match/src/save.rs`

Relevant notes:
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

### Why this matters

This is not an accidental bug; it is a known design limitation of using `serde_json::Value` for data that may contain floating-point values.

Consequences:

- event payloads may not roundtrip identically at the byte level
- equality checks on raw serialized event data are unsafe
- the project correctly relies on `state_hash` as the real identity of the save instead of raw event equality

### Recommended fix

The project already points in the right direction:

- prefer `state_hash` for verification and replay identity
- plan to move away from `serde_json::Value` for serialized event content
- medium-term target: a more deterministic encoding such as `postcard` or a typed string-based representation

This is a known engineering trade-off, not a reason to panic, but it should remain explicitly documented and treated carefully.

## 4) What I did not find

I did not find a severe runtime bug in the core deterministic primitives reviewed so far:

- `pcfm-core::rng`
- `pcfm-core::ids`
- `pcfm-core::event_bus`
- `pcfm-match::engine`
- `pcfm-match::save`

The code appears to be intentionally built around determinism and reproducibility, and the tests around these areas are intentionally strict.

So the issues above are mostly:

- repository hygiene
- documentation integrity
- serialization design constraints

rather than a broken simulation algorithm.

## 5) Suggested next actions

1. Fix broken documentation links in `pcfm-debug` and related docs.
2. Synchronize README and repo tree with actual ADR / docs state.
3. Keep the serialization limitation explicitly documented in all future save-format changes.
4. It may be useful to add a small “docs consistency check” in CI or a contributor guideline to catch stale references before they land.

## Final assessment

The repo is in a healthy conceptual state for an early-stage simulation project, but it has a few real maintainability issues that should be cleaned up before it is treated as a production-grade project. The most important issue is the broken documentation path, followed by documentation drift and the known float-serialization limitation.

These are all fixable and should be treated as maintenance tasks rather than evidence of a deep algorithmic failure.
