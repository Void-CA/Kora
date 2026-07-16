# ADR-0002: Per-feature domain errors + unified API errors

**Date:** 2026-07-16

## Context

Domain errors are specific to each feature (e.g. `EmployeeError::EmptyName`).
HTTP errors are shared across the API layer.

## Decision

- Each feature defines its own error enum in `kora-domain`
- The API crate has a single `ApiError` enum that implements `IntoResponse`
- Use cases return domain errors; handlers map them to `ApiError`
- No global `DomainError` enum — that would couple unrelated features

## Consequences

- Feature errors are independent and can evolve separately
- API error responses are consistent across endpoints
- Adding a new feature never requires touching a global error type
