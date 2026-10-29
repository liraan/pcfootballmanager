# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is (see `REPO-SNAPSHOT.md` for that). Update at the end of a session, not at the start.

**Last commit verified:** `3a7673d` — "docs(session-state): replace Next steps list with two-table plan (status, todo) + details"

**Note on the field above:** it points to the last commit *before* this
bookmark update (semantics S1). It will look "stale" compared to HEAD; that
is expected. Do not "fix" it by pointing to a newer commit without updating
the rest of the bookmark in the same commit.

**Ideas backlog:** `docs/context/IDEAS.md` — open and rejected ideas. Read it at session start to see what is not yet decided; do not implement from it.

<!-- Formato Piano: legenda in alto, due tabelle (Stato lavoro, Da fare) con emoji di priorità, dettagli sotto. Se cambi la forma, aggiorna questa nota nello stesso commit. Non riformattare senza motivo. -->

**Legenda:** ✅ done · 🔵 in corso · ⚫ bloccato · 🔴 urgente · 🟡 prossimo · ⬜ aperto

**Stato lavoro (sessione 2026-10-28/29):**

| Stato | Cosa |
|-------|------|
| ✅    | ADR-021: D9 referee non-fisico (Supersedes ADR-019 D9) |
| ✅    | Hermes policy: §1 reconcile, §8 retitle, Last updated, eccezione push audit |
| ✅    | 0septies chiuso (opzione A: eccezione scritta in `AGENT-POLICY.md`) |
| ✅    | Sketch ×3 in `IDEAS.md`: TACT-001, INFRA-007, STAFF-001 |
| ✅    | Audit Hermes 2026-10-29 verificato: 4/4 finding veri |
| ✅    | `NEW-SESSION-PROMPT` consolidato: 13→8 regole, startup + closing ritual, B1/A1/B2 |
| 🔵    | Tabella Piano in `SESSION-STATE` (questa sessione) |
| ⚫    | — (nessun bloccato) |

**Da fare (in dependency order):**

| Pr. | #  | Cosa                    | Dipende da                   |
|-----|----|-------------------------|------------------------------|
| 🔴  | 1  | ADR-005 / IDEA-PRES-001 | —                            |
| 🟡  | 2  | Fix ADR-003             | —                            |
| 🟡  | 3  | Terminologia            | —                            |
| 🟡  | 4  | Audit 2026-10-29 (T1,T2)| —                            |
| 🟡  | 11 | GitHub agent Phase 2    | ruleset attivo               |
| ⬜  | 5  | Review sketch ×3        | L2-tactics, input, L2-staff  |
| ⬜  | 6  | Doc consistency check   | —                            |
| ⬜  | 7  | Verification cascade    | —                            |
| ⬜  | 8  | ADR-019 deferred 4-9    | —                            |
| ⬜  | 9  | Ratify ADR-017          | —                            |
| ⬜  | 10 | `L1-world/01-nation`    | —                            |

**Dettagli (solo dove servono):**

**1. ADR-005 / IDEA-PRES-001** — re-read ADR-005, decide: revise (new ADR-022) or reject IDEA-PRES-001. Until resolved, IDEA-PRES-001 cannot become code. IDEA-INFRA-004 deferred (requires ADR-012 runtime, not implemented).

**2. Fix ADR-003** — ADR-003 still names `pcfm-debug` and `demo.rs`; `pcfm-demo` reads Plane A without authorization. Options: new ADR-022 (ADR-021 is taken), amend ADR-003, extend ADR-020. Requires explicit confirmation to touch `docs/adr/`.

**3. Terminologia** — T1 (small): `PLUGIN_DEVELOPER_GUIDE.md` §1 + `GLOSSARY.md`. T2 (small): "core module" → "framework crate" in §2 and §11. T3 (large, one commit): rename `modules/` → `specs/`.

**4. Audit 2026-10-29** — T1: sanitize `--html` in `pcfm-demo/src/render.rs:159`. T2: align `CHANGELOG-SIM.md` lines 19/41 (`pcfm-core` → `pcfm-match`). T3 (note, not a task): `pick_players` correlation in `state.rs:222-226`. Fourth finding (`pcfm-debug`) is item 2.

**5. Review sketch 2026-10-29** — IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001. Decide: ADR, task, or stays sketch. Depends on `L2-tactics`, runtime input channel, `L2-staff` (none exist).

**Done (storico):** 1, 2, 3, 6, 10bis, 10ter, 10quater, 0ter — commit hash in `git log --oneline`.

**Files touched this session (2026-10-29):** `docs/context/IDEAS.md` (added IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001); `docs/context/SESSION-STATE.md`; `docs/adr/021-referee-non-physical.md` (new, accepted); `docs/adr/019-match-engine-contract.md` (Superseded by ADR-021 D9 only); `STRUCTURE.md` (ADR status table, added 020/021); `modules/_index.md` (ADR status table, added 020/021); `docs/context/analysis/2026-10-29-18-18-review.md` (renamed from audit-full); `docs/context/AGENT-POLICY.md` (audit-report push exception, §1 and §8); `docs/context/HERMES-AGENT-PROMPT.md` (new, audit agent contract); `docs/context/NEW-SESSION-PROMPT.md` (refreshed counts, startup + closing rituals, rule 13).
