const BASE = 'http://localhost:8000';

export interface FieldHealth { status: string; label: string; value: string; }
export interface CyclePhase { name: string; status: string; day_in_phase: number | null; expected_duration_days: number | null; }
export interface Field { id: string; name: string; hectares: number; crop: string; cycle_id: string; growth: string; last_activity: string; days_to_harvest: number; health: FieldHealth[]; phases: CyclePhase[]; }

export async function getFields(): Promise<Field[]> {
  const r = await fetch(`${BASE}/api/fields`);
  if (!r.ok) throw new Error(`fields ${r.status}`);
  return r.json();
}
export async function getField(id: string): Promise<Field> {
  const r = await fetch(`${BASE}/api/fields/${id}`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`fields/${id} ${r.status}`);
  return r.json();
}
