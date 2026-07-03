// API client for Kora backend.
// Wire format is snake_case (Rust serde default).
// One file, one responsibility: HTTP transport.

const BASE = 'http://localhost:8000';

// ── /api/health ──

export interface Health {
  status: string;
}

export async function getHealth(): Promise<Health> {
  const r = await fetch(`${BASE}/api/health`);
  if (!r.ok) throw new Error(`health ${r.status}`);
  return r.json();
}

// ── /api/operation/today ──

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
  priority: 'high' | 'medium' | 'low';
  reason: string;
  consequence: string;
}

export interface AttentionItem {
  kind: 'delay' | 'budget' | 'info';
  text: string;
  metric: string;
}

export interface OperationToday {
  context_note: string;
  status: StatusCounts;
  next_action: NextAction;
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
  day_in_phase: number | null;
  expected_duration_days: number | null;
}

export interface Field {
  id: string;
  name: string;
  hectares: number;
  crop: string;
  cycle_id: string;
  growth: string;
  last_activity: string;
  days_to_harvest: number;
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
  crop_id: string;
  area_id: string;
  period_start: number;
  period_end: number;
  activity_count: number;
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
  relative_day: number;
}

export interface CycleDetail {
  summary: CycleSummary;
  activities: ActivitySummary[];
  planned_activities: PlannedSummary[];
}

export interface Profitability {
  baseline: string;
  spent: string;
  revenue: string;
  profit: string;
  roi_percent: string;
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

// ── GET /api/cycles/:id/timeline ──

export interface TimelineEvent {
  kind: string;
  timestamp: number;
  label: string;
  detail: string;
  integrity: string[];
}

export interface CycleTimeline {
  cycle_id: string;
  crop_id: string;
  area_id: string;
  period_start: number;
  period_end: number;
  planned: TimelineEvent[];
  executed: TimelineEvent[];
  expenses: TimelineEvent[];
  revenues: TimelineEvent[];
  payroll: TimelineEvent[];
  incidences: TimelineEvent[];
}

export async function getCycleTimeline(id: string): Promise<CycleTimeline> {
  const r = await fetch(`${BASE}/api/cycles/${id}/timeline`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`timeline ${r.status}`);
  return r.json();
}

// ── GET /api/cycles/:id/variance ──

export interface TimingVariance {
  kind: string;
  days: number;
}

export interface CostVariance {
  planned: string;
  actual: string;
  variance: string;
}

export interface MatchedActivity {
  planned_id: string;
  activity_id: string;
  category: string;
  expected_timestamp: number;
  actual_timestamp: number;
  variance: TimingVariance;
  confidence: string;
  cost: CostVariance | null;
}

export interface UnplannedActivity {
  activity_id: string;
  category: string;
  timestamp: number;
  reason: string;
}

export interface MissingPlanned {
  planned_id: string;
  category: string;
  relative_day: number;
  expected_timestamp: number;
}

export interface VarianceTotals {
  matched_count: number;
  unplanned_count: number;
  missing_count: number;
  total_planned: string | null;
  total_actual: string | null;
  total_cost_variance: string | null;
}

export interface CycleVariance {
  cycle_id: string;
  matched: MatchedActivity[];
  unplanned: UnplannedActivity[];
  missing: MissingPlanned[];
  totals: VarianceTotals;
}

export async function getCycleVariance(id: string): Promise<CycleVariance> {
  const r = await fetch(`${BASE}/api/cycles/${id}/variance`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`variance ${r.status}`);
  return r.json();
}

// ── POST /api/cycles/:id/activities ──

export interface PlannedSuggestion {
  planned_id: string;
  category: string;
  relative_day: number;
  expected_timestamp: number;
  drift_days: number;
}

export interface RegisterActivityResponse {
  activity_id: string;
  category: string;
  timestamp: number;
  integrity: string[];
  suggestions: PlannedSuggestion[];
}

export async function registerActivity(
  cycleId: string,
  body: {
    cycle_id: string;
    timestamp: number;
    category: string;
    notes?: string;
    mode: 'suggested' | 'emergent' | 'confirm_match';
    match_against?: string;
  },
): Promise<RegisterActivityResponse> {
  const r = await fetch(`${BASE}/api/cycles/${cycleId}/activities`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`registerActivity ${r.status} ${msg}`);
  }
  return r.json();
}

