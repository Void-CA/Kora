import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-weather-strip',
  template: `
    <div class="weather">
      <span class="weather__temp">{{ temp() }}</span>
      <span class="weather__sep">·</span>
      <span class="weather__forecast">{{ forecast() }}</span>
      @if (humidity()) { <span class="weather__sep">·</span> <span class="weather__humidity">{{ humidity() }} humedad</span> }
      @if (rainExpected()) { <span class="weather__rain">Lluvia esperada</span> }
    </div>
  `,
  styleUrl: './weather-strip.component.scss',
})
export class WeatherStrip {
  readonly temp = input.required<string>();
  readonly forecast = input.required<string>();
  readonly humidity = input<string>('');
  readonly rainExpected = input(false);
}
