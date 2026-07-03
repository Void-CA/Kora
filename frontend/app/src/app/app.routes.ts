import { Routes } from '@angular/router';

export const routes: Routes = [
  {
    path: '',
    loadComponent: () =>
      import('./features/operation-dashboard/operation-dashboard').then(m => m.OperationDashboard),
    title: 'Kora · Operación',
  },
  {
    path: 'campos/:id',
    loadComponent: () =>
      import('./features/field-card/field-card').then(m => m.FieldCard),
    title: 'Kora · Campo',
  },
  { path: '**', redirectTo: '' },
];
