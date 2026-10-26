# Project Idea Log — PCFM

> **Aggiornamento 2026-10-26:** ADR-012, ADR-013 e ADR-015 sono state ratificate. Vedi `docs/REPO-SNAPSHOT.md` per lo stato attuale. Questo log mantiene le voci originali per tracciabilità.

**Scopo:** registrare le idee, le decisioni e le domande aperte del progetto,
con la fonte di ciascuna. Non è un documento di stato del codice (per quello
vedi `REPO-SNAPSHOT.md`). Non è una specifica (per quella vedi `modules/`).

**Regola di questo documento:** ogni affermazione ha una fonte. Se una
decisione è stata presa, c'è il commit o il messaggio che la contiene. Se
un'idea è solo abbozzata, è marcata come "idea". Se non sappiamo qualcosa,
scriviamo "non so" invece di inventare.

**Fonti:**
- **D** = conversazione DeepSeek (`chat.deepseek.com/share/ubaqxjcf8loa3jvy9s`)
- **C** = questa conversazione (GitHub web + Git Bash, dal 2026-10-24 al 2026-10-26)
- **R** = repository (`git log`, `git ls-files`, file letti)

---

## 1. Origine del progetto

Il progetto nasce dalla volontà di costruire un'alternativa community ai
gestionali calcistici commerciali, con tre principi non negoziabili
(enunciati in `README.md` e `modules/00-foundations.md`):

1. **Trasparenza** — ogni formula, ogni regola, ogni fonte dati è pubblica.
2. **Privacy** — niente telemetria, niente account, niente rete.
3. **Profondità tattica** — un motore che premia la comprensione, non i
   click.

La dottrina dei **Tre Piani** (Plane A = Ground Truth, Plane B = Observation,
Plane C = Estimation) è il differenziatore architetturale: scouting non è
lookup, è *stima di un'osservazione della verità*. (Fonte: `README.md`,
`modules/00-foundations.md` §3; C)

---

## 2. Le raccomandazioni della chat DeepSeek (fonte D)

La conversazione DeepSeek si chiude con **5 raccomandazioni** e **3 buchi
tecnici** e **2 documenti mancanti**. Le riporto perché sono la base delle
decisioni successive.

### 2.1 Raccomandazioni

| # | Raccomandazione | Stato al 2026-10-26 |
|---|-----------------|---------------------|
| 1 | Ridurre lo scope v1.0: 20 ruoli (non 56), 10 formazioni (non 20), 5 nazioni (non 20), 3 assi psicologici attivi (non 6) | ⏳ formalizzata in ADR-015 (`accepted`, 2026-10-26) |
| 2 | Scrivere codice prima di completare i 25 moduli | ✅ fatto: L0 implementato, 107 test |
| 3 | Ratificare ADR-003 e ADR-005 prima di scrivere codice che li tocca | ✅ fatto: ADR-003 accepted (con eccezione `pcfm-debug`), ADR-005 accepted |
| 4 | Allineare le versioni dei documenti e unificare la lingua | ✅ fatto: ADR-007 accepted, `00-foundations.md` v2.0, documentazione in inglese |
| 5 | I 3 buchi tecnici e i 2 documenti mancanti | ⏳ aperti (vedi §4) |

### 2.2 I tre buchi tecnici (fonte D)

| # | Buco | Cosa manca | Stato |
|---|------|-----------|-------|
| A | Psicologia a 6 assi | La dottrina dice 3 assi in v1.0, ma il modello dati ne ha 6. Serve esplicitare il flag `active: false` sui 3 assi non v1.0. | ⏳ aperto, tracciato in ADR-015 (§Deferred decisions) |
| B | `ProjectionProfile` | La "banda attesa" non ha metodo di calcolo (bootstrap? bayesiano? quantili empirici?). Senza specifica, la banda è arbitraria. | ⏳ aperto, non ancora indirizzato |
| C | `ScoutReport.DeclaredBias` | Il bias dichiarato non ha formula di applicazione. È solo un'etichetta o entra nel calcolo? | ⏳ aperto, non ancora indirizzato |

