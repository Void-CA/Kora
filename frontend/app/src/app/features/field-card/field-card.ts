import { Component, signal, input, computed, effect } from '@angular/core';
import { ContextChips } from '../../shared/context-chips';
import { CycleTimeline, CyclePhase } from '../../shared/cycle-timeline';
import { MiniMap } from '../../shared/mini-map';
import { StatusPill, HealthStatus } from '../../shared/status-pill';
import { getField, Field as ApiField } from '../../api/kora-api';

@Component({
  selector: 'app-field-card',
  imports: [ContextChips, CycleTimeline, MiniMap, StatusPill],
  templateUrl: './field-card.html',
  styleUrl: './field-card.scss',
})
export class FieldCard {
  readonly id = input.required<string>();
  readonly field = signal<ApiField | null>(null);
  readonly error = signal<string | null>(null);

  constructor() {
    effect(() => {
      const id = this.id();
      this.field.set(null);
      this.error.set(null);
      getField(id)
        .then(data => this.field.set(data))
        .catch(err => this.error.set(err.message === 'not_found' ? 'Campo no encontrado' : err.message));
    });
  }

  readonly contextItems = computed(() => {
    const f = this.field();
    if (!f) return [];
    return [
      { label: `${f.hectares} ha` },
      { label: `${f.lots} lotes` },
      { label: f.crop },
      { label: `${f.daysToHarvest} días a cosecha` },
    ];
  });

  readonly yieldDelta = computed(() => {
    const f = this.field();
    if (!f) return null;
    const diff = f.estimatedYieldTPerHa - f.historicalYieldTPerHa;
    const sign = diff >= 0 ? '+' : '';
    return { text: `${sign}${diff.toFixed(1)} t/ha vs histórico`, sign };
  });
}
