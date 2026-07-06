export interface FieldsView {
  title: string;
  fields: FieldCardData[];
}
export interface FieldCardData {
  id: string; name: string; crop: string; hectares: number;
  progress_percent: number; days_to_harvest: number;
  days_since_last_activity: number; last_activity_name: string;
  responsible: string; cost_accumulated: string;
  health: 'ok' | 'attention' | 'critical';
  phases: { name: string; status: 'done' | 'current' | 'pending'; day: number; total: number }[];
}
