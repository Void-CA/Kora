import { Component, input, computed } from '@angular/core';
import { RouterLink } from '@angular/router';
import { FieldStatus } from './field-status';

export type FieldSummaryVariant = 'compact' | 'default' | 'expanded';

@Component({
  selector: 'kora-field-summary',
  imports: [RouterLink, FieldStatus],
  template: `
    <div class="field" [class.field--compact]="variant()==='compact'" [class.field--expanded]="variant()==='expanded'">
      <a class="field__link" [routerLink]="['/fields', id()]">
        <div class="field__head">
          <span class="field__name">{{ name() }}</span>
          <div class="field__meta">
            <span class="field__crop">{{ crop() }}</span>
            <span class="field__ha">{{ hectares() }} ha</span>
          </div>
        </div>
        @if (variant() !== 'compact') {
          <div class="field__bar">
            <div class="field__fill" [style.width.%]="progress()"></div>
          </div>
          <div class="field__foot">
            <span>{{ daysToHarvest() }} días a cosecha</span>
            <span>{{ lastActivity() }}</span>
          </div>
        }
      </a>
      <div class="field__status">
        <kora-field-status [status]="health()" />
      </div>
    </div>
  `,
  styles: [`
    .field {
      display: flex;
      align-items: flex-start;
      gap: var(--space-3);
      padding: var(--space-3) 0;
      border-bottom: 1px solid var(--border);
    }
    .field:last-child { border-bottom: none; }
    .field--compact { padding: var(--space-2) 0; }
    .field--expanded { padding: var(--space-4) 0; }
    .field__link {
      flex: 1;
      text-decoration: none;
      color: inherit;
      cursor: pointer;
      transition: opacity var(--ease);
    }
    .field__link:hover { opacity: 0.7; }
    .field__head {
      display: flex;
      justify-content: space-between;
      align-items: baseline;
    }
    .field__name { font-size: 0.875rem; font-weight: 500; color: var(--ink); }
    .field__meta { display: flex; gap: var(--space-3); font-size: 0.75rem; color: var(--ink-subtle); }
    .field__bar { height: 2px; background: var(--border); border-radius: 1px; overflow: hidden; margin-top: var(--space-2); }
    .field__fill { height: 100%; background: var(--ink-faint); border-radius: 1px; }
    .field__foot { display: flex; justify-content: space-between; font-size: 0.75rem; color: var(--ink-subtle); margin-top: var(--space-1); }
    .field__status { flex-shrink: 0; margin-top: 0.15rem; }
  `],
})
export class FieldSummary {
  readonly id = input.required<string>();
  readonly name = input.required<string>();
  readonly crop = input.required<string>();
  readonly hectares = input.required<number>();
  readonly progress = input.required<number>();
  readonly daysToHarvest = input.required<number>();
  readonly lastActivity = input.required<string>();
  readonly health = input<string>('ok');
  readonly variant = input<FieldSummaryVariant>('default');
}
