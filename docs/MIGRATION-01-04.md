# Migration — Legacy Modules 01-04

**Status:** pending (content not recovered)
**Date:** 2026-10-25
**Affects:** `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md` (all removed from the current repository, **not recoverable from git history**)
**Blocked by:** ADR-007 (canonical language), the L3 cells not being written yet

<!--
  MAINTAINER NOTE: this document is a MIGRATION PLAN, not a migration itself.

  IMPORTANT: the four legacy modules were never committed to git. The `git log
  --all --full-history` command confirms they do not exist in any commit. They
  were created locally, never pushed, and then overwritten when the concentric
  structure replaced the flat 25-module numbering.

  Their content is partially recoverable from the initial conversation of the
  project (the files were pasted into the chat as attachments), but it has not
  been recreated in the repository. This is a known technical debt.

  Do NOT recreate them speculatively. Recreate them when the migration
  actually happens (i.e. when L3-registry, L3-packs, L1-roster are written).
  Until then, the concentric structure already covers their content in
  fragments (see §2 "Target mapping").

  When the migration is complete, mark `status: done` and move this file to
  `archive/`.
-->

## 1. Context

The repository originally used a **flat 25-module structure** (00 to 24). The current structure is **concentric** (L0 → L1 → L2 → L3), described in `STRUCTURE.md` §2 and `specs/00-foundations.md` §9.

Four modules were written in the flat structure, in Italian:

- `01-architecture.md`
- `02-registry.md`
- `03-pack-system.md`
- `04-data-model.md`

When the concentric structure replaced the flat one, these four files were **removed from the repository** (see `CHANGELOG.md`, entry `[2026-10-25]` → "Removed").

**Critical note:** these files were **never committed to git**. The command:

```bash
git log --all --full-history -- modules/01-architecture.md
```

returns nothing. They were created locally, never pushed, then overwritten. The git history does not contain them.

The content is **partially recoverable** from the initial conversation of the project (the four files were pasted as attachments). It has not been recreated in the repository. This is a known technical debt, tracked here.

## 2. Target mapping

The content of each legacy module is re-integrated into the concentric structure as follows.

### `01-architecture.md` → distributed across L0, L3

| Legacy section | Target | Status |
|---|---|---|
| Workspace structure, Cargo layout, `rust-toolchain.toml` | `STRUCTURE.md` §3 + a new **`L3-quality/00-overview.md`** *(planned)* | partial |
| Determinism rules (RNG, BTreeMap, fixed-point) | `L0-match/01-rng.md` + `00-foundations.md` §4 | ✅ done |
| Dependency rules between crates | `L0-match/00-overview.md` §4 + a new **`L3-quality/02-dependency-rules.md`** *(planned)* | partial |
| Panic strategy at FFI | `docs/adr/004-panic-strategy-ffi.md` | ✅ done |
| Qt binding | `docs/adr/005-qt-binding.md` | ✅ done |
| Testing strategy, Miri, ASan, golden tests | `00-foundations.md` §4 + a new **`L3-quality/01-testing.md`** *(planned)* | partial |
| `SIM_VERSION` bump procedure | `CHANGELOG-SIM.md` | ✅ done |

### `02-registry.md` → L3

| Legacy section | Target | Status |
|---|---|---|
| Whole module (roles, formations, instructions, attributes, deprecations) | A new **`L3-registry/`** cell *(planned)*, with sub-cells `00-overview`, `01-schema`, `02-deprecation`, `03-migrations` | not started |
| Role schema, formation schema | `L3-registry/01-schema.md` *(planned)* | not started |
| Deprecation policy, `replaced_by` | `L3-registry/02-deprecation.md` *(planned)* + `CHANGELOG-REGISTRY.md` | partial |
| Migration scripts, version compatibility | `L3-registry/03-migrations.md` *(planned)* | not started |

### `03-pack-system.md` → L3

| Legacy section | Target | Status |
|---|---|---|
| Whole module (manifest, validation, quarantine, sandbox, rollback) | A new **`L3-packs/`** cell *(planned)*, with sub-cells `00-overview`, `01-manifest`, `02-validation`, `03-sandbox`, `04-rollback` | not started |
| Manifest schema | `L3-packs/01-manifest.md` *(planned)* | not started |
| Validation rules, quarantine | `L3-packs/02-validation.md` *(planned)* | not started |
| Sandbox, capability model, safe DSL | `L3-packs/03-sandbox.md` *(planned)* | not started |
| Backup, rollback, migration | `L3-packs/04-rollback.md` *(planned)* | not started |

### `04-data-model.md` → distributed across L0, L1, L2

| Legacy section | Target | Status |
|---|---|---|
| Minimal entities (`Player`, `Squad`, `Pitch`, `Referee`, `Match`) | `L0-match/03-entities-minimal.md` | ✅ done |
| `MatchEvent`, `EventChain`, `CauseTrace` | `L0-match/05-match-events.md` | ✅ done |
| `GroundTruth` API | `L0-match/10-ground-truth.md` | ✅ done |
| `Nation`, `Competition`, `Stage`, `TiebreakerRule` | A new **`L1-season/01-competitions.md`** and **`L1-world/01-nation.md`** *(planned)* | not started |
| `City`, `Stadium` | A new **`L1-world/02-city.md`** and **`L1-world/03-stadium.md`** *(planned)* | not started |
| Full `Player` (attributes, physical, psychological, career) | A new **`L1-roster/01-player-attributes.md`** *(planned)* | not started |
| `Staff`, `Contract` | A new **`L1-roster/02-staff.md`** and **`L1-roster/03-contracts.md`** *(planned)* | not started |
| `IndexSnapshot`, `ProjectionProfile` | A new **`L3-statistics/00-overview.md`** *(planned)*, free of CA/PA (ADR-003, `00-foundations` §3, §5) | not started |
| `ScoutReport`, `DeclaredBias` | A new **`L2-scouting/00-overview.md`** *(planned)* | not started |
| `PsychologicalProfile` | A new **`L2-psyche/01-axes.md`** *(planned)* | not started |
| `DataProvenance`, `SourceType`, `ConfidenceLevel` | A new **`L3-registry/01-schema.md`** *(planned)* | not started |
| Invariants (squad, economic, temporal, three-planes) | A new **`L3-quality/03-invariants.md`** *(planned)* | not started |
| Serialisation, `SaveGame`, migration | `L0-match/07-save-minimal.md` (minimal) + a new **`L3-quality/04-saves.md`** *(planned, full save system)* | partial |

