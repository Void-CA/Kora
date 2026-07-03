import { Component, input } from '@angular/core';

export type CyclePhaseStatus = 'done' | 'current' | 'pending';

export interface CyclePhase {
  name: string;
  status: CyclePhaseStatus;
  dayInPhase: number | null;
  expectedDurationDays: number | null;
}

@Component({
  selector: 'app-cycle-timeline',
  template: `
    <ol class="timeline">
      @for (phase of phases(); track phase.name) {
        <li class="phase" [attr.data-status]="phase.status">
          <span class="phase__name">
            {{ phase.name }}
            @if (phase.status === 'current' && phase.dayInPhase !== null && phase.expectedDurationDays !== null) {
              <span class="phase__day">
                · día {{ phase.dayInPhase }} de {{ phase.expectedDurationDays }}
              </span>
            }
          </span>
          <span class="phase__bar"></span>
        </li>
      }
    </ol>
  `,
  styles: `
    :host { display: block; }

    .timeline {
      list-style: none;
      margin: 0;
      padding: 0;
      display: flex;
      flex-direction: column;
      gap: var(--space-3);
    }

    .phase {
      display: grid;
      grid-template-columns: 200px 1fr;
      align-items: center;
      gap: var(--space-4);
    }

    .phase__name {
      font-size: 0.8125rem;
      color: var(--ink-muted);
    }

    .phase__day {
      color: var(--ink-subtle);
      font-variant-numeric: tabular-nums;
    }

    .phase__bar {
      display: block;
      height: 6px;
      border-radius: 3px;
      background: var(--surface-muted);
      position: relative;
    }

    .phase__bar::after {
      content: '';
      position: absolute;
      inset: 0;
      border-radius: 3px;
      transform-origin: left;
      transform: scaleX(var(--fill, 0));
    }

    .phase[data-status='done'] .phase__name { color: var(--ink); }
    .phase[data-status='done'] .phase__bar::after { background: var(--state-ok); --fill: 1; }

    .phase[data-status='current'] .phase__name { color: var(--ink); font-weight: 500; }
    .phase[data-status='current'] .phase__bar::after { background: var(--state-info); --fill: 0.25; }

    .phase[data-status='pending'] .phase__bar::after { background: transparent; --fill: 0; }
  `,
})
export class CycleTimeline {
  readonly phases = input.required<CyclePhase[]>();
}
