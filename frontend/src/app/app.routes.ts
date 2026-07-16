import { Routes } from '@angular/router';

export const routes: Routes = [
  {
    path: 'employees',
    loadChildren: () => import('./features/employees/employees.routes'),
  },
  { path: '', redirectTo: 'employees', pathMatch: 'full' },
];
