# ADR-006 — License

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** every `Cargo.toml` example (currently says "Proprietary"), the `LICENSE` file, the README "License" section

## Context

The README says "open data, you own everything". The `Cargo.toml` example in `01-architecture.md` says `license = "Proprietary"`. These cannot both be true. This ADR resolves the contradiction by choosing two licenses: one for code, one for assets.

## Options considered

### Option A — Single permissive license (MIT or Apache-2.0) for everything
- **Pro:** simple, maximum compatibility, maximum adoption.
- **Con:** assets can be repackaged and sold by a commercial actor; project identity unprotected.

### Option B — Single copyleft license (GPL-3.0 or AGPL-3.0) for everything
- **Pro:** forks must remain open.
- **Con:** incompatible with the Rust ecosystem's default expectations; GPL is a software license, awkward for assets.

### Option C — Dual license: permissive for code, copyleft + non-commercial for assets (RECOMMENDED)
- **Pro:** code forkable and embeddable; assets protected from commercial reuse; no GPL contamination; identity protected.
- **Con:** two licenses to explain; NC clause may deter some asset contributors.

### Option D — No license (all rights reserved)
- **Pro:** maximum control.
- **Con:** legally `Proprietary`; makes the project un-contributable. **Rejected.**

## Decision

**Option C.**

### Code — dual MIT / Apache-2.0

Every crate in `crates/` and every executable is dual-licensed under **MIT** or **Apache-2.0**, at the user's option. This is the Rust ecosystem standard. `Cargo.toml` files declare:

```toml
license = "MIT OR Apache-2.0"