// ── /api/areas/:id/history ──

export interface BudgetSummary {
  id: string;
  cycle_id: string;
  baseline: string;
  spent: string;
  remaining: string;
  variance: string;
}

export interface FieldHistory {
  area_id: string;
  area_name: string;
  cycles: string[];
  schedules: string[];
  budgets: BudgetSummary[];
}

export async function getFieldHistory(id: string): Promise<FieldHistory> {
  const r = await fetch(`${BASE}/api/areas/${id}/history`);
  if (!r.ok) throw new Error(`areas/${id}/history ${r.status}`);
  return r.json();
}

// ── GET /api/areas/:id/dashboard ──

export interface DashboardCycleSummary {
  id: string;
  crop_id: string;
  period_start: number;
  period_end: number;
  activity_count: number;
  budget_baseline: string;
  budget_spent: string;
  budget_variance: string;
}

export interface DashboardSoilSummary {
  id: string;
  sampled_at: number;
  quality: string;
  metric_count: number;
  cost: string;
}

export interface DashboardIncidenceSummary {
  id: string;
  cycle_id: string;
  kind: string;
  severity: string;
  description: string;
  detected_at: number;
  economic_impact: string | null;
}

export interface AreaTotals {
  total_baseline: string;
  total_spent: string;
  total_variance: string;
  cycle_count: number;
  incidence_count: number;
}

export interface AreaDashboard {
  area_id: string;
  area_name: string;
  cycles: DashboardCycleSummary[];
  soil_analyses: DashboardSoilSummary[];
  incidences: DashboardIncidenceSummary[];
  totals: AreaTotals;
}

export async function getAreaDashboard(id: string): Promise<AreaDashboard> {
  const r = await fetch(`${BASE}/api/areas/${id}/dashboard`);
  if (!r.ok) throw new Error(`areas/${id}/dashboard ${r.status}`);
  return r.json();
}

// ── /api/soil ──

export interface SoilSummary {
  id: string;
  area_id: string;
  sampled_at: number;
  quality: string;
  cost: string;
  metric_count: number;
}

export async function listSoilForArea(areaId: string): Promise<SoilSummary[]> {
  const r = await fetch(`${BASE}/api/soil/area/${areaId}`);
  if (!r.ok) throw new Error(`soil/area/${areaId} ${r.status}`);
  return r.json();
}

export async function registerSoil(body: {
  area_id: string;
  sampled_at: number;
  quality: string;
  cost_amount: string;
  cost_currency: string;
  metrics: { kind: string; value: string }[];
}): Promise<SoilSummary> {
  const r = await fetch(`${BASE}/api/soil`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`registerSoil ${r.status} ${msg}`);
  }
  return r.json();
}

export async function linkSoilToCycle(body: {
  analysis_id: string;
  cycle_id: string;
  kind: string;
}): Promise<{ analysis_id: string; cycle_id: string; kind: string }> {
  const r = await fetch(`${BASE}/api/soil/link`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`linkSoil ${r.status} ${msg}`);
  }
  return r.json();
}

// ── /api/payroll ──

export interface WorkerSummary {
  id: string;
  name: string;
  role: string | null;
  active: boolean;
}

export interface PayrollEntrySummary {
  id: string;
  worker_id: string;
  amount: string;
  paid_at: number;
  cycle_id: string | null;
  area_id: string | null;
}

export async function listWorkers(): Promise<WorkerSummary[]> {
  const r = await fetch(`${BASE}/api/payroll/workers`);
  if (!r.ok) throw new Error(`payroll/workers ${r.status}`);
  return r.json();
}

export async function registerWorker(body: {
  name: string;
  role?: string;
}): Promise<WorkerSummary> {
  const r = await fetch(`${BASE}/api/payroll/workers`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`registerWorker ${r.status} ${msg}`);
  }
  return r.json();
}

