# Third-pass repo review for `liraan/pcfootballmanager`

Scope: reviewed the repository excluding `docs/context`, with emphasis on code correctness, architecture consistency, documentation integrity, and real implementation risk.

## Verdict

This is not a critical code failure review. I did not find a fatal logic bug in the runtime paths I checked.

However, under a stricter review standard, I would classify the repository as:

- functionally coherent at the L0 primitive layer
- not yet mature enough to be treated as a high-trust production codebase
- burdened by documentation drift, stale status claims, and a few unresolved engineering-contract gaps

The main risk is not “the simulation is broken”; the main risk is “the project presents a stronger state than the repository actually supports.”

## Scope and constraints

Excluded from this review:
- `docs/context/`

This exclusion is intentional because the directory is explicitly marked as briefing material, not as the normative source of truth.

## High-confidence findings

### 1) Broken internal documentation reference
Severity: High for maintainability, Medium for runtime impact
Status: Confirmed

The repo contains an internal doc reference that points to a file that does not exist.

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

Actual repo state:
- `docs/PLUGIN_DEVELOPER_GUIDE.md` exists
- `docs/MODULE_DEVELOPER_GUIDE.md` does not exist

This is a concrete repo inconsistency, and it should be treated as a real issue because it affects onboarding, developer confidence, and hosted docs links.

### 2) README and repo state are out of sync
Severity: High for trust / review quality
Status: Confirmed

The repository contains repeated examples of stale or drifting status claims.

Evidence:
- `README.md` describes a docs/ADR timeline ending at 015
- `STRUCTURE.md` indicates a current state including ADRs through 019
- documentation and file tree do not always agree on the current state of the project

This is not a direct code bug, but under a stricter review it is a material issue because the project is presenting a version of reality that is not consistently backed by the repo itself.

This matters because:
- people reviewing the repo may assume the project is more mature than it is
- AI/process handoff can become unreliable
- the source of truth becomes ambiguous

### 3) The project claims a more stable state than the codebase fully enforces
Severity: Medium to High
Status: Confirmed by structure and documentation

The repo repeatedly presents the project as “ratified”, “complete”, or “implemented,” but the actual tree still contains several placeholders, empty directories, deferred modules, and planned L1/L2/L3 work.

Examples from repo state:
- `archive/` exists with mostly empty placeholder structure
- `docs/calibration/` is present but empty
- `docs/legal/` is present but empty
- `registry/`, `mods/`, `data/` are present but not populated with active code/data
- L1-L3 are documented as planned, not live

This is not inherently wrong, but it means the repo is using a stronger narrative and status tone than the code truly supports. That is a reviewability problem.

### 4) Save compatibility is intentionally limited and should be treated as a design constraint, not a bug
Severity: Medium
Status: Confirmed and well documented

This is the most important code-level issue I still see.

Evidence:
- `crates/pcfm-match/src/save.rs`
```rust
//! # Known limitation: `serde_json` and `f64`
//!
//! `MatchEvent.fields` is a `serde_json::Value`. When it contains an
//! `f64` derived from an `f32` (e.g. `xg_value`), the roundtrip
//! `serialize -> deserialize` may lose 1 ulp (last-bit) of precision.
//! This is a property of `serde_json`, not of the simulation.
```

This is not a logic bug in the simulation, but it is a real engineering constraint:
- save roundtrips are not bitwise-perfect for all float-bearing event payloads
- the identity of the save is intentionally anchored to `state_hash`, not serialized event equality

This is acceptable only if declared and enforced consistently. I do not see evidence the project has fully hardened its serialization policy beyond that, and that should be treated as a design debt.

### 5) The repo is architecture-driven, but the enforcement quality varies by area
Severity: Medium
Status: Confirmed

The project strongly values doctrine, planes, and ADR enforcement, which is good. However, the repository still shows signs that the architecture is better documented than enforced.

Examples:
- `STRUCTURE.md` and `PLUGIN_DEVELOPER_GUIDE.md` define strict rules
- but the actual codebase still contains a few references and status claims outside those rules
- there is no obvious CI gate that converts the doctrine into automated checks

This is not a blocker, but it means the project is currently relying more on human review than on an enforceable repo contract.

## Lower-confidence / non-blocking findings

### 6) Some docs imply “ratified status” but the repo is still in active evolution
Severity: Low to Medium
Status: Observed

This is partly a narrative problem and partly a project-health problem.

A repo can legitimately be early-stage and still use strong doctrine. But when the docs sound final while the actual code is still evolving, it creates a false sense of certainty.

The repo would benefit from a clear “current maturity” statement that distinguishes:
- doctrine status
- implementation status
- ADR status
- current test status

### 7) AI handoff risk remains high if the repo is treated as “fully decided” without verification
Severity: Medium
Status: Observed

This is the practical outcome of the issues above.

A future AI or maintainer could read the README, assume the project is more stable than it is, and then make changes that violate the actual gatekeeping model or status boundaries.

## What I did not find

Under this stricter pass, I still did not find a critical runtime fault in the code paths I inspected:

- `pcfm-core`
- `pcfm-hidden`
- `pcfm-match`
- `pcfm-stats`
- debug/test harness

I also did not find a deep issue in the determinism model itself. The core primitives appear intentionally designed around reproducibility and are internally coherent.

So the repo is not obviously “broken”; it is simply not yet fully trustworthy as a mature, audited, source-of-truth system.

## Final assessment

If I had to give a harsh but fair classification:

- Not a code-level catastrophe
- Not a production-grade trust anchor yet
- Not a repo I would hand to a team without cleanup
- But it is definitely salvageable and structurally promising

The most important risks are:
1. broken internal doc path
2. stale documentation / repo drift
3. status inflation relative to code maturity
4. save-format precision caveat that must remain explicit

## Actionable conclusion for another AI

If another AI is asked to continue review work on this repo, the correct posture is:

- treat the code as promising but under-documented
- trust the deterministic primitive layer more than the narrative layer
- verify every status claim against the files, not against the README alone
- avoid assuming “ratified” means “fully stable” if the repo still has planned modules and incomplete toolchain pieces

## Strict recommendation

The repo should not be described as “healthy and production-ready.”
It should be described as:

- coherent early architecture
- intentionally deterministic
- semi-mature implementation
- documentation and status hygiene need cleanup before broad trust is warranted

## AI handoff summary

If another AI has to continue from here, the one-line summary is:

“Repo is structurally sound and deterministic at the primitive level, but it suffers from documentation drift, a broken doc reference, and a known float-serialization caveat. No critical runtime bug found, but the project is not yet strong enough to be treated as a mature audited codebase without cleanup.”
