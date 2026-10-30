# Session state — PCFM

**Purpose:** a short bookmark for resuming work across sessions. It records where we are, not what the project is. Update at the end of a session, not at the start.

**Last commit verified:** `557e1d4` — "docs(session-state): add Open items section, migrate OI-1..OI-6, update Last commit verified"

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
| ✅    | Step 3 T3: rename `modules/` → `specs/` (13 rename + 62 file con riferimenti aggiornati, commit `5e77d17`) |
| ✅    | Doc-pass `NEW-SESSION-PROMPT.md`: 260→256 righe, B2 aggiornata + B3 "OUTPUT START/END", sezione E (session lifecycle), accorpamento First step, rimozione Performance target, dedup "Where we are" (6 commit, `00bd5ba`..`c86ab8b`) |
| ✅    | Unificazione agent prompt: `GITHUB-SESSION-PROMPT.md` agent-agnostic, rimossi `GITHUB-AGENT-PROMPT.md` e `HERMES-AGENT-PROMPT.md` (`0a20a9d`, `0b941b9`) |
| ✅    | Policy "functional directory" per `docs/context/` in `README.md` + `analysis/archive/` creata (`a2c02c1`, `fe31d5c`) |
| ✅    | C2-bis "beyond baseline" + closing step invertiti (report dopo push) + startup step 3 legge tutti i report (`5365cef`) |
| ✅    | D1 esteso (completeness over speed), B1 senza limiti 120/80 (testato: 195 righe integre) (`45bc60d`, `ac2d39a`) |
| ✅    | Report 2026-10-28/29/30 archiviati; finding in step 6 + `IDEA-DOC-001`; policy di chiusura raffinata (`b0c3b9b`..`64e6713`) |
| ✅    | 🛑 marker su hard-stop (D1, D2, C2) + legenda; E2 riscritta (closing mai automatico, pausa + domanda esplicita); A2 e header closing ripuliti; Rule precedence ridotta a una riga (`5101f9d`) |
| ✅    | Baseline C2 eseguita dopo l audit di Copilot (che l aveva bloccata): build, clippy, test 103/103, fmt, test-runner 6/6 — tutta verde |
| ✅    | Audit prompt 2026-10-30 valutato: test plan rispettato (5 pianificati, 5 skipped con motivazione), evidenza negativa presente, R-01 Critical trovato e verificato — il prompt funziona |
| ✅    | Fase A cleanup repo: `REPO-SNAPSHOT.md` rimosso, 15 riferimenti vivi corretti, `PROJECT-IDEA-LOG.md` §6 tagliato + §5 disclaimer (`e22d9f3`) |
| ✅    | ADR-003: nota rename `pcfm-debug` → `pcfm-test-runner` + split `pcfm-demo`, corpo ADR intatto (`a391b85`) |
| ✅    | R-01 verificato end-to-end: `Factor`→`cause_trace`→`chain_hash`→save. Decisione: rimuovere `CauseTrace`+`Factor` (1b), `risk_modifier`, `temperament`/`injury_risk_modifier` di `DuelOutcome`; `SIM_VERSION` bump |
| ✅    | Fase B cleanup completata: CHANGELOG-SIM (19/41/47/107-109), README (9/156), STRUCTURE §5, L0-match conforms_to v2.0 + stale refs, IDEA-PM-005 (D3 proposal), SESSION-STATE HERMES, NEW-SESSION-PROMPT REPO-SNAPSHOT (verified: none) (`18232f9`..`09e6cf4`) |
| ✅    | D3 Bee doctrine in `NEW-SESSION-PROMPT.md` §D (struttura solo se contenuto concreto oggi; altrimenti `IDEAS.md`, non ADR/tipo/livello) (`6a123d8`) |
| ✅    | Discussioni rinviate per dottrina ape: tassonomia valori (diretto/inferito/costrutto), Plane C, livello interpretazione (già in IDEA-PRES-001+INFRA-006); niente ADR finché manca contenuto |
| ✅    | Step 14 espanso: decisioni R-01, opzione C resolver, forma test anti-leak, fuori scope (`73f7ad1`) |
| ✅    | Step 16 tracciato: censimento repo (file orfani, cartelle vuote, D3) (`da0c1d7`) |
| ✅    | E3 in `NEW-SESSION-PROMPT.md` §E: dopo il closing il default è "traccia, non esegui"; 3 eccezioni (bookmark incoerente, C2 rosso, richiesta esplicita) (`2d6339b`) |
| ⚫    | — (nessun bloccato) |

