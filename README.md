![Status](https://img.shields.io/badge/status-L0_ratified-yellow)
![Structure](https://img.shields.io/badge/structure-L0--L3-blue)
![Code](https://img.shields.io/badge/code-ready_to_start-lightgrey)
![Code](https://img.shields.io/badge/code-103_tests_green-lightgrey)
![Built with](https://img.shields.io/badge/built_with-AI_assistance-purple)

</div>
@@ -21,7 +21,7 @@ PC Football Manager (PCFM) is an attempt to rebuild the classic football managem
- ✅ **Open data** — No proprietary feeds, no licensed datasets baked in
- ✅ **Tactical depth** — Deep role, formation, and instruction systems
- ✅ **Moddable** — Extend the game without compiling anything
- ✅ **Deterministic** — Same seed, same match. Every time.
- ✅ **Deterministic** — Same seed, same match. Every time. *(Currently global; under review for local, see [ADR-013](docs/adr/013-worlds.md).)*
- ✅ **Privacy-first** — Offline-first by design, not as an afterthought

**This is not Football Manager, and it doesn't try to be.** It's a community alternative where design decisions are public, mechanics are explainable, and you own everything.
@@ -39,7 +39,7 @@ This project's documentation is drafted with the help of a large language model,
| **AI drafts, humans decide** | Every architectural decision — the three planes, the ban on CA/PA, the determinism rules — came from a person. The AI helps articulate, structure, and stress-test them. |
| **AI makes mistakes** | Errors caught during review include a **non-existent crate dependency**, a **hasher that would have silently broken determinism**, and a **contradictory panic strategy at the FFI boundary**. Assume any technical detail here needs verification before you rely on it. |
| **AI has no memory** | Each session starts blank. **This repository is the memory.** Nothing persists between conversations unless it lives in a file here. |
| **No AI-written code yet** | There is no code. When there is, the same rule applies: AI may draft, humans review and own it. |
| **No AI-written code yet** | There is no AI-written code. When there is, the same rule applies: AI may draft, humans review and own it. |

**Why disclose this?** Because you deserve to know how a document was produced before you trust it. Projects that quietly hide AI involvement are asking for a kind of trust they haven't earned.

@@ -48,7 +48,7 @@ This project's documentation is drafted with the help of a large language model,
## 🏗️ Project Status

```
L0 — MATCH     ████████████████  100%   ratified (11 cells, ADR-003 accepted)
L0 — MATCH     ████████████████  100%   ratified + implemented (11 cells, 6 crates, 103 tests)
L1 — SEASON    ░░░░░░░░░░░░░░░░    0%   not started (12 cells)
L2 — CAREER    ░░░░░░░░░░░░░░░░    0%   not started (9 cells)
L3 — META      ░░░░░░░░░░░░░░░░    0%   not started (8 cells)
@@ -59,9 +59,11 @@ PHASE 4 — BETA      (Q1 2028)
PHASE 5 — v1.0      (Q2 2028)
```

**Right now:** the repository contains **documentation only**. The L0 specification is complete and ratified; code can begin. L1 (the season) is the next specification milestone. Dates are indicative, not commitments.
**Right now:** the repository contains **documentation and code**. The L0 specification is complete and ratified; L0 is implemented (6 crates, 103 tests). L1 (the season) is the next specification milestone.

The three planes, the ban on CA/PA, the determinism rules, and the Plane A enforcement strategy (ADR-003, option B) are all locked in. The specification is no longer in flux for L0.
The three planes, the ban on CA/PA, the determinism rules, and the Plane A enforcement strategy (ADR-003, option B) are all locked in. The **scope** of the determinism is under review (ADR-013, proposed): today it is **global**, the proposal is to make it **local** (per save, per module configuration). The doctrine says the truth: pillar 1 is "under review".

Dates are indicative, not commitments.

---

@@ -71,12 +73,13 @@ The specification is **concentric**, not flat. Read in this order:

| Layer | File | What it answers |
|---|---|---|
| **1. Doctrine** | [`modules/00-foundations.md`](modules/00-foundations.md) | Why does this project exist? What are the non-negotiable rules? *(v1.1)* |
| **1. Doctrine** | [`modules/00-foundations.md`](modules/00-foundations.md) | Why does this project exist? What are the non-negotiable rules? *(v1.4, pillar 1 under review)* |
| **2. Structure** | [`STRUCTURE.md`](STRUCTURE.md) | How is the repo organised? How do I add or revise a module? |
| **3. Index** | [`modules/_index.md`](modules/_index.md) | What exists at each level? What is its status? |
| **4. Decisions** | [`docs/adr/`](docs/adr/) | Which architectural choices have been made, and why? |
| **5. Glossary** | [`docs/GLOSSARY.md`](docs/GLOSSARY.md) | What does each term mean? Where is it defined? |
| **6. Process** | [`CONTRIBUTING.md`](CONTRIBUTING.md) | How do I propose a change? What's the review process? |
| **6. Developer guide** | [`docs/MODULE_DEVELOPER_GUIDE.md`](docs/MODULE_DEVELOPER_GUIDE.md) | How do I write a module? Why does the simulation/rendering split matter? |
| **7. Process** | [`CONTRIBUTING.md`](CONTRIBUTING.md) | How do I propose a change? What's the review process? |

### Levels

@@ -101,18 +104,18 @@ PCFM separates three distinct layers of data, and keeps them separated:

```
┌──────────────────────────────────────────────────────┐
│  PLANE A — Ground Truth (pcfm-hidden)                 │
│  PLANE A — Ground Truth (pcfm-hidden)                │
│  • Latent attributes (true finishing, passing)       │
│  • Physiological state (internal fatigue)            │
│  • Latent psychological state                        │
│  • NEVER exposed to the player                       │
│                                                      │
│  PLANE B — Observation (pcfm-stats)                   │
│  PLANE B — Observation (pcfm-stats)                  │
│  • MatchEvents (shot, pass, duel)                    │
│  • Ratings computed FROM EVENTS, not from latents    │
│  • Append-only, immutable                            │
│                                                      │
│  PLANE C — Estimation (pcfm-index, pcfm-projection)    │
│  PLANE C — Estimation (pcfm-index, pcfm-projection)  │
│  • Observed attributes (estimates derived from B)    │
│  • Indexes (striker effectiveness, playmaker impact) │
│  • Projections (NOT CA/PA — confidence bands)        │
@@ -143,17 +146,19 @@ projection:
  risk_factors: ["injury_history", "age_curve"]
```

### Total Determinism
### Determinism

Same seed → same world, same season, same match. **Always.**
Same seed → same result. **Today, globally.** The rules are non-negotiable:

- Named RNG streams — adding a new consumer never shifts existing results
- `BTreeMap` everywhere in the simulation path (no non-deterministic `HashMap`)
- Fixed-point arithmetic for money — zero numerical drift
- Golden tests across 10 seasons against a verified state hash
- Golden tests against a verified state hash
- `SIM_VERSION` bumped whenever simulation behaviour changes (see [`CHANGELOG-SIM.md`](CHANGELOG-SIM.md))

Details: [`modules/L0-match/01-rng.md`](modules/L0-match/01-rng.md).
**Under review (ADR-013, proposed).** The **scope** of the determinism is being discussed. The current contract is **global**: same seed across all configurations. The proposal (ADR-013) is **local**: same save + same match + same module configuration. If ratified, the doctrine will change and the pillar will be rewritten. Until then, the doctrine is the current contract.

Details: [`modules/00-foundations.md`](modules/00-foundations.md) §4, [`docs/adr/013-worlds.md`](docs/adr/013-worlds.md) (proposed).

---

@@ -205,14 +210,15 @@ pc-football-manager/
├── CHANGELOG.md
├── CHANGELOG-SIM.md
├── CHANGELOG-REGISTRY.md
├── .gitignore
├── Cargo.toml, Cargo.lock, rust-toolchain.toml
├── .gitignore, .cargo/config.toml
├── .github/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── ISSUE_TEMPLATE/
├── modules/                   # specification, concentric L0–L3
│   ├── 00-foundations.md
│   ├── _index.md
│   ├── L0-match/              # 11 cells, ratified
│   ├── L0-match/              # 11 cells, ratified + implemented
│   ├── L1-*/                  # *(planned)* 12 cells
│   ├── L2-*/                  # *(planned)* 9 cells
│   └── L3-*/                  # *(planned)* 8 cells
@@ -222,13 +228,20 @@ pc-football-manager/
│   ├── doctrine/              # *(planned)* previous 00-foundations versions
│   └── legacy/                # *(planned)* four Italian pre-concentric modules
├── docs/
│   ├── adr/                   # 8 ADRs, 000 → 007
│   ├── adr/                   # 11 ADRs, 000 → 013
│   ├── legal/                 # *(planned)* data source compliance
│   ├── calibration/           # *(planned)* model calibration dashboards
│   ├── GLOSSARY.md
│   └── MIGRATION-01-04.md
│   ├── MIGRATION-01-04.md
│   └── MODULE_DEVELOPER_GUIDE.md
├── crates/                    # code (6 crates, 103 tests)
│   ├── pcfm-core/
│   ├── pcfm-model/
│   ├── pcfm-hidden/
│   ├── pcfm-stats/
│   ├── pcfm-match/
│   └── pcfm-debug/
├── registry/                  # *(planned)* empty until code exists
├── crates/                    # *(planned)* empty until code exists
├── mods/                      # *(planned)* empty until user packs exist
└── data/                      # *(planned)* examples only, never real licensed data
```
@@ -245,7 +258,7 @@ pc-football-manager/
| **Telemetry** | Optional / off | Does not exist |
| **Data** | Proprietary (Wyscout, Opta) | Open + procedural + community packs |
| **Modding** | Limited tooling | Modular registry, local packs |
| **Determinism** | Partial | Total (golden tests) |
| **Determinism** | Partial | Total (golden tests). Scope under review (ADR-013). |
| **Privacy** | Online-adjacent | Offline-first |

---
@@ -261,7 +274,7 @@ See **[`CONTRIBUTING.md`](CONTRIBUTING.md)** for the full process. Quick summary
- Changing an invariant requires a new ADR.
- If you spot an error, say so plainly — including in AI-drafted text.

The **L0 specification is complete and ratified**. Code can begin. If you want to contribute code, read the L0 cells first, then start from `pcfm-core` (see `STRUCTURE.md` §3).
The **L0 specification is complete and ratified, and L0 is implemented**. Code contributions are welcome, and must follow [`docs/MODULE_DEVELOPER_GUIDE.md`](docs/MODULE_DEVELOPER_GUIDE.md).

---

@@ -271,7 +284,7 @@ The **L0 specification is complete and ratified**. Code can begin. If you want t
No. It's an alternative with a different philosophy. FM is proprietary, uses closed datasets, and ships telemetry. PCFM is community-driven, transparent, and offline-first.

**Will you use Wyscout / Opta data?**
No. Bundling licensed datasets is ruled out by design — see the planned Ingest module (L2). We use open data, procedurally generated data, and community packs with clear provenance.
No. Bundling licensed datasets is ruled out by design. Provider integration is an external tool that produces packs, not a core module (ADR-008). We use open data, procedurally generated data, and community packs with clear provenance.

**Can I contribute without knowing how to program?**
Yes. The documentation needs feedback, corrections, and additions. If you understand football or game design, your input is useful.
@@ -280,7 +293,10 @@ Yes. The documentation needs feedback, corrections, and additions. If you unders
This is the documentation phase. Alpha arrives when L0 and L1 are stable and a minimal kernel runs. Indicative timeline: **Q1 2027**.

**Is any of this actually decided?**
Mostly, yes. The doctrine, the Three Planes, the Plane A enforcement strategy (ADR-003, option B), and the FFI panic strategy (ADR-004) are settled. **Three decisions remain open:** the Qt binding (ADR-005), the license (ADR-006), and the canonical language (ADR-007). Open decisions are tracked in [`docs/adr/`](docs/adr/).
Partly. The doctrine, the Three Planes, the Plane A enforcement strategy (ADR-003), the FFI panic strategy (ADR-004), and the provider integration policy (ADR-008) are settled. **Five decisions remain open:** the Qt binding (ADR-005), the license (ADR-006), the canonical language (ADR-007), the module system (ADR-012), and the world system (ADR-013).

**What is ADR-012 / ADR-013 about?**
ADR-012 defines the module contract (the "brick"): what a module is, what it declares, how conflicts are handled. ADR-013 defines the world system: how the bricks compose into worlds, the Level of Detail, the manifest, and **local determinism**. They are **proposed**. If ratified, they will supersede pillar 1 of the doctrine (determinism, from global to local). Read them together: [`012-013-reading-guide.md`](docs/adr/012-013-reading-guide.md).

**How much of this was written by AI?**
See [Built With AI Assistance](#-built-with-ai-assistance). Short answer: drafted by AI, reviewed by a human, and every technical claim should be verified before you depend on it.
@@ -300,16 +316,16 @@ There are no private email addresses, Telegram channels, or WhatsApp groups. Eve
## 📈 Roadmap

```
2026-Q4  ████████████░░░░░░░░░░░░  Documentation (L0 ratified, L1 next)
2027-Q1  ████░░░░░░░░░░░░░░░░░░░░  Kernel prototype (L0 code)
2026-Q4  ████████████░░░░░░░░░░░░  Documentation (L0 ratified + implemented, L1 next)
2027-Q1  ████░░░░░░░░░░░░░░░░░░░░  L1 kernel (season)
2027-Q2  ████████░░░░░░░░░░░░░░░░  Closed alpha (L0–L2 partial)
2027-Q3  ████████████░░░░░░░░░░░░  Public alpha (L0–L3 partial)
2027-Q4  ████████████████░░░░░░░░  Beta
2028-Q1  ████████████████████░░░░  Release candidate
2028-Q2  ████████████████████████  v1.0
```

**These dates are estimates.** Documentation has to be solid before code gets written. The L0 specification is now solid; the L1 specification is next.
**These dates are estimates.** Documentation has to be solid before code gets written. L0 is solid; L1 is next.

---
