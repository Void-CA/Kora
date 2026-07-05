import { Component, signal } from '@angular/core';
import type { OperationsView } from '../../core/view-models';
import { MOCK_OPERATIONS, mockDelay } from '../../core/mock-data';

@Component({
  selector: 'app-operations',
  templateUrl: './operations-page.component.html',
  styleUrl: './operations-page.component.scss',
})
export class OperationsPage {
  readonly vm = signal<OperationsView | null>(null);
  constructor() { mockDelay().then(() => this.vm.set(MOCK_OPERATIONS)); }
}