**Open items (what is not decided or verified yet):**

| # | Cosa | Perché aperto | Prima azione |
|---|------|---------------|--------------|
| OI-1 | **R-01: causa rimossa (1b) o osservata?** La decisione salvata (rimuovere `CauseTrace`+`Factor`) poggia su premesse incomplete: 7 leak (non 5), `primary_cause` ha 3 consumer reali, `05-match-events.md:117` richiede la cause trace. La tua posizione: "la dottrina è migliorabile". | Decisione di design sospesa: 1b vs causa osservata (motore, non resolver) vs rinvio | Leggere `05-match-events.md:64-68,117-160` e `10-ground-truth.md:96-111` per intero, poi scrivere ADR-023 come decisione (non come fix) |
| OI-2 | **Nuovo audit `2026-10-30-20-24-review.md`** non tracciato: R-01..R-11 (R-07 `CODE_TOUR:128` "5 references" nuovo; gli altri R-04..R-11 già in step 2/4/6/14/15 o step 6) | Triagare: quali finding sono già coperti, quali nuovi step | Leggere i finding e mappare su step esistenti |
| OI-4 | **`README.md:148` cita `pcfm-gen`** come esistente, ma non è tra i 9 crate | Verificare se è nota intenzionale o stale | `grep -rn "pcfm-gen" --include="*.md" --include="*.rs" .` |
| OI-5 | **`CHANGELOG-SIM:17-19,41` "(planned)" stale**: "No code exists yet", "defined in pcfm-match (planned)" | Deferito: si aggiorna al primo bump reale (step 14) | Aggiornare insieme al `SIM_VERSION` bump |
| OI-6 | **`docs/performance.md` e `docs/save-security.md`** mancanti (dal §2.3 di `PROJECT-IDEA-LOG`, ora archiviato) | Mai creati, non tracciati altrove | Decidere se e quando crearli (probabilmente pre-release, insieme a step 13) |
| OI-7 | **Gap B: `ProjectionProfile` senza metodo di calcolo** (dal §2.2 di `PROJECT-IDEA-LOG`): bootstrap? Bayesiano? quantili empirici? Senza specifica, la banda è arbitraria | Mai tracciato altrove; con (C) si perderebbe | Decidere il metodo quando si apre `L2-scouting` |
| OI-8 | **Gap C: `ScoutReport.DeclaredBias` senza formula** (dal §2.2): è solo un'etichetta o entra nel calcolo? | Mai tracciato altrove; con (C) si perderebbe | Decidere quando si apre `L2-scouting` |

**Da fare (in dependency order):**

| Pr. | #  | Cosa                    | Dipende da                   |
|-----|----|-------------------------|------------------------------|
| 🔴  | 14 | **R-01 Critical**: latenti Plane A in `Factor.value` → eventi Plane B → save (ADR-023 richiesto) | — |
| 🟡  | 12 | TUI demo (IDEA-INFRA-006): ADR breve + codice | — |
| 🟡  | 2  | Fix ADR-003 (nota rename fatta `a391b85`; restano `demo.rs`, autorizzazione `pcfm-demo`) | —                            |
| 🟡  | 3  | Terminologia (T1+T2, T3 done) | —                        |
| 🟡  | 4  | Audit 2026-10-29 (T1,T2)| —                            |
| 🟡  | 11 | GitHub agent Phase 2    | ruleset attivo               |
| ⬜  | 5  | Review sketch ×3        | L2-tactics, input, L2-staff  |
| ⬜  | 6  | Doc consistency check   | —                            |
| ⬜  | 7  | Verification cascade    | —                            |
| ⬜  | 8  | ADR-019 deferred 4-9    | —                            |
| ⬜  | 9  | Ratify ADR-017          | —                            |
| ⬜  | 10 | `L1-world/01-nation`    | —                            |
| ⬜  | 13 | `cargo-audit`/`cargo-deny` pre-release | — |
| 🟡  | 16 | **Censimento repo (igiene documentale, D3)**: verificare file orfani e cartelle vuote fuori da `docs/context/`. Sospetti: `docs/PLUGINS-WORLDS-READING-GUIDE.md` (mai citato?), `PROJECT-STATUS.md` root (README lo dice "planned"), `archive/` `data/` `mods/` `registry/` `docs/calibration/` `docs/legal/` (solo `.gitkeep`?). Nessun rischio sicurezza: urgenza 🟡, ma da fare prima che qualcuno ci costruisca sopra | — |

