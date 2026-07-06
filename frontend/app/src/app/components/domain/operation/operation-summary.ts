import { Component, input, computed } from '@angular/core';

@Component({
  selector: 'kora-operation-summary',
  template: `
    <div class="ops">
      <span class="ops__label">{{ label() }}</span>
      <span class="ops__value">{{ value() }}</span>
      @if (detail()) { <span class="ops__detail">{{ detail() }}</span> }
    </div>
  `,
  styleUrl: './operation-summary.component.scss',
})
export class OperationSummary {
  readonly label = input.required<string>();
  readonly value = input.required<string>();
  readonly detail = input<string>('');
}
