import { Routes } from '@angular/router';

export default [
  {
    path: '',
    loadComponent: () => import('./employee-list/employee-list.component').then(m => m.EmployeeListComponent),
  },
] as Routes;
