# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is (see `REPO-SNAPSHOT.md` for that). Update at the end of a session, not at the start.

**Last commit verified:** `72398d2` — "Create GitHub agent review report after aligning file naming convention"

**Note on the field above:** it points to the last commit *before* this
bookmark update (semantics S1). It will look "stale" compared to HEAD; that
is expected. Do not "fix" it by pointing to a newer commit without updating
the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md` — open and rejected ideas. Read it at session start to see what is not yet decided; do not implement from it.

**Next steps (in dependency order):**
0. **User priority for next session:** re-read ADR-005 (Qt for L3-ui) and
   decide on `IDEA-PRES-001`: revise ADR-005, or reject the idea. Until
   resolved, IDEA-PRES-001 cannot become code. IDEA-INFRA-004 is deferred
   (requires ADR-012 runtime, not yet implemented).
0bis. **Fix ADR-003 Plane A exception after ADR-020.** ADR-003 still names
   `pcfm-debug` and `demo.rs`; `pcfm-demo` reads Plane A (`GroundTruth`)
   without authorization. The code violates ADR-003 in letter. Options:
   new ADR-021 (recommended), amend ADR-003, or extend ADR-020. Requires
   explicit confirmation to touch `docs/adr/`.
1. ~~**Finish Grade A/B stale sweep**~~ — done (5/5 files: `PROJECT-IDEA-LOG.md`, `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `REPO-SNAPSHOT.md`, `MIGRATION-01-04.md`).
2. ~~**Revise `L0.05`**~~ — done (v1.1, two-phase temporal events, `PositionSnapshot`, `attempt_event_id`; see commit `e06d8cd`).
3. ~~**Grade C**~~ — done (4 files: `GLOSSARY.md`, `CODE_TOUR.md`, `ARCHITECTURE.md`, `PLUGIN_DEVELOPER_GUIDE.md`; commits `58c0ae9`, `5fa2b82`, `cb54ba5`, `9476456`, `b40fa0c`, `e2d69d8`).
3bis. ~~Riorganizzazione docs/context~~ — done (README onesto, review datate in `analysis/`, `GITHUB-AGENT-PROMPT.md` in radice, `PROJECT-STATUS.md` in root; commits `5498395`…`15f72f5`).
4. **Add "Documentation consistency" check** to `NEW-SESSION-PROMPT.md` (the grep + classification).
5. **Verification cascade** — re-read top-down (README → STRUCTURE → docs → ADR → modules) to confirm no logical blind spots remain.
5bis. **Functional doc-vs-repo verification** — use the GitHub agent (via an *issue*, not a chat prompt) to scan docs against the live repository: test counts, ADR ranges, paths, version numbers, stale claims. The agent produces `docs/context/analysis/YYYY-MM-DD-HH-MM-review.md`; the chat agent verifies every finding with `grep`/`sed` before applying. The agent finds, we verify.
6. ~~**Update `PROJECT-IDEA-LOG.md`** §3.10/§4.4~~ — done (same commit as L0.05, `e06d8cd`).
7. **C-4…C-9** — remaining ADR-019 deferred (deferred 3, possession, resolved).
8. **Ratify ADR-017**.
9. **`L1-world/01-nation.md`**.
10. **GitHub agent workflow (Phase 2)** — first test run on 2026-10-28: Copilot Coding Agent produced a valid report (2 findings, both verified and applied) but created an empty branch and delivered the report in chat instead of committing. Next run: assign an issue, not a chat prompt. Reports archived in `docs/context/analysis/`. Reactivate the ruleset on `main` only if the agent gets a write token (not the default — see `AGENT-POLICY.md` §8).
10bis. ~~**Rename `pcfm-debug` to `pcfm-test-runner`**~~ — done (ADR-020 implemented, commits `ca7537c`…`f2a8dfb`). 9 crates now: `pcfm-test-runner`, `pcfm-demo`, `pcfm-fixtures` added. 103 tests, clippy/fmt clean, `--check` 6/6. Docs updated: CONTRIBUTING, CODE_TOUR, PLUGIN_DEVELOPER_GUIDE, 00-foundations, PROJECT-STATUS, STRUCTURE, README, ARCHITECTURE. `REPO-SNAPSHOT.md` marked stale (regeneration deferred).
10ter. ~~**Record TUI-first presentation sketch**~~ — done (`IDEA-PRES-001`, commit `ef104e5`). IDEA-INFRA-004/005 aligned to the `pcfm-test-runner` rename (commit `3982147`). Next: re-read ADR-005 (step 0).
10quater. ~~**Record player footedness sketch**~~ — done (`IDEA-PM-005`,
   commit `0571d31`). Sketch only, open questions in the entry itself: model
   shape (two per-foot ratings FM-style vs categorical + weak-foot rating),
   which L0 actions read it, how to treat ambidexterity. Implementation
   requires an L0 contract revision (new module or revision + ADR if it
   touches ADR-019). Do not implement from the sketch.

0ter. **Amend ADR-019 D9** (referee non-physical). D9 currently says
   "in-pitch, with `Position` and functional state", but the agreed
   correction (2026-10-29) is: the referee is visible and follows play, but
   is NOT a physical obstacle (no collision, no influence on outcomes).
   Perception (what the referee sees or does not see) drives decisions.
   Requires ADR-019 amendment or a new ADR.

0quater. **Terminology: remove "module" from the repo** (2026-10-29
   decision). Four terms: pack (content), plugin (code), specification
   (document under `specs/`), framework crate (base crates like
   `pcfm-match`). Three tasks:
   - T1 (small): replace "module" as extension in `PLUGIN_DEVELOPER_GUIDE.md`
     §1; add the four terms to `GLOSSARY.md`.
   - T2 (small): "core module" → "framework crate" in
     `PLUGIN_DEVELOPER_GUIDE.md` §2 and §11.
   - T3 (large, separate commit): rename `modules/` → `specs/`. One logical
     commit. Do after closing in-progress discussions, not mixed with other
     work.

0quinquies. **Review new sketches from 2026-10-29** (`IDEA-TACT-001`,
   `IDEA-INFRA-007`, `IDEA-STAFF-001`). Decide which become ADRs, which
   become tasks, which stay sketches. All three depend on prerequisites
   that do not exist today: `L2-tactics`, runtime input channel, `L2-staff`.

**Files touched this session (2026-10-29):** `docs/context/IDEAS.md` (added IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001), `docs/context/SESSION-STATE.md`.
