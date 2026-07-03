import { Component, signal } from '@angular/core';
import { listWorkers, WorkerSummary } from '../../api/kora-api';

@Component({
  selector: 'app-workers',
  imports: [],
  template: `
    <header class="page-header">
      <h1 class="page-title">Personal</h1>
      <span class="page-count">{{ workers()?.length ?? 0 }} trabajadores</span>
    </header>

    @if (workers(); as list) {
      <table class="table">
        <thead>
          <tr>
            <th>Nombre</th>
            <th>Rol</th>
            <th>Activo</th>
          </tr>
        </thead>
        <tbody>
          @for (w of list; track w.id) {
            <tr class="row">
              <td class="cell">{{ w.name }}</td>
              <td class="cell">{{ w.role ?? '—' }}</td>
              <td class="cell">
                <span class="badge" [class.badge--yes]="w.active"
                      [class.badge--no]="!w.active">
                  {{ w.active ? 'Sí' : 'No' }}
                </span>
              </td>
            </tr>
          }
        </tbody>
      </table>
    } @else {
      <p class="empty">Cargando personal…</p>
    }
  `,
  styles: `
    .page-header {
      display: flex;
      align-items: baseline;
      gap: var(--space-3);
      margin-bottom: var(--space-8);
    }
    .page-title {
      font-size: 1.25rem;
      font-weight: 600;
      margin: 0;
      color: var(--ink);
    }
    .page-count { font-size: 0.8125rem; color: var(--ink-subtle); }
    .table { width: 100%; border-collapse: collapse; }
    .row:hover { background: var(--surface-muted); }
    .cell {
      padding: var(--space-3) var(--space-4);
      font-size: 0.875rem;
      border-bottom: 1px solid var(--border);
      color: var(--ink);
    }
    th {
      padding: var(--space-2) var(--space-4);
      font-size: 0.75rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: var(--ink-subtle);
      text-align: left;
      border-bottom: 1px solid var(--border-strong);
    }
    .badge {
      font-size: 0.75rem;
      padding: 2px 8px;
      border-radius: 999px;
    }
    .badge--yes { background: #f0fdf4; color: #15803d; }
    .badge--no { background: #fef2f2; color: #b91c1c; }
    .empty { color: var(--ink-subtle); font-size: 0.875rem; }
  `,
})
export class WorkersPage {
  readonly workers = signal<WorkerSummary[] | null>(null);

  constructor() {
    listWorkers().then(data => this.workers.set(data));
  }
}
