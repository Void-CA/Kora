export interface FinancesView {
  total_budget: string; total_spent: string; total_revenue: string;
  profit: string; roi: string;
  cycles: FinanceCycleRow[];
}
export interface FinanceCycleRow {
  cycle_name: string; field: string; budget: string; spent: string;
  revenue: string; profit: string; status: 'ok' | 'attention' | 'critical';
}
