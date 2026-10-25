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