### 2.3 I due documenti mancanti (fonte D)

| Documento | Cosa dovrebbe contenere | Stato |
|-----------|-------------------------|-------|
| `docs/performance.md` | Benchmark attesi (stagione < 60 s su hardware medio), piano di misura, costo computazionale per tick | ❌ non esiste |
| `docs/save-security.md` (o sezione in `L0-match/07-save-minimal.md`) | Checksum, firma, backup automatico, rollback per salvataggi corrotti | ❌ non esiste |

---

## 3. Decisioni prese in questa conversazione (fonte C + R)

In ordine cronologico. Ogni decisione ha il commit che l'ha registrata (quando disponibile).

### 3.1 Licenza (ADR-006)

- **Decisione:** dual license. Codice `MIT OR Apache-2.0`, asset `CC-BY-NC-SA-4.0`.
- **File creati:** `LICENSE-MIT`, `LICENSE-APACHE`, `LICENSE-ASSETS`, `LICENSE.md`.
- **ADR-006:** da `proposed` a `accepted` (2026-10-25).
- **Commit:** `99c8946` (LICENSE-ASSETS), `8871ab4` (LICENSE.md), più i commit di ratifica ADR-006.

### 3.2 Qt binding (ADR-005)

- **Decisione:** `cxx-qt` (opzione A). Solo binding che copre sia QWidgets (data grid) sia QML (skin).
- **ADR-005:** da `proposed` a `accepted` (2026-10-25).
- **Commit:** ratifica insieme al batch ADR.

### 3.3 Lingua canonica (ADR-007)

- **Decisione:** inglese canonico per codice, commit, ADR, documentazione tecnica. Italiano come traduzione in `locales/it/` (pianificato).
- **ADR-007:** da `proposed` a `accepted` (2026-10-25).
- **Commit:** ratifica insieme al batch ADR.

### 3.4 Dottrina (00-foundations.md)

- **Decisione:** rimuovere le note "under review" su ADR-013. La dottrina era v1.4, pillar 1 (determinismo globale) stabile. ADR-013 era citato come `proposed, not ratified` (poi ratificato il 2026-10-26, dottrina aggiornata a **v2.0** con pillar 1 riscritto a determinismo locale).
- **File:** `modules/00-foundations.md` v2.0, `status: "ratified"`, `last_updated: 2026-10-26`.
- **Commit:** `d6c3c8b` + `fd2c298` (v2.0).

### 3.5 ADR-003 (Plane A enforcement)

- **Decisione:** documentare l'eccezione `pcfm-debug`. Il crate può dipendere da `pcfm-hidden` solo per costruire `GroundTruth` da passare a `pcfm-match`, mai per leggere latenti. L'eccezione è confinata a `demo.rs` e `check.rs`.
- **File:** `docs/adr/003-plane-a-enforcement.md`, sezione "Exception: pcfm-debug". Corpo completo ricomposto da `571d48f` + `70ee57f`.
- **Commit:** `b309f23` (ricomposizione completa).

### 3.6 Scope v1.0 (ADR-015)

- **Decisione:** formalizzare la riduzione di scope (20 ruoli, 10 formazioni, 5 nazioni, 3 assi psicologici attivi, subset L2/L3).
- **File:** `docs/adr/015-v1-scope.md` (rinominato da `014-v1-scope.md` perché 014 era occupato da `014-mobile-strategy.md`).
- **Status:** `accepted` (2026-10-26). **Dipendeva da ADR-012 e ADR-013**, entrambi ratificati il 2026-10-25.
- **Commit:** `1a59b81` (rinomina), `8d8d347` (creazione originale), `4d826dc` (ratifica), `4c93214` (propagazione).
- **Nota:** l'ADR contiene 7 **Deferred decisions** (quali 3 assi? quali 5 nazioni? ecc.), da risolvere in ADR-016 ("v1.0 content selections").

### 3.7 Framework di coretest (`pcfm-testkit`)

