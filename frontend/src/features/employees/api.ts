import { API_BASE } from '@/api'
import type { CreateEmployeeDto, CreateWorkLogDto, Employee, WorkLog } from './data/model'

const EMPLOYEES = `${API_BASE}/employees`
const WORK_LOGS = `${API_BASE}/work-logs`

export async function listEmployees(): Promise<Employee[]> {
  const res = await fetch(EMPLOYEES)
  if (!res.ok) throw new Error('Failed to fetch employees')
  return res.json()
}

export async function createEmployee(body: CreateEmployeeDto): Promise<Employee> {
  const res = await fetch(EMPLOYEES, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw new Error('Failed to create employee')
  return res.json()
}

export async function listWorkLogs(employeeId: string): Promise<WorkLog[]> {
  const res = await fetch(`${EMPLOYEES}/${employeeId}/work-logs`)
  if (!res.ok) throw new Error('Failed to fetch work logs')
  return res.json()
}

export async function createWorkLog(body: CreateWorkLogDto): Promise<WorkLog> {
  const res = await fetch(WORK_LOGS, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) throw new Error('Failed to create work log')
  return res.json()
}
