# AGENTS.md

## Purpose

This repository is developed with the assistance of AI coding agents.

Before making changes, read:

- `docs/SPECIFICATION.md` for product and functional requirements.
- `docs/ARCHITECTURE.md` for technical and architectural decisions.

Do not duplicate or reinterpret those documents here. They are the sources of truth.

## Working Rules

- Follow the specifications and architecture as written.
- Do not invent requirements, business rules, or architectural decisions.
- Do not remove existing functionality unless explicitly requested.
- Do not change unrelated behavior while implementing a task.
- Do not implement hypothetical or illustrative features unless they are part of the specifications.
- Prefer focused, reviewable changes over broad rewrites.
- Preserve existing behavior unless the requested change explicitly modifies it.
- Reuse existing abstractions and patterns before introducing new ones.
- Use idiomatic Rust and leverage the type system whenever practical.
- Prefer separation of concerns and expose simple interfaces that compose those concerns when needed.
- Add or update tests for changed behavior when appropriate.
- Run the relevant formatting, linting, compilation and tests before considering work complete.
- Add database migrations for schema changes. Never modify historical migrations to represent a new change.
- Never silently perform destructive operations or data loss.
- Never commit secrets, credentials, tokens, passwords, or deployment-specific sensitive values.

## Scope

When working on a task:

1. Read the relevant specification.
2. Inspect the existing implementation before modifying it.
3. Identify the smallest coherent set of changes.
4. Implement the requested behavior.
5. Test the affected behavior.
6. Update documentation if the implementation changes documented behavior or architecture.

Do not opportunistically refactor unrelated code.

A larger refactor is acceptable when it is genuinely required for the requested change, but its necessity should be made explicit.

## Ambiguity and Conflicts

Do not make arbitrary decisions when an ambiguity has meaningful consequences for:

- product behavior;
- data models;
- architecture;
- module boundaries;
- public APIs;
- UX;
- security;
- destructive operations.

Ask for clarification instead.

For minor implementation details that do not affect those areas, use reasonable engineering judgment and stay consistent with the existing codebase.

If `docs/SPECIFICATION.md`, `docs/ARCHITECTURE.md`, the existing implementation, and the current task conflict, surface the conflict instead of silently choosing one.

The current explicit task takes precedence when it intentionally changes an existing requirement or architectural decision.

## Documentation

Do not change `docs/SPECIFICATION.md` merely to make it match an implementation shortcut.

Update `docs/ARCHITECTURE.md` when an architectural decision is intentionally changed.

If implementation reveals that either document is outdated or inconsistent, report it.

## Completion

Before finishing a task:

- ensure the requested behavior is implemented;
- ensure relevant tests pass;
- ensure formatting and linting pass;
- ensure no unrelated functionality was changed;
- report any remaining limitation, uncertainty, or follow-up work.

Never present incomplete or knowingly failing work as complete.
