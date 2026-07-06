import type {
  FieldsView, OperationsView, TeamView, FinancesView, HistoryView,
} from './view-models';

const USD = (n: number) => `$${n.toLocaleString('es')}`;

export const MOCK_HOME = null;

export const MOCK_FIELDS: FieldsView = {
  title: 'Campos',
  fields: [
    {
      id: 'campo-norte', name: 'Campo Norte', crop: 'Maíz', hectares: 12,
      progress_percent: 73, days_to_harvest: 23, days_since_last_activity: 2,
      last_activity_name: 'Riego por goteo', responsible: 'Juan Pérez',
      cost_accumulated: USD(10250), health: 'attention' as const,
      phases: [
        { name: 'Preparación', status: 'done' as const, day: 20, total: 20 },
        { name: 'Siembra', status: 'done' as const, day: 5, total: 5 },
        { name: 'Crecimiento', status: 'current' as const, day: 32, total: 45 },
        { name: 'Floración', status: 'pending' as const, day: 0, total: 20 },
        { name: 'Cosecha', status: 'pending' as const, day: 0, total: 15 },
      ],
    },
    {
      id: 'campo-sur', name: 'Campo Sur', crop: 'Frijol', hectares: 8,
      progress_percent: 45, days_to_harvest: 67, days_since_last_activity: 0,
      last_activity_name: 'Siembra', responsible: 'Ana López',
      cost_accumulated: USD(3200), health: 'ok' as const,
      phases: [
        { name: 'Preparación', status: 'done' as const, day: 15, total: 15 },
        { name: 'Siembra', status: 'current' as const, day: 4, total: 10 },
        { name: 'Crecimiento', status: 'pending' as const, day: 0, total: 40 },
        { name: 'Floración', status: 'pending' as const, day: 0, total: 20 },
        { name: 'Cosecha', status: 'pending' as const, day: 0, total: 15 },
      ],
    },
    {
      id: 'lote-tres', name: 'Lote Tres', crop: 'Maíz', hectares: 5,
      progress_percent: 92, days_to_harvest: 7, days_since_last_activity: 5,
      last_activity_name: 'Monitoreo de madurez', responsible: 'Pedro Sánchez',
      cost_accumulated: USD(4800), health: 'critical' as const,
      phases: [
        { name: 'Preparación', status: 'done' as const, day: 18, total: 18 },
        { name: 'Siembra', status: 'done' as const, day: 4, total: 4 },
        { name: 'Crecimiento', status: 'done' as const, day: 42, total: 42 },
        { name: 'Floración', status: 'done' as const, day: 18, total: 18 },
        { name: 'Cosecha', status: 'current' as const, day: 3, total: 15 },
      ],
    },
  ],
};

export const MOCK_OPERATIONS: OperationsView = {
  date: '3 jul 2026',
  pending: [
    { id: 'a1', title: 'Fertilización nitrogenada', field: 'Campo Norte', crop: 'Maíz', scheduled_time: 'Hoy 09:00', status: 'pending' as const, responsible: 'Juan Pérez', notes: 'NDVI 0.62 — aplicar 120 kg N/ha' },
    { id: 'a2', title: 'Riego por aspersión', field: 'Campo Sur', crop: 'Frijol', scheduled_time: 'Hoy 14:00', status: 'pending' as const, responsible: null, notes: 'Humedad al 28%, programar 2h' },
    { id: 'a3', title: 'Monitoreo de plagas', field: 'Lote Tres', crop: 'Maíz', scheduled_time: 'Hoy 11:00', status: 'pending' as const, responsible: 'María García', notes: 'Revisar trampas antes de cosecha' },
  ],
  in_progress: [
    { id: 'a4', title: 'Aplicación de fungicida', field: 'Campo Norte', crop: 'Maíz', scheduled_time: '08:00 — 10:00', status: 'in_progress' as const, responsible: 'Pedro Sánchez', notes: 'Producto: Amistar Xtra 0.5 L/ha' },
  ],
  completed: [
    { id: 'a5', title: 'Corte de maleza', field: 'Campo Sur', crop: 'Frijol', scheduled_time: 'Ayer', status: 'completed' as const, responsible: 'Ana López', notes: 'Completado sin novedades' },
    { id: 'a6', title: 'Revisión de riego', field: 'Campo Norte', crop: 'Maíz', scheduled_time: 'Ayer', status: 'completed' as const, responsible: 'Juan Pérez', notes: 'Sistema funcionando correctamente' },
  ],
};

