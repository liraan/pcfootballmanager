# ADR-016 — v1.0 Content Selections

- **Status:** proposed
- **Date:** 2026-10-26
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** `L1-world/01-nation.md`, `L2-tactics`, `L2-psyche`, `L3-registry`, `L3-packs`, the v1.0 content freeze
- **Depends on:** ADR-015 (accepted), ADR-012 (accepted), ADR-013 (accepted), ADR-007 (accepted)

## Context

ADR-015 ratified the principle of reduced v1.0 scope and deferred seven content decisions to this ADR (ADR-015 §Deferred decisions). This ADR resolves them.

Two facts shape the decision:

1. **Most of the target content does not exist yet.** `L2-tactics`, `L2-psyche`, `L3-registry` are planned but unwritten. The role taxonomy (56 roles), the psychological axis set (6 axes), and the nation data model do not have a specification file. Therefore, for D1, D2, D3 and D4 this ADR is not a *selection from an existing catalogue*: it is a **forward commitment** that constrains the future module specs. The selection is binding: a future module that wants to add a 21st role must go through a new ADR (or an amendment to this one).

2. **`data/` is empty.** ADR-015 D4 ("the 5 nations with the most complete open data") cannot be resolved from the repository, because there is no data in it. The selection must be based on a **criterion**, not on an inventory. This ADR proposes a criterion and flags the choice as an open question for ratification.

## Decision drivers

1. **Shippability over completeness** (ADR-015 driver 1).
2. **Local determinism** (`00-foundations.md` §4, ADR-013).
3. **No invented real data presented as certain** (`00-foundations.md` §5).
4. **No unlicensed assets** (`00-foundations.md` §5).
5. **Playability**: v1.0 must be a football management game, not a tech demo. A missing core module (e.g. tactics) is worse than a missing secondary one (e.g. media).
6. **Doctrine precedent**: 3 axes in v1.0 (`00-foundations.md` §7).
7. **Modularity** (ADR-012): every post-v1.0 addition is a pack.

## The seven deferred decisions

### D1 — Which 3 psychological axes are active in v1.0?

**Candidate set** (from ADR-015 D1): professionalism, ambition, loyalty, pressure, consistency, temperament.

**Options:**

