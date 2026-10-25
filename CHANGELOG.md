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
- Create `archive/` directory with the three Prompt Master versions (v5, v6, v7).
- Create `docs/MIGRATION-01-04.md` and complete the migration of the four legacy Italian modules.
- Add `.github/PULL_REQUEST_TEMPLATE.md` and `.github/ISSUE_TEMPLATE/*`.
- Add `.gitignore`.
- Add empty placeholder directories (`crates/`, `registry/`, `mods/`, `data/`) with `.gitkeep`.
- Add `docs/legal/` and `docs/calibration/` placeholders.

---

## [2026-10-25] — Structure v1.1: concentric model, L0 complete, ADRs

### Added

- **Concentric structure (L0–L3)** in `00-foundations.md` §9 and `STRUCTURE.md` §2.
- **`modules/L0-match/`** — 10 cells: `00-overview`, `01-rng`, `02-event-bus`, `03-entities-minimal`, `04-match-engine`, `05-match-events`, `06-match-output`, `07-save-minimal`, `08-random-events`, `09-context-modifiers`.
- **`docs/adr/`** — 8 ADRs: `000-template`, `001-language-rust`, `002-project-name-and-scope`, `003-plane-a-enforcement`, `004-panic-strategy-ffi`, `005-qt-binding`, `006-license`, `007-canonical-language`.
- **`LICENSE.md`**, `LICENSE-MIT`, `LICENSE-APACHE`, `LICENSE-ASSETS` (proposed, pending ADR-006 ratification).
- **`CONTRIBUTING.md`** — process, review checklist, ADR workflow, licensing of contributions.
- **`STRUCTURE.md`** — meta-document: layout, frontmatter, ADR process, language policy, level rules.

### Changed

- **`README.md`** — updated to reflect the concentric structure, the dual license proposal, and the four open ADRs. Removed the flat "modules 5 of 25" badge.
- **`modules/00-foundations.md`** — bumped to `v1.1`, added §9 (Concentric Structure), linked to ADR-003 explicitly.
- **`modules/_index.md`** — bumped to `v2.1`. Rewritten as a concentric map. L1–L3 cells are marked `planned` and their links are not clickable (they do not exist yet).
- **`docs/adr/002-project-name-and-scope.md`** — `Blocks:` no longer mentions `22-roadmap.md` (the flat module no longer exists) or `LICENSE.md` (that is ADR-006's block).

### Removed

- **Flat 25-module structure** (previously implied by `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md`). These four files were removed from the repository. Their content is being migrated to the concentric structure; a formal `docs/MIGRATION-01-04.md` will track the migration. Until then, the content is considered lost from the repository and must be recovered from history or from the author's local copy.

### Fixed

- **Broken links to `archive/Prompt_Master_v7.md`** in `00-foundations.md`, `L0.00`, `L0.01`, `L0.04`, `L0.05`, `L0.06`, `L0.08`. Now marked `(planned)` in plain text; they will be restored when the `archive/` directory is created.
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
-->
