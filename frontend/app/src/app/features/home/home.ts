import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';
import { HomeService } from '../../core/services/home.service';
import type { HomeView } from '../../core/view-models';

@Component({
  selector: 'app-home',
  imports: [RouterLink],
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
})
export class HomePage {
  readonly vm = signal<HomeView | null>(null);

  constructor() {
    HomeService.load()
      .then(data => this.vm.set(data))
      .catch(() => {});
  }
}
