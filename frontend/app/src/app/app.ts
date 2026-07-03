import { Component, signal } from '@angular/core';
import { OperationDashboard } from './features/operation-dashboard/operation-dashboard';

@Component({
  selector: 'app-root',
  imports: [OperationDashboard],
  template: `<app-operation-dashboard />`,
  styles: `
    :host {
      display: block;
      min-height: 100vh;
    }
  `,
})
export class App {}
