export interface HistoryView {
  campaigns: CampaignCardData[];
}
export interface CampaignCardData {
  id: string; crop: string; field: string; started: string; ended: string | null;
  status: 'active' | 'completed'; progress_percent: number;
  total_activities: number; completed_activities: number;
  budget: string; spent: string; revenue: string;
  profitability: string; health: 'ok' | 'attention' | 'critical';
}