**Dettagli (solo dove servono):**

**12. TUI demo (IDEA-INFRA-006)** — short ADR required (second renderer touches `simulation_is_shared_and_renderer_agnostic`), then code in `pcfm-demo`. **First step: read `crates/pcfm-demo/src/` and `MatchResult` (~200-300 lines) to know what the engine actually exposes; only then estimate.** The ADR says: validation prototype, not L3-ui, no engine change.

**2. Fix ADR-003** — ADR-003 still names `pcfm-debug` and `demo.rs`; `pcfm-demo` reads Plane A without authorization. Options: new ADR-023 (ADR-021 and ADR-022 are taken), amend ADR-003, extend ADR-020. Requires explicit confirmation to touch `docs/adr/`.

**3. Terminologia** — T1 (small): `PLUGIN_DEVELOPER_GUIDE.md` §1 + `GLOSSARY.md`. T2 (small): "core module" → "framework crate" in §2 and §11. T3 (large, one commit): rename `modules/` → `specs/` — **✅ done, commit `5e77d17`**.

**4. Audit 2026-10-29** — T1: sanitize `--html` in `pcfm-demo/src/render.rs:159`. T2: align `CHANGELOG-SIM.md` lines 19/41 (`pcfm-core` → `pcfm-match`). T3 (note, not a task): `pick_players` correlation in `state.rs:222-226`. Fourth finding (`pcfm-debug`) is item 2.


**14. R-01 Critical — Plane A leak** (audit 2026-10-30-13-39, verificato sui file:line). Latenti `finishing`/`passing`/`injury_proneness`/`attacker_score` scritti in `Factor.value` (`pcfm-hidden/src/lib.rs:140-145,182,213,262`), `Factor` è `Serialize` (`pcfm-model/src/cause.rs:11-16`), propagato in `MatchEvent.cause_trace` (`pcfm-stats/src/event.rs:109`), emesso (`pcfm-match/src/state.rs:118-120,141-143`), serializzato nel save (`save.rs:131`). Viola `00-foundations` §3/§5, `05-match-events.md:21-24,117`, `10-ground-truth.md:96-111`. **Serve ADR-023** (politica delle cause senza disclosure) prima di qualsiasi fix. Test mancanti: nessun test serializza un save e verifica l assenza di latenti. **DoD:** ADR-023 accepted + fix implementata + **test anti-leak** (serializza un save, verifica che nessun valore di `PlayerLatents` compaia nel JSON) + baseline C2 verde.
   - **Decisioni prese (2026-10-30), da riflettere nell'ADR:** rimuovere `CauseTrace` e `Factor` del tutto (opzione 1b, non `Option`), perché `narrative.rs` non li legge, `primary_cause` duplica `EventType`, `explanation_template` non ha consumatori; rimuovere `InjuryOutcome.risk_modifier` (funzione lineare esatta di `injury_proneness`, zero consumer); rimuovere `DuelOutcome.injury_risk_modifier` e `temperament_modifier` (costanti `1.0`, zero consumer). `SIM_VERSION` bump necessario (il payload evento entra in `chain_hash`, `chain.rs:48-58`); `SCHEMA_VERSION` no (serde ignora campi sconosciuti).
   - **Prerequisito tecnico per il resolver (opzione C, preferita):** `Player` ha già i dati pubblici (`height_cm`, `preferred_foot`, `physical`, `psychological`); `Squad` espone `Vec<Player>`; `Match` espone `home`/`away`. I `Context` in `pcfm-hidden/src/context.rs` contengono solo `PlayerId`. L'opzione C = `CauseDescriptor` tipizzato (enum con solo dati pubblici), costruito dal resolver e convertito in `CauseTrace` (o sostituto) dal motore. Opzioni scartate: A (pubblici nel `Context`), B (resolver senza causa).
   - **Forma corretta del test anti-leak:** non "cerca i valori di `PlayerLatents`" (tipo privato, irraggiungibile dal test). Deve essere strutturale: (1) nessun `value` in `contributing_factors`; (2) nessun latente in `MatchEvent.fields`; (3) nessun campo di `Outcome` è funzione esatta di un latente.
   - **Fuori scope ADR-023:** tassonomia dei valori (diretto/inferito/costrutto), Plane C, livello di interpretazione. Sono rinviati per D3 (bee doctrine) finché manca contenuto concreto.