export const MOCK_TEAM: TeamView = {
  title: 'Equipo',
  today: { working: 3, absent: 1, total: 4 },
  workers: [
    { id: 'w1', name: 'Juan Pérez', role: 'Operario', status: 'working' as const, today_activity: 'Fertilización Campo Norte', last_payment: '$500' },
    { id: 'w2', name: 'Ana López', role: 'Supervisor', status: 'working' as const, today_activity: 'Riego Campo Sur', last_payment: '$650' },
    { id: 'w3', name: 'Pedro Sánchez', role: 'Tractorista', status: 'working' as const, today_activity: 'Fungicida Campo Norte', last_payment: '$550' },
    { id: 'w4', name: 'María García', role: 'Técnico', status: 'absent' as const, today_activity: null, last_payment: '$600' },
  ],
  recent_payments: [
    { worker: 'Juan Pérez', amount: '$500', date: '1 jul 2026', cycle: 'Maíz Campo Norte' },
    { worker: 'Ana López', amount: '$650', date: '1 jul 2026', cycle: 'Frijol Campo Sur' },
    { worker: 'Pedro Sánchez', amount: '$550', date: '1 jul 2026', cycle: 'Maíz Campo Norte' },
  ],
};

export const MOCK_FINANCES: FinancesView = {
  total_budget: '$18,000', total_spent: '$12,250', total_revenue: '$28,500',
  profit: '$16,250', roi: '188%',
  cycles: [
    { cycle_name: 'Maíz — Campo Norte', field: 'Campo Norte', budget: '$10,000', spent: '$6,500', revenue: '$18,000', profit: '$11,500', status: 'ok' as const },
    { cycle_name: 'Frijol — Campo Sur', field: 'Campo Sur', budget: '$8,000', spent: '$3,200', revenue: '$10,500', profit: '$7,300', status: 'ok' as const },
    { cycle_name: 'Maíz — Lote Tres', field: 'Lote Tres', budget: '$5,000', spent: '$4,800', revenue: '' as any, profit: '' as any, status: 'attention' as const },
  ],
};

export const MOCK_HISTORY: HistoryView = {
  campaigns: [
    { id: 'c1', crop: 'Maíz', field: 'Campo Norte', started: '15 feb 2026', ended: null, status: 'active' as const, progress_percent: 73, total_activities: 15, completed_activities: 11, budget: '$10,000', spent: '$6,500', revenue: '$18,000', profitability: '+80%', health: 'ok' as const },
    { id: 'c2', crop: 'Frijol', field: 'Campo Sur', started: '10 mar 2026', ended: null, status: 'active' as const, progress_percent: 45, total_activities: 8, completed_activities: 4, budget: '$8,000', spent: '$3,200', revenue: '$10,500', profitability: '+31%', health: 'ok' as const },
    { id: 'c3', crop: 'Maíz', field: 'Lote Tres', started: '1 ene 2026', ended: '25 jun 2026', status: 'completed' as const, progress_percent: 100, total_activities: 22, completed_activities: 22, budget: '$5,000', spent: '$4,800', revenue: '$7,500', profitability: '+50%', health: 'attention' as const },
    { id: 'c4', crop: 'Tomate', field: 'Invernadero', started: '20 abr 2025', ended: '20 sep 2025', status: 'completed' as const, progress_percent: 100, total_activities: 30, completed_activities: 30, budget: '$12,000', spent: '$11,200', revenue: '$15,000', profitability: '+25%', health: 'ok' as const },
  ],
};

export function mockDelay(ms = 300): Promise<void> {
  return new Promise(r => setTimeout(r, ms));
}
