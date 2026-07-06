import { Component, input, computed } from '@angular/core';

@Component({
  selector: 'kora-budget-bar',
  template: `
    <div class="budget">
      <div class="budget__header">
        <span class="budget__label">{{ label() }}</span>
        <span class="budget__pct">{{ percent() }}%</span>
      </div>
      <div class="budget__bar">
        <div class="budget__fill" [style.width.%]="percent()"
             [class.budget__fill--high]="percent() > 80"
             [class.budget__fill--mid]="percent() > 50 && percent() <= 80"></div>
      </div>
      <div class="budget__footer">
        <span>{{ spent() }} usados</span>
        <span>{{ total() }} presupuesto</span>
      </div>
    </div>
  `,
  styleUrl: './budget-bar.component.scss',
})
export class BudgetBar {
  readonly label = input('Presupuesto');
  readonly percent = input.required<number>();
  readonly spent = input.required<string>();
  readonly total = input.required<string>();
}
