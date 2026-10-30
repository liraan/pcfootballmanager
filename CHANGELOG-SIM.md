# Changelog — SIM_VERSION

This file records every bump of `SIM_VERSION`, the version identifier of the **simulation behaviour**.

It is separate from [`CHANGELOG.md`](CHANGELOG.md) (which tracks documentation and structure) and from [`CHANGELOG-REGISTRY.md`](CHANGELOG-REGISTRY.md) (which tracks registry deprecations and migrations).

**Rule:** if the outcome of a simulation changes for the same seed and the same inputs, `SIM_VERSION` is bumped, and an entry is added here **before** the change is merged.

---

## Current SIM_VERSION

```
SIM_VERSION = 1
```

This is the **initial baseline**. No code exists yet, so no bump has occurred. When the first line of simulation code is merged, this value becomes the reference against which all future bumps are measured.

`SIM_VERSION` is defined in `pcfm-match` (planned) and embedded in every save (see `L0.07` §2.1).

---

## When to bump

`SIM_VERSION` is bumped when **any** of the following is true:

1. **RNG behaviour changes** — algorithm, seed derivation, or stream naming (see `L0.01` §2.6).
2. **Event emission changes** — an event type is added, removed, or its emission rules change in a way that affects the event stream (see `L0.05`).
3. **Resolution logic changes** — a `resolve_*` function in `pcfm-hidden` returns a different outcome for the same inputs (see `L0.03` §3.2).
4. **Tick loop changes** — the order of operations within a tick changes, or the tick granularity changes (see `L0.04` §2.2).
5. **Random event catalogue changes** — a `RandomEventDef` is added, removed, or its probability function changes in a way that affects the distribution (see `L0.08` §2.5).

If none of the above is true, `SIM_VERSION` is **not** bumped. A refactor that preserves behaviour does not bump `SIM_VERSION`. A new feature that does not affect the match does not bump `SIM_VERSION`.

---

## Bump procedure

A `SIM_VERSION` bump requires **all five steps**, in order. The CI will not merge a bump that skips any of them.

1. **Bump the value** in `pcfm-match` (planned). The value is a monotonic integer. It never goes down.
2. **Regenerate the golden hash** — the expected state hash for the 10-season golden test (see `00-foundations` §4). The new hash is committed alongside the bump.
3. **Write the entry** in the "History" section below, using the template.
4. **Verify save compatibility** — every existing save fixture must still load. If a save cannot be reconstructed under the new `SIM_VERSION`, the entry must say so explicitly and a migration must be provided (see `L0.07` §2.3).
5. **Review by a second pair of eyes** — a `SIM_VERSION` bump is never merged by the author alone. The reviewer confirms that the golden hash change is expected and that the entry is honest.

The full migration procedure for **saves** lives in [`docs/MIGRATION-01-04.md`](docs/MIGRATION-01-04.md). That document points back to this changelog for the bump side and forward to the save side.

---

## Entry template

Every entry has the same shape. Copy this template and fill it in.

```markdown
## [SIM_VERSION N] — YYYY-MM-DD

### Why the bump

One paragraph. What changed and why it could not be done without changing
simulation behaviour.

### What changes for the same seed

Concrete. "Match X (seed 12345) now ends 2-1 instead of 1-1 because ...".
If the change is statistical (affects distributions but not any single
match), say so and give the distribution delta.

### Saves affected

- Can existing saves still be loaded? (yes / no / with migration)
- Can existing saves be replayed? (yes / no / with warning)
- If a migration is required, where does it live? (link)

### Golden hash

- Previous: `0x...`
- New: `0x...`
- Verified by: (reviewer name or handle)

### Related changes

- `CHANGELOG.md` entry: (link or "none")
- `CHANGELOG-REGISTRY.md` entry: (link or "none")
- ADRs affected: (list or "none")
```

---

## History

*(no bumps yet — the project is at `SIM_VERSION = 1`, the initial baseline)*

---

<!--
  NOTES FOR FUTURE EDITORS:

  - Do not edit a past entry. If a past entry was wrong, add a correction
    entry and reference the one it corrects. The history is append-only.
  - Do not bump SIM_VERSION "to be safe". A bump is a promise to the user
    that their saves and replays are affected. Bump only when necessary.
  - If you are unsure whether a change requires a bump, ask in an issue
    before merging. The cost of a missed bump is a silent determinism
    regression, which is the worst kind of bug this project can have.
  - This file is linked from:
      - specs/00-foundations.md §4 (determinism)
      - specs/L0-match/01-rng.md §2.6
      - specs/L0-match/07-save-minimal.md §2.3
    If you move or rename this file, update those links.
-->
