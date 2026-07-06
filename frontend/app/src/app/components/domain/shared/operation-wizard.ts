import { Component, input, output, signal, computed } from '@angular/core';

export interface WizardStep {
  question: string;
  key: string;
  type: 'text' | 'number' | 'select' | 'date';
  options?: { label: string; value: string }[];
  placeholder?: string;
  required?: boolean;
}

@Component({
  selector: 'app-operation-wizard',
  template: `
    <div class="wizard">
      @if (!completed()) {
        <div class="wizard__step">
          <p class="wizard__question">{{ currentStep().question }}</p>
          @switch (currentStep().type) {
            @case ('select') {
              <select class="input" #sel (change)="answer(sel.value)">
                <option value="">Seleccionar…</option>
                @for (opt of currentStep().options ?? []; track opt.value) {
                  <option [value]="opt.value" [selected]="answers()[currentStep().key] === opt.value">{{ opt.label }}</option>
                }
              </select>
            }
            @case ('number') {
              <input class="input" type="number" #num [placeholder]="currentStep().placeholder ?? ''"
                     [value]="answers()[currentStep().key] ?? ''" (change)="answer(num.value)" />
            }
            @default {
              <input class="input" type="text" #txt [placeholder]="currentStep().placeholder ?? ''"
                     [value]="answers()[currentStep().key] ?? ''" (change)="answer(txt.value)" />
            }
          }
          <div class="wizard__nav">
            @if (stepIndex() > 0) {
              <button class="btn btn--ghost" (click)="prev()">Atrás</button>
            }
            @if (stepIndex() < steps().length - 1) {
              <button class="btn btn--primary" (click)="next()" [disabled]="!canProceed()">Siguiente</button>
            } @else {
              <button class="btn btn--primary" (click)="finish()" [disabled]="!canProceed()">Registrar</button>
            }
          </div>
        </div>
      }
    </div>
  `,
  styles: [`
    .wizard { max-width: 480px; }
    .wizard__step { display: flex; flex-direction: column; gap: var(--space-4); }
    .wizard__question { font-size: 1.125rem; font-weight: 500; color: var(--ink); margin: 0; }
    .input { padding: var(--space-3) var(--space-4); font-size: 0.875rem; border: 1px solid var(--border); border-radius: var(--radius-sm); background: var(--surface-raised); color: var(--ink); width: 100%; }
    .input:focus { outline: none; border-color: var(--border-focus); }
    .wizard__nav { display: flex; gap: var(--space-3); margin-top: var(--space-2); }
    .btn { padding: var(--space-2) var(--space-5); border-radius: var(--radius-sm); font-size: 0.8125rem; font-weight: 500; border: 1px solid transparent; cursor: pointer; transition: background var(--ease); }
    .btn--primary { background: var(--btn-primary); color: var(--btn-primary-text); }
    .btn--primary:hover { background: var(--btn-primary-hover); }
    .btn--primary:disabled { opacity: 0.4; pointer-events: none; }
    .btn--ghost { background: transparent; color: var(--ink-muted); border-color: var(--border); }
    .btn--ghost:hover { background: var(--surface-muted); }
  `],
})
export class OperationWizard {
  readonly steps = input.required<WizardStep[]>();
  readonly complete = output<Record<string, string>>();

  readonly stepIndex = signal(0);
  readonly answers = signal<Record<string, string>>({});
  readonly completed = signal(false);

  readonly currentStep = computed(() => this.steps()[this.stepIndex()]);

  readonly canProceed = computed(() => {
    const s = this.currentStep();
    if (!s.required) return true;
    return !!this.answers()[s.key];
  });

  answer(value: string): void {
    this.answers.update(a => ({ ...a, [this.currentStep().key]: value }));
  }

  next(): void {
    if (this.stepIndex() < this.steps().length - 1) {
      this.stepIndex.update(i => i + 1);
    }
  }

  prev(): void {
    if (this.stepIndex() > 0) {
      this.stepIndex.update(i => i - 1);
    }
  }

  finish(): void {
    this.complete.emit(this.answers());
    this.completed.set(true);
  }
}
