# Context directory — canonical layout

This directory is organised for AI handoff and continuity, not as normative project documentation.

## Rule

The repository itself remains the source of truth. The files in `docs/context/` are session memory, snapshots, policies, and review notes. Before treating any item here as live fact, validate it against the current repo state (`git status`, `git log`, `git ls-files`, and the code itself).

## Logical structure

The directory is intentionally grouped by purpose:

- `session/` — current session bookmark and task state
- `briefing/` — repo snapshots, launch prompts, startup context
- `policy/` — agent policy and operational constraints
- `backlog/` — idea logs and pending or rejected directions
- `analysis/` — review findings and AI handoff notes

## Canonical files

- `session/README.md` — current session backbone
- `briefing/README.md` — repo snapshot and prompt bootstrap
- `policy/README.md` — agent and write policy
- `backlog/README.md` — ideas and non-finalized directions
- `analysis/README.md` — review notes and assessments

## Backward compatibility

The original files at the top level remain available for compatibility and contain the full session data. The subfolders are the organized canonical view for an AI resuming work.

## Read order for a new chat

1. `session/README.md`
2. `briefing/README.md`
3. `new-session prompt` or startup brief in `briefing/`
4. only then the normative docs (`STRUCTURE.md`, `docs/ARCHITECTURE.md`, `docs/CODE_TOUR.md`, `modules/...`)

## In short

`docs/context/` is a continuity layer. It helps with context retention, not with defining the project’s authoritative state.
