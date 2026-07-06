import { Component, input } from '@angular/core';

export interface CyclePhaseData {
  name: string;
  status: 'done' | 'current' | 'pending';
  day?: number;
  total?: number;
}

@Component({
  selector: 'kora-crop-timeline',
  template: `
    <div class="timeline">
      @for (phase of phases(); track phase.name) {
        <div class="node" [class.node--done]="phase.status==='done'" [class.node--current]="phase.status==='current'">
          <div class="node__marker">
            <span class="node__dot"></span>
            @if (!$last) { <span class="node__line"></span> }
          </div>
          <div class="node__body">
            <span class="node__name">{{ phase.name }}</span>
            @if (phase.status === 'current' && phase.day != null && phase.total != null) {
              <span class="node__day">día {{ phase.day }}/{{ phase.total }}</span>
            }
          </div>
        </div>
      }
    </div>
  `,
  styleUrl: './crop-timeline.component.scss',
})
export class CropTimeline {
  readonly phases = input.required<CyclePhaseData[]>();
}
