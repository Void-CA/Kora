import { Component, signal } from '@angular/core';
import { HomeService } from '../../core/services/home.service';
import type { HomeView } from '../../core/view-models';
import { FieldMap } from '../field-map/field-map';
import { FieldSummary } from '../../components/domain/territory/field-summary';
import { WeatherStrip } from '../../components/domain/operation/weather-strip';

@Component({
  selector: 'app-home',
  imports: [FieldMap, FieldSummary, WeatherStrip],
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
