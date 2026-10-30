# Context directory — agent workspace

This directory is an **agent workspace**, not human documentation. The documentation for collaborators is `PROJECT-STATUS.md` at the repo root (planned).

## Rule

The repository itself remains the source of truth. The files in `docs/context/` are session memory, snapshots, policies, and review notes. Before treating any item here as live fact, validate it against the current repo state (`git status`, `git log`, `git ls-files`, `cargo test`).
Review notes under `analysis/` are evaluative material, not source-of-truth repository state: they are not the project specification and not a substitute for the live code and live git state.

## Files in this directory

**Entry points (read in this order):**

1. `SESSION-STATE.md` — current session bookmark: last commit, next steps, files touched. Read this first when resuming after a pause.
2. `NEW-SESSION-PROMPT.md` — prompt for the chat agent. Contains the operating rules and the codebase-specific rules.
3. `AGENT-POLICY.md` — policy for any agent (chat or GitHub). What it may do, what requires confirmation, what it must never do, how approval works, how revocation works.

**GitHub agent entry point:**

- `GITHUB-AGENT-PROMPT.md` — prompt for the GitHub agent (analysis reports, not code changes).

**Supporting material (not normative):**

- `REPO-SNAPSHOT.md` — factual snapshot of the repository (crate-by-crate). May lag behind; treat as a briefing.
- `PROJECT-IDEA-LOG.md` — history, decisions, sources, what remains open.
- `IDEAS.md` — open and rejected ideas. Do not implement from it.

**Subfolders:**

- `analysis/` — review outputs and AI handoff assessments. Reviews are dated files (`YYYY-MM-DD-HH-MM-review.md`); the latest date is the current review. Written by the GitHub agent, never by the chat agent.

## Two agents, two prompts, one policy

- **Chat agent** (interactive): reads `NEW-SESSION-PROMPT.md`, follows the operating rules, edits files, commits, and pushes on approval.
- **GitHub agent** (asynchronous): reads `GITHUB-AGENT-PROMPT.md`, produces one Markdown report in `analysis/`, does not modify code. Role and limits are in `AGENT-POLICY.md` §8.
- **Both** follow the same `AGENT-POLICY.md`. The two prompts cite it; they do not duplicate it.

## Workflow

1. The GitHub agent produces `analysis/YYYY-MM-DD-HH-MM-review.md` (findings table: ID, severity, file:line, issue, recommendation).
2. The chat agent triages: for each finding, propose **apply** (with commit + verification), **reject** (with reason), or **backlog** (add to `IDEAS.md`).
3. The maintainer approves or rejects. Nothing is applied because a report says so.
4. The maintainer may authorize the GitHub agent to apply a scoped modification as an exception (see `AGENT-POLICY.md` §8). For synchronous emergencies, use the chat agent or the maintainer directly.

## Read order for a new chat

1. `SESSION-STATE.md`
2. `NEW-SESSION-PROMPT.md`
3. `AGENT-POLICY.md`
4. Only then the normative docs (`STRUCTURE.md`, `docs/ARCHITECTURE.md`, `docs/CODE_TOUR.md`, `specs/...`).

## In short

`docs/context/` is a continuity layer for agents. It helps with context retention, not with defining the project's authoritative state.
