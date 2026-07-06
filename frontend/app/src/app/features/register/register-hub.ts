import { Component } from '@angular/core';
import { RouterLink } from '@angular/router';

const OPERATIONS = [
  { id: 'activity', label: 'Actividad', desc: 'Registrar una labor realizada en el campo', icon: 'check' },
  { id: 'expense', label: 'Gasto', desc: 'Registrar un costo asociado a una campaña', icon: 'dollar' },
];

@Component({
  selector: 'app-register',
  imports: [RouterLink],
  template: `
    <header class="header">
      <h1 class="title">¿Qué ocurrió?</h1>
      <p class="subtitle">Seleccioná el tipo de operación para empezar a registrar</p>
    </header>
    <div class="grid">
      @for (op of operations; track op.id) {
        <a class="card" [routerLink]="['/register', op.id]">
          <span class="card__icon">{{ iconMap[op.icon] }}</span>
          <span class="card__label">{{ op.label }}</span>
          <span class="card__desc">{{ op.desc }}</span>
        </a>
      }
    </div>
  `,
  styles: [`
    .header { margin-bottom: var(--space-8); }
    .title { font-size: 1.25rem; font-weight: 600; margin: 0; color: var(--ink); letter-spacing: -0.02em; }
    .subtitle { font-size: 0.875rem; color: var(--ink-muted); margin: var(--space-2) 0 0; }
    .grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: var(--space-4); }
    .card {
      display: flex; flex-direction: column; gap: var(--space-2);
      padding: var(--space-5); background: var(--surface-raised);
      border: 1px solid var(--border); border-radius: var(--radius);
      text-decoration: none; color: inherit; cursor: pointer;
      transition: border-color var(--ease);
    }
    .card:hover { border-color: var(--border-strong); }
    .card__icon { font-size: 1.25rem; }
    .card__label { font-size: 0.875rem; font-weight: 600; color: var(--ink); }
    .card__desc { font-size: 0.75rem; color: var(--ink-muted); }
  `],
})
export class RegisterHub {
  readonly operations = OPERATIONS;
  readonly iconMap: Record<string, string> = { check: '✓', dollar: '$', plus: '+' };
}
