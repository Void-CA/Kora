import { Component, signal } from '@angular/core';
import { SlicePipe } from '@angular/common';
import { ActivatedRoute } from '@angular/router';
import { getCycle, CycleDetail as CycleDetailType } from '../../api/kora-api';
import { TimestampPipe } from '../../shared/pipes/timestamp.pipe';

@Component({
  selector: 'app-cycle-detail',
  imports: [TimestampPipe, SlicePipe],
  template: `
    @if (cycle(); as c) {
      <header class="page-header">
        <h1 class="page-title">Ciclo {{ c.summary.id | slice:0:8 }}…</h1>
        <span class="page-period">
          {{ c.summary.period_start | ts }} — {{ c.summary.period_end | ts }}
        </span>
      </header>

      <div class="tabs">
        <button class="tab tab--active">Resumen</button>
        <button class="tab">Actividades</button>
        <button class="tab">Cronograma</button>
        <button class="tab">Sanidad</button>
        <button class="tab" disabled>Suelo</button>
      </div>

      <section class="grid-2">
        <div class="card">
          <h3 class="card__title">Actividades ejecutadas</h3>
          <p class="card__value">{{ c.activities.length }}</p>
        </div>
        <div class="card">
          <h3 class="card__title">Actividades planificadas</h3>
          <p class="card__value">{{ c.planned_activities.length }}</p>
        </div>
      </section>

      @if (c.activities.length > 0) {
        <section>
          <h2 class="section-label">Actividades</h2>
          <table class="table">
            <thead>
              <tr><th>Tipo</th><th>Momento</th><th>Integridad</th></tr>
            </thead>
            <tbody>
              @for (a of c.activities; track a.id) {
                <tr class="row">
                  <td class="cell">{{ a.category }}</td>
                  <td class="cell">{{ a.timestamp | ts }}</td>
                  <td class="cell">
                    @for (i of a.integrity; track i) {
                      <span class="integrity-badge">{{ i }}</span>
                    }
                  </td>
                </tr>
              }
            </tbody>
          </table>
        </section>
      }
    } @else {
      <p class="empty">Cargando…</p>
    }
  `,
  styles: [
    `.page-header { display: flex; align-items: baseline; gap: var(--space-3); margin-bottom: var(--space-6); }`,
    `.page-title { font-size: 1.25rem; font-weight: 600; margin: 0; color: var(--ink); }`,
    `.page-period { font-size: 0.8125rem; color: var(--ink-subtle); }`,
    `.tabs { display: flex; gap: 0; margin-bottom: var(--space-6); border-bottom: 1px solid var(--border); }`,
    `.tab { padding: var(--space-2) var(--space-4); font-size: 0.875rem; border: none; background: none; color: var(--ink-muted); cursor: pointer; border-bottom: 2px solid transparent; transition: color 0.12s, border-color 0.12s; }`,
    `.tab:hover { color: var(--ink); }`,
    `.tab--active { color: var(--ink); font-weight: 500; border-bottom-color: var(--ink); }`,
    `.tab:disabled { opacity: 0.4; cursor: default; }`,
    `.grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-4); margin-bottom: var(--space-6); }`,
    `.card { padding: var(--space-5); background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius-lg); }`,
    `.card__title { font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--ink-subtle); margin: 0 0 var(--space-1); }`,
    `.card__value { font-size: 1.5rem; font-weight: 700; color: var(--ink); margin: 0; }`,
    `.section-label { font-size: 0.875rem; font-weight: 600; margin: 0 0 var(--space-3); color: var(--ink); }`,
    `.table { width: 100%; border-collapse: collapse; }`,
    `.row:hover { background: var(--surface-muted); }`,
    `.cell { padding: var(--space-2) var(--space-4); font-size: 0.875rem; border-bottom: 1px solid var(--border); color: var(--ink); }`,
    `th { padding: var(--space-2) var(--space-4); font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--ink-subtle); text-align: left; border-bottom: 1px solid var(--border-strong); }`,
    `.integrity-badge { display: inline-block; font-size: 0.6875rem; padding: 1px 6px; border-radius: 999px; background: var(--surface-muted); color: var(--ink-muted); margin-right: var(--space-1); }`,
    `.empty { color: var(--ink-subtle); font-size: 0.875rem; }`,
  ],
})
export class CycleDetail {
  readonly cycle = signal<CycleDetailType | null>(null);

  constructor(route: ActivatedRoute) {
    const id = route.snapshot.paramMap.get('id')!;
    getCycle(id).then(data => this.cycle.set(data));
  }
}
