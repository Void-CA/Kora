import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';

interface FieldsOverview {
  title: string;
  fields: FieldCardData[];
}
interface FieldCardData {
  id: string; name: string; crop: string; hectares: number;
  progress_percent: number; days_to_harvest: number;
  days_since_last_activity: number; last_activity_name: string;
  responsible: string; cost_accumulated: string;
  health: string;
  phases: { name: string; status: string; day: number; total: number }[];
}

const BASE = 'http://localhost:8000';

@Component({
  selector: 'app-fields',
  imports: [RouterLink],
  templateUrl: './fields-page.component.html',
  styleUrl: './fields-page.component.scss',
})
export class FieldsPage {
  readonly vm = signal<FieldsOverview | null>(null);

  constructor() {
    fetch(`${BASE}/api/fields/overview`)
      .then(r => r.json())
      .then(data => this.vm.set(data))
      .catch(() => {});
  }
}