- **A. Professionalism, ambition, pressure** (ADR-015's proposal).
- **B. Professionalism, ambition, consistency** — replaces pressure with consistency (performance variance).
- **C. Professionalism, loyalty, pressure** — replaces ambition with loyalty (retention and renewals).
- **D. All six** — rejected, contradicts ADR-015.

**Trade-offs:**

- *Professionalism* is non-negotiable: it drives training, tactical discipline, reaction to setbacks. Without it, the career loop has no friction.
- *Ambition* drives transfer requests, motivation, and growth. Without it, the market is inert. Central to `L2-market`.
- *Pressure* drives performance in high-stakes matches (penalties, finals, relegation six-pointers). It is the axis with the most **visible** effect in the match engine (Plane B). It makes the match more readable.
- *Consistency* is variance of performance. It is important but overlaps with pressure in the match engine: both modulate the same observable (performance under different conditions). Keeping both would make the effect hard to attribute ("did I lose because of pressure or inconsistency?"), violating pillar 5 (radical transparency).
- *Loyalty* is retention. It matters for `L2-market` and `L2-finance`, but is secondary to ambition.
- *Temperament* overlaps with pressure (cards, reactions) and is less distinctive.

**Recommendation:** **Option A (professionalism, ambition, pressure).**

**Implementation requirement (gap A, from `PROJECT-IDEA-LOG.md` §4.4):** the data model has six fields. The three inactive axes (loyalty, consistency, temperament) MUST carry an explicit `active: false` flag. This is a requirement for `L2-psyche`, not for this ADR's ratification. Without the flag, the model implies six active axes, contradicting ADR-015.

### D2 — Which 20 roles are in v1.0?

**Fact:** the 56-role taxonomy does not exist as a file. `L2-tactics` is planned. Therefore this ADR **creates** the v1.0 role set; it does not select from a catalogue.

**Criterion:** cover every position on the pitch with at least one basic role, plus one or two specialist variants that exercise the engine's differentiating mechanics (width, depth, pressing, finishing style).

**Proposed set (20 roles):**

| # | Role | Code | Family | Rationale |
|---|------|------|--------|-----------|
| 1 | Goalkeeper | GK | GK | Mandatory |
| 2 | Sweeper Keeper | SK | GK | Exercises the "high line" mechanic (L0 already has a `sweeper` duel) |
| 3 | Centre-Back | CB | DEF | Mandatory |
| 4 | Ball-Playing Defender | BPD | DEF | Exercises the "build from the back" mechanic |
| 5 | Full-Back | FB | DEF | Mandatory |
| 6 | Wing-Back | WB | DEF | Exercises the "width" mechanic |
| 7 | Defensive Midfielder | DM | MID | Mandatory |
| 8 | Ball-Winning Midfielder | BWM | MID | Exercises the "press" mechanic |
| 9 | Central Midfielder | CM | MID | Mandatory |
| 10 | Box-to-Box Midfielder | B2B | MID | Exercises the "transition" mechanic |
| 11 | Advanced Playmaker | AP | MID | Exercises the "creativity" mechanic |
| 12 | Attacking Midfielder | AM | MID | Mandatory |
| 13 | Mezzala | MEZ | MID | Exercises the "half-space" mechanic |
| 14 | Winger | W | ATT | Mandatory |
| 15 | Inside Forward | IF | ATT | Exercises the "cut inside" mechanic |
| 16 | Striker | ST | ATT | Mandatory |
| 17 | Centre-Forward | CF | ATT | Mandatory (link-up variant) |
| 18 | Advanced Forward | AF | ATT | Exercises the "run in behind" mechanic |
| 19 | Deep-Lying Forward | DLF | ATT | Exercises the "drop deep" mechanic |
| 20 | Target Man | TM | ATT | Exercises the "aerial" mechanic |

**Trade-offs:**

- The set is **FM-flavoured**. An alternative would be a smaller, more abstract set (e.g. 8 roles), but ADR-015 fixes 20. Within 20, FM-flavoured roles maximise recognisability for the target audience.
- The set is **not final**: `L2-tactics` will refine the definitions. This ADR fixes the **names and codes**; the **mechanical definitions** are deferred to `L2-tactics`.

**Recommendation:** **the table above.** It is a forward commitment: adding a 21st role requires a new ADR (or an amendment).

**Open question for ratification:** the set is my proposal. If the maintainer has a different 20-role list in mind (e.g. one that matches a specific FM version's role set), this is the moment to replace it. Once ratified, it is binding for `L2-tactics`.

### D3 — Which 10 formations are in v1.0?

**Candidate set** (from ADR-015 D3): 4-4-2, 4-3-3, 4-2-3-1, 3-5-2, 3-4-3, 4-4-1-1, 4-1-4-1, 5-3-2, 4-3-1-2, 4-2-2-2.

**Criterion:** cover the main shapes (back four / back three / back five), the main midfield shapes (flat / diamond / double pivot), and the main attacking shapes (one striker / two strikers / three forwards). Every formation must be expressible with the 20 roles of D2.

**Check against D2:**

- 4-4-2: GK, 2 CB, 2 FB, 2 CM, 2 W, 2 ST → OK (uses CB, FB, CM, W, ST).
- 4-3-3: GK, 2 CB, 2 FB, 1 DM, 2 CM, 2 W, 1 ST → OK.
- 4-2-3-1: GK, 2 CB, 2 FB, 2 DM, 1 AM, 2 W, 1 ST → OK (uses DM).
- 3-5-2: GK, 3 CB, 2 WB, 2 CM, 1 AM, 2 ST → OK (uses WB, AM).
- 3-4-3: GK, 3 CB, 2 WB, 2 CM, 2 W, 1 ST → OK.
- 4-4-1-1: GK, 2 CB, 2 FB, 2 CM, 2 W, 1 AM, 1 ST → OK.
- 4-1-4-1: GK, 2 CB, 2 FB, 1 DM, 2 CM, 2 W, 1 ST → OK.
- 5-3-2: GK, 3 CB, 2 WB, 2 CM, 1 AM, 2 ST → OK (uses WB).
- 4-3-1-2: GK, 2 CB, 2 FB, 2 CM, 1 AM, 2 ST → OK.
- 4-2-2-2: GK, 2 CB, 2 FB, 2 DM, 2 AM, 2 ST → OK.

All ten are expressible with the 20 roles. **Recommendation: confirm the set.**

**Trade-off:** the set is Eurocentric and FM-flavoured. It excludes e.g. 4-3-2-1 (Christmas tree) and 4-2-4. The excluded ten are deferred to a pack, per ADR-015.

### D4 — Which 5 nations are in v1.0?

**Fact:** `data/` is empty. ADR-015 D4 says "the 5 nations with the most complete open data (to be determined from `data/` or ADR-008's provider policy)". This cannot be resolved from the repository.

**Options:**

- **A. Five real nations with generated data.** England, Italy, Spain, Germany, France. Data (clubs, players, cities, stadiums) is generated from a deterministic seed. Names are clearly fictional ("London FC", not "Chelsea"). Complies with `00-foundations.md` §5 ("no invented real data presented as certain": the data is not presented as certain, it is presented as generated).
- **B. Five real nations with real open data.** Requires a legal review of what data is actually open and licensable. `data/` is empty; this is not resolvable today.
- **C. Five fictional nations with generated data.** Maximum freedom, zero legal risk, loses the "soul of PC Calcio".
- **D. No nations, single fictional world.** Rejected: contradicts the spirit of L1-world (nations, cities, stadiums).

**Trade-offs:**

- Option A is the only one that is both **playable** and **legally safe** today. It uses real nation names (which are facts, not assets) and fictional clubs/players (which are generated).
- Option B is the "ideal" long-term answer, but it is blocked on a legal review that does not exist. Choosing B without the review would be a commitment to a data source that has not been verified, violating `00-foundations.md` §5.
- Option C is safe but flavourless.
- The **choice of which five nations** in Option A is a **flavour** decision, not an architectural one. England, Italy, Spain, Germany, France are the five "top" European leagues, which maximises recognisability for the target audience.

**Recommendation:** **Option A, with England, Italy, Spain, Germany, France.** Flag the choice as a flavour decision that can be changed with an amendment (the architecture does not depend on which five nations are chosen).

**Open question for ratification:** the maintainer may prefer a different five (e.g. include Brazil or Argentina for geographic variety). The architecture supports any five; only the generated content changes.

### D5 — Which L2 modules are in v1.0?

**Fact:** `_index.md` lists nine L2 modules: tactics, market, finance, youth, scouting, psyche, crowd, media, ai.

**ADR-015's proposal:** market, finance, youth (basic), scouting, psyche (3 axes), crowd (basic), media (basic), AI (basic) — **eight modules**. **It omits `tactics`.**

**This omission is a problem.** Tactics is not a "nice to have": without tactics, the player cannot set up the team, and the match engine has no way to differentiate the 20 roles of D2. `L2-tactics` is the module that makes D2 and D3 *do* something.

**Options:**

- **A. All nine L2 modules, with reduced scope per module.** `tactics` (full), `market` (full), `finance` (full), `youth` (basic), `scouting` (full), `psyche` (3 axes), `crowd` (basic), `media` (basic), `ai` (basic). This is consistent with "basic X ships, advanced X does not" (ADR-015 "What is explicitly out of v1.0"). The "subset" in ADR-015 then refers to **functionality**, not to **modules**.
- **B. Eight L2 modules, with `tactics` added and `media` removed.** `tactics`, `market`, `finance`, `youth` (basic), `scouting`, `psyche` (3 axes), `crowd` (basic), `ai` (basic). Media basic is deferred to a pack. This diverges from ADR-015 ("basic media ships"), but keeps the count at eight and fixes the tactics omission.
- **C. ADR-015's eight, without tactics.** Rejected: unplayable. The match engine has no tactical input.

**Recommendation:** **Option A.** All nine modules, reduced functionality per module. This is consistent with ADR-015's "basic X ships, advanced X does not" and fixes the tactics omission. The "L2 subset" in ADR-015 is interpreted as "subset of functionality", not "subset of modules".

**If the maintainer prefers Option B:** the ADR must explicitly say that `L2-media` is deferred, contradicting ADR-015's "basic media ships". This would require an amendment to ADR-015. Not recommended.

**Scope per module (Option A):**

| Module | v1.0 scope | Deferred to pack |
|--------|-----------|-------------------|
| `L2-tactics` | Full: 10 formations, 20 roles, basic instructions | Advanced instructions, opposition instructions |
| `L2-market` | Full: transfers, loans, contracts | Agent networks, release clauses |
| `L2-finance` | Full: budgets, wages, FFP-lite | Sponsorship deals, infrastructure |
| `L2-youth` | Basic: intake, basic development | Full academy economy, youth leagues |
| `L2-scouting` | Full: reports, bias, confidence bands | Scouting networks, live scouting |
| `L2-psyche` | 3 active axes (D1) | The other 3 axes |
| `L2-crowd` | Basic: attendance, atmosphere | Chants, tifos, ultras groups |
| `L2-media` | Basic: press conferences, news | Weekly show, pundits |
| `L2-ai` | Basic: squad selection, basic transfers | Long-term planning, adaptive AI |

### D6 — Which L3 modules are in v1.0?

**Fact:** `_index.md` lists eight L3 modules: registry, packs, generator, editor, assets, ui, statistics, quality.

**ADR-015's proposal:** registry, packs (contract only), generator (basic), editor (basic), assets, UI (functional), statistics (full), quality — **eight modules**, all present, with reduced scope for packs, generator, editor, UI.

**Recommendation:** **confirm ADR-015's proposal.** It is coherent with `_index.md` and with the "basic X ships, advanced X does not" pattern.

**Scope per module:**

| Module | v1.0 scope | Deferred to pack / post-v1.0 |
|--------|-----------|-------------------------------|
| `L3-registry` | Full: entity registry, deprecation | — |
| `L3-packs` | Contract only (ADR-012) | Marketplace, dependency resolution |
| `L3-generator` | Basic: deterministic world generation | Advanced scenario generation |
| `L3-editor` | Basic: edit entities | Visual editor, scripting |
| `L3-assets` | Full: asset loading, licences | — |
| `L3-ui` | Functional: data grids, no skins | Skins, themes, animations |
| `L3-statistics` | Full: query engine over Plane B | — |
| `L3-quality` | Full: validation, gates | — |

### D7 — Dependency on ADR-012 / ADR-013 (resolved)

ADR-012 and ADR-013 are **accepted** (2026-10-25). The "packs" mechanism exists. The "in a pack" answer is safe. **No action needed.**

## Decision

**Summary table:**

| # | Decision | Outcome |
|---|----------|---------|
| D1 | 3 active psychological axes | professionalism, ambition, pressure. The other 3 carry `active: false` (gap A). |
| D2 | 20 roles | The 20-role table in D2. Forward commitment: binding on `L2-tactics`. |
| D3 | 10 formations | 4-4-2, 4-3-3, 4-2-3-1, 3-5-2, 3-4-3, 4-4-1-1, 4-1-4-1, 5-3-2, 4-3-1-2, 4-2-2-2. |
| D4 | 5 nations | England, Italy, Spain, Germany, France, with generated data. Flavour decision, changeable by amendment. |
| D5 | L2 modules in v1.0 | All nine, with reduced functionality per module. **Diverges from ADR-015's eight (which omitted `tactics`).** |
| D6 | L3 modules in v1.0 | All eight, with reduced functionality per module (as in ADR-015). |
| D7 | Dependency on ADR-012/013 | Resolved (both accepted). |

## Consequences

### Easier

- The content freeze for v1.0 has a concrete list. `L2-tactics` and `L2-psyche` can be written against a known target.
- The role set (D2) and formation set (D3) are a stable contract for the match engine: a role code in an event refers to a known role.
- The five-nation choice (D4) is a flavour decision that can be revisited without architectural impact.

### Harder

- D2 and D4 are **forward commitments** made before the modules that will consume them exist. If `L2-tactics` later discovers that a role is mechanically indistinguishable from another, the 20-role set cannot be changed without an amendment. This is the price of committing early; it is accepted because the alternative (no commit) blocks `L2-tactics` from starting.
- D5 diverges from ADR-015 (adds `tactics`, keeps all nine). If the maintainer prefers ADR-015's eight, an amendment to ADR-015 is required. This ADR recommends Option A (all nine) and flags the divergence explicitly.

### Locked in

- The 20 roles, 10 formations, 3 axes, and 5 nations are binding. Changing them requires a new ADR (or an amendment to this one).
- The role codes (GK, CB, ...) are stable identifiers in Plane B events. A role code cannot be renamed without a `SIM_VERSION` bump (it is part of the event schema).

## Open questions (to be resolved before ratification)

1. **D2 (roles):** is the proposed 20-role set acceptable, or does the maintainer have a different list? Once ratified, it is binding on `L2-tactics`.
2. **D4 (nations):** is the "generated data" approach acceptable? Are the five chosen nations the right flavour? An alternative (e.g. Brazil instead of France) is supported by the architecture.
3. **D5 (L2):** confirm Option A (all nine modules) over Option B (eight, `media` deferred) and over ADR-015's eight (no `tactics`).

Until these three are resolved, this ADR is `proposed`, not `accepted`.

## References

- ADR-015 — v1.0 scope (accepted; the seven deferred decisions resolved here)
- ADR-012 — Plugins: The Brick Contract (accepted)
- ADR-013 — Worlds: LOD, Manifest, and Local Determinism (accepted)
- ADR-007 — Canonical language (English; this ADR is in English)
- `modules/00-foundations.md` §5 (banned things), §7 (non-goals)
- `modules/_index.md` — the L1/L2/L3 module inventory
- `docs/PROJECT-IDEA-LOG.md` §4.4 — the three technical gaps (gap A is resolved in D1)
