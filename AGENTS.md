# AGENTS.md

Guidance for coding agents working in this repository. **This file is
self-contained** — everything needed to work here lives in this repo (this file,
`CLAUDE.md`, and the README); it depends on no external or workspace-level file.

## Role

Learning-oriented Rust tutorials. Keep tutorial prose and its runnable examples in
sync.

## How work arrives

**If your task is a slice brief**, it is self-contained — work from it, and stop to
ask if you hit a gap. Most work here is ad-hoc; follow the rules below. (Reviewers
may read any plan or spec.)

## Build & workflow

```bash
cargo run --example <tutorial_name>
```

The repo pins `centaur_technical_indicators` to a specific version; update that
dependency **deliberately** if tutorials should track newer core behavior.

## Working rules

- Smallest safe diff; no opportunistic refactors or "while I was here" changes — note
  unrelated issues separately.
- Preserve existing structure and naming unless the task requires otherwise.
- Keep tutorial prose and its runnable example in sync.
- Stage only the files your task names; never `git add .` / `git add -A`.

## Downstream & cross-repo impact

Tutorials track the published core library; keep examples aligned with current
behavior. Don't make cross-repo or architectural decisions from inside this repo;
surface them to the maintainer.

## Stop-and-report

**Never guess or assume.** If information is missing, the task is ambiguous, or two
implementations are plausible, **stop and ask for input** before proceeding — don't
pick one and run. Beyond that, stop and report (don't work around) when an example
fails to build for reasons outside your change, or instructions conflict with the
repo's actual state.

## Commit conventions

Branch off the latest `origin/main` (never commit on `main`); conventional-commit
subject under 72 chars. End every agent-authored commit with:

    Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>
