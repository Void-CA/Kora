import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-activity-card',
  template: `
    <div class="card" [class.card--active]="status()==='in_progress'">
      <div class="card__head">
        <span class="card__title">{{ title() }}</span>
        <span class="card__status" [class.status--pending]="status()==='pending'"
              [class.status--active]="status()==='in_progress'"
              [class.status--done]="status()==='completed'">{{ statusLabel() }}</span>
      </div>
      <div class="card__details">
        <span>{{ field() }}</span>@if (crop()) {<span class="card__sep"> · </span><span>{{ crop() }}</span>}
        <span class="card__sep"> · </span><span>{{ scheduledTime() }}</span>
      </div>
      @if (responsible(); as r) { <p class="card__resp">{{ r }}</p> }
      @if (notes(); as n) { <p class="card__notes">{{ n }}</p> }
      @if (status() === 'pending') { <div class="card__actions"><button class="btn-ghost">Comenzar</button></div> }
    </div>
  `,
  styleUrl: './activity-card.component.scss',
})
export class ActivityCard {
  readonly id = input.required<string>();
  readonly title = input.required<string>();
  readonly field = input.required<string>();
  readonly crop = input<string>('');
  readonly scheduledTime = input.required<string>();
  readonly status = input<'pending' | 'in_progress' | 'completed'>('pending');
  readonly responsible = input<string | null>(null);
  readonly notes = input<string | null>(null);
  statusLabel(): string {
    const map: Record<string, string> = { pending: 'Pendiente', in_progress: 'En progreso', completed: 'Completado' };
    return map[this.status()] ?? this.status();
  }
}