export async function listPayrollForCycle(cycleId: string): Promise<PayrollEntrySummary[]> {
  const r = await fetch(`${BASE}/api/payroll/cycle/${cycleId}`);
  if (!r.ok) throw new Error(`payroll/cycle/${cycleId} ${r.status}`);
  return r.json();
}

export async function recordPayroll(body: {
  worker_id: string;
  amount: string;
  currency: string;
  paid_at: number;
  cycle_id?: string;
  area_id?: string;
}): Promise<PayrollEntrySummary> {
  const r = await fetch(`${BASE}/api/payroll`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`recordPayroll ${r.status} ${msg}`);
  }
  return r.json();
}

// ── /api/incidence ──

export interface IncidenceSummary {
  id: string;
  cycle_id: string;
  kind: string;
  severity: string;
  description: string;
  action_taken: string;
  detected_at: number;
  resolved: boolean;
  economic_impact: string | null;
}

export async function listIncidencesForCycle(cycleId: string): Promise<IncidenceSummary[]> {
  const r = await fetch(`${BASE}/api/incidence/cycle/${cycleId}`);
  if (!r.ok) throw new Error(`incidence/cycle/${cycleId} ${r.status}`);
  return r.json();
}

export async function registerIncidence(body: {
  cycle_id: string;
  kind: string;
  severity: string;
  description: string;
  action_taken: string;
  detected_at: number;
  economic_impact_amount?: string;
  economic_impact_currency?: string;
}): Promise<IncidenceSummary> {
  const r = await fetch(`${BASE}/api/incidence`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`registerIncidence ${r.status} ${msg}`);
  }
  return r.json();
}

// ── /api/revenue ──

export interface RevenueSummary {
  id: string;
  cycle_id: string | null;
  amount: string;
  received_at: number;
  source: string;
}

export async function listRevenueForCycle(cycleId: string): Promise<RevenueSummary[]> {
  const r = await fetch(`${BASE}/api/revenue/cycle/${cycleId}`);
  if (!r.ok) throw new Error(`revenue/cycle/${cycleId} ${r.status}`);
  return r.json();
}

export async function registerRevenue(body: {
  cycle_id?: string;
  amount: string;
  currency: string;
  received_at: number;
  source: string;
}): Promise<RevenueSummary> {
  const r = await fetch(`${BASE}/api/revenue`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`registerRevenue ${r.status} ${msg}`);
  }
  return r.json();
}

// ── /api/budgets ──

export interface BudgetLineSummary {
  category: string;
  amount: string;
}

export interface PlanningBudgetSummary {
  id: string;
  cycle_id: string;
  baseline: string;
  lines: BudgetLineSummary[];
}

export async function createBudget(body: {
  cycle_id: string;
  period_start: number;
  period_end: number;
  baseline_amount: string;
  baseline_currency: string;
  estimated_lines: { category: string; amount: string; currency: string }[];
}): Promise<PlanningBudgetSummary> {
  const r = await fetch(`${BASE}/api/budgets`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`createBudget ${r.status} ${msg}`);
  }
  return r.json();
}

// ── /api/schedules ──

export interface PlannedActivitySummary {
  id: string;
  category: string;
  relative_day: number;
}

export interface ScheduleSummary {
  id: string;
  cycle_id: string;
  anchor: string;
  anchor_date: number;
  planned: PlannedActivitySummary[];
}

export async function getSchedule(cycleId: string): Promise<ScheduleSummary> {
  const r = await fetch(`${BASE}/api/schedules/cycle/${cycleId}`);
  if (r.status === 404) throw new Error('not_found');
  if (!r.ok) throw new Error(`schedule/cycle/${cycleId} ${r.status}`);
  return r.json();
}

export async function addPlannedActivity(body: {
  cycle_id: string;
  category: string;
  relative_day: number;
}): Promise<{ id: string }> {
  const r = await fetch(`${BASE}/api/schedules/activities`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
  if (!r.ok) {
    const msg = await r.text().catch(() => '');
    throw new Error(`addPlannedActivity ${r.status} ${msg}`);
  }
  return r.json();
}
