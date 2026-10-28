# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is (see `REPO-SNAPSHOT.md` for that). Update at the end of a session, not at the start.

**Last commit verified:** `4c639c6` — "docs(context): add bookmark-semantics and sed-pitfalls rules to prompt"

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
4. **Add "Documentation consistency" check** to `NEW-SESSION-PROMPT.md` (the grep + classification).
5. **Verification cascade** — re-read top-down (README → STRUCTURE → docs → ADR → modules) to confirm no logical blind spots remain.
6. ~~**Update `PROJECT-IDEA-LOG.md`** §3.10/§4.4~~ — done (same commit as L0.05, `e06d8cd`).
7. **C-4…C-9** — remaining ADR-019 deferred (deferred 3, possession, resolved).
8. **Ratify ADR-017**.
9. **`L1-world/01-nation.md`**.
10. **GitHub agent workflow (Phase 2)** — set up the GitHub agent per `docs/context/github-agent/PROMPT.md`; it produces analysis reports, not code changes. Reactivate the ruleset on `main` only if the agent gets a write token (not the default — see `AGENT-POLICY.md` §8).

**Files touched this session:** `docs/context/PROJECT-IDEA-LOG.md`, `docs/context/NEW-SESSION-PROMPT.md`, `docs/context/SESSION-STATE.md`, `docs/context/REPO-SNAPSHOT.md`, `docs/MIGRATION-01-04.md`, `modules/L0-match/05-match-events.md`, `docs/context/IDEAS.md`.
