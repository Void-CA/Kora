import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';

interface HomeResponse {
  greeting: string;
  date: string;
  today: { pending: number; critical: number; completed: number; next_action: NextActionData | null };
  fields: HomeFieldPreview[];
  team: TeamPreview;
  finances: FinancePreview;
  alerts: AlertItem[];
  weather: WeatherInfo | null;
}

interface NextActionData {
  title: string; field: string; crop: string; when: string; priority: string;
}
interface HomeFieldPreview {
  id: string; name: string; crop: string; hectares: number;
  progress_percent: number; health: string; days_to_harvest: number;
  last_activity: string; next_activity: string;
}
interface TeamPreview {
  total: number; working_today: number; missing_yesterday: number;
  entries: { name: string; status: string }[];
}
interface FinancePreview {
  total_budget: string; budget_used_percent: number; total_spent: string;
  alerts: { cycle: string; text: string }[];
}
interface AlertItem {
  kind: string; text: string; field: string | null; severity: string;
}
interface WeatherInfo {
  forecast: string; rain_expected: boolean; temp: string; humidity: string;
}

const BASE = 'http://localhost:8000';

@Component({
  selector: 'app-home',
  imports: [RouterLink],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
})
export class HomePage {
  readonly vm = signal<HomeResponse | null>(null);

  constructor() {
    fetch(`${BASE}/api/home`)
      .then(r => r.json())
      .then(data => this.vm.set(data))
      .catch(() => {
        // fallback — backend not running
      });
  }
}
