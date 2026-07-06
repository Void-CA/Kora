const BASE = 'http://localhost:8000';

export interface BudgetSummary { id: string; cycle_id: string; baseline: string; lines: { category: string; amount: string }[]; }
export interface ScheduleSummary { id: string; cycle_id: string; anchor: string; anchor_date: number; planned: { id: string; category: string; relative_day: number }[]; }
export interface RevenueSummary { id: string; cycle_id: string | null; amount: string; received_at: number; source: string; }

export async function createBudget(body: any): Promise<BudgetSummary> {
  const r = await fetch(`${BASE}/api/budgets`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`createBudget ${r.status}`);
  return r.json();
}
export async function getSchedule(cycleId: string): Promise<ScheduleSummary> {
  const r = await fetch(`${BASE}/api/schedules/cycle/${cycleId}`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`schedule ${r.status}`);
  return r.json();
}
export async function addPlannedActivity(body: any): Promise<any> {
  const r = await fetch(`${BASE}/api/schedules/activities`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`addPlannedActivity ${r.status}`);
  return r.json();
}
export async function listRevenueForCycle(cycleId: string): Promise<RevenueSummary[]> {
  const r = await fetch(`${BASE}/api/revenue/cycle/${cycleId}`);
  if (!r.ok) throw new Error(`revenue ${r.status}`);
  return r.json();
}
export async function registerRevenue(body: any): Promise<RevenueSummary> {
  const r = await fetch(`${BASE}/api/revenue`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`registerRevenue ${r.status}`);
  return r.json();
}
