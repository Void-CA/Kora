import { Component, signal } from '@angular/core';

interface OperationsToday {
  date: string;
  pending: ActivityCardData[];
  in_progress: ActivityCardData[];
  completed: ActivityCardData[];
}
interface ActivityCardData {
  id: string; title: string; field: string; crop: string;
  scheduled_time: string; status: string;
  responsible: string | null; notes: string;
}

const BASE = 'http://localhost:8000';

@Component({ selector: 'app-operations',
  templateUrl: './operations-page.component.html',
  styleUrl: './operations-page.component.scss',
})
export class OperationsPage {
  readonly vm = signal<OperationsToday | null>(null);
  constructor() {
    fetch(`${BASE}/api/operations/today`)
      .then(r => r.json())
      .then(data => this.vm.set(data))
      .catch(() => {});
  }
}
