export interface HomeFieldPreview {
  id: string; name: string; crop: string; hectares: number;
  progress_percent: number; health: string; days_to_harvest: number;
  last_activity: string; next_activity: string;
}
export interface TeamPreview {
  total: number; working_today: number; missing_yesterday: number;
  entries: { name: string; status: string }[];
}
export interface FinancePreview {
  total_budget: string; budget_used_percent: number; total_spent: string;
  alerts: { cycle: string; text: string }[];
}
export interface AlertItem { kind: string; text: string; field: string | null; severity: string; }
export interface WeatherInfo { forecast: string; rain_expected: boolean; temp: string; humidity: string; }

export interface HomeView {
  greeting: string; date: string;
  today: { pending: number; critical: number; completed: number; next_action: NextActionData | null; };
  fields: HomeFieldPreview[]; team: TeamPreview; finances: FinancePreview;
  alerts: AlertItem[]; weather: WeatherInfo | null;
}
export interface NextActionData { title: string; field: string; crop: string; when: string; priority: string; }
