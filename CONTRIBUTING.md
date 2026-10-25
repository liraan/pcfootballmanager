# Contributing

Thanks for wanting to help. This project is documentation-first, and **good documentation is the contribution** right now. Code exists (L0 implemented), but documentation and review are just as important.

---

## Current state (as of 2026-10-25)

- **L0 is implemented.** 6 crates (`pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-debug`), 103 tests passing.
- **8 ADRs exist:** 5 accepted (001, 002, 003, 004, 008), 3 proposed (005, 006, 007), 1 template (000).
- **2 ADRs are proposed:** 012 (modules) and 013 (worlds). They define the module system and the world system, and may revise pillar 1 of `00-foundations` (determinism, from global to local). They are **not** ratified: the framework does not implement the module system yet.
- **`LICENSE.md`** exists but is marked `proposed` pending ADR-006 ratification. The license terms are described in the file and in ADR-006.

If you're new, read the files in the order below before opening a PR.

---

## Read in this order

1. [`README.md`](README.md) — what the project is
2. [`modules/00-foundations.md`](modules/00-foundations.md) — the doctrine. **Non-negotiable.** *(Pillar 1 is under review, see ADR-013.)*
3. [`STRUCTURE.md`](STRUCTURE.md) — how the repo works
4. [`modules/_index.md`](modules/_index.md) — what exists, what's missing
5. [`docs/GLOSSARY.md`](docs/GLOSSARY.md) — the terms used across the spec
6. [`docs/MODULE_DEVELOPER_GUIDE.md`](docs/MODULE_DEVELOPER_GUIDE.md) — how to write a module (and why the simulation/rendering split matters)
7. [`docs/adr/`](docs/adr/) — the decisions, including open ones

---

## Ways to contribute

| What | How | Good for |
|---|---|---|
| **Fix an error** | Open a PR with the fix, or an issue if you're unsure | Everyone |
| **Revise a cell** | Follow §"Revise a cell" below | People who know the domain |
| **Write a missing cell** | Follow §"Add a cell" below | People who want a big task |
| **Propose a decision** | Write an ADR in `docs/adr/` | Architects |
| **Review an ADR** | Comment on the PR, argue with the trade-offs | Everyone |
| **Write code** | See `docs/MODULE_DEVELOPER_GUIDE.md` | Rust developers |
| **Write a module** | ADR-012 (proposed) defines the contract | Rust developers, once ratified |
| **Translate** | Contact the maintainers — translations go in `locales/` | Bilingual contributors |
| **Report a contradiction** | Open an issue with the `inconsistency` label | Careful readers |

---

## The rules

### One change per PR

- One cell. One ADR. One crate. One fix. No bundling.
- If a change touches two cells, two PRs.
- If a change requires an ADR, the ADR goes in a separate PR *before* the cell change.

### Frontmatter is mandatory

Every cell has it. See `STRUCTURE.md` §4. No frontmatter, no merge.

### The doctrine wins

If your change contradicts `00-foundations`, you have two options:

1. Change the change so it fits the doctrine.
2. Write a doctrine ADR that amends the doctrine, get it accepted, then change the cell.

You cannot silently contradict the doctrine.

### The levels matter

The specification is concentric (L0 → L1 → L2 → L3). A cell at level N may read the **output** of a level `< N` cell, never its internals, and never depend on a level `> N`. See `STRUCTURE.md` §2.

If your change requires a level N cell to depend on a level `> N` cell, the change is wrong. Discuss it in an issue first.

### Cite sources

Every factual claim gets a source. If you can't find one, say "unverified". Do not invent.

### Explain the WHY

A cell that says *what* without *why* is a draft. The `design_note` fields exist for this. Use them.

### If you spot an error, say so

Including in AI-drafted text. Especially in AI-drafted text. The AI disclosure in the README is not decoration — it's a warning.

---

## Revise a cell

1. **Bump the version** in the frontmatter.
   - Minor (`1.0 → 1.1`) for clarifications, typo fixes, added examples.
   - Major (`1.0 → 2.0`) for changes that affect downstream cells.
2. **If major, write an ADR first.** The ADR explains why. It's referenced from the cell.
3. **Archive the previous version** in `archive/` if the change is structural.
4. **Update `last_updated`.**
5. **Update `referenced_by`** in downstream cells if the dependency changed.
6. **Open a PR with one cell.**
7. **If the change is a status transition** (e.g. `draft → ratified`), update both `modules/_index.md` and the overview cell of the level (`00-overview.md` for L0).

---

## Add a cell

1. **Pick the level** (L0, L1, L2, L3) the new cell belongs to. See `STRUCTURE.md` §2.
2. **Check the dependency rules.** A level N cell may not depend on level `> N`.
3. **Create the directory** `modules/L{N}-{kebab-name}/`.
4. **Add `00-overview.md`** with the frontmatter (see `STRUCTURE.md` §4). The overview is the "border" of the cell: contracts, dependencies, DoD, risks.
5. **Add the sub-cells** as files inside the directory.
6. **Add a row to `modules/_index.md`.**
7. **Open a PR with one cell.**

A cell without:

- §0 Purpose
- Specification sections
- Definition of Done
- Risks & Mitigations

