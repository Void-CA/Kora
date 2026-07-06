export interface TeamView {
  title: string;
  today: { working: number; absent: number; total: number };
  workers: TeamWorkerData[];
  recent_payments: { worker: string; amount: string; date: string; cycle: string }[];
}
export interface TeamWorkerData {
  id: string; name: string; role: string;
  status: 'working' | 'absent' | 'off';
  today_activity: string | null; last_payment: string | null;
}
