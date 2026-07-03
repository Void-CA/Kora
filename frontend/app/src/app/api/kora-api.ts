// API client for Kora backend.
// One file, one responsibility: HTTP transport.
// No abstractions over fetch — when a third endpoint appears with
// different concerns (auth, retry, etc.), then we extract.

export type Priority = 'high' | 'medium' | 'low';
export type AttentionKind = 'delay' | 'budget' | 'weather' | 'info';
export type HealthStatus = 'ok' | 'attention' | 'critical' | 'info';
export type PhaseStatus = 'done' | 'current' | 'pending';

export interface StatusCounts {
  ok: number;
  attention: number;
  critical: number;
}

export interface NextAction {
  title: string;
  field: string;
  lot: string;
  crop: string;
  when: string;
  priority: Priority;
  reason: string;
  consequence: string;
}

export interface AttentionItem {
  kind: AttentionKind;
  text: string;
  metric: string;
}

export interface OperationToday {
  contextNote: string;
  status: StatusCounts;
  nextAction: NextAction;
  attention: AttentionItem[];
}

export interface FieldHealth {
  status: HealthStatus;
  label: string;
  value: string;
}

export interface CyclePhase {
  name: string;
  status: PhaseStatus;
  dayInPhase: number | null;
  expectedDurationDays: number | null;
}

export interface Field {
  id: string;
  name: string;
  hectares: number;
  lots: number;
  crop: string;
  growth: string;
  lastActivity: string;
  daysToHarvest: number;
  estimatedYieldTPerHa: number;
  historicalYieldTPerHa: number;
  health: FieldHealth[];
  phases: CyclePhase[];
}

const BASE = 'http://localhost:8000';

export async function getOperationToday(): Promise<OperationToday> {
  const r = await fetch(`${BASE}/api/operation/today`);
  if (!r.ok) throw new Error(`operation/today ${r.status}`);
  return r.json();
}

export async function getField(id: string): Promise<Field> {
  const r = await fetch(`${BASE}/api/fields/${id}`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`fields/${id} ${r.status}`);
  return r.json();
}
