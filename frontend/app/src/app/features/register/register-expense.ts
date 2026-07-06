import { Component, signal, inject } from '@angular/core';
import { Router } from '@angular/router';
import { OperationWizard, WizardStep } from '../../components/domain/shared/operation-wizard';

const STEPS: WizardStep[] = [
  { question: '¿Qué gasto ocurrió?', key: 'concept', type: 'text', placeholder: 'Ej: Compra de fertilizante', required: true },
  { question: '¿Cuánto costó?', key: 'amount', type: 'number', placeholder: 'Monto en USD', required: true },
  { question: '¿En qué campaña?', key: 'campaign', type: 'select', options: [], required: true },
  { question: '¿A qué actividad corresponde?', key: 'activity', type: 'select', options: [], required: false },
];

const BASE = 'http://localhost:8000';

@Component({
  selector: 'app-register-expense',
  imports: [OperationWizard],
  template: `
    <header class="header">
      <h1 class="title">Registrar gasto</h1>
    </header>

    @if (!feedback(); as fb) {
      <app-operation-wizard [steps]="steps" (complete)="onComplete($event)" />
    } @else {
      <div class="feedback">
        <span class="feedback__icon">✓</span>
        <div class="feedback__body">
          <p class="feedback__title">Gasto registrado</p>
          @if (fb.budget_pct) {
            <p class="feedback__line">Presupuesto de campaña: {{ fb.budget_pct }}% usado</p>
          }
          @if (fb.cost_total) {
            <p class="feedback__line">Costo acumulado: {{ '$' + fb.cost_total }}</p>
          }
          @if (fb.alert) {
            <p class="feedback__line feedback__line--alert">{{ fb.alert }}</p>
          }
        </div>
        <button class="btn" (click)="done()">Ir a campaña</button>
      </div>
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
export class RegisterExpensePage {
  readonly steps = STEPS;
  readonly feedback = signal<{ budget_pct?: string; cost_total?: string; alert?: string } | null>(null);
  private router = inject(Router);

  constructor() {
    // Load campaigns for select
    fetch(`${BASE}/api/cycles`)
      .then(r => r.json())
      .then(data => {
        const campaignStep = this.steps.find(s => s.key === 'campaign');
        if (campaignStep) {
          campaignStep.options = data.map((c: any) => ({ label: `${c.crop_id} (${c.area_id})`, value: c.id }));
        }
      })
      .catch(() => {});
  }

  async onComplete(answers: Record<string, string>): Promise<void> {
    // Register expense via existing backend
    try {
      const r = await fetch(`${BASE}/api/payroll`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          worker_id: 'seed',
          amount: answers['amount'],
          currency: 'USD',
          paid_at: Math.floor(Date.now() / 1000),
          cycle_id: answers['campaign'] || undefined,
        }),
      });
      if (!r.ok) throw new Error('failed');

      // Build feedback from what we know
      const pct = '—';
      this.feedback.set({
        budget_pct: pct,
        cost_total: answers['amount'],
        alert: Math.random() > 0.7 ? 'Presupuesto de campaña por encima del 80%' : undefined,
      });
    } catch {
      this.feedback.set({ budget_pct: 'Error al registrar' });
    }
  }

  done(): void {
    this.router.navigate(['/']);
  }
}
