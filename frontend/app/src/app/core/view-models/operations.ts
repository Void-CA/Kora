export interface OperationsView {
  date: string;
  pending: ActivityCardData[];
  in_progress: ActivityCardData[];
  completed: ActivityCardData[];
}
export interface ActivityCardData {
  id: string; title: string; field: string; crop: string;
  scheduled_time: string; status: 'pending' | 'in_progress' | 'completed' | 'delayed';
  responsible: string | null; notes: string;
}
