import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-profitability-summary',
  template: `
    <div class="summary">
      <div class="summary__row">
        <span class="summary__label">Presupuesto</span><span class="summary__value">{{ baseline() }}</span>
      </div>
      <div class="summary__row">
        <span class="summary__label">Gastado</span><span class="summary__value">{{ spent() }}</span>
      </div>
      <div class="summary__row">
        <span class="summary__label">Ingresos</span><span class="summary__value">{{ revenue() }}</span>
      </div>
      <div class="summary__row summary__row--total">
        <span class="summary__label">Ganancia</span>
        <span class="summary__value" [class.summary__value--pos]="profitSign()==='+'" [class.summary__value--neg]="profitSign()==='-'">{{ profit() }}</span>
      </div>
      <div class="summary__row">
        <span class="summary__label">ROI</span>
        <span class="summary__value summary__value--pos">{{ roi() }}</span>
      </div>
    </div>
  `,
  styleUrl: './profitability-summary.component.scss',
})
export class ProfitabilitySummary {
  readonly baseline = input.required<string>();
  readonly spent = input.required<string>();
  readonly revenue = input.required<string>();
  readonly profit = input.required<string>();
  readonly roi = input.required<string>();
  readonly profitSign = input<'+' | '-'>('+');
}
