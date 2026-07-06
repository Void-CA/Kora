import { Component, input, computed } from '@angular/core';

export type FieldHealthStatus = 'ok' | 'attention' | 'critical' | 'info';

const LABELS: Record<FieldHealthStatus, string> = {
  ok: 'Estable', attention: 'Atención', critical: 'Crítico', info: 'Información',
};

@Component({
  selector: 'kora-field-status',
  template: `
    <span class="status" [class.status--ok]="status()==='ok'"
          [class.status--attention]="status()==='attention'"
          [class.status--critical]="status()==='critical'"
          [class.status--info]="status()==='info'">
      <span class="status__dot"></span>
      @if (showLabel()) {
        <span class="status__label">{{ label() }}</span>
      }
    </span>
  `,
  styles: [`
    .status { display: inline-flex; align-items: center; gap: 0.35rem; }
    .status__dot { width: 7px; height: 7px; border-radius: 50%; flex-shrink: 0; }
    .status__label { font-size: 0.75rem; font-weight: 500; }
    .status--ok .status__dot { background: var(--state-ok); }
    .status--ok .status__label { color: var(--state-ok); }
    .status--attention .status__dot { background: var(--state-attention); }
    .status--attention .status__label { color: var(--state-attention); }
    .status--critical .status__dot { background: var(--state-critical); }
    .status--critical .status__label { color: var(--state-critical); }
    .status--info .status__dot { background: var(--ink-subtle); }
    .status--info .status__label { color: var(--ink-subtle); }
  `],
})
export class FieldStatus {
  readonly status = input<FieldHealthStatus>('ok');
  readonly showLabel = input(true);
  readonly label = computed(() => LABELS[this.status()]);
}
