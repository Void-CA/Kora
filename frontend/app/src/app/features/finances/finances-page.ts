import { Component, signal } from '@angular/core';

interface FinancesOverview {
  total_budget: string; total_spent: string; total_revenue: string;
  profit: string; roi: string;
  cycles: FinanceCycleRow[];
}
interface FinanceCycleRow {
  cycle_name: string; field: string; budget: string; spent: string;
  revenue: string; profit: string; status: string;
}

const BASE = 'http://localhost:8000';

@Component({ selector: 'app-finances',
  templateUrl: './finances-page.component.html',
  styleUrl: './finances-page.component.scss',
})
export class FinancesPage {
  readonly vm = signal<FinancesOverview | null>(null);
  constructor() {
    fetch(`${BASE}/api/finances/overview`)
      .then(r => r.json())
      .then(data => this.vm.set(data))
      .catch(() => {});
  }
}
