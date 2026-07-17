/// Domain model for the Employees feature.
///
/// DTOs are defined alongside the model because they represent
/// the same concept — they differ only in serialization shape.
/// If they ever diverge significantly, extract to `data/dto.ts`.

// ─── Model ──────────────────────────────────────────────────────────

export interface Employee {
  id: string
  name: string
  active: boolean
}

export interface WorkLog {
  id: string
  employee_id: string
  worked_on: string
  hours: number
}

// ─── DTOs (create / update payloads) ────────────────────────────────

export interface CreateEmployeeDto {
  name: string
}

export interface CreateWorkLogDto {
  employee_id: string
  worked_on: string
  hours: number
}
