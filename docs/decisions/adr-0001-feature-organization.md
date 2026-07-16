# ADR-0001: Feature-based code organization

**Date:** 2026-07-16

## Context

Kora was previously organized by technical layers (handlers/, entities/,
repositories/). This made it hard to understand what the system does by
looking at the project tree, and required touching multiple directories
for a single feature change.

## Decision

Organize code by product features (employees, payroll, territory, etc.).
Each feature is a self-contained module in both backend and frontend.

- `kora-domain/features/<feature>/` — entities, use cases, repository traits, errors
- `kora-api/features/<feature>/` — Axum handlers, DTOs, SQLx repository impl
- `frontend/src/app/features/<feature>/` — Angular components, services

Cross-cutting concerns (`common/`) start empty and grow only by extraction
from real duplication.

## Consequences

- Adding a new feature means creating one directory in each layer
- Understanding a feature means opening one directory
- Common abstractions emerge organically instead of being designed upfront
