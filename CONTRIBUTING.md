# Contributing

Thanks for wanting to help. This project is documentation-first, and **good documentation is the contribution** right now. Code comes later.

---

## Read in this order

1. [`README.md`](README.md) — what the project is
2. [`modules/00-foundations.md`](modules/00-foundations.md) — the doctrine. **Non-negotiable.**
3. [`STRUCTURE.md`](STRUCTURE.md) — how the repo works
4. [`modules/_index.md`](modules/_index.md) — what exists, what's missing
5. [`docs/adr/`](docs/adr/) — the decisions, including open ones

---

## Ways to contribute

| What | How | Good for |
|---|---|---|
| **Fix an error** | Open a PR with the fix, or an issue if you're unsure | Everyone |
| **Revise a cell** | Follow §"Revise a cell" below | People who know the domain |
| **Write a missing cell** | Follow §"Add a cell" below | People who want a big task |
| **Propose a decision** | Write an ADR in `docs/adr/` | Architects |
| **Review an ADR** | Comment on the PR, argue with the trade-offs | Everyone |
| **Translate** | Contact the maintainers — translations go in `locales/` | Bilingual contributors |
| **Report a contradiction** | Open an issue with the `inconsistency` label | Careful readers |

---

## The rules

### One change per PR

- One cell. One ADR. One fix. No bundling.
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

### Current open ADRs

| ADR | Title | Blocks |
|---|---|---|
| 003 | Plane A enforcement | `L0.04`, `L0.08` |
| 005 | Qt binding | `L3-ui`, `fmr-bridge` |
| 006 | License | `LICENSE.md`, every `Cargo.toml` example |
| 007 | Canonical language | translations, language policy |

---

## Code (when it exists)

Not yet. When it does:

- `cargo fmt` and `cargo clippy -- -D warnings` clean
- Tests for every feature
- Property tests for invariants (money conservation, squad invariants)
- Golden tests for determinism
- No `unwrap()`/`expect()`/`panic!` in simulation paths
- No dependency without an ADR justifying it
- Public API documented

---

## Commit messages

Imperative mood, one line ≤ 72 chars, then a body if needed.

```
Add ADR-006 on license choice

The README says "open" but Cargo.toml says "Proprietary".
This ADR proposes MIT + CC-BY-NC-SA-4.0 and explains
the trade-offs.
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
- [ ] No contradiction with existing cells (or the contradiction is the point of the PR)
- [ ] Language is English (or a translation in `locales/`)

---

## Licensing of contributions

By contributing, you agree that:

- **Code contributions** are licensed under **MIT OR Apache-2.0** (see `LICENSE.md`).
- **Asset and documentation contributions** are licensed under **CC-BY-NC-SA-4.0** (see `LICENSE.md`).

If you are contributing on behalf of an employer, make sure you have the right to do so under these terms.

---

## Code of conduct

Be plain. Argue with the idea, not the person. If you're wrong, say so. If you don't know, say so. The project's honesty about AI is the same honesty we expect from each other.

---

## Questions?

Open an issue with the `question` label. There are no private channels. Everything happens on GitHub.
