import { Component, input } from '@angular/core';

export interface AttentionItem {
  kind: 'delay' | 'budget' | 'weather' | 'info';
  text: string;
}

@Component({
  selector: 'app-attention-list',
  template: `
    <ul class="list">
      @for (item of items(); track item.text) {
        <li class="list__item">
          <span class="list__dot" [attr.data-kind]="item.kind"></span>
          <span class="list__text">{{ item.text }}</span>
        </li>
      }
    </ul>
  `,
  styles: `
    :host { display: block; }

    .list {
      list-style: none;
      margin: 0;
      padding: 0;
      display: flex;
      flex-direction: column;
      gap: var(--space-3);
    }

    .list__item {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      padding: var(--space-3) var(--space-4);
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
    }

    .list__dot {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: 50%;
      flex-shrink: 0;
    }

    .list__dot[data-kind='delay']    { background: var(--state-attention); }
    .list__dot[data-kind='budget']   { background: var(--state-critical); }
    .list__dot[data-kind='weather']  { background: var(--state-info); }
    .list__dot[data-kind='info']     { background: var(--ink-subtle); }

    .list__text {
      font-size: 0.875rem;
      color: var(--ink);
    }
  `,
})
export class AttentionList {
  readonly items = input.required<AttentionItem[]>();
}
