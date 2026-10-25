---
title: "L0.01 — RNG (deterministic streams)"
level: 0
cell: "L0-match"
module_id: "L0.01"
version: "1.0"
status: "ratified"
last_updated: "2026-10-25"
depends_on: ["00-foundations"]
referenced_by: ["L0.02", "L0.04", "L0.08", "L1-season", "L2-market", "L2-psyche"]
conforms_to: "00-foundations (v1.0)"
---

# L0.01 — RNG

## 0. Purpose

Provide deterministic, isolated, named random streams. The single source of randomness for the entire project. No cell uses `rand::thread_rng()` or any system-seeded RNG.

## 1. Context and dependencies

- Depends only on `00-foundations` (determinism pillar)
- Used by every cell that consumes randomness
- No cell creates its own RNG. All RNG comes from the registry defined here.

## 2. Specification

### 2.1 Algorithm

`ChaCha20Rng` from the `rand_chacha` crate. Chosen because:

- Reproducible across platforms and architectures
- Cryptographically strong (irrelevant here, but a nice side effect)
- Well-audited
- Deterministic seed derivation

### 2.2 Stream model

Every stream is identified by a **name** and derived from `(world_seed, stream_name)`.

```rust
pub struct RngRegistry {
    world_seed: u64,
    streams: BTreeMap<StreamName, ChaCha20Rng>,
}

impl RngRegistry {
    pub fn new(world_seed: u64) -> Self;
    pub fn stream(&mut self, name: &StreamName) -> &mut ChaCha20Rng;
}
