// Design System — consistent page sections.

import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-section',
  template: `
    <section class="section">
      @if (label()) {
        <h2 class="section__label">{{ label() }}</h2>
      }
      <ng-content />
    </section>
  `,
  styles: [`
    .section {
      margin-bottom: var(--space-8);
    }
    .section__label {
      font-size: 0.6875rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      color: var(--ink-subtle);
      margin: 0 0 var(--space-4);
    }
  `],
})
export class KoraSection {
  readonly label = input<string>('');
}
