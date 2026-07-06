import { Component, signal, inject } from '@angular/core';
import { Router } from '@angular/router';
import { OperationWizard, WizardStep } from '../../components/domain/shared/operation-wizard';

const STEPS: WizardStep[] = [
  { question: '¿Qué acabamos de hacer?', key: 'category', type: 'select',
    options: [
      { label: 'Siembra', value: 'Sowing' },
      { label: 'Fertilización', value: 'Maintenance' },
      { label: 'Control sanitario', value: 'SanitaryControl' },
      { label: 'Cosecha', value: 'Harvest' },
    ], required: true },
  { question: '¿En qué campo?', key: 'field', type: 'select', options: [], required: true },
  { question: '¿Cuándo? (timestamp)', key: 'timestamp', type: 'number', placeholder: 'Unix timestamp (ej: 1712345678)', required: true },
  { question: '¿Quién lo hizo?', key: 'worker', type: 'select', options: [], required: false },
  { question: 'Notas (opcional)', key: 'notes', type: 'text', placeholder: 'Ej: Aplicamos 120 kg N/ha', required: false },
];

const BASE = 'http://localhost:8000';

@Component({
  selector: 'app-register-activity',
  imports: [OperationWizard],
  template: `
    <header class="header">
      <h1 class="title">Registrar actividad</h1>
    </header>

    @if (feedback(); as fb) {
      <div class="feedback">
        <span class="feedback__icon">✓</span>
        <div class="feedback__body">
          <p class="feedback__title">Actividad registrada</p>
          @if (fb.activity) { <p class="feedback__line">{{ fb.activity }}</p> }
          @if (fb.integrity && fb.integrity.length > 0) {
            <p class="feedback__line">Integridad: {{ fb.integrity.join(', ') }}</p>
          }
          @if (fb.match) { <p class="feedback__line feedback__line--alert">{{ fb.match }}</p> }
        </div>
        <button class="btn" (click)="done()">Volver al inicio</button>
      </div>
    } @else {
      <app-operation-wizard [steps]="steps" (complete)="onComplete($event)" />
    }
  `,
  styles: [`
    .header { margin-bottom: var(--space-6); }
    .title { font-size: 1.25rem; font-weight: 600; margin: 0; color: var(--ink); letter-spacing: -0.02em; }
    .feedback { display: flex; flex-direction: column; gap: var(--space-4); padding: var(--space-6); background: var(--surface-raised); border: 1px solid var(--border); border-radius: var(--radius); max-width: 480px; }
    .feedback__icon { font-size: 1.5rem; color: var(--state-ok); }
    .feedback__title { font-size: 1rem; font-weight: 600; margin: 0; color: var(--ink); }
    .feedback__line { font-size: 0.8125rem; color: var(--ink-muted); margin: var(--space-1) 0; }
    .feedback__line--alert { color: var(--state-attention); font-weight: 500; }
    .btn { padding: var(--space-2) var(--space-5); border-radius: var(--radius-sm); font-size: 0.8125rem; font-weight: 500; border: 1px solid var(--border); background: transparent; color: var(--ink); cursor: pointer; align-self: flex-start; }
    .btn:hover { background: var(--surface-muted); }
  `],
})
export class RegisterActivityPage {
  private router = inject(Router);
  readonly steps = STEPS;
  readonly feedback = signal<{ activity?: string; integrity?: string[]; match?: string } | null>(null);

  // Map field name → cycle id
  private fieldToCycle: Record<string, string> = {};

  constructor() {
    // Load fields for select
    fetch(`${BASE}/api/fields`)
      .then(r => r.json())
      .then((fields: any[]) => {
        const fieldStep = this.steps.find(s => s.key === 'field');
        if (fieldStep) {
          fieldStep.options = fields.map(f => ({ label: `${f.name} (${f.crop})`, value: f.id }));
          fields.forEach(f => { this.fieldToCycle[f.id] = f.cycle_id; });
        }
      })
      .catch(() => {});

    // Load workers for select
    fetch(`${BASE}/api/payroll/workers`)
      .then(r => r.json())
      .then((workers: any[]) => {
        const workerStep = this.steps.find(s => s.key === 'worker');
        if (workerStep) {
          workerStep.options = workers.map((w: any) => ({ label: w.name, value: w.id }));
        }
      })
      .catch(() => {});
  }

  async onComplete(answers: Record<string, string>): Promise<void> {
    const cycleId = this.fieldToCycle[answers['field']];
    if (!cycleId) {
      this.feedback.set({ activity: 'Error: no se encontró la campaña activa para este campo' });
      return;
    }
    try {
      const r = await fetch(`${BASE}/api/cycles/${cycleId}/activities`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          cycle_id: cycleId,
          timestamp: parseInt(answers['timestamp']) || Math.floor(Date.now() / 1000),
          category: answers['category'],
          notes: answers['notes'] || undefined,
          mode: 'suggested',
        }),
      });
      if (!r.ok) throw new Error('HTTP ' + r.status);
      const data = await r.json();
      this.feedback.set({
        activity: `${answers['category']} registrada en ${answers['field']}`,
        integrity: data.integrity || [],
        match: data.suggestions?.length > 0 ? 'Coincide con actividad planificada' : undefined,
      });
    } catch (e) {
      this.feedback.set({ activity: 'Error al registrar: ' + String(e) });
    }
  }

  done(): void {
    this.router.navigate(['/']);
  }
}