- **Decisione:** creare un crate pubblico `pcfm-testkit` che definisce il contratto `Scenario` / `Feature` / `TestRegistry`, **senza dipendenze da altri crate `pcfm-*`**. I plugin possono implementare `Scenario` senza dipendere da `pcfm-debug`.
- **File creati:** `crates/pcfm-testkit/` (7 file), `crates/pcfm-debug/src/features/` (2 file), `crates/pcfm-debug/src/commands.rs`.
- **File modificati:** `args.rs` (aggiunti `--list`, `--coverage`), `main.rs` (dispatch), `check.rs` (thin wrapper sul registry).
- **Commit:** serie `928f734`, `aff7a13`, `ff13f2d`, `5bdcb78` e successivi.
- **Stato:** funzionante. `--check` 6/6, `--list` 6 scenari, `--coverage` 2/26 (7%).

### 3.8 Documentazione aggiornata (2026-10-26)

- **README.md:** riformattato per fruibilità, aggiunti `--list` / `--coverage`, aggiornato stato L0 ("ratified + implemented + tested"), FAQ aggiornata ("All decisions are now accepted").
- **STRUCTURE.md:** v1.7 → v1.8. 7 crate, 15 ADR, ADR-005/006/007 accepted, ADR-014/015 aggiunti.
- **PLUGIN_DEVELOPER_GUIDE.md:** v1.0 → v1.1. Aggiunta §11 "Contributing test scenarios", correzione `fmr-pack` → `pcfm-pack`, tabella crate aggiornata.
- **`_index.md`:** v2.8 → v2.9. ADR-012/013/014 accepted, doctrine v2.0, 7 crate / 107 test, 12 accepted ADR.
- **`CONTRIBUTING.md`:** stato aggiornato (7 crate, 107 test, 12 accepted, 0 proposed).
- **Commit:** `7c98612`, `8c67ec8`, `be929c4`, `635bc61`, `f1dc5fe`, `4c93214`.

### 3.9 Documentazione di architettura e code tour (2026-10-26)

- **Decisione:** aggiungere due documenti che mancavano: `ARCHITECTURE.md` (come i crate interagiscono, flusso di un match, simulation/rendering split, testkit contract) e `CODE_TOUR.md` (percorso di lettura in 5 stadi: infrastruttura, entità, piani, motore, testkit). Più 5 commenti mirati nei punti criptici del codice (`action_probability = 0.0005`, split 60/25/12/3 in `MatchState::tick`, `noise * 0.2` in `resolve_duel`, uniformità dei latents, scelta dei 6 scenari L0).
- **File creati:** `docs/ARCHITECTURE.md` (210 righe), `docs/CODE_TOUR.md` (215 righe).
- **File modificati:** `context.rs`, `state.rs`, `hidden/lib.rs`, `latents.rs`, `l0_match.rs` (solo commenti, nessun cambiamento di comportamento).
- **Commit:** `e35d15e` (i 5 file + 2 documenti), `0a329d4` (fix `DoD` in backtick per `clippy::doc_markdown`).
- **Motivazione:** il codebase aveva `///` ovunque, ma mancava una mappa architetturale e un percorso di lettura. L'idea originale ("tanti commenti in tutti i file") è stata scartata perché i commenti che ripetono il codice invecchiano e duplicano la specifica. Sono stati aggiunti solo dove il codice è genuinamente criptico.

---

## 4. Decisioni ratificate (al 2026-10-26)

### 4.1 ADR-012 (Plugins: The Brick Contract)

- **Status:** `accepted` (2026-10-25).
- **Cosa decide:** il contratto dei moduli (plugin): cosa è un modulo, cosa dichiara (`sheet.toml`), come si gestiscono i conflitti.
- **Blocca:** il sistema di plugin (step 11+), e indirettamente ADR-015 (perché "in a pack" non è una risposta sicura finché il contratto dei moduli non esiste).
- **Ratificato:** 2026-10-25.

### 4.2 ADR-013 (Worlds: LOD, Manifest, and Local Determinism)

