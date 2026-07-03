import { Routes } from '@angular/router';
import { Shell } from './features/shell/shell';

export const routes: Routes = [
  {
    path: '',
    component: Shell,
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/operation-dashboard/operation-dashboard').then(m => m.OperationDashboard),
        title: 'Kora · Operación',
      },
      {
        path: 'lotes',
        loadComponent: () =>
          import('./features/area-list/area-list').then(m => m.AreaList),
        title: 'Kora · Lotes',
      },
      {
        path: 'lotes/:id',
        loadComponent: () =>
          import('./features/area-detail/area-detail').then(m => m.AreaDetail),
        title: 'Kora · Lote',
      },
      {
        path: 'ciclos',
        loadComponent: () =>
          import('./features/cycle-list/cycle-list').then(m => m.CycleList),
        title: 'Kora · Ciclos',
      },
      {
        path: 'ciclos/:id',
        loadComponent: () =>
          import('./features/cycle-detail/cycle-detail').then(m => m.CycleDetail),
        title: 'Kora · Ciclo',
      },
      {
        path: 'personal',
        loadComponent: () =>
          import('./features/workers/workers-page').then(m => m.WorkersPage),
        title: 'Kora · Personal',
      },
    ],
  },
  { path: '**', redirectTo: '' },
];
