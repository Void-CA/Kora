import { Component, input, computed } from '@angular/core';

export type RiskLevel = 'low' | 'medium' | 'high' | 'critical';

const LEVELS: Record<RiskLevel, { label: string; color: string }> = {
  low: { label: 'Bajo', color: 'var(--state-ok)' },
  medium: { label: 'Medio', color: 'var(--state-attention)' },
  high: { label: 'Alto', color: 'var(--state-critical)' },
  critical: { label: 'Crítico', color: 'var(--state-critical)' },
};

@Component({
  selector: 'kora-risk-indicator',
  template: `
    <div class="risk" [class.risk--high]="level()==='high'||level()==='critical'"
          [class.risk--medium]="level()==='medium'">
      <span class="risk__dot" [style.background]="dotColor()"></span>
      <span class="risk__label">{{ label() }}</span>
    </div>
  `,
  styles: [`
    .risk { display: inline-flex; align-items: center; gap: 0.35rem; padding: 0.2rem 0.5rem; border-radius: var(--radius-sm); background: transparent; }
    .risk--high { background: var(--state-critical-soft); }
    .risk--medium { background: var(--state-attention-soft); }
    .risk__dot { width: 6px; height: 6px; border-radius: 50%; flex-shrink: 0; }
    .risk__label { font-size: 0.75rem; font-weight: 500; color: var(--ink-muted); }
    .risk--high .risk__label { color: var(--state-critical); }
    .risk--medium .risk__label { color: var(--state-attention); }
  `],
})
export class RiskIndicator {
  readonly level = input<RiskLevel>('low');
  readonly dotColor = computed(() => LEVELS[this.level()].color);
  readonly label = computed(() => LEVELS[this.level()].label);
}
