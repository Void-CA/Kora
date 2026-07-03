import { Component, input } from '@angular/core';

export type HealthStatus = 'ok' | 'attention' | 'critical' | 'info';

@Component({
  selector: 'app-status-pill',
  template: `
    <div class="pill" [attr.data-status]="status()">
      <span class="pill__dot"></span>
      <span class="pill__label">{{ label() }}</span>
      <span class="pill__value">{{ value() }}</span>
    </div>
  `,
  styles: `
    :host { display: block; }

    .pill {
      display: flex;
      align-items: center;
      gap: var(--space-2);
      padding: var(--space-2) var(--space-3);
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
    }

    .pill__dot {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: 50%;
      flex-shrink: 0;
    }

    .pill__label {
      font-size: 0.8125rem;
      color: var(--ink-muted);
      flex: 1;
    }

    .pill__value {
      font-size: 0.875rem;
      font-weight: 500;
      color: var(--ink);
    }

    .pill[data-status='ok'] .pill__dot        { background: var(--state-ok); }
    .pill[data-status='attention'] .pill__dot { background: var(--state-attention); }
    .pill[data-status='critical'] .pill__dot  { background: var(--state-critical); }
    .pill[data-status='info'] .pill__dot      { background: var(--state-info); }
  `,
})
export class StatusPill {
  readonly status = input.required<HealthStatus>();
  readonly label = input.required<string>();
  readonly value = input.required<string>();
}
