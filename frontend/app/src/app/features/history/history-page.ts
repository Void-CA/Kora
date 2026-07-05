import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';
import type { HistoryView } from '../../core/view-models';
import { MOCK_HISTORY, mockDelay } from '../../core/mock-data';

@Component({
  selector: 'app-history',
  imports: [RouterLink],
  templateUrl: './history-page.component.html',
  styleUrl: './history-page.component.scss',
})
export class HistoryPage {
  readonly vm = signal<HistoryView | null>(null);
  constructor() { mockDelay().then(() => this.vm.set(MOCK_HISTORY)); }
}
