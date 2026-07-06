const BASE = 'http://localhost:8000';

export interface SoilSummary { id: string; area_id: string; sampled_at: number; quality: string; cost: string; metric_count: number; }
export interface IncidenceSummary { id: string; cycle_id: string; kind: string; severity: string; description: string; action_taken: string; detected_at: number; resolved: boolean; economic_impact: string | null; }

export async function listSoilForArea(areaId: string): Promise<SoilSummary[]> {
  const r = await fetch(`${BASE}/api/soil/area/${areaId}`);
  if (!r.ok) throw new Error(`soil ${r.status}`);
  return r.json();
}
export async function registerSoil(body: any): Promise<SoilSummary> {
  const r = await fetch(`${BASE}/api/soil`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`registerSoil ${r.status}`);
  return r.json();
}
export async function linkSoilToCycle(body: any): Promise<any> {
  const r = await fetch(`${BASE}/api/soil/link`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`linkSoil ${r.status}`);
  return r.json();
}
export async function listIncidencesForCycle(cycleId: string): Promise<IncidenceSummary[]> {
  const r = await fetch(`${BASE}/api/incidence/cycle/${cycleId}`);
  if (!r.ok) throw new Error(`incidences ${r.status}`);
  return r.json();
}
export async function registerIncidence(body: any): Promise<IncidenceSummary> {
  const r = await fetch(`${BASE}/api/incidence`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
  if (!r.ok) throw new Error(`registerIncidence ${r.status}`);
  return r.json();
}
