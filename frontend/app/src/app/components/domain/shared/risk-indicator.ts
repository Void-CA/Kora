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
  styleUrl: './risk-indicator.component.scss',
})
export class RiskIndicator {
  readonly level = input<RiskLevel>('low');
  readonly dotColor = computed(() => LEVELS[this.level()].color);
  readonly label = computed(() => LEVELS[this.level()].label);
}
