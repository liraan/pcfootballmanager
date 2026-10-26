# New Session Prompt — PCFM

**Scopo:** prompt da incollare come primo messaggio di una nuova chat, per riprendere il lavoro sul progetto senza perdere contesto. Aggiornare questo file quando cambia lo stato del progetto o le regole operative.

**Ultimo aggiornamento:** 2026-10-26 (commit `a15e81a`)

---

## Come si usa

1. Apri una nuova chat.
2. Incolla il contenuto del blocco ` ```markdown ... ``` ` qui sotto (senza i backtick esterni).
3. L'AI leggerà questo file dal repository e ripartirà da qui.

---

## Prompt

Ripresa progetto PCFM — nuova sessione
Sto riprendendo il lavoro sul progetto PC Football Manager (PCFM). La chat precedente si è chiusa in un punto preciso. Ti passo il contesto per ripartire senza perdere nulla.

Il progetto
Repository: https://github.com/liraan/pcfootballmanager (pubblico)

Cos'è: un'alternativa community ai gestionali calcistici commerciali. Trasparente, offline-first, deterministico. Dottrina dei Tre Piani (Plane A = Ground Truth, Plane B = Observation, Plane C = Estimation).

Lingua: documentazione tecnica in inglese, conversazione con me in italiano.

Stato: L0 (match engine) completo e implementato. L1–L3 pianificati.

Dove siamo (commit a15e81a, 2026-10-26)
7 crate: pcfm-core, pcfm-model, pcfm-hidden, pcfm-stats, pcfm-match, pcfm-testkit, pcfm-debug.

107 test, clippy pulito (-D warnings), --check 6/6, --coverage 2/26 (7%).

13 ADR accepted (001-008, 012-015), 0 proposed, 1 template (000), 1 reading guide (docs/PLUGINS-WORLDS-READING-GUIDE.md).

11 moduli L0 ratified + implemented.

Working tree clean.

16 incongruenze identificate e risolte (tutte con commit).

Documenti da leggere per primo (in quest'ordine)
docs/PROJECT-IDEA-LOG.md — storia, decisioni, fonti (DeepSeek + questa conversazione), cosa resta aperto.

docs/REPO-SNAPSHOT.md — fotografia fattuale: albero, file, 7 crate con API/test, metriche, 16 incongruenze risolte.

docs/ARCHITECTURE.md — come i crate interagiscono, flusso di un match, simulation/rendering split, testkit contract.

docs/CODE_TOUR.md — percorso di lettura in 5 stadi.

modules/00-foundations.md (v2.0) — la dottrina.

modules/_index.md (v2.9) — mappa dei moduli.

Cosa resta da fare (dal PROJECT-IDEA-LOG §7)
In ordine di dipendenza:

ADR-016 — "v1.0 content selections". Deve risolvere le 7 Deferred decisions di ADR-015 (quali 3 assi psicologici? quali 20 ruoli? quali 10 formazioni? quali 5 nazioni? quali L2/L3 in v1.0?). È il passo naturale.

ProjectionProfile — metodo di calcolo delle bande (bootstrap? bayesiano? quantili empirici?).

ScoutReport.DeclaredBias — formula di applicazione del bias.

docs/performance.md — benchmark attesi, piano di misura.

docs/save-security.md — checksum, firma, backup, rollback.

L1 — primo modulo: modules/L1-world/01-nation.md.

Regole operative (importanti — le abbiamo stabilite insieme)
File completi, sempre. Quando una modifica riguarda un file < 500 righe, dammi il file completo da incollare sul web di GitHub. Niente perl/sed per modifiche a file piccoli. Per file > 500 righe, chiedimi se preferisco perl o file completo.

Io lavoro su GitHub web + Git Bash su Windows. I file li modifico incollando sul web. I comandi (git pull, cargo, git mv) li eseguo in Git Bash. Quando mi dai un comando, deve funzionare su Git Bash (Windows), non su bash Linux puro.

Pull prima di ogni verifica. Dopo ogni modifica fatta sul web, devo fare git pull prima di eseguire cargo. Se me lo dimentico, ricordamelo.

Sincerità reciproca, zero invenzioni. Se non hai letto un file, dillo. Se un'informazione non è certa, dillo. Non inventare numeri, commit, o contenuti per "riempire" un buco. La regola è: "non so" > invenzione.

Commit piccoli e atomici. Un commit per file (o per gruppo logico). Messaggi chiari in inglese, imperativo.

Verifica sempre con cargo build + cargo clippy -- -D warnings + cargo test --workspace + cargo run --release --bin pcfm-debug -- --check --seed 42 dopo ogni modifica al codice. Se qualcosa è rosso, fermati e sistemiamo.

Niente unwrap/expect/panic! nei percorsi di simulazione. Niente HashMap nei percorsi di simulazione. Niente unsafe fuori da pcfm-bridge. Queste regole sono già nel codice, ma vale ricordarle.

Se ti chiedo di leggere un file, incollami il comando cat da Git Bash (es. for f in ...; do echo "FILE: $f"; cat "$f"; done), non inventare il contenuto.

Cosa mi aspetto da te in questa nuova chat
Leggi i documenti chiave (almeno PROJECT-IDEA-LOG.md, REPO-SNAPSHOT.md, ARCHITECTURE.md) prima di propormi qualsiasi cosa.

Conferma lo stato con i comandi che ti chiedo, senza assumere che sia quello che descrivo.

Proponimi il prossimo passo concreto (ADR-016 o quello che preferisco) e, se serve, dammi il file completo da incollare.

Non inventare nulla. Se non sai, chiedi.

Cosa NON fare
Non darmi "tanti commenti" sparsi nel codice: il codebase è già documentato (///, //!, # Errors, # Panics, # Example), più ARCHITECTURE.md e CODE_TOUR.md. Se vuoi aggiungere documentazione, proponi dove e perché, non "a tappeto".

Non assumere che ADR-012, ADR-013, ADR-015 siano "proposed". Sono accepted (2026-10-25/26).

Non assumere che 00-foundations.md sia v1.4. È v2.0 (pillar 1 riscritto: determinismo locale).

Non assumere che ci siano "14 ADR". Sono 13 ADR accepted + 1 template (000) + 1 reading guide (fuori da docs/adr/).

Non inventare commit, hash, o contenuti di file che non hai letto.

Primo passo che voglio fare
Scrivere ADR-016 ("v1.0 content selections"), che risolve le 7 Deferred decisions di ADR-015. Ma prima:

Leggi i documenti chiave.

Verifica lo stato attuale con git pull + git log --oneline -5.

Dammi il comando cat per leggere i file che ti servono (probabilmente modules/_index.md, modules/00-foundations.md §7, docs/adr/015-v1-scope.md).

Poi proponimi una bozza di ADR-016 con le opzioni e le tue raccomandazioni, in italiano.
