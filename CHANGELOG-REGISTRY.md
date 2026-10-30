# Changelog — Registry

This file records every change to the **registry**: new entities, deprecations, replacements, migrations.

It is separate from [`CHANGELOG.md`](CHANGELOG.md) (which tracks documentation and structure) and from [`CHANGELOG-SIM.md`](CHANGELOG-SIM.md) (which tracks simulation behaviour).

**Rule:** the registry is **append-only**. An entity is never deleted. It is deprecated, and a replacement is indicated. Every deprecation and every migration is recorded here.

---

## Current registry state

```
Registry version: 1 (baseline, no entries yet)
Active entities:  0
Deprecated:       0
```

The registry module is planned at **L3** (`specs/L3-registry/`). No registry files exist yet. When the first registry file is written, this section is updated.

---

## When to write an entry

An entry is added to this file when **any** of the following is true:

1. **A new entity is introduced** — a role, a formation, an instruction, an attribute, a metric, or an index model is added to the registry.
2. **An entity is deprecated** — the entity is no longer recommended for new content, but remains callable for backward compatibility.
3. **An entity is replaced** — a deprecated entity gets a `replaced_by` pointer to a new entity.
4. **A migration script is added** — a script that transforms existing saves or presets from an old registry version to a new one.
5. **A pack extension changes the registry** — a pack adds new entities or overrides metadata (see `L3-packs` *(planned)*).

An entry is **not** written for:

- Cosmetic changes (typos in a `design_note`, reformatting).
- A change that does not affect the set of entities or their semantics.
- A pack that only consumes the registry without extending it.

---

## Deprecation rules

These rules come from `00-foundations` §5 ("No deleting entities from the registry") and from the registry module (`L3-registry` *(planned)*).

1. **An entity is never deleted.** It is marked `deprecated_in` with the version that deprecates it.
2. **A deprecated entity keeps working.** Old saves, old presets, old pack references must still resolve.
3. **A deprecated entity indicates its replacement.** The field `replaced_by` points to the new entity, or is `null` if there is no direct replacement.
4. **The UI hides deprecated entities by default.** The user can still see them, but they are not offered for new content.
5. **A deprecation requires a migration script** if existing content (saves, presets, pack references) needs to be transformed to use the replacement. The script is tested with a saved fixture.
6. **A deprecation is recorded here** before the migration is merged. No silent deprecations.

---

## Entry template

Every entry has the same shape. Copy this template and fill it in.

```markdown
## [Registry YYYY-MM-DD-N] — YYYY-MM-DD

### Type

One of: `introduced` / `deprecated` / `replaced` / `migration` / `pack-extension`.

### Entities affected

A table:

| Entity ID | Domain | Change | Replaced by | Reason |
|---|---|---|---|---|
| `cm-box-to-box` | roles | deprecated | `cm-mezzala` | Overlapping role, clearer distinction |
| `cm-mezzala` | roles | introduced | — | Replaces `cm-box-to-box` with a clearer identity |

### Migration script

- Path: `registry/migrations/YYYY-MM-DD-N.yaml` *(planned)*
- Tested against: (fixture name)
- Rollback available: (yes / no)

### Backward compatibility

- Existing saves load: (yes / yes with migration / no)
- Existing presets resolve: (yes / yes with migration / no)
- Pack extensions that reference the old entity: (unaffected / need update)

### Related changes

- `CHANGELOG.md` entry: (link or "none")
- `CHANGELOG-SIM.md` entry: (link or "none") — only if the change affects simulation behaviour
- ADRs affected: (list or "none")
```

---

## History

*(no entries yet — the registry has not been created)*

---

## Planned deprecations (pre-registry)

These are deprecations that have been decided in the documentation but not yet implemented, because the registry does not exist. They are recorded here so they are not lost when the registry is built.

| Entity ID | Domain | Status | Replaced by | Source |
|---|---|---|---|---|
| `lm-deep-lying` | roles | planned deprecation | `cm-playmaker` | Originally in the flat `02-registry.md` (removed); to be re-introduced in `L3-registry` |

When `L3-registry` is written, these planned deprecations are promoted to real entries in the "History" section, with a migration script and a date.

---

<!--
  NOTES FOR FUTURE EDITORS:

  - This file is a log, not a specification. The rules for the registry live
    in modules/L3-registry/ (planned). This file only records what changed.
  - Do not edit a past entry. If a past entry was wrong, add a correction
    entry and reference the one it corrects. The history is append-only.
  - A deprecation without a migration script is a bug. If a save breaks
    because of a deprecation, the migration was missing.
  - This file is linked from:
      - CHANGELOG.md (top of the file)
      - modules/L3-registry/00-overview.md (planned)
    If you move or rename this file, update those links.
-->
