import { Component, input } from '@angular/core';
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
          <div class="field__bar"><div class="field__fill" [style.width.%]="progress()"></div></div>
          <div class="field__foot">
            <span>{{ daysToHarvest() }} días a cosecha</span>
            <span>{{ lastActivity() }}</span>
          </div>
        }
      </a>
      <div class="field__status"><kora-field-status [status]="health()" /></div>
    </div>
  `,
  styleUrl: './field-summary.component.scss',
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
