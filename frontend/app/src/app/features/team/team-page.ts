import { Component, signal } from '@angular/core';
import type { TeamView } from '../../core/view-models';
import { MOCK_TEAM, mockDelay } from '../../core/mock-data';

@Component({
  selector: 'app-team',
  templateUrl: './team-page.component.html',
  styleUrl: './team-page.component.scss',
})
export class TeamPage {
  readonly vm = signal<TeamView | null>(null);
  constructor() { mockDelay().then(() => this.vm.set(MOCK_TEAM)); }
}
