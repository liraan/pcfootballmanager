# PC Football Manager

<div align="center">

**An open football management simulation built around transparency, privacy, and tactical depth.**

![Status](https://img.shields.io/badge/status-documentation_phase-yellow)
![Modules](https://img.shields.io/badge/modules-5_of_25-blue)
![Code](https://img.shields.io/badge/code-not_written_yet-lightgrey)
![Built with](https://img.shields.io/badge/built_with-AI_assistance-purple)

</div>

---

## 🎯 What Is This?

PC Football Manager (PCFM) is an attempt to rebuild the classic football management experience **without the compromises** that come with commercial titles:

- ✅ **No telemetry** — Nothing leaves your machine
- ✅ **Open data** — No proprietary feeds, no licensed datasets baked in
- ✅ **Tactical depth** — Deep role, formation, and instruction systems
- ✅ **Moddable** — Extend the game without compiling anything
- ✅ **Deterministic** — Same seed, same match. Every time.
- ✅ **Privacy-first** — Offline-first by design, not as an afterthought

**This is not Football Manager, and it doesn't try to be.** It's a community alternative where design decisions are public, mechanics are explainable, and you own everything.

---

## 🤖 Built With AI Assistance

This project's documentation is drafted with the help of a large language model, then reviewed and ratified by humans. Being upfront about this matters more than looking impressive.

**What that means in practice:**

| Reality | Detail |
|---|---|
| **AI drafts, humans decide** | Every architectural decision — the three planes, the ban on CA/PA, the determinism rules — came from a person. The AI helps articulate, structure, and stress-test them. |
| **AI makes mistakes** | Errors caught during review include a **non-existent crate dependency**, a **hasher that would have silently broken determinism**, and a **contradictory panic strategy at the FFI boundary**. Assume any technical detail here needs verification before you rely on it. |
| **AI has no memory** | Each session starts blank. **This repository is the memory.** Nothing persists between conversations unless it lives in a file here. |
| **No AI-written code yet** | There is no code. When there is, the same rule applies: AI may draft, humans review and own it. |

**Why disclose this?** Because you deserve to know how a document was produced before you trust it. Projects that quietly hide AI involvement are asking for a kind of trust they haven't earned.

---

## 🏗️ Project Status

```
PHASE 1 — DOCUMENTATION (NOW)
  Foundation modules (00-04)    ████████████████  100%   drafted
  Simulation modules (05-10)    ░░░░░░░░░░░░░░░░    0%
  UI / Editor modules (11-24)   ░░░░░░░░░░░░░░░░    0%

PHASE 2 — PROTOTYPE (Q1 2027)
  Core engine, world generation, text match

PHASE 3 — ALPHA (Q3 2027)
  Minimal UI, complete gameplay loop

PHASE 4 — BETA (Q1 2028)
  Pack system, modding, polish
```

**Right now:** this repository contains **documentation only**. No code exists. That's deliberate — we define *what* and *why* before writing *how*.

Two of the five foundation modules have already been revised after review, and a third has a correction pending. Drafted is not the same as finished.

Dates above are indicative, not commitments. Solid documentation beats fragile software shipped early.

---

## 📚 Documentation

The specification is organised into **25 independent but interconnected modules**.

### Foundation
| Module | Description | Status |
|--------|-------------|--------|
| [`00 — Foundations`](modules/00-foundations.md) | Core philosophy, constraints, vision | ✅ rev 1.0 |
| [`01 — Architecture`](modules/01-architecture.md) | Rust workspace, determinism, dependencies | ✅ rev 1.1 |
| [`02 — Registry`](modules/02-registry.md) | Modular registry, versioning, deprecation | ✅ rev 1.1 |
| [`03 — Pack System`](modules/03-pack-system.md) | Local expansions, validation, rollback | ✅ rev 1.0 |
| [`04 — Data Model`](modules/04-data-model.md) | Canonical schema, entities, invariants | ✅ rev 1.0 · rev 1.1 pending |

### Simulation *(not yet written)*
| Module | Description | Status |
|--------|-------------|--------|
| `05 — Generator` | Procedural world generation, seeds, provenance | 🚧 |
| `06 — Match Engine` | Text match simulation, events, ratings | 🚧 |
| `07 — Tactics` | Tactical system, roles, formations | 🚧 |
| `08 — Training` | Individual and squad training | 🚧 |
| `09 — Youth` | Academies, youth intake, development | 🚧 |
| `10 — Ingest` | Importing data from external sources | 🚧 |

### Economy & Market *(not yet written)*
| Module | Description | Status |
|--------|-------------|--------|
| `11 — Scouting` | Scout reports, declared bias, reliability | 🚧 |
| `12 — Market` | Transfers, offers, negotiations | 🚧 |
| `13 — Finance` | Budgets, wages, sponsorships, taxation | 🚧 |

### Player Experience *(not yet written)*
| Module | Description | Status |
|--------|-------------|--------|
| `14 — UI` | Screen layouts, flows, accessibility | 🚧 |
| `15 — Assets` | Kits, crests, localised names | 🚧 |
| `16 — Editor` | Tactics, presets, careers | 🚧 |
| `17 — AI` | Opposition managers, squad behaviour | 🚧 |

### World & Data *(not yet written)*
| Module | Description | Status |
|--------|-------------|--------|
| `18 — World` | Nations, competitions, legal rules | 🚧 |
| `19 — Statistics` | Event history, queries, aggregation | 🚧 |
| `20 — Psyche` | Player psychology, mental state | 🚧 |

### Quality & Future *(not yet written)*
| Module | Description | Status |
|--------|-------------|--------|
| `21 — Quality` | Testing, benchmarks, profiling | 🚧 |
| `22 — Roadmap` | Planning, milestones, risks | 🚧 |
| `23 — Contract` | Contract systems, clauses | 🚧 |
| `24 — Glossary` | Terminology, abbreviations | 🚧 |

---

## 🔑 Technical Philosophy

### The Three Planes

PCFM separates three distinct layers of data, and keeps them separated:

```
┌──────────────────────────────────────────────────────┐
│  PLANE A — Ground Truth (fmr-hidden)                 │
│  • Latent attributes (true finishing, passing)       │
│  • Physiological state (internal fatigue)            │
│  • Latent psychological state                        │
│  • NEVER exposed to the player                       │
│                                                      │
│  PLANE B — Observation (fmr-stats)                   │
│  • MatchEvents (shot, pass, duel)                    │
│  • Ratings computed FROM EVENTS, not from latents    │
│  • Append-only, immutable                            │
│                                                      │
│  PLANE C — Estimation (fmr-index, fmr-projection)    │
│  • Observed attributes (estimates derived from B)    │
│  • Indexes (striker effectiveness, playmaker impact) │
│  • Projections (NOT CA/PA — confidence bands)        │
└──────────────────────────────────────────────────────┘
```

**Why bother?** Because the gap between *how good a player really is* (A), *what we saw him do* (B), and *what we think he's worth* (C) is the entire point of a management game. Collapse those layers and scouting becomes a lookup table.

### No CA / PA

**Banned:** `current_ability`, `potential_ability`, or any equivalent scalar.

**Replaced by:**
- **IndexSnapshot** — Windowed observed measurement over time
- **ProjectionProfile** — A forecast expressed as a **confidence band**, never a single number

```yaml
# Instead of: "potential: 172"
projection:
  expected_band: [72, 88]     # a band, not a number
  confidence_curve:
    - { year: "2027", confidence: 0.95 }
    - { year: "2029", confidence: 0.72 }
    - { year: "2031", confidence: 0.48 }
  positive_drivers: ["consistent_minutes", "improving_metrics"]
  risk_factors: ["injury_history", "age_curve"]
```

### Total Determinism

Same seed → same world, same season, same match. **Always.**

- Named RNG streams — adding a new consumer never shifts existing results
- `BTreeMap` everywhere in the simulation path (no non-deterministic `HashMap`)
- Fixed-point arithmetic for money — zero numerical drift
- Golden tests across 10 seasons against a verified state hash
- `SIM_VERSION` bumped whenever simulation behaviour changes

---

## 🛠️ Technology Stack

### Core
| Technology | Version | Why |
|------------|---------|-----|
| Rust | 1.85+ | Type safety, zero-cost abstractions, guaranteed determinism |
| Cargo | — | Integrated package manager, reproducible builds |
| `fixed` | * | Fixed-point arithmetic for money |
| `libm` | * | Portable transcendentals (no floating-point drift) |

*Version numbers are deliberately left unpinned in the specs — any number written there would be invented. Every dependency gets verified against crates.io and justified in an ADR before adoption.*

### Testing & Analysis
| Tool | Purpose |
|------|---------|
| `proptest` | Property-based testing |
| `criterion` | Benchmarking |
| Miri | Undefined-behaviour detection (excluding FFI) |
| AddressSanitizer | Memory errors |
| `cargo-deny` | Dependency auditing |

### UI (undecided)
| Candidate | Status | Notes |
|-----------|--------|-------|
| `cxx-qt` + QWidgets | Under evaluation | Maturity, model/view support for data grids |
| `iced` | To explore | Pure Rust, cross-platform |
| `egui` | To explore | Immediate mode, simple |

---

## 📁 Repository Structure

```
pc-football-manager/
├── README.md
├── CONTRIBUTING.md
├── CHANGELOG-SIM.md             # SIM_VERSION bump log
├── CHANGELOG-REGISTRY.md        # Deprecation and migration log
├── modules/                     # Technical specification
│   ├── 00-foundations.md
│   ├── 01-architecture.md       # rev 1.1
│   ├── 02-registry.md           # rev 1.1
│   └── ... (25 modules total)
├── archive/                     # Historical versions
│   ├── Prompt_Master_v5.md
│   ├── Prompt_Master_v6.md
│   └── Prompt_Master_v7.md
├── docs/
│   ├── adr/                     # Architecture Decision Records
│   │   ├── 000-template.md
│   │   ├── 001-language-rust.md
│   │   ├── 004-panic-strategy-ffi.md
│   │   └── ...
│   ├── legal/                   # Data source compliance
│   └── calibration/             # Model calibration dashboards
├── registry/                    # Empty until code exists
├── crates/                      # Empty until code exists
├── mods/                        # Empty until user packs exist
└── data/                        # Examples only, never real licensed data
```

---

## 🎮 How It Differs From Commercial Titles

| Feature | Traditional FM | PC Football Manager |
|---------|----------------|---------------------|
| **Potential** | Single secret scalar (CA/PA) | Transparent confidence band |
| **Attributes** | True value = displayed value | True ≠ displayed; displayed is an estimate |
| **Scouting** | Accurate reports | Reports with **declared bias** |
| **Telemetry** | Optional / off | Does not exist |
| **Data** | Proprietary (Wyscout, Opta) | Open + procedural + community packs |
| **Modding** | Limited tooling | Modular registry, local packs |
| **Determinism** | Partial | Total (golden tests) |
| **Privacy** | Online-adjacent | Offline-first |

---

## 🤝 Contributing

### Documentation Phase (NOW)

1. **Fork the repository**
2. **Edit one module** in `modules/`
3. **Open a Pull Request**
4. **Discussion** — design happens in public
5. **Merge** — when there's rough consensus

### Guidelines

- One module per PR. No giant commits.
- Use the standard Markdown format
- Cite sources for factual claims
- Explain the **WHY**, not just the what
- Changing an invariant requires a new ADR
- If you spot an error, say so plainly. Including in AI-drafted text.

### Code (WHEN IT EXISTS)

- Follow the linting standard (`cargo clippy -- -D warnings`)
- Tests for every feature
- Document the public API
- No dependency without justification in an ADR

---

## 🙋 FAQ

**Is this a Football Manager clone?**
No. It's an alternative with a different philosophy. FM is proprietary, uses closed datasets, and ships telemetry. PCFM is community-driven, transparent, and offline-first.

**Will you use Wyscout / Opta data?**
No. Bundling licensed datasets is ruled out by design — see the planned Ingest module. We use open data, procedurally generated data, and community packs with clear provenance.

**Can I contribute without knowing how to program?**
Yes. The documentation needs feedback, corrections, and additions. If you understand football or game design, your input is useful.

**When is the alpha?**
This is the documentation phase. Alpha arrives when modules 00–10 are stable and a minimal kernel runs. Indicative timeline: **Q1 2027**.

**Is any of this actually decided?**
Partly. Open decisions are marked as such in the modules and tracked in `docs/adr/`. Two remain unresolved: the Plane A enforcement strategy and the Qt binding choice. The FFI panic strategy was settled in Architecture rev 1.1.

**How much of this was written by AI?**
See [Built With AI Assistance](#built-with-ai-assistance). Short answer: drafted by AI, reviewed by a human, and every technical claim should be verified before you depend on it.

---

## 📬 Contact

- **Discussions:** GitHub Discussions
- **Bugs / Features:** GitHub Issues
- **Community chat:** *to be decided*

There are no private email addresses, Telegram channels, or WhatsApp groups. Everything happens publicly on GitHub.

---

## 📈 Roadmap

```
2026-Q4  ████████████░░░░░░░░░░░░  Documentation (modules 00-24)
2027-Q1  ████░░░░░░░░░░░░░░░░░░░░  Kernel prototype (05-06)
2027-Q2  ████████░░░░░░░░░░░░░░░░  Closed alpha (00-13)
2027-Q3  ████████████░░░░░░░░░░░░  Public alpha (00-17)
2027-Q4  ████████████████░░░░░░░░  Beta (00-21)
2028-Q1  ████████████████████░░░░  Release candidate
2028-Q2  ████████████████████████  v1.0
```

**These dates are estimates.** Documentation has to be solid before code gets written. Delaying is better than shipping something fragile.

---

## 🙏 Acknowledgements

- The Football Manager community, for showing what a management game can be
- Developers of earlier management sims who opened the road
- The open-source ecosystem that makes transparency possible
- Anyone who contributes to this project

---

<div align="center">

**© 2026 PC Football Manager Team**

*Transparent by design. Open data. Your control.*

</div>

---

<!--
NOTES FOR FUTURE EDITORS:
- Keep the status section honest. If nothing moved for months, say so.
- Do not promise dates you cannot meet.
- Mark undecided things as undecided.
- Cite sources for factual claims.
- One module change = one PR.
- The AI disclosure section stays. If it stops being true, update it — do not delete it.
-->
