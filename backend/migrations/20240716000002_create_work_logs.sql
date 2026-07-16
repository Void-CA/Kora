CREATE TABLE work_logs (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id),
    worked_on   DATE NOT NULL,
    hours       DOUBLE PRECISION NOT NULL
);
