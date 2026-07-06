import { Component, signal, input, computed, effect } from '@angular/core';
import { ContextChips } from '../../shared/components/context-chips';
import { CycleTimeline } from '../../shared/components/cycle-timeline';
import { MiniMap } from '../../shared/components/mini-map';
import { StatusPill } from '../../shared/components/status-pill';
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
      { label: f.crop },
      { label: `${f.days_to_harvest} días a cosecha` },
    ];
  });
}
