# Changelog

All notable changes to the **documentation and project structure** are recorded here.

For simulation-behaviour changes, see [`CHANGELOG-SIM.md`](CHANGELOG-SIM.md).
For registry deprecations and migrations, see [`CHANGELOG-REGISTRY.md`](CHANGELOG-REGISTRY.md).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project does not use semantic versioning for the docs; the version of each module is in its own frontmatter.

---

## [Unreleased]

### Planned

- Write L1 cells (season, world, roster): 12 files.
- Write L2 cells (tactics, market, finance, youth, scouting, psyche, crowd, media, ai): 9 cells.
- Write L3 cells (registry, packs, generator, editor, assets, ui, statistics, quality): 8 cells.
- Recover and archive the three Prompt Master versions (v5, v6, v7) into `archive/prompts/`.
- Complete the migration of the four legacy Italian modules (`01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md`), tracked in [`docs/MIGRATION-01-04.md`](docs/MIGRATION-01-04.md).
- Begin code: `fmr-core`, `fmr-hidden`, `fmr-match` (L0 implementation).
- Ratify ADR-005 (Qt binding), ADR-006 (license), ADR-007 (canonical language).

---

## [2026-10-25] — ADR-003 ratified: Plane A enforcement (option B)

### Changed

- **ADR-003** (`docs/adr/003-plane-a-enforcement.md`): status `proposed` → **`accepted`**. Option B (capability API) is now the binding decision. `fmr-hidden` exposes only `resolve_*` and `drain_observations`; latents never cross the crate boundary.
- **`modules/L0-match/04-match-engine.md`**: version 1.1 → **1.2**, status `draft` → **`ratified`**. The match engine can now be implemented.
- **`modules/L0-match/08-random-events.md`**: version 1.1 → **1.2**, status `draft` → **`ratified`**. The random events cell can now be implemented.
- **`modules/L0-match/10-ground-truth.md`**: version 1.0 → **1.1**, status `draft` → **`ratified`**. The `GroundTruth` API is now the implementation contract.
- **`modules/L0-match/00-overview.md`**: version 1.1 → **1.2**. All 11 cells `ratified`; DoD split into "specification (complete)" and "implementation (pending)".
- **`modules/_index.md`**: version 2.2 → **2.3**. All L0 cells `ratified`. ADR-003 moved from "Open ADRs" to "Accepted". Cross-cell components table updated.
- **`STRUCTURE.md`**: version 1.2 → **1.3**. L0 directory layout shows 11 cells (was 10). ADR-003 status updated in §5. Open ADRs reduced to 005, 006, 007.
- **`CONTRIBUTING.md`**: version 1.0 → **1.1**. "Current state" reflects L0 ratified and code-ready. ADR status table shows ADR-003 accepted. "Code (when it exists)" → "Code (now that L0 is ratified)".
- **`README.md`**: version 1.0 → **1.1**. Project status shows L0 at 100% (11 cells, ADR-003 accepted). Badge `status-L0_ratified`. FAQ: "Four remain unresolved" → "Three remain unresolved" (ADR-003 removed). Documentation Map includes GLOSSARY.

### Summary

Before this entry, three L0 cells were `draft` (L0.04, L0.08, L0.10) and blocked by ADR-003. After this entry:

- **All 11 L0 cells are `ratified`.**
- **ADR-003 is `accepted`** (option B: capability API, `GroundTruth` with `resolve_*` only).
- **The L0 specification is complete.**
- **Code can begin**, starting from `fmr-core`, `fmr-hidden`, and `fmr-match` (see `STRUCTURE.md` §3).

The Doctrine is unchanged. `00-foundations` remains at v1.1.

---

## [2026-10-25] — Structure v1.1: glossary, L0.10, L0 cells v1.1

### Added

- **`docs/GLOSSARY.md`** (v1.0) — 30+ terms with definitions and links to the source. Sections: Core, Data model, Determinism, Architecture, Process, Banned, External, Abbreviations, Historical.
- **`modules/L0-match/10-ground-truth.md`** (v1.0, `draft`) — the cell that centralises the public API of `fmr-hidden`. Before this cell, the API was scattered across L0.03 §3.2, L0.04 §1, L0.05 §2.3 and ADR-003.
- **9 `.gitkeep` files** in `crates/`, `registry/`, `mods/`, `data/`, `docs/legal/`, `docs/calibration/`, `archive/prompts/`, `archive/doctrine/`, `archive/legacy/` — placeholder directories kept in git.

### Changed

- **`modules/_index.md`**: version 2.1 → **2.2** (then → 2.3, see next entry). L0.10 row added. Cross-cell components table added. ADR status table added.
- **`modules/L0-match/00-overview.md`**: version 1.0 → **1.1**. L0.10 added to the cell table and dependency rules. §5 (Three Planes) now points to L0.10 for the `GroundTruth` API.
- **`modules/L0-match/03-entities-minimal.md`**: version 1.0 → **1.1**. §3.2 "Where are the latents?" now points to L0.10 instead of duplicating the API.
- **`modules/L0-match/04-match-engine.md`**: version 1.0 → **1.1**. Depends on L0.10. The `GroundTruth` API is referenced, not duplicated.
- **`modules/L0-match/07-save-minimal.md`**: version 1.0 → **1.1**. Depends on L0.10. §2.6 (GroundTruth reconstruction) points to L0.10 §2.1 for the API contract.
- **`modules/L0-match/08-random-events.md`**: version 1.0 → **1.1**. Depends on L0.10. The `resolve_*` API is referenced, not duplicated.
- **`modules/L0-match/09-context-modifiers.md`**: version 1.0 → **1.1**. §7 References now includes `docs/GLOSSARY.md`.

