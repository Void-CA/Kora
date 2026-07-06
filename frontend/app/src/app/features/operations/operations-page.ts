import { Component, signal } from '@angular/core';
import { ActivityCard } from '../../components/domain/operation/activity-card';

interface OperationsToday {
  date: string;
  in_progress: ActivityData[];
  pending: ActivityData[];
  completed: ActivityData[];
}
interface ActivityData {
  id: string; title: string; field: string; crop: string;
  scheduled_time: string; status: string; responsible: string | null; notes: string;
}

const BASE = 'http://localhost:8000';

@Component({
  selector: 'app-operations',
  imports: [ActivityCard],
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
