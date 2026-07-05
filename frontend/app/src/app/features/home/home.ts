import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';
import type { HomeView } from '../../core/view-models';
import { MOCK_HOME, mockDelay } from '../../core/mock-data';

@Component({
  selector: 'app-home',
  imports: [RouterLink],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
})
export class HomePage {
  readonly vm = signal<HomeView | null>(null);

  constructor() {
    mockDelay().then(() => this.vm.set(MOCK_HOME));
  }
}
