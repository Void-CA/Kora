import { Component, input, computed } from '@angular/core';

export type FieldHealthStatus = 'ok' | 'attention' | 'critical' | 'info';
const LABELS: Record<FieldHealthStatus, string> = { ok: 'Estable', attention: 'Atención', critical: 'Crítico', info: 'Información' };

@Component({
  selector: 'kora-field-status',
  template: `
    <span class="status" [class.status--ok]="status()==='ok'"
          [class.status--attention]="status()==='attention'"
          [class.status--critical]="status()==='critical'"
          [class.status--info]="status()==='info'">
      <span class="status__dot"></span>
      @if (showLabel()) { <span class="status__label">{{ label() }}</span> }
    </span>
  `,
  styleUrl: './field-status.component.scss',
})
export class FieldStatus {
  readonly status = input<FieldHealthStatus>('ok');
  readonly showLabel = input(true);
  readonly label = computed(() => LABELS[this.status()]);
}
