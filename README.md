# PC Football Manager

<div align="center">

**An open football management simulation built around transparency, privacy, and tactical depth.**

![Status](https://img.shields.io/badge/status-documentation_phase-yellow)
![Structure](https://img.shields.io/badge/structure-L0--L3-blue)
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

The full doctrine — the Three Planes, the ban on CA/PA, the determinism rules — lives in **[`modules/00-foundations.md`](modules/00-foundations.md)**. Read that first. Everything else is downstream of it.

---

## 🤖 Built With AI Assistance

This project's documentation is drafted with the help of a large language model, then reviewed and ratified by humans. Being upfront about this matters more than looking impressive.

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
L0 — MATCH     ████████████████  100%   specified (2 cells draft, blocked by ADR-003)
L1 — SEASON    ░░░░░░░░░░░░░░░░    0%   not started
L2 — CAREER    ░░░░░░░░░░░░░░░░    0%   not started
L3 — META      ░░░░░░░░░░░░░░░░    0%   not started

PHASE 2 — PROTOTYPE (Q1 2027)
PHASE 3 — ALPHA     (Q3 2027)
PHASE 4 — BETA      (Q1 2028)
PHASE 5 — v1.0      (Q2 2028)
```

**Right now:** the repository contains **documentation only**. No code exists. That's deliberate — we define *what* and *why* before writing *how*.

L0 (the match) is fully specified. L1 (the season) is the next milestone. Dates are indicative, not commitments.

---

## 📚 Documentation Map

The specification is **concentric**, not flat. Read in this order:

| Layer | File | What it answers |
|---|---|---|
| **1. Doctrine** | [`modules/00-foundations.md`](modules/00-foundations.md) | Why does this project exist? What are the non-negotiable rules? *(v1.1)* |
| **2. Structure** | [`STRUCTURE.md`](STRUCTURE.md) | How is the repo organised? How do I add or revise a module? |
| **3. Index** | [`modules/_index.md`](modules/_index.md) | What exists at each level? What is its status? |
| **4. Decisions** | [`docs/adr/`](docs/adr/) | Which architectural choices have been made, and why? |
| **5. Process** | [`CONTRIBUTING.md`](CONTRIBUTING.md) | How do I propose a change? What's the review process? |

### Levels

The specification is organised in four concentric levels. Dependencies flow **inward**: a level may depend on the levels below it, never above.

```
L0 — MATCH       Two teams play one match. Deterministic, savable.
L1 — SEASON      A league, a calendar, a world.
L2 — CAREER      Market, finance, youth, scouting, psyche, crowd, media, AI.
L3 — META        Registry, packs, generator, editor, assets, UI, statistics, quality.
```

Full map: [`modules/_index.md`](modules/_index.md).

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

Full doctrine: [`modules/00-foundations.md`](modules/00-foundations.md).

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

Details: [`modules/L0-match/01-rng.md`](modules/L0-match/01-rng.md).

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

### UI
| Technology | Status | Notes |
|------------|--------|-------|
| `cxx-qt` + QWidgets + QML | Chosen (ADR-005) | Only binding covering both QWidgets (data grids) and QML (skins) |

See [`docs/adr/005-qt-binding.md`](docs/adr/005-qt-binding.md).

---

## 📁 Repository Structure

See **[`STRUCTURE.md`](STRUCTURE.md)** for the full layout and the rules that govern it.

The layout below is the **target** structure. Items marked *(planned)* do not exist yet.

```
pc-football-manager/
├── README.md
├── STRUCTURE.md
├── CONTRIBUTING.md
├── LICENSE.md                 # *(planned)* dual license: MIT/Apache-2.0 + CC-BY-NC-SA-4.0
├── CHANGELOG.md               # *(planned)*
├── CHANGELOG-SIM.md           # *(planned)*
├── CHANGELOG-REGISTRY.md      # *(planned)*
├── modules/                   # specification, concentric L0–L3
├── archive/                   # *(planned)* historical prompt masters (v5, v6, v7)
├── docs/
│   ├── adr/                   # architecture decision records
│   ├── legal/                 # *(planned)* data source compliance
│   └── calibration/           # *(planned)* model calibration dashboards
├── registry/                  # *(planned)* empty until code exists
├── crates/                    # *(planned)* empty until code exists
├── mods/                      # *(planned)* empty until user packs exist
└── data/                      # *(planned)* examples only, never real licensed data
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

See **[`CONTRIBUTING.md`](CONTRIBUTING.md)** for the full process. Quick summary:

- One change per PR (one module, one ADR, one fix).
- Use the standard frontmatter (see `STRUCTURE.md` §4).
- Cite sources for factual claims.
- Explain the **WHY**, not just the what.
- Changing an invariant requires a new ADR.
- If you spot an error, say so plainly — including in AI-drafted text.

---

## 🙋 FAQ

**Is this a Football Manager clone?**
No. It's an alternative with a different philosophy. FM is proprietary, uses closed datasets, and ships telemetry. PCFM is community-driven, transparent, and offline-first.

**Will you use Wyscout / Opta data?**
No. Bundling licensed datasets is ruled out by design — see the planned Ingest module (L2). We use open data, procedurally generated data, and community packs with clear provenance.

**Can I contribute without knowing how to program?**
Yes. The documentation needs feedback, corrections, and additions. If you understand football or game design, your input is useful.

**When is the alpha?**
This is the documentation phase. Alpha arrives when L0 and L1 are stable and a minimal kernel runs. Indicative timeline: **Q1 2027**.

**Is any of this actually decided?**
Partly. Open decisions are tracked in [`docs/adr/`](docs/adr/). Four remain unresolved: the Plane A enforcement strategy (ADR-003), the Qt binding (ADR-005), the license (ADR-006), and the canonical language (ADR-007). The FFI panic strategy was settled in ADR-004.

**How much of this was written by AI?**
See [Built With AI Assistance](#-built-with-ai-assistance). Short answer: drafted by AI, reviewed by a human, and every technical claim should be verified before you depend on it.

---

## 📬 Contact

- **Discussions:** GitHub Discussions
- **Bugs / Features:** GitHub Issues
- **Community chat:** *to be decided*

There are no private email addresses, Telegram channels, or WhatsApp groups. Everything happens publicly on GitHub.

---

## 📈 Roadmap

```
2026-Q4  ████████████░░░░░░░░░░░░  Documentation (L0 + L1)
2027-Q1  ████░░░░░░░░░░░░░░░░░░░░  Kernel prototype (L0)
2027-Q2  ████████░░░░░░░░░░░░░░░░  Closed alpha (L0–L2 partial)
2027-Q3  ████████████░░░░░░░░░░░░  Public alpha (L0–L3 partial)
2027-Q4  ████████████████░░░░░░░░  Beta
2028-Q1  ████████████████████░░░░  Release candidate
2028-Q2  ████████████████████████  v1.0
```

**These dates are estimates.** Documentation has to be solid before code gets written.

---

## 📄 License

**Proposed: dual license** (see [`docs/adr/006-license.md`](docs/adr/006-license.md)).

- **Code** (`crates/`, executables): MIT OR Apache-2.0.
- **Assets and documentation** (`mods/`, `data/`, `assets/`, `locales/`, `docs/`, `modules/`): CC-BY-NC-SA-4.0.
- **Trademark:** the name "PC Football Manager" and the logo are trademarks of the project team. The licenses do not grant trademark rights.

The `LICENSE.md` file will be created when ADR-006 is ratified.

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
- Mark undecided things as undecided. Link to the ADR.
- Cite sources for factual claims.
- One module change = one PR.
- The AI disclosure section stays. If it stops being true, update it — do not delete it.
- The doctrine lives in modules/00-foundations.md. Do not duplicate it here; link to it.
-->
