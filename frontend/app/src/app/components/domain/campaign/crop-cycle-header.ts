import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-crop-cycle-header',
  template: `
    <div class="header">
      <div class="header__primary">
        <h2 class="header__crop">{{ crop() }}</h2>
        <span class="header__field">{{ field() }}</span>
      </div>
      <div class="header__meta">
        <span class="header__period">{{ period() }}</span>
        <span class="header__sep">·</span>
        <span class="header__status" [class.header__status--active]="status()==='active'">{{ status() === 'active' ? 'En curso' : 'Completado' }}</span>
      </div>
    </div>
  `,
  styleUrl: './crop-cycle-header.component.scss',
})
export class CropCycleHeader {
  readonly crop = input.required<string>();
  readonly field = input.required<string>();
  readonly period = input.required<string>();
  readonly status = input<'active' | 'completed'>('active');
}
