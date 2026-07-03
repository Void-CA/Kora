import { Component, signal, computed, input } from '@angular/core';
import { ContextChips } from '../../shared/context-chips';
import { CycleTimeline, CyclePhase } from '../../shared/cycle-timeline';
import { MiniMap } from '../../shared/mini-map';
import { StatusPill, HealthStatus } from '../../shared/status-pill';

interface FieldHealth {
  status: HealthStatus;
  label: string;
  value: string;
}

interface FieldMock {
  name: string;
  hectares: number;
  lots: number;
  crop: string;
  growth: string;
  lastActivity: string;
  health: FieldHealth[];
  phases: CyclePhase[];
}

@Component({
  selector: 'app-field-card',
  imports: [ContextChips, CycleTimeline, MiniMap, StatusPill],
  templateUrl: './field-card.html',
  styleUrl: './field-card.scss',
})
export class FieldCard {
  // Mock: tres campos de muestra para que la ruta /campos/:id tenga datos.
  // En Fase 3 esto se reemplaza por una llamada HTTP.
  readonly fields = signal<Record<string, FieldMock>>({
    'campo-norte': {
      name: 'Campo Norte',
      hectares: 12,
      lots: 3,
      crop: 'Maíz',
      growth: 'Crecimiento',
      lastActivity: 'hace 3 días',
      health: [
        { status: 'ok', label: 'Estado salud', value: 'Saludable' },
        { status: 'attention', label: 'Cronograma', value: '2 pendientes' },
        { status: 'ok', label: 'Presupuesto', value: '35% usado' },
      ],
      phases: [
        { name: 'Preparación', status: 'done' },
        { name: 'Siembra', status: 'done' },
        { name: 'Crecimiento', status: 'current' },
        { name: 'Floración', status: 'pending' },
        { name: 'Cosecha', status: 'pending' },
      ],
    },
    'campo-sur': {
      name: 'Campo Sur',
      hectares: 8,
      lots: 2,
      crop: 'Frijol',
      growth: 'Siembra',
      lastActivity: 'hoy',
      health: [
        { status: 'ok', label: 'Estado salud', value: 'Saludable' },
        { status: 'ok', label: 'Cronograma', value: 'Al día' },
        { status: 'ok', label: 'Presupuesto', value: '20% usado' },
      ],
      phases: [
        { name: 'Preparación', status: 'done' },
        { name: 'Siembra', status: 'current' },
        { name: 'Crecimiento', status: 'pending' },
        { name: 'Floración', status: 'pending' },
        { name: 'Cosecha', status: 'pending' },
      ],
    },
  });

  // Vinculado al parámetro :id de la ruta vía withComponentInputBinding.
  readonly id = input.required<string>();

  readonly current = computed<FieldMock | null>(() => {
    return this.fields()[this.id()] ?? null;
  });

  readonly contextItems = computed(() => {
    const f = this.current();
    if (!f) return [];
    return [
      { label: `${f.hectares} ha` },
      { label: `${f.lots} lotes` },
      { label: f.crop },
    ];
  });
}