## 3. What must be removed from the legacy content

The four legacy modules were written before ADR-003 (Plane A enforcement) and before the explicit ban on CA/PA. When their content is re-integrated, the following must be **removed or rewritten**:

1. **Any reference to `LatentAttributes` as a public struct or field.** Under ADR-003, option B, latents live only in `pcfm-hidden` and are accessed only via `resolve_*`. A public `Player` struct with a `latent: LatentHandle` field is **not** acceptable (see `L0.03` §2.1).
2. **Any mention of CA/PA.** The `current_ability` / `potential_ability` scalars are banned (`00-foundations` §5). Where the legacy text uses them, replace with `IndexSnapshot` / `ProjectionProfile`.
3. **Any reference to `HashMap` in simulation paths.** Banned (`00-foundations` §4, `L0.01` §2.5). Replace with `BTreeMap` / `BTreeSet` / sorted `Vec`.
4. **Any hardcoded balance value.** Move to `balance/*.toml` *(planned)*.
5. **Any Italian prose.** Under ADR-007, the canonical language is English. The migration includes translation.
6. **Any reference to the flat 25-module numbering.** Replace with the concentric cell ID (`L0.01`, `L1.03`, ...).
7. **Any reference to `Proprietary` as a license.** Replace with `MIT OR Apache-2.0` for code, `CC-BY-NC-SA-4.0` for assets (ADR-006).

## 4. Procedure (when the migration actually happens)

The migration is done **one legacy module at a time**, in the order below. Each step is a separate PR.

**The migration is not urgent.** The concentric structure already covers the content in fragments (see §2). The migration should happen **when the corresponding L1/L3 cell is written**, not before. Recreating the legacy files now, only to migrate them immediately, is wasted work.

### Step 1 — Recover (if possible)

If the four files still exist on the author's local disk (Desktop, Downloads, a pre-concentric copy of the repository), copy them into `archive/legacy/` via the GitHub web interface, one file at a time. This preserves them for traceability.

If they no longer exist, the content is recoverable from the initial project conversation (they were pasted as attachments). This is a manual copy-paste, done once, at the time of the migration.

### Step 2 — Map

For each section of the legacy file, decide which target cell it goes to, using the table in §2. If a section does not fit any target, open an issue. Do not invent a new cell without a discussion.

### Step 3 — Rewrite

For each target cell:

1. Copy the relevant legacy section into the target cell (as English prose).
2. Apply the removals in §3.
3. Adapt the cell frontmatter (level, cell, module_id, version, status).
4. Add the target cell to `specs/_index.md`.
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
- [ ] `specs/_index.md` updated.
- [ ] `CHANGELOG.md` entry added.

## 5. Current status

| Legacy module | Recoverable from disk | Recoverable from chat | Migrated | Archived |
|---|---|---|---|---|
| `01-architecture.md` | unknown | yes | 🟡 partial | ❌ |
| `02-registry.md` | unknown | yes | ❌ not started | ❌ |
| `03-pack-system.md` | unknown | yes | ❌ not started | ❌ |
| `04-data-model.md` | unknown | yes | 🟡 partial | ❌ |

Legend: ❌ not started · 🟡 in progress · ✅ done · "unknown" = not checked

## 6. Risks

| Risk | Severity | Mitigation |
|---|---|---|
| Content lost because it was never committed | **Realized** | Content is recoverable from the initial chat, but not from git. This is a known debt. |
| Migration never finishes | High | Tracked here, one PR per legacy module, triggered by L1/L3 cells being written |
| Migration introduces a CA/PA reference by copy-paste | Critical | Review checklist in §4 Step 4; CI grep for `current_ability`, `potential_ability` |
| Migration introduces a latent field by copy-paste | Critical | Review checklist in §4 Step 4; ADR-003 option B is the guard |
| The debt is forgotten | Medium | This file is linked from `CHANGELOG.md` and `STRUCTURE.md`; the `archive/legacy/` directory has a `.gitkeep` |

## 7. References

- [`../CHANGELOG.md`](../CHANGELOG.md) — entry `[2026-10-25]` → "Removed"
- [`../STRUCTURE.md`](../STRUCTURE.md) §2 — concentric levels
- [`../specs/00-foundations.md`](../specs/00-foundations.md) §3, §4, §5 — the bans that the migration must respect
- [`../specs/L0-match/03-entities-minimal.md`](../specs/L0-match/03-entities-minimal.md) — the `Player` struct without latents
- [`adr/003-plane-a-enforcement.md`](adr/003-plane-a-enforcement.md) — no public latents
- [`adr/006-license.md`](adr/006-license.md) — MIT/Apache-2.0 + CC-BY-NC-SA-4.0
- [`adr/007-canonical-language.md`](adr/007-canonical-language.md) — English canonical
- [`../archive/legacy/`](../archive/legacy/) — where the legacy files will be moved *(planned — the directory exists, empty)*
