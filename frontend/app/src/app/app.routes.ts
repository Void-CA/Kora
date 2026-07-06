import { Routes } from '@angular/router';
import { Shell } from './features/shell/shell';

export const routes: Routes = [
  {
    path: '',
    component: Shell,
    children: [
      { path: '', loadComponent: () => import('./features/home/home').then(m => m.HomePage), title: 'Kora · Inicio' },
      { path: 'fields', loadComponent: () => import('./features/fields/fields-page').then(m => m.FieldsPage), title: 'Kora · Campos' },
      { path: 'fields/:id', loadComponent: () => import('./features/area-detail/area-detail').then(m => m.AreaDetail), title: 'Kora · Campo' },
      { path: 'operations', loadComponent: () => import('./features/operations/operations-page').then(m => m.OperationsPage), title: 'Kora · Trabajo' },
      { path: 'team', loadComponent: () => import('./features/team/team-page').then(m => m.TeamPage), title: 'Kora · Equipo' },
      { path: 'finances', loadComponent: () => import('./features/finances/finances-page').then(m => m.FinancesPage), title: 'Kora · Finanzas' },
      { path: 'history', loadComponent: () => import('./features/history/history-page').then(m => m.HistoryPage), title: 'Kora · Historial' },
      { path: 'history/:id', loadComponent: () => import('./features/cycle-detail/cycle-detail').then(m => m.CycleDetail), title: 'Kora · Campaña' },
      { path: 'register', loadComponent: () => import('./features/register/register-hub').then(m => m.RegisterHub), title: 'Kora · Registrar' },
      { path: 'register/expense', loadComponent: () => import('./features/register/register-expense').then(m => m.RegisterExpensePage), title: 'Kora · Registrar gasto' },
      { path: 'register/activity', loadComponent: () => import('./features/register/register-activity').then(m => m.RegisterActivityPage), title: 'Kora · Registrar actividad' },
    ],
  },
  { path: '**', redirectTo: '' },
];
