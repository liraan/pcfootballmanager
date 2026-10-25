# Migration — Legacy Modules 01-04

**Status:** pending
**Date:** 2026-10-25
**Affects:** `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md` (all removed from the current repository)
**Blocked by:** ADR-007 (canonical language), ADR-003 (Plane A enforcement)

<!--
  MAINTAINER NOTE: this document is a MIGRATION PLAN, not a migration itself.
  The four legacy modules (01-04) were removed when the concentric structure
  (L0-L3) replaced the flat 25-module numbering. Their content needs to be
  recovered from git history (or from the author's local copy) and re-integrated
  into the new structure.

  This file exists so the content is not lost. Do not delete it until the
  migration is complete (status: done).

  When complete, mark `status: done` and move this file to `archive/`.
-->

## 1. Context

The repository originally used a **flat 25-module structure** (00 to 24). The current structure is **concentric** (L0 → L1 → L2 → L3), described in `STRUCTURE.md` §2 and `modules/00-foundations.md` §9.

Four modules were written in the flat structure, in Italian:

- `01-architecture.md`
- `02-registry.md`
- `03-pack-system.md`
- `04-data-model.md`

When the concentric structure replaced the flat one, these four files were **removed from the repository** (see `CHANGELOG.md`, entry `[2026-10-25]` → "Removed"). Their content is recoverable from:

1. **Git history** — `git log --all --full-history -- modules/01-architecture.md` (and the same for the other three).
2. **The author's local copy** — if it exists.
3. **The archived prompt masters** — `Prompt_Master_v5.md`, `v6.md`, `v7.md` contain overlapping material. *(planned — the `archive/` directory does not exist yet)*

Until recovered, the repository does not contain the architectural detail (workspace layout, dependency rules, registry schema, pack validation, canonical data model) that those four modules provided. **This is a temporary gap.** It is not a decision that the content is unnecessary.

## 2. Target mapping

The content of each legacy module is re-integrated into the concentric structure as follows.

### `01-architecture.md` → distributed across L0, L3

| Legacy section | Target |
|---|---|
| Workspace structure, Cargo layout, `rust-toolchain.toml` | `L0-match/00-overview.md` (partially done) + a new **`L3-quality/00-overview.md`** *(planned)* |
| Determinism rules (RNG, BTreeMap, fixed-point) | Already extracted into `L0-match/01-rng.md` (done) and `00-foundations.md` §4 (done) |
| Dependency rules between crates | `L0-match/00-overview.md` §4 (partially done) + a new **`L3-quality/02-dependency-rules.md`** *(planned)* |
| Panic strategy at FFI | Extracted into `docs/adr/004-panic-strategy-ffi.md` (done) |
| Qt binding | Extracted into `docs/adr/005-qt-binding.md` (done) |
| Testing strategy, Miri, ASan, golden tests | `00-foundations.md` §4 + a new **`L3-quality/01-testing.md`** *(planned)* |
| `SIM_VERSION` bump procedure | `CHANGELOG-SIM.md` (done) |

### `02-registry.md` → L3

| Legacy section | Target |
|---|---|
| Whole module (roles, formations, instructions, attributes, deprecations) | A new **`L3-registry/`** cell *(planned)*, with sub-cells `00-overview`, `01-schema`, `02-deprecation`, `03-migrations` |
| Role schema, formation schema | `L3-registry/01-schema.md` *(planned)* |
| Deprecation policy, `replaced_by` | `L3-registry/02-deprecation.md` *(planned)* + `CHANGELOG-REGISTRY.md` (done) |
| Migration scripts, version compatibility | `L3-registry/03-migrations.md` *(planned)* |

### `03-pack-system.md` → L3

| Legacy section | Target |
|---|---|
| Whole module (manifest, validation, quarantine, sandbox, rollback) | A new **`L3-packs/`** cell *(planned)*, with sub-cells `00-overview`, `01-manifest`, `02-validation`, `03-sandbox`, `04-rollback` |
| Manifest schema | `L3-packs/01-manifest.md` *(planned)* |
| Validation rules, quarantine | `L3-packs/02-validation.md` *(planned)* |
| Sandbox, capability model, safe DSL | `L3-packs/03-sandbox.md` *(planned)* |
| Backup, rollback, migration | `L3-packs/04-rollback.md` *(planned)* |

### `04-data-model.md` → distributed across L0, L1, L2

The data model was the single largest legacy module. Its content is **already partially distributed**, and the rest is mapped as follows.

| Legacy section | Target | Status |
|---|---|---|
| Minimal entities (`Player`, `Squad`, `Pitch`, `Referee`, `Match`) | `L0-match/03-entities-minimal.md` | ✅ done |
| `MatchEvent`, `EventChain`, `CauseTrace` | `L0-match/05-match-events.md` | ✅ done |
| `Nation`, `Competition`, `Stage`, `TiebreakerRule` | A new **`L1-season/01-competitions.md`** and **`L1-world/01-nation.md`** *(planned)* | pending |
| `City`, `Stadium` | A new **`L1-world/02-city.md`** and **`L1-world/03-stadium.md`** *(planned)* | pending |
| Full `Player` (attributes, physical, psychological, career) | A new **`L1-roster/01-player-attributes.md`** *(planned)* | pending |
| `Staff`, `Contract` | A new **`L1-roster/02-staff.md`** and **`L1-roster/03-contracts.md`** *(planned)* | pending |
| `IndexSnapshot`, `ProjectionProfile` | A new **`L3-statistics/00-overview.md`** *(planned)*, with the caveat that the definitions must be free of CA/PA (ADR-003 and `00-foundations` §3, §5) | pending |
| `ScoutReport`, `DeclaredBias` | A new **`L2-scouting/00-overview.md`** *(planned)* | pending |
| `PsychologicalProfile` | A new **`L2-psyche/01-axes.md`** *(planned)* | pending |
| `DataProvenance`, `SourceType`, `ConfidenceLevel` | A new **`L3-registry/01-schema.md`** *(planned)*, as part of the registry schema | pending |
| Invariants (squad, economic, temporal, three-planes) | A new **`L3-quality/03-invariants.md`** *(planned)* | pending |
| Serialisation, `SaveGame`, migration | `L0-match/07-save-minimal.md` (minimal) + a new **`L3-quality/04-saves.md`** *(planned, full save system)* | partial |

## 3. What must be removed from the legacy content

The four legacy modules were written before ADR-003 (Plane A enforcement) and before the explicit ban on CA/PA. When their content is re-integrated, the following must be **removed or rewritten**:

1. **Any reference to `LatentAttributes` as a public struct or field.** Under ADR-003, option B, latents live only in `fmr-hidden` and are accessed only via `resolve_*`. A public `Player` struct with a `latent: LatentHandle` field is **not** acceptable (see `L0.03` §2.1).
2. **Any mention of CA/PA.** The `current_ability` / `potential_ability` scalars are banned (`00-foundations` §5). Where the legacy text uses them, replace with `IndexSnapshot` / `ProjectionProfile`.
3. **Any reference to `HashMap` in simulation paths.** Banned (`00-foundations` §4, `L0.01` §2.5). Replace with `BTreeMap` / `BTreeSet` / sorted `Vec`.
4. **Any hardcoded balance value.** Move to `balance/*.toml` *(planned)*.
5. **Any Italian prose.** Under ADR-007, the canonical language is English. The migration includes translation.
6. **Any reference to the flat 25-module numbering.** Replace with the concentric cell ID (`L0.01`, `L1.03`, ...).
7. **Any reference to `Proprietary` as a license.** Replace with `MIT OR Apache-2.0` for code, `CC-BY-NC-SA-4.0` for assets (ADR-006).

## 4. Procedure

The migration is done **one legacy module at a time**, in the order below. Each step is a separate PR.

### Step 1 — Recover

For each legacy module, run:

```bash
git log --all --full-history -- modules/01-architecture.md
git show <commit>:modules/01-architecture.md > /tmp/01-architecture.legacy.md
```

(and the same for the other three). The recovered file is **not** committed. It is a reference for the next step.

### Step 2 — Map

For each section of the recovered file, decide which target cell it goes to, using the table in §2. If a section does not fit any target, open an issue. Do not invent a new cell without a discussion.

### Step 3 — Rewrite

For each target cell:

1. Copy the relevant legacy section into the target cell (as English prose).
2. Apply the removals in §3.
3. Adapt the cell frontmatter (level, cell, module_id, version, status).
4. Add the target cell to `modules/_index.md`.
5. Update `referenced_by` in the cells that now reference it.

### Step 4 — Verify

- [ ] No reference to `LatentHandle` as a public field.
- [ ] No reference to CA/PA.
- [ ] No `HashMap` in a simulation path.
- [ ] No hardcoded balance value.
- [ ] English prose.
- [ ] Concentric cell ID, not flat module number.
- [ ] `MIT OR Apache-2.0` for code, `CC-BY-NC-SA-4.0` for assets.
- [ ] `conforms_to: "00-foundations (v1.1)"` in the frontmatter.
- [ ] `modules/_index.md` updated.
- [ ] `CHANGELOG.md` entry added.

### Step 5 — Archive

Once the four modules are fully migrated:

1. Create `archive/` *(planned)*.
2. Move the recovered legacy files into `archive/legacy/`.
3. Update `archive/README.md` *(planned)* to list them.
4. Mark this file `status: done`.
5. Move this file to `archive/MIGRATION-01-04.md`.

## 5. Current status

| Legacy module | Recovered | Mapped | Rewritten | Archived |
|---|---|---|---|---|
| `01-architecture.md` | ❌ | 🟡 partial (see §2) | 🟡 partial | ❌ |
| `02-registry.md` | ❌ | ❌ | ❌ | ❌ |
| `03-pack-system.md` | ❌ | ❌ | ❌ | ❌ |
| `04-data-model.md` | ❌ | 🟡 partial (see §2) | 🟡 partial | ❌ |

Legend: ❌ not started · 🟡 in progress · ✅ done

## 6. Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Content lost because it was never committed | Critical | Recover from git history or from the author's local copy before the history is rewritten |
| Migration introduces a CA/PA reference by copy-paste | Critical | Review checklist in §4 Step 4; CI grep for `current_ability`, `potential_ability` |
| Migration introduces a latent field by copy-paste | Critical | Review checklist in §4 Step 4; ADR-003 option B is the guard |
| Migration never finishes | High | Tracked here, one PR per legacy module, deadline TBD |
| Target cell does not exist | Medium | Create it as a stub first, mark `status: draft`, fill it in the same PR |

## 7. References

- [`../CHANGELOG.md`](../CHANGELOG.md) — entry `[2026-10-25]` → "Removed"
- [`../STRUCTURE.md`](../STRUCTURE.md) §2 — concentric levels
- [`../modules/00-foundations.md`](../modules/00-foundations.md) §3, §4, §5 — the bans that the migration must respect
- [`../modules/L0-match/03-entities-minimal.md`](../modules/L0-match/03-entities-minimal.md) — the `Player` struct without latents
- [`adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md) — no public latents
- [`adr/006-license.md`](adr/006-license.md) — MIT/Apache-2.0 + CC-BY-NC-SA-4.0
- [`adr/007-canonical-language.md`](adr/007-canonical-language.md) — English canonical
- `../archive/` — where the legacy files will be moved *(planned)*
