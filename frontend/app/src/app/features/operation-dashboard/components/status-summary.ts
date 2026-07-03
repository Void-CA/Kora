import { Component, input } from '@angular/core';

export interface StatusCounts {
  ok: number;
  attention: number;
  critical: number;
}

@Component({
  selector: 'app-status-summary',
  template: `
    <div class="chips">
      <div class="chip chip--ok">
        <span class="dot"></span>
        <span class="count">{{ counts().ok }}</span>
        <span class="label">completadas</span>
      </div>
      <div class="chip chip--attention">
        <span class="dot"></span>
        <span class="count">{{ counts().attention }}</span>
        <span class="label">pendientes</span>
      </div>
      <div class="chip chip--critical">
        <span class="dot"></span>
        <span class="count">{{ counts().critical }}</span>
        <span class="label">críticas</span>
      </div>
    </div>
  `,
  styles: `
    :host { display: block; }

    .chips {
      display: flex;
      gap: var(--space-6);
      flex-wrap: wrap;
    }

    .chip {
      display: inline-flex;
      align-items: baseline;
      gap: var(--space-2);
      font-size: 0.875rem;
    }

    .dot {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: 50%;
      display: inline-block;
      transform: translateY(0.05em);
    }

    .count {
      font-weight: 600;
      font-size: 1.5rem;
      color: var(--ink);
    }

    .label {
      color: var(--ink-muted);
    }

    .chip--ok .dot        { background: var(--state-ok); }
    .chip--attention .dot { background: var(--state-attention); }
    .chip--critical .dot  { background: var(--state-critical); }
  `,
})
export class StatusSummary {
  readonly counts = input.required<StatusCounts>();
}
