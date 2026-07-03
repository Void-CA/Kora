import { Component, input } from '@angular/core';

export interface ContextItem {
  label: string;
  href?: string;
}

@Component({
  selector: 'app-context-chips',
  template: `
    <ul class="chips">
      @for (item of items(); track item.label) {
        <li class="chip">
          @if (item.href) {
            <a [href]="item.href">{{ item.label }}</a>
          } @else {
            <span>{{ item.label }}</span>
          }
        </li>
      }
    </ul>
  `,
  styles: `
    :host { display: block; }

    .chips {
      list-style: none;
      margin: 0;
      padding: 0;
      display: flex;
      flex-wrap: wrap;
      gap: var(--space-1) var(--space-2);
    }

    .chip {
      font-size: 0.8125rem;
      color: var(--ink-muted);
    }

    .chip:not(:last-child)::after {
      content: '·';
      margin-left: var(--space-2);
      color: var(--ink-subtle);
    }

    a {
      color: inherit;
      text-decoration: none;
    }

    a:hover {
      color: var(--ink);
      text-decoration: underline;
    }
  `,
})
export class ContextChips {
  readonly items = input.required<ContextItem[]>();
}
