import { Component, signal, computed } from '@angular/core';
import { BudgetBar } from '../../components/domain/campaign/budget-bar';
import { ProfitabilitySummary } from '../../components/domain/campaign/profitability-summary';

interface FinancesOverview {
  total_budget: string; total_spent: string; total_revenue: string;
  profit: string; roi: string;
  cycles: CycleRow[];
}
interface CycleRow { cycle_name: string; field: string; budget: string; spent: string; revenue: string; profit: string; status: string; }

const BASE = 'http://localhost:8000';

@Component({
  selector: 'app-finances',
  imports: [BudgetBar, ProfitabilitySummary],
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

  spentPct(spent: string, budget: string): number {
    const s = parseFloat(spent.replace(/[$,]/g, ''));
    const b = parseFloat(budget.replace(/[$,]/g, ''));
    if (!b) return 0;
    return Math.round((s / b) * 100);
  }
}