…is a draft. That's fine, mark it `status: draft` and merge it as a draft. But say so.

---

## The ADR process

An ADR captures **one decision**. It is immutable once accepted. To change a decision, write a new ADR that supersedes it.

### Propose an ADR

1. Copy `docs/adr/000-template.md` to `docs/adr/NNN-your-title.md`.
2. Fill in Context, Options, Decision (proposed), Consequences.
3. Set `status: proposed`.
4. Open a PR. Discussion happens on the PR.
5. When there's rough consensus, a maintainer merges it with `status: accepted`.

### Review an ADR

You don't need to be a maintainer to review. If the options are unbalanced, say so. If the decision is wrong, say so. If the ADR contradicts the doctrine, say so — that's the most important review you can do.

### Open ADRs block code

If an ADR is `proposed` and touches an area, **no code is written in that area until it's accepted**. This is how we avoid the "we'll decide later" trap.

### Current ADR status

| ADR | Title | Status | Blocks |
|---|---|---|---|
| 001 | Language: Rust | ✅ accepted | — |
| 002 | Project name and scope | ✅ accepted | — |
| 003 | Plane A enforcement | ✅ accepted | — (unblocked L0.04, L0.08, L0.10) |
| 004 | Panic strategy at FFI | ✅ accepted | — |
| 005 | Qt binding | 🟡 proposed | `L3-ui`, `pcfm-bridge` |
| 006 | License | 🟡 proposed | `LICENSE.md` (currently marked `proposed`), `Cargo.toml` examples |
| 007 | Canonical language | 🟡 proposed | translations, language policy in `STRUCTURE.md` §6 |
| 008 | Provider integration is external | ✅ accepted | — (reclassified `L2-ingest` as external tool) |
| 012 | Modules: The Brick Contract | 🟡 proposed | The module system (step 11+). Ratified when the framework implements it. |
| 013 | Worlds: LOD, Manifest, and Local Determinism | 🟡 proposed | The world system (step 11+). Supersedes pillar 1 of `00-foundations`. Ratified when the framework implements it. |

**Open ADRs:** 005, 006, 007, 012, 013. **Accepted:** 001, 002, 003, 004, 008.

**ADR-012 and ADR-013 are two halves of one decision.** Read them together, in order: [`012-013-reading-guide.md`](docs/adr/012-013-reading-guide.md). ADR-013 depends on ADR-012.

---

## Code

The L0 specification is complete and ratified, and L0 is implemented (6 crates, 103 tests). Code contributions are welcome, but they must follow `docs/MODULE_DEVELOPER_GUIDE.md`.

When you write code:

- `cargo fmt` and `cargo clippy -- -D warnings` clean
- Tests for every feature
- Property tests for invariants (money conservation, squad invariants)
- Golden tests for determinism
- No `unwrap()`/`expect()`/`panic!` in simulation paths
- No dependency without an ADR justifying it
- Public API documented
- `catch_unwind` at every FFI entry point (ADR-004)
- No public field on `GroundTruth`, no `Serialize`, custom `Debug` (ADR-003)
- The simulation/rendering split is respected: `simulate_match` is the only simulation, renderers are thin wrappers (see `docs/MODULE_DEVELOPER_GUIDE.md` §3)

---

## Commit messages

Imperative mood, one line ≤ 72 chars, then a body if needed.

```
Add ADR-012 on module contract

The module sheet is mandatory and validated. Behaviour is open,
metadata is closed. Critical conflicts block, minor conflicts warn.
```

Not: `fixed stuff`, `update`, `wip`.

---

## Review checklist (for reviewers)

- [ ] Frontmatter present and correct (see `STRUCTURE.md` §4)
- [ ] Doctrine not contradicted (or doctrine ADR linked)
- [ ] Level dependency rules not violated (see `STRUCTURE.md` §2)
- [ ] Sources cited for factual claims
- [ ] The WHY is explained, not just the what
- [ ] One change per PR
- [ ] If it touches an open ADR, the ADR is accepted first
- [ ] `modules/_index.md` updated if a cell was added
- [ ] If it is a status transition, the level's overview cell is updated too
- [ ] No contradiction with existing cells (or the contradiction is the point of the PR)
- [ ] Language is English (or a translation in `locales/`)
- [ ] No `License: Proprietary` string anywhere
- [ ] No `TBD` left in a field that should have a value
- [ ] If it is code, `docs/MODULE_DEVELOPER_GUIDE.md` is respected (plane separation, simulation/rendering split, no dependency without an ADR)

---

## Licensing of contributions

By contributing, you agree that:

- **Code contributions** are licensed under **MIT OR Apache-2.0**.
- **Asset and documentation contributions** are licensed under **CC-BY-NC-SA-4.0**.

See [`LICENSE.md`](LICENSE.md). The file is currently marked `proposed` pending ADR-006 ratification, but the terms are already the intended ones.

If you are contributing on behalf of an employer, make sure you have the right to do so under these terms.

---

## Code of conduct

Be plain. Argue with the idea, not the person. If you're wrong, say so. If you don't know, say so. The project's honesty about AI is the same honesty we expect from each other.

---

## Questions?

Open an issue with the `question` label. There are no private channels. Everything happens on GitHub.
