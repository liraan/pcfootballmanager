# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is (see `REPO-SNAPSHOT.md` for that). Update at the end of a session, not at the start.

**Last commit verified:** `37391db` — "docs(session-state): remove done detail, add missing done rows to status table"

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
| ✅    | Tabella Piano in `SESSION-STATE` + nota di formato |
| ✅    | Doc-pass ADR-022 completo: README (2 righe), STRUCTURE (ADR-005 amended), ARCHITECTURE (TUI primary). STATUS e 00-foundations già neutri |
| ✅    | ADR-022: TUI-first, GUI later (amends ADR-005, view model separation normativo) |
| ✅    | Closing ritual: obbligo di presentare la tabella Piano in chat |
| ✅    | IDEA-REJ-003 (ritratti TUI), IDEA-REJ-004 (nomi reali) |
| ✅    | Closing ritual step 4: ogni step deve avere la prima azione concreta |
| ✅    | Tabella "Prime azioni" per tutti gli step aperti |
| ⚫    | — (nessun bloccato) |

**Da fare (in dependency order):**

| Pr. | #  | Cosa                    | Dipende da                   |
|-----|----|-------------------------|------------------------------|
| 🟡  | 12 | TUI demo (IDEA-INFRA-006): ADR breve + codice | — |
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
| ⬜  | 13 | `cargo-audit`/`cargo-deny` pre-release | — |

**Dettagli (solo dove servono):**

**12. TUI demo (IDEA-INFRA-006)** — short ADR required (second renderer touches `simulation_is_shared_and_renderer_agnostic`), then code in `pcfm-demo`. **First step: read `crates/pcfm-demo/src/` and `MatchResult` (~200-300 lines) to know what the engine actually exposes; only then estimate.** The ADR says: validation prototype, not L3-ui, no engine change.

**2. Fix ADR-003** — ADR-003 still names `pcfm-debug` and `demo.rs`; `pcfm-demo` reads Plane A without authorization. Options: new ADR-023 (ADR-021 and ADR-022 are taken), amend ADR-003, extend ADR-020. Requires explicit confirmation to touch `docs/adr/`.

**3. Terminologia** — T1 (small): `PLUGIN_DEVELOPER_GUIDE.md` §1 + `GLOSSARY.md`. T2 (small): "core module" → "framework crate" in §2 and §11. T3 (large, one commit): rename `modules/` → `specs/`.

**4. Audit 2026-10-29** — T1: sanitize `--html` in `pcfm-demo/src/render.rs:159`. T2: align `CHANGELOG-SIM.md` lines 19/41 (`pcfm-core` → `pcfm-match`). T3 (note, not a task): `pick_players` correlation in `state.rs:222-226`. Fourth finding (`pcfm-debug`) is item 2.

**5. Review sketch 2026-10-29** — IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001. Decide: ADR, task, or stays sketch. Depends on `L2-tactics`, runtime input channel, `L2-staff` (none exist).

**Prime azioni (per step):**

| #  | Prima azione |
|----|--------------|
| 2  | Leggere ADR-003 §Exception: `sed -n '204,235p' docs/adr/003-plane-a-enforcement.md` |
| 3  | Leggere `PLUGIN_DEVELOPER_GUIDE.md` §1: `sed -n '13,40p'` |
| 4  | Leggere `render.rs:150-170` e `CHANGELOG-SIM.md:19,41` |
| 5  | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| 6  | Da definire in sessione: il riferimento "grep + classification" non è in nessun file |
| 7  | Decidere da quale file parte la cascade (candidato: `README.md`) |
| 8  | Rileggere ADR-019 §Deferred, decidere quali dei 4-9 chiudere (3 già resolved) |
| 9  | Leggere ADR-017, decidere: ratificare o rinviare |
| 10 | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| 11 | Decisione del maintainer: attivare o no il ruleset |
| 12 | Leggere `crates/pcfm-demo/src/` e `MatchResult` |
| 13 | Installare `cargo-audit` e `cargo-deny` (o verificarne la disponibilità) e lanciarli sul workspace |

**Done (storico):** 1, 2, 3, 6, 10bis, 10ter, 10quater, 0ter — commit hash in `git log --oneline`.

**Files touched this session (2026-10-29):** `docs/context/IDEAS.md` (added IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001; rejected IDEA-REJ-003, IDEA-REJ-004); `docs/context/SESSION-STATE.md`; `docs/adr/021-referee-non-physical.md` (new, accepted); `docs/adr/019-match-engine-contract.md` (Superseded by ADR-021 D9 only); `STRUCTURE.md` (ADR status table, added 020/021); `modules/_index.md` (ADR status table, added 020/021); `docs/context/analysis/2026-10-29-18-18-review.md` (renamed from audit-full); `docs/context/AGENT-POLICY.md` (audit-report push exception, §1 and §8); `docs/context/HERMES-AGENT-PROMPT.md` (new, audit agent contract); `docs/context/NEW-SESSION-PROMPT.md` (refreshed counts, startup + closing rituals, rule 13; closing ritual now requires presenting the Plan table in chat); `docs/adr/022-presentation-tui-first.md` (new, accepted); `docs/adr/005-qt-binding.md` (Amended by ADR-022); `STRUCTURE.md` + `modules/_index.md` (ADR 022 added, count 18).
