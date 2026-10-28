# Context directory — usage and status

This directory holds session memory and project briefings for PCFM.

## Status

These files are intentionally not normative documentation. They are working notes for continuity between chats and session handoffs.

The real source of truth for the repository is:
- the current git state (`git status`, `git log`, `git branch`)
- the current files in the repo
- the active code and tests in the workspace

## Purpose

`docs/context/` exists to help a new chat or a new agent resume work without losing the recent session context. It is useful for:
- current task state
- recent architecture decisions
- recent session constraints
- repo snapshot summaries
- agent operating policy

## Important rule

Before treating any file in this directory as current fact, validate it against the live repo.

The following are not authoritative unless they match the current repository state:
- metrics and counts
- last commit values
- status summaries
- “working tree clean” notes
- “all files read” claims
- coverage or test totals

## Recommended reading order

For a resumed session:
1. `SESSION-STATE.md`
2. `REPO-SNAPSHOT.md`
3. `NEW-SESSION-PROMPT.md`
4. only then the normative docs (`STRUCTURE.md`, `docs/ARCHITECTURE.md`, `docs/CODE_TOUR.md`, `modules/...`)

## File roles

- `SESSION-STATE.md` — short bookmark for the current session
- `REPO-SNAPSHOT.md` — historical snapshot of the repo structure and counts
- `NEW-SESSION-PROMPT.md` — bootstrap prompt for a resumed chat
- `AGENT-POLICY.md` — operating policy for agents and write permissions
- `REVIEW-FINDINGS.md` — review notes, not source-of-truth repo status
- `PROJECT-IDEA-LOG.md` and `IDEAS.md` — backlog/context, not current requirements

## Rule for future edits

When editing anything in this directory:
- write dates and commit references only as historical references
- do not present a snapshot as the current truth unless it has been freshly revalidated
- prefer short, specific, action-oriented notes over long narrative summaries
- keep the distinction between “historical context” and “live source of truth” explicit

## Final principle

`docs/context/` is a convenience layer, not a replacement for the real repository.
