const BASE = 'http://localhost:8000';

export interface CycleSummary { id: string; crop_id: string; area_id: string; period_start: number; period_end: number; activity_count: number; }
export interface CycleDetail { summary: CycleSummary; activities: ActivitySummary[]; planned_activities: PlannedSummary[]; }
export interface ActivitySummary { id: string; category: string; timestamp: number; integrity: string[]; }
export interface PlannedSummary { id: string; category: string; relative_day: number; }
export interface Profitability { baseline: string; spent: string; revenue: string; profit: string; roi_percent: string; remaining: string; variance: string; }
export interface TimelineEvent { kind: string; timestamp: number; label: string; detail: string; integrity: string[]; }
export interface CycleTimeline { cycle_id: string; crop_id: string; area_id: string; period_start: number; period_end: number; planned: TimelineEvent[]; executed: TimelineEvent[]; expenses: TimelineEvent[]; revenues: TimelineEvent[]; payroll: TimelineEvent[]; incidences: TimelineEvent[]; }
export interface CycleVariance { cycle_id: string; matched: any[]; unplanned: any[]; missing: any[]; totals: any; }

export async function getCycles(): Promise<CycleSummary[]> {
  const r = await fetch(`${BASE}/api/cycles`);
  if (!r.ok) throw new Error(`cycles ${r.status}`);
  return r.json();
}
export async function getCycle(id: string): Promise<CycleDetail> {
  const r = await fetch(`${BASE}/api/cycles/${id}`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`cycles/${id} ${r.status}`);
  return r.json();
}
export async function getProfitability(id: string): Promise<Profitability> {
  const r = await fetch(`${BASE}/api/cycles/${id}/profitability`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`profitability ${r.status}`);
  return r.json();
}
export async function getCycleTimeline(id: string): Promise<CycleTimeline> {
  const r = await fetch(`${BASE}/api/cycles/${id}/timeline`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`timeline ${r.status}`);
  return r.json();
}
export async function getCycleVariance(id: string): Promise<CycleVariance> {
  const r = await fetch(`${BASE}/api/cycles/${id}/variance`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`variance ${r.status}`);
  return r.json();
}
export async function registerActivity(cycleId: string, body: any): Promise<any> {
  const r = await fetch(`${BASE}/api/cycles/${cycleId}/activities`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) { const msg = await r.text().catch(() => ''); throw new Error(`registerActivity ${r.status} ${msg}`); }
  return r.json();
}
