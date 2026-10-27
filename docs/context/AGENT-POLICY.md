# Agent policy — PCFM

**Purpose:** define what an agent (Warp, DeepSeek-TUI, or a future custom bot) may and may not do on this repository, and how its actions are approved. This is a **policy**, not a specification: it constrains behaviour, not architecture.

## 1. Principles

- The agent **proposes**, the human **approves**. No action on the remote repo happens without explicit confirmation.
- Every change goes through a **branch and a pull request**. No direct push to `main`.
- The agent **does not merge** pull requests. Only the human merges.
- The agent **does not force-push** `main`, **does not delete** `main`, **does not create tags** without explicit confirmation.
- The agent works on `agent/<task>-<date>` branches. These branches are intentionally **not protected**: they are the agent's workspace. `main` is protected by a GitHub ruleset (see §6).

## 2. What the agent may do autonomously

- Read any file in the repository.
- Run `cargo test`, `cargo clippy`, `cargo fmt --check`.
- Run `git status`, `git diff`, `git log`, `git branch`.
- Propose a diff or a patch in the chat.
- Create a local branch (`agent/<task>-<date>`).
- Commit on that branch (the local pre-commit hook enforces fmt, clippy, test on Rust changes).
- Push that branch to `origin` (the branch is not protected; `main` is).

## 3. What requires explicit confirmation

- Opening a pull request.
- Modifying any file under `docs/adr/`, `modules/`, `docs/context/`.
- Modifying `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`.
- Modifying `LICENSE*`, `.github/`, `.githooks/`.
- Any operation not listed in §2.

## 4. What the agent must never do

- `git push --force` on `main` (blocked by the ruleset, not just by policy).
- `git push` directly to `main` (blocked by the ruleset).
- Merge a pull request.
- `git tag`, `git reset --hard`, `git clean -fd` on the remote.
- Modify files outside the repository.
- Disable or bypass the ruleset on `main`.

## 5. How approval works

- The agent shows the exact command it wants to run, and the diff it will produce.
- The human reviews and confirms (or rejects).
- Only then the agent executes.
- If the agent is uncertain, it asks. It never guesses.

## 6. Defences in place

| Defence | Where | What it blocks |
|---|---|---|
| **GitHub ruleset on `main`** | Server-side (GitHub) | Direct push, force-push, deletion, merge without PR + 1 approval |
| **Pre-commit hook** (`.githooks/pre-commit`) | Local | Commit with Rust code that fails fmt, clippy, or test |
| **Branch naming** (`agent/<task>-<date>`) | Convention | Not a security control: traceability |
| **PR + merge by human** | Process | Unreviewed changes reaching `main` |

The ruleset is the **primary** defence. The hook is a local safety net. The branch naming is traceability, not security.

## 7. Revocation

If the agent misbehaves, or if its access must be withdrawn:

- **Revoke the token**: the agent's GitHub token is revoked from GitHub settings (Settings → Developer settings → Personal access tokens). The agent can no longer push anywhere.
- **Disable the ruleset bypass**: if a bypass actor was configured, remove it.
- **Delete `agent/*` branches** that contain unreviewed work.
- **Document the incident** in `docs/context/SESSION-STATE.md` (what happened, what was revoked).

No agent action is irreversible: `main` is protected, and `agent/*` branches can be deleted.

## 8. What this policy is not

- It is not a substitute for `CONTRIBUTING.md` (process for humans).
- It is not a specification of the agent's implementation.
- It is not a guarantee: a misconfigured agent can still do damage. The policy is a contract, not a sandbox.
