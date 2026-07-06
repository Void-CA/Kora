import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';

interface HistoryOverview {
  campaigns: CampaignCardData[];
}
interface CampaignCardData {
  id: string; crop: string; field: string; started: string; ended: string | null;
  status: string; progress_percent: number;
  total_activities: number; completed_activities: number;
  budget: string; spent: string; revenue: string;
  profitability: string; health: string;
}

const BASE = 'http://localhost:8000';

@Component({ selector: 'app-history', imports: [RouterLink],
  templateUrl: './history-page.component.html',
  styleUrl: './history-page.component.scss',
})
export class HistoryPage {
  readonly vm = signal<HistoryOverview | null>(null);
  constructor() {
    fetch(`${BASE}/api/history/overview`)
      .then(r => r.json())
      .then(data => this.vm.set(data))
      .catch(() => {});
  }
}
