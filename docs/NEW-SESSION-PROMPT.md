# New Session Prompt — PCFM

**Scopo:** prompt da incollare come primo messaggio di una nuova chat, per riprendere il lavoro sul progetto senza perdere contesto. Aggiornare questo file quando cambia lo stato del progetto o le regole operative.

**Ultimo aggiornamento:** 2026-10-26 (commit `84b4217`)

---

## Come si usa

1. Apri una nuova chat.
2. Copia il contenuto del blocco qui sotto (dal `# Ripresa` all'ultima riga, senza i 4 backtick).
3. Incollalo come primo messaggio.

---

## Prompt

````
# Ripresa progetto PCFM — nuova sessione

Sto riprendendo il lavoro sul progetto **PC Football Manager (PCFM)**. La chat precedente si è chiusa in un punto preciso. Ti passo il contesto per ripartire senza perdere nulla.

## Il progetto

- **Repository:** https://github.com/liraan/pcfootballmanager (pubblico)
- **Cos'è:** un'alternativa community ai gestionali calcistici commerciali. Trasparente, offline-first, deterministico. Dottrina dei Tre Piani (Plane A = Ground Truth, Plane B = Observation, Plane C = Estimation).
- **Lingua:** documentazione tecnica in inglese, conversazione con me in italiano.
- **Stato:** L0 (match engine) completo e implementato. L1–L3 pianificati.
- **Target performance** (`00-foundations.md` §6): 60 fps UI, stagione completa < 60 s su hardware medio. Non ancora misurato.

## Dove siamo (commit `84b4217`, 2026-10-26)

- **7 crate:** `pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`, `pcfm-testkit`, `pcfm-debug`.
- **107 test**, clippy pulito (`-D warnings`), `--check` 6/6, `--list` 6 scenari, `--coverage` 2/26 (7%).
- **13 ADR accepted** (001-008, 012-015; 015 ratificato il 2026-10-26), **0 proposed**, 1 template (000), 1 reading guide (`docs/PLUGINS-WORLDS-READING-GUIDE.md`, fuori da `docs/adr/`).
- **11 moduli L0** ratified + implemented.
- **16 incongruenze** identificate e risolte (tutte con commit).
- **Working tree clean.**

## I 3 file che DEVI leggere per primo (in quest'ordine)

1. **`docs/PROJECT-IDEA-LOG.md`** (212 righe) — storia, decisioni, fonti (DeepSeek + questa conversazione), cosa resta aperto. Se leggi solo un file, leggi questo.
2. **`docs/REPO-SNAPSHOT.md`** (348 righe) — fotografia fattuale: albero, file, 7 crate con API/test, metriche, 16 incongruenze risolte. Contiene anche la lista completa dei file letti: se un contenuto non è qui, non è stato letto.
3. **`docs/ARCHITECTURE.md`** (210 righe) — come i crate interagiscono, flusso di un match, simulation/rendering split, testkit contract.

Poi, se devi toccare il codice: **`docs/CODE_TOUR.md`** (215 righe) — percorso di lettura in 5 stadi (infrastruttura → entità → piani → motore → testkit).

Poi la dottrina: **`modules/00-foundations.md`** (v2.0, pillar 1 = determinismo locale) e **`modules/_index.md`** (v2.9).

## Cosa resta da fare (dal PROJECT-IDEA-LOG §7)

In ordine di dipendenza:

1. **ADR-016** — "v1.0 content selections". Deve risolvere le **7 Deferred decisions** di ADR-015 (quali 3 assi psicologici? quali 20 ruoli? quali 10 formazioni? quali 5 nazioni? quali L2/L3 in v1.0?). È il passo naturale.
2. **`ProjectionProfile`** — metodo di calcolo delle bande (bootstrap? bayesiano? quantili empirici?).
3. **`ScoutReport.DeclaredBias`** — formula di applicazione del bias.
4. **`docs/performance.md`** — benchmark attesi, piano di misura.
5. **`docs/save-security.md`** — checksum, firma, backup, rollback.
6. **L1** — primo modulo: `modules/L1-world/01-nation.md`. DoD implicita: una stagione di lega giocabile, standings visibili, mondo (nazioni, città, stadi) esistente, rose complete. Vedi `modules/_index.md` §L1.

## Regole operative (importanti — le abbiamo stabilite insieme)

1. **File completi, sempre.** Quando una modifica riguarda un file < 500 righe, dammi il **file completo** da incollare sul web di GitHub. Niente `perl`/`sed` per modifiche a file piccoli. Per file > 500 righe, chiedimi se preferisco `perl` o file completo. **Eccezione:** i fix di 1 riga in file > 500 righe possono usare `perl -i -pe` con `grep -n` di verifica.
2. **Io lavoro su GitHub web + Git Bash su Windows.** I file li modifico incollando sul web. I comandi (`git pull`, `cargo`, `git mv`) li eseguo in Git Bash. Quando mi dai un comando, deve funzionare su Git Bash (Windows). **Niente script Python** (non è installato). Niente comandi Linux-only (`apt`, `systemctl`, ecc.).
3. **Pull prima di ogni verifica.** Dopo ogni modifica fatta sul web, devo fare `git pull --rebase` prima di eseguire `cargo`. Se me lo dimentico, ricordamelo. Se il pull si ferma per modifiche locali non committate, fermati e chiedimi.
4. **Sincerità reciproca, zero invenzioni.** Se non hai letto un file, dillo. Se un'informazione non è certa, dillo. Non inventare numeri, commit, o contenuti per "riempire" un buco. La regola è: "non so" > invenzione.
5. **Commit piccoli e atomici.** Un commit per file (o per gruppo logico). Messaggi chiari in inglese, imperativo.
6. **Verifica dopo ogni modifica al codice:** `cargo build --release` + `cargo clippy --workspace --all-targets -- -D warnings` + `cargo test --workspace` + `cargo fmt --check` + `cargo run --release --bin pcfm-debug -- --check --seed 42`. Se qualcosa è rosso, **fermati e sistemiamo**. Non "aggiustare" un test per farlo passare: il test è il contratto.
7. **Niente `unwrap`/`expect`/`panic!` nei percorsi di simulazione.** Niente `HashMap` nei percorsi di simulazione. Niente `unsafe` fuori da `pcfm-bridge` (futuro). Queste regole sono già nel codice, ma vale ricordarle.
8. **Se ti chiedo di leggere un file, incollami il comando `cat` da Git Bash** (es. `for f in ...; do echo "FILE: $f"; cat "$f"; done`), non inventare il contenuto.

## Regole specifiche del codebase (da non violare)

- **`pcfm-hidden` è speciale.** Contiene Plane A. Non ha campi pubblici, non è `Serialize`, `Debug` custom. L'unica porta è `resolve_*` + `drain_observations`. **Non aggiungere getter sui latenti.** Se un nuovo modulo ha bisogno di un latent, si aggiunge un `resolve_*` (con ADR).
- **`pcfm-testkit` non ha dipendenze `pcfm-*`.** È il contratto che chiunque (core, plugin, tool) implementa. **Non aggiungere `pcfm-match` o altri crate come dipendenze.**
- **Il modello di simulazione è un prototipo.** `action_probability = 0.0005`, split 60/25/12/3, `noise * 0.2` in `resolve_duel`, latents uniformi in [0,1). **Non "ottimizzare" senza ADR.** Un cambiamento a `resolve_*` o `MatchState::tick` richiede bump di `SIM_VERSION` (vedi `CHANGELOG-SIM.md`).
- **La dottrina vince.** Se una modifica contraddice `00-foundations.md`, serve un ADR di dottrina.
- **`docs/adr/` contiene solo ADR.** Il reading guide è in `docs/`, non in `docs/adr/`. Se aggiungi un ADR, `Status: proposed` → discussione → `Status: accepted` (o `rejected`).

## Cosa mi aspetto da te in questa nuova chat

1. **Leggi i 3 documenti chiave** (`PROJECT-IDEA-LOG.md`, `REPO-SNAPSHOT.md`, `ARCHITECTURE.md`) prima di propormi qualsiasi cosa.
2. **Conferma lo stato** con i comandi che ti chiedo (`git pull`, `git log`, `cargo test`, ecc.), senza assumere che sia quello che descrivo.
3. **Proponimi il prossimo passo concreto** (ADR-016 o quello che preferisco) e, se serve, dammi il file completo da incollare.
4. **Non inventare nulla.** Se non sai, chiedi. Se un file non l'hai letto, dillo.
5. **Se un test fallisce, fermati.** Non "aggiustare" il test per farlo passare.

## Cosa NON fare

- Non darmi "tanti commenti" sparsi nel codice: il codebase è già documentato (`///`, `//!`, `# Errors`, `# Panics`, `# Example`), più `ARCHITECTURE.md` e `CODE_TOUR.md`. Se vuoi aggiungere documentazione, proponi **dove** e **perché**, non "a tappeto".
- Non assumere che ADR-012, ADR-013, ADR-015 siano "proposed". Sono **accepted** (2026-10-25/26).
- Non assumere che `00-foundations.md` sia v1.4. È **v2.0** (pillar 1 riscritto: determinismo locale).
- Non assumere che ci siano "14 ADR". Sono **13 ADR accepted + 1 template (000) + 1 reading guide** (fuori da `docs/adr/`).
- Non inventare commit, hash, o contenuti di file che non hai letto.
- Non proporre script Python (non è installato). Usa Bash/Git Bash.
- Non assumere che il CI esista: non c'è `.github/workflows/`. Clippy/test/build/fmt/check sono **manuali**, vanno eseguiti a ogni modifica.
- Non "ottimizzare" il modello di simulazione senza ADR e bump di `SIM_VERSION`.

## Primo passo che voglio fare

**Scrivere ADR-016** ("v1.0 content selections"), che risolve le 7 Deferred decisions di ADR-015. Ma prima:

1. Leggi i 3 documenti chiave.
2. Verifica lo stato attuale con `git pull` + `git log --oneline -5`.
3. Dammi il comando `cat` per leggere i file che ti servono (probabilmente `modules/_index.md`, `modules/00-foundations.md` §7, `docs/adr/015-v1-scope.md`).
4. Poi proponimi una bozza di ADR-016 con le opzioni e le tue raccomandazioni, in italiano.
````
