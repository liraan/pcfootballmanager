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

### IDEA-INFRA-004 — `pcfm-debug` covers installed modules and plugins
- **Status:** discussing
- **Category:** Infrastructure (testing / tooling)
- **Source:** conversation 2026-10-27 (user, next-session goal)
- **Idea:** extend `pcfm-debug` so that `--check` / `--list` / `--coverage`
  also account for scenarios and features contributed by installed modules
  and plugins, not only the 6 built-in L0 scenarios.
- **What exists today (verified in code, 2026-10-27):**
  - `pcfm-testkit::Feature` is `#[non_exhaustive]` and already has a
    `Plugin(String)` variant alongside the 26 built-in features
    (`crates/pcfm-testkit/src/feature.rs:13,87,118`).
  - `TestRegistry::register()` accepts any `impl Scenario`
    (`crates/pcfm-testkit/src/registry.rs:26`); the contract is open.
  - `pcfm-debug` hardcodes the discovery path: `check.rs:47-49`,
    `commands.rs:16-17`, `commands.rs:49-50` all call
    `features::register_all(&mut registry)`, and `features/mod.rs:15`
    documents that this is "the only entry point the rest of the binary
    uses".
  - `--coverage` denominator is `Feature::all_builtin()` only
    (`commands.rs:54`); `Plugin(_)` features are never counted.
- **What is missing (the actual gap):**
  1. A discovery step for installed plugin scenarios ("what is installed?").
  2. A way to feed them into the same `TestRegistry` that `pcfm-debug` builds.
  3. A report format that distinguishes built-in from plugin scenarios
     (`Feature::Plugin(_)` exists but is not surfaced).
  4. A decision on the coverage denominator: built-in only, or union.
- **Ambiguity to resolve before designing:** "cover installed plugins" can
  mean (a) run plugin-contributed test scenarios alongside L0's, or (b)
  verify that a plugin is loaded and functional (smoke test of the plugin
  system itself). They are different features. Pick one.
- **Constraints from existing decisions:**
  - ADR-012 §1: the plugin sheet is mandatory and validated. Discovery
    should read the sheet, not scan arbitrary directories.
  - ADR-013 §3: the manifest is part of the save and validated. The save is
    a natural discovery source, but only for an in-progress save; a fresh
    workspace has none.
  - "`pcfm-testkit` has no `pcfm-*` dependencies" (codebase rule). Any
    metadata type added to the testkit must respect this. `Feature::Plugin(String)`
    is already a minimal representation that does.
  - The plugin system itself is not implemented yet (no loader, no
    `pcfm-pack`). This idea can be **designed** now but **implemented** only
    after ADR-012's runtime contract exists.
- **Open questions:**
  1. Discovery: `sheet.toml` scan, save manifest, or explicit list passed to
     `pcfm-debug`? Each has different determinism and UX implications.
  2. Aggregation in the report: per-plugin rows, per-feature rows, or both?
     What is the stable machine-readable format?
  3. Coverage denominator: built-in only, union, or two separate lines?
  4. Does `Feature::Plugin(String)` suffice, or does the testkit need a
     plugin metadata struct (name + version + feature)?
  5. Does `--check` fail (non-zero exit) when a plugin scenario fails, or
     does it warn?
- **Critical note:** before designing, read (a) the 6 scenarios in
  `crates/pcfm-debug/src/features/l0_match.rs` to see what a good `Scenario`
  looks like, (b) `docs/adr/012-plugins.md` §1–§3 for the sheet contract,
  (c) `docs/adr/013-worlds.md` §3 for the manifest. `REPO-SNAPSHOT.md` §6.2
  is not sufficient.

### IDEA-INFRA-005 — Separate diagnostics and benchmark from the test runner
- **Status:** discussing
- **Category:** Infrastructure (tooling / observability)
- **Source:** conversation 2026-10-28 (user)
- **Idea:** today `pcfm-debug` is effectively a **test runner** (`--check`,
  `--list`, `--coverage`, `--seed`). The name suggests a debugger, the
  function is scenario execution. Two distinct directions:
  1. **Diagnostics** (log / trace of startup and runtime). Name must
     describe the function; candidates: `pcfm-log`, `pcfm-trace`,
     `pcfm-diagnostics` (plural, coherent with `pcfm-stats`). Final name
     decided in the ADR, after local analysis.
  2. **Benchmark** (performance measurement on user hardware, target
     `00-foundations.md` §6: 60 fps UI, season < 60 s). **Deferred** — the
     targets are not measurable today (no UI, no L1). Revisit when the
     targets become measurable.
- **What is missing before any code:** a local reading of
  `crates/pcfm-debug/src/main.rs` and `crates/pcfm-testkit/src/lib.rs` to
  establish (a) what `pcfm-debug` actually does today beyond `--check`,
  `--list`, `--coverage`, (b) what the `Scenario` trait exposes, (c) the
  real cost of renaming.
- **Relationship with IDEA-INFRA-004:** that idea is about the **runner**
  (covering installed plugins and scenarios). It stays valid regardless of
  the runner's crate name; if `pcfm-debug` is renamed, IDEA-INFRA-004
  follows the new name.
- **Open questions:**
  1. Does the diagnostics crate share the `Scenario` contract with the
     runner, or does it need a different one?
  2. Does `pcfm-debug` become a wrapper CLI with subcommands (`run`,
     `list`, `coverage`, `replay`, `inspect`), or do we split into two
     crates now?
  3. Hooks: how many, for which concrete use case? (Discussion 2026-10-28:
     minimal, no generic hooks "for the future".)
- **Critical note:** an ADR is required before any crate is created or
  renamed, because the change touches the doctrine (dependencies,
  determinism, `pcfm-testkit` as contract). The doctrine wins. No code
  until the ADR is accepted.

---

## Rejected ideas (for traceability)

### IDEA-REJ-001 — Drones over the pitch
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** they clutter the view of the pitch; no gameplay value.

### IDEA-REJ-002 — Sensor in the ball
- **Source:** conversation 2026-10-27 (user, then discarded)
- **Why rejected:** unnecessary. The moment of contact is already the event timestamp. A sensor as a separate entity would be a complication without benefit.