**Altri finding audit 2026-10-30** (dopo R-01, da verificare e triagare): R-02 attempt/outcome assenti (già noto); R-03 save contract (già noto); R-04 doc drift (`STRUCTURE.md` 17 ADR vs 20 file, `CHANGELOG-SIM.md` dice `pcfm-core`) → confluisce in step 6; R-05 stream senza `match_id` (`state.rs:57-63`); R-06 `pick_players` (già T3 noto); R-07 panic handler non isolati (`event_bus.rs:81-85,126-140`); R-08 `expect` in simulazione (`state.rs:57-63`, `hidden/lib.rs:68-71`); R-09 `MatchContext` senza bus/modifiers (già noto); R-10 `GroundTruth` getter/Debug (`hidden/lib.rs:84-91,351-356`).
**5. Review sketch 2026-10-29** — IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001. Decide: ADR, task, or stays sketch. Depends on `L2-tactics`, runtime input channel, `L2-staff` (none exist).

**6. Doc consistency check** — finding dai report 2026-10-28 (da chiudere):
- **F1** `crates/pcfm-test-runner/Cargo.toml:10`: `documentation = ".../docs/MODULE_DEVELOPER_GUIDE.md"` → `.../docs/PLUGIN_DEVELOPER_GUIDE.md` (file inesistente, verificato).
- **F2** `README.md:9` badge tests `107` → `103` (verificato: 103 reali); `README.md:156` ADR range `012–020` → `012–022`.
- **F6** Aggiungere in `README.md` una "maturity statement" (doctrine / implementation / ADR / test status), come chiesto dal report 09-42.
- **F3/F5/F7** narrativi: F7 → coperto da D1 esteso; F5 → "Do not assume CI exists"; F3 → backlog in `IDEAS.md` (IDEA-DOC-001).

**Prime azioni (per step):**

| #  | Prima azione |
|----|--------------|
| 14 | Scrivere ADR-023 (politica cause senza disclosure); prima leggere `pcfm-hidden/src/lib.rs:130-270`, `pcfm-model/src/cause.rs`, `05-match-events.md:21-24` |
| 2  | Leggere ADR-003 §Exception: `sed -n '204,235p' docs/adr/003-plane-a-enforcement.md` |
| 3  | Leggere `PLUGIN_DEVELOPER_GUIDE.md` §1: `sed -n '13,40p'` |
| 4  | Leggere `render.rs:150-170` e `CHANGELOG-SIM.md:19,41` |
| 5  | Leggere i tre sketch in `IDEAS.md` (TACT-001, INFRA-007, STAFF-001) |
| 6  | Leggere `crates/pcfm-test-runner/Cargo.toml:10`, `README.md:9,156` (F1, F2) e decidere dove inserire la maturity statement (F6) |
| 7  | Decidere da quale file parte la cascade (candidato: `README.md`) |
| 8  | Rileggere ADR-019 §Deferred, decidere quali dei 4-9 chiudere (3 già resolved) |
| 9  | Leggere ADR-017, decidere: ratificare o rinviare |
| 10 | Scrivere `L1.04` (`L1-world/00-overview.md`) prima di `L1.05` |
| 11 | Decisione del maintainer: attivare o no il ruleset |
| 12 | Leggere `crates/pcfm-demo/src/` e `MatchResult` |
| 13 | Installare `cargo-audit` e `cargo-deny` (o verificarne la disponibilità) e lanciarli sul workspace |
| 16 | `ls -R archive data mods registry docs/calibration docs/legal 2>&1; git ls-files | wc -l; grep -rn "PLUGINS-WORLDS-READING-GUIDE\|PROJECT-STATUS" --include="*.md" . | head -20` — poi classificare vivo/orfano/alveare e proporre tagli |

**Done (storico):** 1, 2, 6, 10bis, 10ter, 10quater, 0ter, 15; OI-3 (C: PROJECT-IDEA-LOG archived) — commit hash in `git log --oneline`.

**Archiviati:** `PROJECT-IDEA-LOG.md` → `archive/PROJECT-IDEA-LOG.md` (2026-10-30). Le decisioni durevoli sono in ADR; le minori nei commit message; le narrative storiche nel file archiviato. §2.2 (gap B, C) e §2.3 (2 documenti) sono migrati in OI-7, OI-8, OI-6.

