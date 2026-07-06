import { Component, signal } from '@angular/core';

interface TeamOverview {
  title: string;
  today: { working: number; absent: number; total: number };
  workers: TeamWorkerData[];
  recent_payments: { worker: string; amount: string; date: string; cycle: string }[];
}
interface TeamWorkerData {
  id: string; name: string; role: string;
  status: string; today_activity: string | null; last_payment: string | null;
}

const BASE = 'http://localhost:8000';

@Component({ selector: 'app-team',
  templateUrl: './team-page.component.html',
  styleUrl: './team-page.component.scss',
})
export class TeamPage {
  readonly vm = signal<TeamOverview | null>(null);
  constructor() {
    fetch(`${BASE}/api/team/overview`)
      .then(r => r.json())
      .then(data => this.vm.set(data))
      .catch(() => {});
  }
}
