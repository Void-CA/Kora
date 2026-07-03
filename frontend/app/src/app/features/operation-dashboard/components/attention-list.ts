import { Component, input } from '@angular/core';
import { RouterLink } from '@angular/router';

export interface AttentionItem {
  kind: 'delay' | 'budget' | 'weather' | 'info';
  text: string;
  metric: string;
}

@Component({
  selector: 'app-attention-list',
  imports: [RouterLink],
  template: `
    <ul class="list">
      @for (item of items(); track item.text) {
        <li class="list__item" [attr.data-kind]="item.kind">
          <span class="list__dot"></span>
          <div class="list__body">
            <a class="list__text" [routerLink]="['/campos', fieldId(item)]">{{ item.text }}</a>
            <p class="list__metric">{{ item.metric }}</p>
          </div>
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
      align-items: flex-start;
      gap: var(--space-3);
      padding: var(--space-4);
      background: var(--surface);
      border: 1px solid var(--border);
      border-left: 3px solid var(--state-info);
      border-radius: var(--radius);
    }

    .list__item[data-kind='delay']   { border-left-color: var(--state-attention); }
    .list__item[data-kind='budget']  { border-left-color: var(--state-critical); }
    .list__item[data-kind='weather'] { border-left-color: var(--state-info); }
    .list__item[data-kind='info']    { border-left-color: var(--border-strong); }

    .list__dot {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: 50%;
      flex-shrink: 0;
      margin-top: 0.4em;
    }

    .list__item[data-kind='delay']   .list__dot { background: var(--state-attention); }
    .list__item[data-kind='budget']  .list__dot { background: var(--state-critical); }
    .list__item[data-kind='weather'] .list__dot { background: var(--state-info); }
    .list__item[data-kind='info']    .list__dot { background: var(--ink-subtle); }

    .list__body {
      flex: 1;
      min-width: 0;
    }

    .list__text {
      font-size: 0.875rem;
      color: var(--ink);
      text-decoration: none;
      font-weight: 500;
    }

    .list__text:hover {
      text-decoration: underline;
    }

    .list__metric {
      font-size: 0.75rem;
      color: var(--ink-muted);
      margin: var(--space-1) 0 0 0;
      font-variant-numeric: tabular-nums;
    }
  `,
})
export class AttentionList {
  readonly items = input.required<AttentionItem[]>();

  // MVP heuristic: el primer item con texto en minúscula + espacios
  // matching "Campo X" linkea a esa ficha. Cuando haya 2+ items con
  // campos distintos, agregamos `fieldId` como propiedad del item.
  fieldId(item: AttentionItem): string {
    const m = item.text.match(/Campo\s+(\w+)/i);
    if (m) return `campo-${m[1].toLowerCase()}`;
    return 'campo-norte';
  }
}