**Files touched this session (2026-10-29):** `docs/context/IDEAS.md` (added IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001; rejected IDEA-REJ-003, IDEA-REJ-004); `docs/context/SESSION-STATE.md`; `docs/adr/021-referee-non-physical.md` (new, accepted); `docs/adr/019-match-engine-contract.md` (Superseded by ADR-021 D9 only); `STRUCTURE.md` (ADR status table, added 020/021); `specs/_index.md` (ADR status table, added 020/021); `docs/context/analysis/2026-10-29-18-18-review.md` (renamed from audit-full); `docs/context/AGENT-POLICY.md` (audit-report push exception, §1 and §8); `docs/context/HERMES-AGENT-PROMPT.md` (new, audit agent contract); `docs/context/NEW-SESSION-PROMPT.md` (refreshed counts, startup + closing rituals, rule 13; closing ritual now requires presenting the Plan table in chat); `docs/adr/022-presentation-tui-first.md` (new, accepted); `docs/adr/005-qt-binding.md` (Amended by ADR-022); `STRUCTURE.md` + `specs/_index.md` (ADR 022 added, count 18). Audit workflow 2026-10-30: `GITHUB-SESSION-PROMPT.md` wrapper paste-ready + test plan + 3b ambiguities + comprehensive report + evidenza negativa (`55a932a`, `5fbaf34`, `c8f3291`); audit Copilot pubblicato (`943b317`, `2026-10-30-13-39-review.md`); R-01 Critical verificato e tracciato come step 14 🔴 (`9c937a0`); baseline C2 verde post-audit. B3 fix (`2178270`): literal echo template, sentinel was ambiguous in new sessions.
 🛑 hard-stop marker + E2 rewritten + Rule precedence trimmed (`5101f9d`).
**Files touched this session (2026-10-30, seconda sessione):** `docs/context/REPO-SNAPSHOT.md` (deleted); `docs/context/NEW-SESSION-PROMPT.md`, `docs/context/README.md`, `docs/ARCHITECTURE.md`, `docs/CODE_TOUR.md`, `docs/context/IDEAS.md`, `docs/context/PROJECT-IDEA-LOG.md`, `docs/context/SESSION-STATE.md` (reference cleanup + trim, `e22d9f3`); `docs/adr/003-plane-a-enforcement.md` (note rename, `a391b85`). `docs/context/SESSION-STATE.md` (step 14 detail `73f7ad1`, step 16 tracking `da0c1d7`). `docs/context/NEW-SESSION-PROMPT.md` (E3, `2d6339b`).
**Files touched this session (2026-10-30):** `docs/context/NEW-SESSION-PROMPT.md` (removed duplicate B1 bullet); `docs/context/SESSION-STATE.md` (step 13 cargo-audit; step 3 T3 done); `STRUCTURE.md` (Italian example → English); **rename `modules/` → `specs/`** with 62 files updated: `.github/*`, `crates/*` (Cargo.toml + src), `docs/*.md`, `docs/adr/*.md`, `docs/context/*` (except `analysis/` and `PROJECT-IDEA-LOG.md` body), `archive/README.md` (live refs only), `CHANGELOG-REGISTRY.md:19`, `specs/*` self-references; note in `PROJECT-IDEA-LOG.md` header; historical commands preserved in `archive/README.md:35-36` and `docs/MIGRATION-01-04.md:45`. Doc-pass on `NEW-SESSION-PROMPT.md` (6 commits `00bd5ba`..`c86ab8b`): B3 renamed to OUTPUT START/END, section E session lifecycle, First step merged into What remains, perf target removed, dedup Where we are. Agent prompt unification (`0a20a9d`..`fe31d5c`): new `GITHUB-SESSION-PROMPT.md`, removed `GITHUB-AGENT-PROMPT.md` + `HERMES-AGENT-PROMPT.md`, functional-directory policy in `README.md`, `analysis/archive/` created, `AGENT-POLICY.md` §8 and startup step 3 updated. D1 extended with completeness-over-speed (`45bc60d`); B1 paste limits removed (`ac2d39a`); 2026-10-28 audit findings tracked in step 6 + IDEA-DOC-001 (`b0c3b9b`); 6 reports archived to `analysis/archive/` (`cd882dc`); report-closure policy refined (`1c50db1`); audit prompt points to archived example (`64e6713`). 🛑 hard-stop marker + E2 rewritten + Rule precedence trimmed (`5101f9d`).
