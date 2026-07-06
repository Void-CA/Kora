// Design System — Kora's button. Uses ui-button as infrastructure.
// Adds Kora-specific visual language (palette, radius, typography).
// All page-level buttons use this, never `<ui-button>` directly.

import { Component, input } from '@angular/core';
import { UiButton } from '../ui/button';

export type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'danger';

@Component({
  selector: 'kora-button',
  imports: [UiButton],
  template: `
    <button ui-button [disabled]="disabled()" [class]="'btn btn--' + variant()">
      <ng-content />
    </button>
  `,
  styles: [`
    .btn {
      padding: 0.5rem 1.25rem;
      border-radius: var(--radius-sm);
      font-size: 0.8125rem;
      font-weight: 500;
      transition: background var(--ease), color var(--ease);
    }
    .btn--primary {
      background: var(--btn-primary);
      color: var(--btn-primary-text);
    }
    .btn--primary:hover {
      background: var(--btn-primary-hover);
    }
    .btn--secondary {
      background: var(--surface-muted);
      color: var(--ink);
    }
    .btn--secondary:hover {
      background: var(--border);
    }
    .btn--ghost {
      color: var(--ink-muted);
    }
    .btn--ghost:hover {
      background: var(--surface-muted);
    }
    .btn--danger {
      background: var(--state-critical);
      color: white;
    }
  `],
})
export class KoraButton {
  readonly disabled = input(false);
  readonly variant = input<ButtonVariant>('primary');
}
