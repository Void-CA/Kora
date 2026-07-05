import { Component, signal } from '@angular/core';
import type { FinancesView } from '../../core/view-models';
import { MOCK_FINANCES, mockDelay } from '../../core/mock-data';

@Component({
  selector: 'app-finances',
  templateUrl: './finances-page.component.html',
  styleUrl: './finances-page.component.scss',
})
export class FinancesPage {
  readonly vm = signal<FinancesView | null>(null);
  constructor() { mockDelay().then(() => this.vm.set(MOCK_FINANCES)); }
}
