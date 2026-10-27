# Ideas — PCFM

**Purpose:** a curated backlog of ideas that are not yet decisions. Unlike `PROJECT-IDEA-LOG.md` (historical record), this file holds ideas in motion: sketches, open questions, trade-offs.

**Admission rule:** an idea enters only if (a) it has been discussed, (b) it is not already decided or placed elsewhere (ADR, module, `WORLD-MODEL.md`), (c) it is worth tracking. Rejected ideas are kept in a separate section, with motivation, to avoid re-discussing them.

**Promotion rule:** when an idea becomes a decision, mark it `promoted → ADR-NNN` (or `promoted → module`) and keep it here for traceability. Do not delete.

**Categories:** Player model, Match engine, Career (L2), World, UX / Presentation, Infrastructure.

---

## Open ideas

### IDEA-PM-001 — Dynamic attributes over a season
- **Status:** sketch
- **Category:** Player model (L2 career)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** attributes evolve over a season: physical decline with age, technical improvement possible, confidence modulates match performance (over/under-performance).
- **Open questions:** age curves; training and game-time effect; confidence source (wins? form? role?); determinism (Plane A vs B); interaction with ADR-016 psyche axes.
- **Critical note:** "losing improves technique" is a weak heuristic — it can create a perverse incentive (losing is good). Better: the loss is the context, professionalism (an ADR-016 axis) is the mechanism. Without professionalism, losing teaches nothing.

### IDEA-PM-002 — Multi-attribute success range per action
- **Status:** sketch
- **Category:** Player model (match resolution)
- **Source:** conversation 2026-10-27 (user, proposal 2)
- **Idea:** every action (shot, pass, duel) is resolved by a *combination* of attributes — technical, physical, psychological — that together define a **success range**. Example: a shot depends on shooting, composure, finishing, stamina, balance, concentration, aggression, teamwork. No single attribute decides the outcome.
- **Open questions:** the formula (weighted sum? thresholds? probability distribution?); which attributes per action; how the range is computed deterministically (integers / fixed-point); how it interacts with the zone of influence (ADR-019 D8) and with the two-phase events (D11).
- **Critical note:** the formula is deferred to `docs/calibration/`. Without calibration, the weights are arbitrary. The principle (multi-attribute, not single-attribute) is the decision; the numbers are not.

### IDEA-PM-003 — Confidence as a psychological axis or modulator
- **Status:** sketch
- **Category:** Player model (L2 psyche)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** ADR-016 has three active axes (professionalism, ambition, pressure). "Confidence" (fiducia) is not one of them. It could be a fourth axis, or a modulator that depends on form/wins and affects performance.
- **Open questions:** is confidence a stable axis (like professionalism) or a dynamic state (like form)? How does it interact with pressure? Does it belong to ADR-016 (content) or to the match engine (parameter)?
- **Critical note:** ADR-016 fixed three axes as a binding decision. Adding a fourth requires an amendment (or a new ADR). Do not treat this as a minor addition.

### IDEA-RULE-001 — Contract for rule modules
- **Status:** discussing
- **Category:** Match engine (L0 rules)
- **Source:** conversation 2026-10-27 (ADR-019 discussion)
- **Idea:** refereeing rules (offside, fouls, cards) are behaviour, extensible by modules (ADR-012). The core provides primitives (positions, ball state, temporal events); rule modules read them and produce outcomes. The contract for rule modules is deferred to a dedicated ADR.
- **Open questions:** what a rule module may change (thresholds? priorities? the state machine?); how rule modules interact (conflicts, priority); how they are recorded in the save manifest (ADR-013); performance cost of per-event rule evaluation.
- **Critical note:** "behaviour open" must not become "anything goes": a rule module that rewrites the engine would violate ADR-012. The contract must define what is open and what is closed.

### IDEA-INFRA-001 — Session-start tooling
- **Status:** sketch
- **Category:** Infrastructure (process)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** a small script (or `make` target) that prints `SESSION-STATE.md` + `git log --oneline -5` + `git status --short` in a single block, to paste at the start of a new session. Reduces the number of manual commands.
- **Open questions:** script language (Bash, no Python); where it lives (`tools/`); whether it is versioned; how it handles encoding on Windows (Git Bash).
- **Critical note:** the script is convenience, not a replacement for the checkpoint rule. The bookmark must be updated regardless.

### IDEA-INFRA-002 — Agent tooling for Git
- **Status:** discussing
- **Category:** Infrastructure (agent)
- **Source:** conversation 2026-10-27 (user)
- **Idea:** options for giving an agent (Warp, DeepSeek-TUI) controlled access to Git: MCP server (e.g. `mcp-server-git`), `gh` CLI wrapper, or DeepSeek-TUI with a write token. Phase 2 of `AGENT-POLICY.md`.
- **Open questions:** which option is mature enough; how to enforce the policy (branch, PR, no force-push) at the tool level; token scope and revocation; interaction with the ruleset on `main`.
- **Critical note:** no agent gets a write token until the ruleset on `main` is reactivated. The policy is already written (`AGENT-POLICY.md` §2–§4).

---

## Rejected ideas (for traceability)

### IDEA-REJ-001 — Drones over the pitch
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** they clutter the view of the pitch; no gameplay value.

### IDEA-REJ-002 — Sensor in the ball
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** unnecessary. The moment of contact is already the event timestamp. A sensor as a separate entity would be a complication without benefit.
