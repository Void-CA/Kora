/// Kora API client configuration.
///
/// All HTTP calls use relative paths (`/api/v1/...`). In development,
/// Vite's dev-server proxies `/api/*` to the backend (see vite.config.ts):
///
///   :5173/api/*  →  localhost:8000/api/*
///
/// In production, a reverse proxy (nginx, etc.) handles the same mapping.

export const API_BASE = '/api/v1'
