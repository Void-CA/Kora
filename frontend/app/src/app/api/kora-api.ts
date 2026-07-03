// API client for Kora backend.
// One file, one responsibility: HTTP transport.

const BASE = 'http://localhost:8000';

// ── /api/operation/today ──

export type Priority = 'high' | 'medium' | 'low';
export type AttentionKind = 'delay' | 'budget' | 'info';

export interface StatusCounts {
  ok: number;
  attention: number;
  critical: number;
}

export interface NextAction {
  title: string;
  field: string;
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

export async function getOperationToday(): Promise<OperationToday> {
  const r = await fetch(`${BASE}/api/operation/today`);
  if (!r.ok) throw new Error(`operation/today ${r.status}`);
  return r.json();
}

// ── /api/fields ──

export type HealthStatus = 'ok' | 'attention' | 'critical' | 'info';
export type PhaseStatus = 'done' | 'current' | 'pending';

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
  crop: string;
  cycleId: string;
  growth: string;
  lastActivity: string;
  daysToHarvest: number;
  health: FieldHealth[];
  phases: CyclePhase[];
}

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

// ── /api/cycles ──

export interface CycleSummary {
  id: string;
  cropId: string;
  areaId: string;
  periodStart: number;
  periodEnd: number;
  activityCount: number;
}

export interface ActivitySummary {
  id: string;
  category: string;
  timestamp: number;
  integrity: string[];
}

export interface PlannedSummary {
  id: string;
  category: string;
  relativeDay: number;
}

export interface CycleDetail {
  summary: CycleSummary;
  activities: ActivitySummary[];
  plannedActivities: PlannedSummary[];
}

export interface Profitability {
  baseline: string;
  spent: string;
  remaining: string;
  variance: string;
}

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

// ── /api/areas/:id/history ──

export interface BudgetSummary {
  id: string;
  cycleId: string;
  baseline: string;
  spent: string;
  remaining: string;
  variance: string;
}

export interface FieldHistory {
  areaId: string;
  areaName: string;
  cycles: string[];
  schedules: string[];
  budgets: BudgetSummary[];
}

export async function getFieldHistory(id: string): Promise<FieldHistory> {
  const r = await fetch(`${BASE}/api/areas/${id}/history`);
  if (!r.ok) throw new Error(`areas/${id}/history ${r.status}`);
  return r.json();
}
