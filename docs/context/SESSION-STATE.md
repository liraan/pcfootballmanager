# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is (see `REPO-SNAPSHOT.md` for that). Update at the end of a session, not at the start.

**Last commit verified:** `f2a8dfb` — "docs(repo-snapshot): mark stale since ADR-020"

**Note on the field above:** it points to the last commit *before* this
bookmark update (semantics S1). It will look "stale" compared to HEAD; that
is expected. Do not "fix" it by pointing to a newer commit without updating
the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md` — open and rejected ideas. Read it at session start to see what is not yet decided; do not implement from it.

**Next steps (in dependency order):**
0. **User priority for next session:** IDEA-INFRA-004 — extend `pcfm-debug` to cover installed modules and plugins (see `IDEAS.md`).
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

**Files touched this session:** `docs/context/README.md`, `docs/context/NEW-SESSION-PROMPT.md`, `docs/context/AGENT-POLICY.md`, `docs/context/SESSION-STATE.md`, `docs/context/GITHUB-AGENT-PROMPT.md`, `docs/context/IDEAS.md`, `docs/context/REPO-SNAPSHOT.md`, `docs/context/analysis/2026-10-28-09-42-review.md`, `docs/context/analysis/2026-10-28-14-14-review.md`, `docs/adr/020-split-test-runner-from-demo.md`, `PROJECT-STATUS.md`, `README.md`, `Cargo.toml`, `Cargo.lock`, `CONTRIBUTING.md`, `STRUCTURE.md`, `modules/00-foundations.md`, `docs/ARCHITECTURE.md`, `docs/CODE_TOUR.md`, `docs/PLUGIN_DEVELOPER_GUIDE.md`, `crates/pcfm-test-runner/`, `crates/pcfm-demo/`, `crates/pcfm-fixtures/`.