### Summary

The `fmr-hidden` API now has a single home (`L0.10`). Before, a reader who wanted to understand "what can `fmr-hidden` do?" had to read four files. After, they read one. The other L0 cells link to it.

---

## [2026-10-25] — Structure v1.1: concentric model, L0 complete, ADRs

### Added

- **Concentric structure (L0–L3)** in `00-foundations.md` §9 and `STRUCTURE.md` §2.
- **`modules/L0-match/`** — 10 cells: `00-overview`, `01-rng`, `02-event-bus`, `03-entities-minimal`, `04-match-engine`, `05-match-events`, `06-match-output`, `07-save-minimal`, `08-random-events`, `09-context-modifiers`.
- **`docs/adr/`** — 8 ADRs: `000-template`, `001-language-rust`, `002-project-name-and-scope`, `003-plane-a-enforcement`, `004-panic-strategy-ffi`, `005-qt-binding`, `006-license`, `007-canonical-language`.
- **`LICENSE.md`**, `LICENSE-MIT`, `LICENSE-APACHE`, `LICENSE-ASSETS` (proposed, pending ADR-006 ratification).
- **`CONTRIBUTING.md`** — process, review checklist, ADR workflow, licensing of contributions.
- **`STRUCTURE.md`** — meta-document: layout, frontmatter, ADR process, language policy, level rules.
- **`.github/PULL_REQUEST_TEMPLATE.md`** and `.github/ISSUE_TEMPLATE/` (`bug_report.md`, `module-revision.md`).
- **`.gitignore`** — Rust artifacts, profiling, IDE, OS, user data, secrets, pack cache, generated worlds.
- **`CHANGELOG.md`**, `CHANGELOG-SIM.md`, `CHANGELOG-REGISTRY.md`.
- **`docs/MIGRATION-01-04.md`** — migration plan for the four legacy Italian modules.
- **`archive/README.md`** — rules for the archive folder.

### Changed

- **`README.md`** — updated to reflect the concentric structure, the dual license proposal, and the four open ADRs. Removed the flat "modules 5 of 25" badge.
- **`modules/00-foundations.md`** — bumped to `v1.1`, added §9 (Concentric Structure), linked to ADR-003 explicitly.
- **`modules/_index.md`** — bumped to `v2.0`. Rewritten as a concentric map. L1–L3 cells are marked `planned` and their links are not clickable (they do not exist yet).
- **`docs/adr/002-project-name-and-scope.md`** — `Blocks:` no longer mentions `22-roadmap.md` (the flat module no longer exists) or `LICENSE.md` (that is ADR-006's block).

### Removed

- **Flat 25-module structure** (previously implied by `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md`). These four files were removed from the repository. Their content is being migrated to the concentric structure; a formal `docs/MIGRATION-01-04.md` tracks the migration. Until then, the content is considered lost from the repository and must be recovered from history or from the author's local copy.

### Fixed

- **Broken links to `archive/Prompt_Master_v7.md`** in `00-foundations.md`, `L0.00`, `L0.01`, `L0.04`, `L0.05`, `L0.06`, `L0.08`. Now marked `(planned)` in plain text; they will be restored when the `archive/` directory is populated.
- **Broken links to `balance/*.toml`, `crates/*`, `locales/*`, `L1-*`, `L2-*`, `L3-*`** in all L0 cells. Now marked `(planned)` in plain text.
- **Inconsistent `conforms_to` field** — all ratified L0 cells now declare `conforms_to: "00-foundations (v1.1)"`.

---

## [2026-10-24] — Initial documentation drop

### Added

- `README.md` — project overview, AI disclosure, status, technical philosophy.
- `modules/00-foundations.md` — v1.0, the doctrine.
- `modules/01-architecture.md` — Rust workspace, determinism, dependencies.
- `modules/02-registry.md` — modular registry.
- `modules/03-pack-system.md` — local packs.
- `modules/04-data-model.md` — canonical schema.

### Notes

- This drop used a **flat 25-module numbering**. It was superseded the following day by the concentric model.
- `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md` were written in Italian. Under ADR-007 they will be translated to English before being re-integrated into the concentric structure.

---

<!--
  NOTES FOR FUTURE EDITORS:

  - Keep this file honest. If a release date slips, update the date and say so.
    Do not delete entries.
  - One entry per logical change. A change that touches code and docs goes in
    both this file and CHANGELOG-SIM.md (if it changes simulation behaviour).
  - A SIM_VERSION bump is recorded in CHANGELOG-SIM.md, not here.
  - A registry deprecation is recorded in CHANGELOG-REGISTRY.md, not here.
  - Do not use marketing language. "Refactor", "fix", "remove" are fine.
  - When an ADR is ratified, add an entry with the list of files whose status
    changed as a consequence (this is the L0.04 / L0.08 / L0.10 pattern).
-->
