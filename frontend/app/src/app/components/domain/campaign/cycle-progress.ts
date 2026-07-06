import { Component, input } from '@angular/core';

@Component({
  selector: 'kora-cycle-progress',
  template: `
    <div class="progress">
      <div class="progress__bar">
        <div class="progress__fill" [style.width.%]="percent()"></div>
      </div>
      <div class="progress__labels">
        <span>{{ percent() }}% completado</span>
        <span>{{ daysElapsed() }} / {{ daysTotal() }} días</span>
      </div>
    </div>
  `,
  styleUrl: './cycle-progress.component.scss',
})
export class CycleProgress {
  readonly percent = input.required<number>();
  readonly daysElapsed = input.required<number>();
  readonly daysTotal = input.required<number>();
}
