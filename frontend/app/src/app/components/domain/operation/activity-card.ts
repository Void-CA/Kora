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
        <span class="card__field">{{ field() }}</span>
        @if (crop()) { <span class="card__sep">·</span> }
        <span class="card__crop">{{ crop() }}</span>
        <span class="card__sep">·</span>
        <span class="card__time">{{ scheduledTime() }}</span>
      </div>
      @if (responsible(); as r) {
        <p class="card__resp">{{ r }}</p>
      }
      @if (notes(); as n) {
        <p class="card__notes">{{ n }}</p>
      }
      @if (status() === 'pending') {
        <div class="card__actions">
          <button class="btn-ghost">Comenzar</button>
        </div>
      }
    </div>
  `,
  styles: [`
    .card { padding: var(--space-4); background: var(--surface-raised); border: 1px solid var(--border); border-radius: var(--radius); margin-bottom: var(--space-2); }
    .card--active { border-left: 3px solid var(--state-info); }
    .card__head { display: flex; justify-content: space-between; align-items: center; }
    .card__title { font-size: 0.875rem; font-weight: 500; color: var(--ink); }
    .card__status { font-size: 0.6875rem; font-weight: 500; color: var(--state-info); }
    .status--pending { color: var(--state-attention); }
    .status--active { color: var(--state-info); }
    .status--done { color: var(--state-ok); }
    .card__details { display: flex; gap: var(--space-2); font-size: 0.75rem; color: var(--ink-muted); margin-top: var(--space-1); flex-wrap: wrap; }
    .card__sep { color: var(--ink-faint); }
    .card__resp { font-size: 0.75rem; color: var(--ink-muted); margin: var(--space-1) 0 0; }
    .card__notes { font-size: 0.75rem; color: var(--ink-muted); margin: var(--space-2) 0 0; padding: var(--space-2); background: var(--surface-muted); border-radius: var(--radius-sm); }
    .card__actions { margin-top: var(--space-2); }
    .btn-ghost { padding: var(--space-1) var(--space-3); border-radius: var(--radius-sm); font-size: 0.75rem; font-weight: 500; border: 1px solid var(--border); background: transparent; color: var(--ink-muted); cursor: pointer; transition: background var(--ease); }
    .btn-ghost:hover { background: var(--surface-muted); }
  `],
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
