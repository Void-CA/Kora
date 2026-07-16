CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE employees (
    id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name    TEXT NOT NULL,
    active  BOOLEAN NOT NULL DEFAULT TRUE
);
