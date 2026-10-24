# PC Football Manager

<div align="center">

**Un gestionale calcistico open-source con focus su trasparenza, privacy e profondità tattica.**

[![Build Status](https://img.shields.io/badge/build-documentation-brightgreen)](./modules/)
[![License](https://img.shields.io/badge/license-proprietary--docs--CC--BY--NC--SA--grey)](#license)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/status-documentation_phase-yellow)]()

![Logo](assets/logo-placeholder.png)

</div>

---

## 🎯 Cos'è PC Football Manager?

PC Football Manager è un progetto ambizioso per ricreare l'esperienza del gestionale calcistico classico **senza compromessi**:

- ✅ **Zero telemetria** — Tutto rimane sulla tua macchina
- ✅ **Open data** — Niente fornitori proprietari, dati trasparenti
- ✅ **Profondità tattica** — Tanti ruoli, formazioni, istruzioni personalizzabili
- ✅ **Moddabilità** — Estendi il gioco senza compilare nulla
- ✅ **Determinismo** — Stesso seed = stessa partita, sempre
- ✅ **Privacy-first** — Nessun dato esce dal tuo computer

**Non siamo Football Manager. Non cerchiamo di esserlo.** Vogliamo essere una **alternativa comunitaria** dove le decisioni di design sono pubbliche, le meccaniche sono comprensibili, e tu controlli tutto.

---

## 🏗️ Stato Attuale del Progetto

```
╭─────────────────────────────────────────────────────────╮
│  FASE 1: DOCUMENTAZIONE (ORA)                           │
│  ├─ Moduli fondamentali (00-04) ████████████░░░░░ 80%   │
│  ├─ Moduli simulazione (05-10) ░░░░░░░░░░░░░░░░░  0%     │
│  └─ Moduli UI/Editor (11-24) ░░░░░░░░░░░░░░░░░  0%       │
│                                                        │
│  FASE 2: PROTOTIPO (Q1 2027)                           │
│  └─ Engine base, generazione mondo, match text        │
│                                                        │
│  FASE 3: ALFA (Q3 2027)                                │
│  └─ UI minimale, workflow completo                    │
│                                                        │
│  FASE 4: BETA (Q1 2028)                                │
│  └─ Pack system, modding, polishing                   │
╰─────────────────────────────────────────────────────────╯
```

**Attualmente:** questa repository contiene **solo documentazione tecnica**. Il codice non esiste ancora. Questo è intenzionale: definiamo prima il *cosa* e il *perché*, poi scriviamo il *come*.

---

## 📚 Documentazione

La documentazione è strutturata in **24 moduli** indipendenti ma collegati:

### Fondamenta
| Modulo | Descrizione | Stato |
|--------|-------------|-------|
| [`00 — Fondamenta`](modules/00-fondamenta.md) | Principi filosofici, vincoli, visione | ✅ Completa |
| [`01 — Architettura`](modules/01-architettura.md) | Workspace Rust, determinismo, dipendenze | ✅ v1.1 |
| [`02 — Registry`](modules/02-registry.md) | Registro modulare, versioning, deprecation | ✅ v1.1 |
| [`03 — Pack System`](modules/03-pack-system.md) | Gestione espansioni locali, validazione, rollback | ✅ |
| [`04 — Modelli`](modules/04-modelli.md) | Schema dati canonico, entità, invarianti | ✅ |

### Simulazione (da scrivere)
| Modulo | Descrizione | Stato |
|--------|-------------|-------|
| `05 — Generator` | Generazione procedurale mondi, seed, provenance | 🚧 |
| `06 — Match Engine` | Motore partita testuale, eventi, pagelle | 🚧 |
| `07 — Tactics` | Sistema tattico, ruoli, formazioni | 🚧 |
| `08 — Training` | Allenamento individuale e di squadra | 🚧 |
| `09 — Youth` | Accademia giovanile, scouting interni | 🚧 |
| `10 — Ingest` | Importazione dati da fonti multiple | 🚧 |

### Economia e Mercato (da scrivere)
| Modulo | Descrizione | Stato |
|--------|-------------|-------|
| `11 — Scouting` | Rapporti scout, bias dichiarati, affidabilità | 🚧 |
| `12 — Market` | Trasferimenti, offerte, negoziazioni | 🚧 |
| `13 — Finance` | Bilanci, salari, sponsor, tassazione | 🚧 |

### Esperienza Giocatore (da scrivere)
| Modulo | Descrizione | Stato |
|--------|-------------|-------|
| `14 — UI` | Layout schermate, flussi, accessibilità | 🚧 |
| `15 — Assets` | Kit, stemmi, nomi localizzati | 🚧 |
| `16 — Editor` | Tattiche, presete, carriere | 🚧 |
| `17 — AI` | Gestione squadre avversarie, comportamento | 🚧 |

### Mondo e Dati (da scrivere)
| Modulo | Descrizione | Stato |
|--------|-------------|-------|
| `18 — World` | Nazioni, competizioni, regole legali | 🚧 |
| `19 — Statistics` | Storico eventi, query, aggregazioni | 🚧 |
| `20 — Psiche` | Psicologia giocatori, stato mentale | 🚧 |

### Qualità e Futuro (da scrivere)
| Modulo | Descrizione | Stato |
|--------|-------------|-------|
| `21 — Quality` | Test, benchmark, profilazione | 🚧 |
| `22 — Roadmap` | Pianificazione, milestone, rischi | 🚧 |
| `23 — Contract` | Sistemi contrattuali, clausole | 🚧 |
| `24 — Glossario` | Terminologia, abbreviazioni | 🚧 |

---

## 🔑 Filosofia Tecnica

### I Tre Piani

PCFM separa esplicitamente tre livelli di dati:

```
┌─────────────────────────────────────────────────────┐
│  PIANO A — Ground Truth (in fmr-hidden)             │
│  • Attributi latenti (finishing vero, passing)     │
│  • Stati fisiologici (fatigue interna)             │
│  • Stati psicologici latenti                       │
│  • MAI esposti al giocatore                        │
│                                                     │
│  PIANO B — Osservazione (in fmr-stats)             │
│  • MatchEvent (tiro, passaggio, duello)            │
│  • Pagelle calcolate DAGLI EVENTI, non dai latenti │
│  • Append-only, immutabile                         │
│                                                     │
│  PIANO C — Stima (in fmr-index, fmr-projection)    │
│  • Attributi osservati (stime dai dati Piano B)    │
│  • Index (efficacia attaccante, playmaker, ecc.)   │
│  • Proiezioni (non CA/PA! Banda di confidenza)      │
└─────────────────────────────────────────────────────┘
```

**Perché?** Perché la differenza tra *quanto è bravo realmente* (A), *quanto vediamo in campo* (B), e *quanto pensiamo che sia* (C) è il cuore dell'esperienza gestionale.

### Nessun CA/PA

**Vietato:** campi `current_ability`, `potential_ability`, o equivalenti scalari.

**Sostituito con:**
- **IndexSnapshot** — Misura osservata finestrata nel tempo
- **ProjectionProfile** — Previsione con **banda di confidenza**, non valore singolo

```yaml
# Invece di: "potenziale: 172"
projection:
  expected_band: [72, 88]  # Banda, non numero
  confidence_curve:
    - year: "2027"  conf: 0.95
    - year: "2029"  conf: 0.72
    - year: "2031"  conf: 0.48
  positive_drivers: ["consistent_minutes", "improving_metrics"]
  risk_factors: ["injury_history", "age_curve"]
```

### Determinismo Totale

Stesso seed → stesso mondo, stessa stagione, stessa partita. **Sempre.**

- RNG stream nominati (aggiungere consumatori non sposta risultati esistenti)
- `BTreeMap` ovunque nel percorso critico (no `HashMap` non deterministiche)
- Fixed-point per denaro (zero drift numerico)
- Golden test su 10 stagioni con hash verificato
- `SIM_VERSION` bump quando cambia la simulazione

---

## 🛠️ Stack Tecnologico

### Core
| Tecnologia | Versione | Perché |
|------------|----------|--------|
| Rust | 1.85+ | Type-safety, zero-cost abstractions, determinismo garantito |
| Cargo | — | Package manager integrato, riproducibilità build |
| fixed | * | Fixed-point arithmetic per denaro |
| libm | * | Funzioni matematiche portabili (no floating drift) |

### Testing
| Strumento | Scopo |
|-----------|-------|
| `proptest` | Property-based testing |
| `criterion` | Benchmarking |
| Miri | UB detection (non FFI) |
| AddressSanitizer | Memory errors |
| cargo-deny | Dependency auditing |

### UI (da scegliere)
| Candidato | Stato | Note |
|-----------|-------|------|
| `cxx-qt` + QWidgets | In valutazione | Maturation, model/view per tabelle |
| `iced` | Da esplorare | Pure Rust, cross-platform |
| `egui` | Da esplorare | Immediate mode, semplice |

---

## 📁 Struttura Repository

```
pc-fm/
├── README.md
├── LICENSE                      # Da decidere (vedi §License)
├── CONTRIBUTING.md
├── CHANGELOG-SIM.md             # Registro bump SIM_VERSION
├── modules/                     # Documentazione tecnica
│   ├── 00-fondamenta.md
│   ├── 01-architettura.md       # v1.1
│   ├── 02-registry.md           # v1.1
│   └── ... (24 moduli totali)
├── archive/                     # Versioni storiche
│   ├── Prompt_Master_v5.md
│   ├── Prompt_Master_v6.md
│   └── Prompt_Master_v7.md
├── docs/
│   ├── adr/                     # Architecture Decision Records
│   │   ├── 000-template.md
│   │   ├── 001-linguaggio-rust.md
│   │   ├── 004-panic-strategy-ffi.md
│   │   └── ...
│   ├── legal/                   # Compliance fonti dati
│   └── calibration/             # Dashboard modelli
├── registry/                    # Vuoto fino al codice
├── crates/                      # Vuoto fino al codice
├── mods/                        # Vuoto fino ai pack utente
└── data/                        # Esempi, non dati reali
```

---

## 🎮 Differenze Chiave vs FM Classico

| Caratteristica | FM Tradizionale | PC Football Manager |
|----------------|-----------------|---------------------|
| **Potenziale** | Valore singolo segreto (CA/PA) | Banda di confidenza trasparente |
| **Attributi** | Vero = visibile | Vero ≠ visibile = stima (3 piani) |
| **Scouting** | Report accurati | Report con bias dichiarati |
| **Telemetria** | Opzionale/off | Non esiste |
| **Dati** | Proprietari (Wyscout, Opta) | Open + procedurali + pack community |
| **Modding** | Strumenti limitati | Registry modulare, pack locali |
| **Determinismo** | Parziale | Totale (golden test) |
| **Privato** | No (online) | Sì (offline-first) |

---

## 🤝 Come Contribuire

### Fase Documentazione (ORA)

1. **Fork la repository**
2. **Modifica un modulo** in `modules/`
3. **Crea una Pull Request**
4. **Discussione** — Il design è pubblico
5. **Merge** — Quando ci sono consensi

### Linee Guida

- Un modulo per PR (non commit giganteschi)
- Usa il formato Markdown standardizzato
- Cita le fonti per claims fattuali
- Spiega il PERCHÉ, non solo il COSA
- Se cambi un'invariante, crea un ADR

### Codice (QUANDO ARRIVA)

- Segui il linting standard (`cargo clippy -- -D warnings`)
- Scrivi test per ogni feature
- Documenta la pubblica API
- Nessuna dipendenza senza giustificazione in ADR

---

## ⚖️ License

### Documento vs Codice

| Contenuto | License |
|-----------|---------|
| **Documentazione** (`modules/*.md`, `README.md`) | CC BY-NC-SA 4.0 |
| **Codice futuro** | Da decidere (MIT / Apache-2.0 / Proprietary) |
| **Dati di esempio** | Pubblico dominio |
| **Asset** | Da decidere per ciascun file |

**Perché licenza separata?** La documentazione deve essere condivisibile e modificabile dalla community. Il codice sarà scelto quando scriveremo il codice.

**Licenza documentazione (CC BY-NC-SA):**
- ✅ Puoi condividere e modificare
- ❌ Non uso commerciale
- ✅ Condividi allo stesso modo

**Se vuoi cambiare la licenza del codice:** aprici un'issue e discutiamolo.

---

## 🙋 FAQ

### È un clone di Football Manager?
No. È un'alternativa con filosofia diversa. FM è proprietario, usa dati chiusi, ha telemetria. PCFM è comunitario, trasparente, offline-first.

### Userai Wyscout/Opta come dati?
**No.** Il Modulo 10 vieta esplicitamente di incorporare dati licenziati. Usiamo:
- Dati open (es. Transfermarkt, API pubbliche)
- Dati procedurali generati dal seed
- Pack forniti dalla community con licenza chiara

### Posso usare questo per lavoro?
La documentazione è CC BY-NC-SA (non commerciale). Il codice futuro avrà licenza separata. Se vuoi uso commerciale, aprici una discussione.

### Quando uscirà la alpha?
Questa è la fase documentazione. L'alpha arriverà quando almeno i moduli 00-10 saranno stabili e il kernel minimo funzionerà. Timeline indicativa: **Q1 2027**.

### Posso contribuire senza saper programmare?
Sì! La documentazione ha bisogno di feedback, correzioni, integrazioni. Se capisci di calcio o di design di giochi, il tuo contributo è utile.

---

## 📬 Contatti

- **Discussioni:** GitHub Discussions
- **Bug/Feature:** GitHub Issues
- **Chat comunitaria:** [Da creare — Discord/Matrix?]

**Attenzione:** non ci sono email dirette, canali Telegram privati, o WhatsApp. Tutto è pubblico su GitHub.

---

## 📈 Roadmap (Indicatori, non impegni)

```
2026-Q4  ████████████░░░░░░░░░░░░  Documentazione (moduli 00-24)
2027-Q1  ████░░░░░░░░░░░░░░░░░░░░  Prototipo kernel (05-06)
2027-Q2  ████████░░░░░░░░░░░░░░░░  Alpha closed (00-13)
2027-Q3  ████████████░░░░░░░░░░░░  Alpha public (00-17)
2027-Q4  ████████████████░░░░░░░░  Beta (00-21)
2028-Q1  ████████████████████░░░░  Release Candidate
2028-Q2  ████████████████████████  v1.0
```

**Le date sono indicative.** La documentazione deve essere solida prima di scrivere codice. Meglio ritardare che fare software fragile.

---

## 🙏 Ringraziamenti

- La community di Football Manager per averci insegnato cosa significa "gestionale"
- I developer di FM classici che hanno aperto la strada
- Gli sviluppatori open-source che rendono possibile la trasparenza
- Chiunque contribuirà a questo progetto

---

<div align="center">

**© 2026 PC Football Manager Team**

*Open source. Open data. Your control.*

</div>

---

<!--
NOTE TO FUTURE EDITORS:
- Keep sections up to date
- Don't promise dates you can't meet
- If something is undecided, mark it as such
- Cite sources for factual claims
- One module change = one PR, not massive commits
-->
