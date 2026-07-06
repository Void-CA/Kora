const BASE = 'http://localhost:8000';

export interface WorkerSummary { id: string; name: string; role: string | null; active: boolean; }
export interface PayrollEntrySummary { id: string; worker_id: string; amount: string; paid_at: number; cycle_id: string | null; area_id: string | null; }

export async function listWorkers(): Promise<WorkerSummary[]> {
  const r = await fetch(`${BASE}/api/payroll/workers`);
  if (!r.ok) throw new Error(`workers ${r.status}`);
  return r.json();
}
export async function registerWorker(body: any): Promise<WorkerSummary> {
  const r = await fetch(`${BASE}/api/payroll/workers`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`registerWorker ${r.status}`);
  return r.json();
}
export async function listPayrollForCycle(cycleId: string): Promise<PayrollEntrySummary[]> {
  const r = await fetch(`${BASE}/api/payroll/cycle/${cycleId}`);
  if (!r.ok) throw new Error(`payroll ${r.status}`);
  return r.json();
}
export async function recordPayroll(body: any): Promise<PayrollEntrySummary> {
  const r = await fetch(`${BASE}/api/payroll`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`recordPayroll ${r.status}`);
  return r.json();
}
