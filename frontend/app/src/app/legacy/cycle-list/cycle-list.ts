import { Component, signal } from '@angular/core';
import { SlicePipe } from '@angular/common';
import { RouterLink } from '@angular/router';
import { getCycles, CycleSummary } from '../../api/kora-api';
import { TimestampPipe } from '../../shared/pipes/timestamp.pipe';

@Component({
  selector: 'app-cycle-list',
  imports: [RouterLink, TimestampPipe, SlicePipe],
  template: `
    <header class="page-header">
      <h1 class="page-title">Ciclos</h1>
      <span class="page-count">{{ cycles()?.length ?? 0 }} ciclos</span>
    </header>

    @if (cycles(); as list) {
      <table class="table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Inicio</th>
            <th>Fin</th>
            <th>Actividades</th>
          </tr>
        </thead>
        <tbody>
          @for (c of list; track c.id) {
            <tr class="row" [routerLink]="['/ciclos', c.id]">
              <td class="cell cell--id">{{ c.id | slice:0:8 }}…</td>
              <td class="cell">{{ c.period_start | ts }}</td>
              <td class="cell">{{ c.period_end | ts }}</td>
              <td class="cell">{{ c.activity_count }}</td>
            </tr>
          }
        </tbody>
      </table>
    } @else {
      <p class="empty">Cargando ciclos…</p>
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
    .page-count {
      font-size: 0.8125rem;
      color: var(--ink-subtle);
    }
    .table {
      width: 100%;
      border-collapse: collapse;
    }
    .row {
      cursor: pointer;
      transition: background 0.12s;
    }
    .row:hover { background: var(--surface-muted); }
    .cell {
      padding: var(--space-3) var(--space-4);
      font-size: 0.875rem;
      border-bottom: 1px solid var(--border);
      color: var(--ink);
    }
    .cell--id {
      font-family: var(--font-mono);
      font-size: 0.8125rem;
      color: var(--ink-muted);
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
    .empty {
      color: var(--ink-subtle);
      font-size: 0.875rem;
    }
  `,
})
export class CycleList {
  readonly cycles = signal<CycleSummary[] | null>(null);

  constructor() {
    getCycles().then(data => this.cycles.set(data));
  }
}
