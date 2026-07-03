import { Component, signal, computed } from '@angular/core';
import { StatusSummary, StatusCounts } from './components/status-summary';
import { NextActionCard, NextAction } from './components/next-action-card';
import { AttentionList, AttentionItem } from './components/attention-list';

interface OperationToday {
  status: StatusCounts;
  nextAction: NextAction;
  attention: AttentionItem[];
  contextNote: string;
}

@Component({
  selector: 'app-operation-dashboard',
  imports: [StatusSummary, NextActionCard, AttentionList],
  templateUrl: './operation-dashboard.html',
  styleUrl: './operation-dashboard.scss',
})
export class OperationDashboard {
  // Mock de datos: la fuente real vivirá en un servicio HTTP en Fase 3.
  // No creamos un servicio todavía — señal de que el conocimiento aún no lo exige.
  readonly state = signal<OperationToday>({
    status: { ok: 18, attention: 4, critical: 1 },
    nextAction: {
      title: 'Aplicar fertilizante',
      field: 'Campo Norte',
      lot: 'Lote A',
      crop: 'Maíz',
      when: 'Hoy 09:00',
      priority: 'high',
    },
    attention: [
      { kind: 'delay', text: '2 lotes con retraso en cronograma' },
      { kind: 'budget', text: '1 gasto fuera de presupuesto' },
      { kind: 'weather', text: 'Lluvia prevista en 8h — revisar fumigación' },
    ],
    contextNote: 'Operación · hoy',
  });

  readonly today = computed(() => this.state().contextNote);
}
