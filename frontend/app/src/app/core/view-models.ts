// Kora domain read-models (projections).
// These represent WHAT the user sees, not what the database stores.

export interface HomeView {
  greeting: string;
  date: string;
  today: {
    pending: number;
    critical: number;
    completed: number;
    nextAction: {
      title: string;
      field: string;
      crop: string;
      when: string;
      priority: 'high' | 'medium' | 'low';
    } | null;
  };
  fields: HomeFieldPreview[];
  team: TeamPreview;
  finances: FinancePreview;
  alerts: AlertItem[];
  weather: WeatherInfo | null;
}

export interface HomeFieldPreview {
  id: string;
  name: string;
  crop: string;
  hectares: number;
  progressPercent: number;
  health: 'ok' | 'attention' | 'critical';
  daysToHarvest: number;
  lastActivity: string;
  nextActivity: string;
}

export interface TeamPreview {
  total: number;
  workingToday: number;
  missingYesterday: number;
  entries: { name: string; status: 'working' | 'missing' | 'completed' }[];
}

export interface FinancePreview {
  totalBudget: string;
  budgetUsedPercent: number;
  totalSpent: string;
  alerts: { cycle: string; text: string }[];
}

export interface AlertItem {
  kind: 'risk' | 'delay' | 'budget' | 'team' | 'weather';
  text: string;
  field?: string;
  severity: 'low' | 'medium' | 'high';
}

export interface WeatherInfo {
  forecast: string;
  rainExpected: boolean;
  temp: string;
  humidity: string;
}

export interface FieldsView {
  title: string;
  fields: FieldCardData[];
}

export interface FieldCardData {
  id: string;
  name: string;
  crop: string;
  hectares: number;
  progressPercent: number;
  daysToHarvest: number;
  daysSinceLastActivity: number;
  lastActivityName: string;
  responsible: string;
  costAccumulated: string;
  health: 'ok' | 'attention' | 'critical';
  phases: {
    name: string;
    status: 'done' | 'current' | 'pending';
    day: number;
    total: number;
  }[];
}

export interface OperationsView {
  date: string;
  pending: ActivityCardData[];
  inProgress: ActivityCardData[];
  completed: ActivityCardData[];
}

export interface ActivityCardData {
  id: string;
  title: string;
  field: string;
  crop: string;
  scheduledTime: string;
  status: 'pending' | 'in_progress' | 'completed' | 'delayed';
  responsible: string | null;
  notes: string;
}

export interface TeamView {
  title: string;
  today: {
    working: number;
    absent: number;
    total: number;
  };
  workers: TeamWorkerData[];
  recentPayments: {
    worker: string;
    amount: string;
    date: string;
    cycle: string;
  }[];
}

export interface TeamWorkerData {
  id: string;
  name: string;
  role: string;
  status: 'working' | 'absent' | 'off';
  todayActivity: string | null;
  lastPayment: string | null;
}

export interface FinancesView {
  totalBudget: string;
  totalSpent: string;
  totalRevenue: string;
  profit: string;
  roi: string;
  cycles: FinanceCycleRow[];
}

export interface FinanceCycleRow {
  cycleName: string;
  field: string;
  budget: string;
  spent: string;
  revenue: string;
  profit: string;
  status: 'ok' | 'attention' | 'critical';
}

export interface HistoryView {
  campaigns: CampaignCardData[];
}

export interface CampaignCardData {
  id: string;
  crop: string;
  field: string;
  started: string;
  ended: string | null;
  status: 'active' | 'completed';
  progressPercent: number;
  totalActivities: number;
  completedActivities: number;
  budget: string;
  spent: string;
  revenue: string;
  profitability: string;
  health: 'ok' | 'attention' | 'critical';
}
