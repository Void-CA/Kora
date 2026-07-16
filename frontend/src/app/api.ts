/// Kora API client configuration.
///
/// The frontend uses relative paths (`/api/v1/...`). In development,
/// Angular's dev-server proxies `/api/*` to the backend (see proxy.conf.json):
///
///   :4200/api/*  →  localhost:8000/api/*
///
/// In production, a reverse proxy (nginx, etc.) handles the same mapping.
///
/// Change this single constant if the API path prefix ever changes.

export const API_BASE = '/api/v1';