- **Status:** `accepted` (2026-10-25).
- **Cosa decide:** il sistema dei mondi (come i mattoni si compongono in un mondo), il Level of Detail, il manifest del salvataggio, e — se ratificato — sostituisce il pillar 1 della dottrina (determinismo da globale a locale).
- **Dipende da:** ADR-012.
- **Blocca:** ADR-015, il sistema dei mondi, il determinismo locale.
- **Ratificato:** 2026-10-25.

### 4.3 ADR-015 (v1.0 scope)

- **Status:** `accepted` (2026-10-26).
- **Dipendeva da:** ADR-012 e ADR-013, entrambi ratificati il 2026-10-25.
- **Da fare:** rispondere alle 7 Deferred decisions (quali 3 assi psicologici? quali 20 ruoli? quali 10 formazioni? quali 5 nazioni? quali L2/L3 in v1.0?). Decisione delegata ad **ADR-016** ("v1.0 content selections"), non ancora scritto.

### 4.4 I tre buchi tecnici (fonte D)

- **A — Psicologia 6 assi:** aggiornare il modello dati per rendere esplicito il flag `active: false` sui 3 assi non v1.0. Decisione delegata ad ADR-016.
- **B — `ProjectionProfile`:** scrivere la sezione "Metodo di calcolo delle bande" (bootstrap? bayesiano? quantili empirici?). Non ancora indirizzato.
- **C — `ScoutReport.DeclaredBias`:** specificare la formula di applicazione del bias, o dichiarare esplicitamente che in v1.0 è solo informativo. Non ancora indirizzato.

### 4.5 I due documenti mancanti (fonte D)

- `docs/performance.md`: benchmark attesi, piano di misura, costo computazionale per tick.
- `docs/save-security.md`: checksum, firma, backup, rollback per salvataggi corrotti.

---

## 5. Stato attuale del progetto (fonte R, 2026-10-26)

| Metrica | Valore |
|---------|--------|
| Crate | 7 (`pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-match`, `pcfm-stats`, `pcfm-testkit`, `pcfm-debug`) |
| Test | 107 (tutti verdi) |
| Clippy | pulito (`-D warnings`) |
| `--check` | 6/6 |
| `--list` | 6 scenari |
| `--coverage` | 2/26 feature coperte (7%) |
| ADR | 14 file in `docs/adr/`: 12 accepted (001-008, 012-015), 0 proposed, 1 template (000). Più 1 reading guide in `docs/`. |
| Moduli L0 | 11 celle, ratified + implemented |
| Moduli L1–L3 | non ancora scritti |
| Documenti completi | `PROJECT-IDEA-LOG.md`, `REPO-SNAPSHOT.md`, `ARCHITECTURE.md`, `CODE_TOUR.md` |
| Ultimo commit | `4c93214` |

---

## 6. Cosa NON so (e non voglio inventare)

Tutti i file tracciati sono stati letti (vedi `REPO-SNAPSHOT.md` §7). Resta fuori solo:

1. **Eventuali altre chat o decisioni prese fuori da questa sessione.** Se ce ne sono, non le vedo.

---

## 7. Prossimi passi (in ordine di dipendenza)

1. **Scrivere ADR-016** ("v1.0 content selections"): rispondere alle 7 Deferred decisions di ADR-015 (quali 3 assi psicologici? quali 20 ruoli? quali 10 formazioni? quali 5 nazioni? quali L2/L3 in v1.0?). Richiede di sapere cosa c'è in `modules/L2-*` e `modules/_index.md` (che esistono solo come planned).
2. **Indirizzare i 2 buchi tecnici** (`ProjectionProfile`, `ScoutReport.DeclaredBias`). Il terzo (psyche) è dentro ADR-016.
3. **Scrivere i 2 documenti mancanti** (`performance.md`, `save-security.md`).
4. **Iniziare a scrivere L1** (primo modulo: `L1-world/01-nation.md`). Sblocca i primi scenari L1 nel coretest.
