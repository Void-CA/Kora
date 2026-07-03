import { Component, input, computed } from '@angular/core';
import { ContextChips } from '../../../shared/context-chips';

export interface NextAction {
  title: string;
  field: string;
  lot: string;
  crop: string;
  when: string;
  priority: 'high' | 'medium' | 'low';
}

@Component({
  selector: 'app-next-action-card',
  imports: [ContextChips],
  template: `
    <article class="action-card" [class.action-card--high]="action().priority === 'high'">
      <div class="action-card__body">
        <h3 class="action-card__title">{{ action().title }}</h3>
        <app-context-chips [items]="contextItems()" />
        <p class="action-card__when">{{ action().when }}</p>
      </div>
      <div class="action-card__actions">
        <button type="button" class="btn btn--primary">Comenzar</button>
        <button type="button" class="btn btn--ghost">Posponer</button>
      </div>
    </article>
  `,
  styles: `
    :host { display: block; }

    .action-card {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: var(--space-6);
      padding: var(--space-6);
      background: var(--surface);
      border: 1px solid var(--border);
      border-left: 3px solid var(--state-info);
      border-radius: var(--radius-lg);
    }

    .action-card--high {
      border-left-color: var(--state-critical);
    }

    .action-card__body {
      flex: 1;
      min-width: 0;
    }

    .action-card__title {
      font-size: 1rem;
      font-weight: 600;
      color: var(--ink);
      margin: 0 0 var(--space-2) 0;
    }

    .action-card__when {
      font-size: 0.8125rem;
      color: var(--ink-subtle);
      margin: var(--space-2) 0 0 0;
    }

    .action-card__actions {
      display: flex;
      gap: var(--space-2);
      flex-shrink: 0;
    }

    .btn {
      padding: var(--space-2) var(--space-4);
      border-radius: var(--radius);
      font-size: 0.875rem;
      font-weight: 500;
      border: 1px solid transparent;
      transition: background 0.12s, border-color 0.12s;
    }

    .btn--primary {
      background: var(--ink);
      color: var(--surface);
    }

    .btn--primary:hover {
      background: #292524;
    }

    .btn--ghost {
      background: transparent;
      color: var(--ink-muted);
      border-color: var(--border);
    }

    .btn--ghost:hover {
      background: var(--surface-muted);
    }
  `,
})
export class NextActionCard {
  readonly action = input.required<NextAction>();

  readonly contextItems = computed(() => {
    const a = this.action();
    return [
      { label: a.field },
      { label: a.lot },
      { label: a.crop },
    ];
  });
}
