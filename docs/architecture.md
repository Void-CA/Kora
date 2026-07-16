# Kora Architecture Principles

## 1. Features first

The project is organized by product capabilities, not by technical layers.
A feature contains everything it needs: entities, use cases, repository traits,
handlers, DTOs, and frontend components.

```text
backend/kora-domain/features/employees/
backend/kora-api/features/employees/
frontend/src/app/features/employees/
```

## 2. Common is extracted, never designed

Nothing starts in `common/`. Types and functions live in the feature that
creates them. They move to `common/` only when extracted from real,
demonstrated duplication across two or more features.

Rule: every extraction must eliminate existing duplication, never prevent
future duplication.

## 3. The product defines the architecture

The project structure should tell the product story. Running `tree backend/`
should list the capabilities of Kora: employees, payroll, territory,
operations, inventory — not layers like handlers, entities, repositories.

## 4. Every PR leaves the system runnable

No PR is merged unless:
- `cargo test` passes
- `ng test` passes
- The application boots

Incomplete features are guarded behind routes, never by broken code.

## 5. A feature is complete only when it works end-to-end

A feature includes backend domain logic, API endpoints, database migrations,
and frontend components. Partial implementations are valid PRs as long as
rule 4 holds.

## 6. Prefer explicitness over flexibility

Newtypes for IDs, explicit `&dyn Trait` in use cases, per-feature error
types. Generics and abstractions are introduced when repeating the same
pattern a third time, not the first.

## 7. Grow the model only when reality requires it

Don't design the second problem before solving the first. Start with the
simplest model that handles the current use case. Expand when a real
need emerges, never from speculative requirements.
