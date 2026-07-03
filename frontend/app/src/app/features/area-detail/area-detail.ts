import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';
import { ActivatedRoute } from '@angular/router';
import { getAreaDashboard, AreaDashboard } from '../../api/kora-api';
import { FieldMap } from '../field-map/field-map';
import { MoneyPipe } from '../../shared/pipes/money.pipe';
import { TimestampPipe } from '../../shared/pipes/timestamp.pipe';

@Component({
  selector: 'app-area-detail',
  imports: [RouterLink, FieldMap, MoneyPipe, TimestampPipe],
  template: `
    @if (d(); as d) {
      <header class="page-header">
        <h1 class="page-title">{{ d.area_name }}</h1>
      </header>

      <div class="totals">
        <div class="total-card"><span class="total-label">Presupuesto total</span><span class="total-value">{{ d.totals.total_baseline | money }}</span></div>
        <div class="total-card"><span class="total-label">Gastado</span><span class="total-value">{{ d.totals.total_spent | money }}</span></div>
        <div class="total-card"><span class="total-label">Variación</span><span class="total-value">{{ d.totals.total_variance | money }}</span></div>
        <div class="total-card"><span class="total-label">Ciclos / Incidencias</span><span class="total-value">{{ d.totals.cycle_count }} / {{ d.totals.incidence_count }}</span></div>
      </div>

      <app-field-map [caption]="d.area_name" />

      @if (d.cycles.length > 0) {
        <section>
          <h2 class="section-label">Ciclos</h2>
          <table class="table">
            <thead><tr><th>Período</th><th>Act.</th><th>Presupuesto</th><th>Gastado</th><th>Variación</th></tr></thead>
            <tbody>
              @for (c of d.cycles; track c.id) {
                <tr class="row" [routerLink]="['/ciclos', c.id]">
                  <td class="cell">{{ c.period_start | ts }} — {{ c.period_end | ts }}</td>
                  <td class="cell">{{ c.activity_count }}</td>
                  <td class="cell">{{ c.budget_baseline | money }}</td>
                  <td class="cell">{{ c.budget_spent | money }}</td>
                  <td class="cell">{{ c.budget_variance | money }}</td>
                </tr>
              }
            </tbody>
          </table>
        </section>
      }

      @if (d.soil_analyses.length > 0) {
        <section>
          <h2 class="section-label">Análisis de suelo</h2>
          <table class="table">
            <thead><tr><th>Fecha</th><th>Calidad</th><th>Métricas</th><th>Costo</th></tr></thead>
            <tbody>
              @for (s of d.soil_analyses; track s.id) {
                <tr><td class="cell">{{ s.sampled_at | ts }}</td><td class="cell">{{ s.quality }}</td><td class="cell">{{ s.metric_count }}</td><td class="cell">{{ s.cost | money }}</td></tr>
              }
            </tbody>
          </table>
        </section>
      }

      @if (d.incidences.length > 0) {
        <section>
          <h2 class="section-label">Incidencias sanitarias</h2>
          <table class="table">
            <thead><tr><th>Tipo</th><th>Severidad</th><th>Descripción</th><th>Impacto</th></tr></thead>
            <tbody>
              @for (i of d.incidences; track i.id) {
                <tr><td class="cell">{{ i.kind }}</td><td class="cell">{{ i.severity }}</td><td class="cell">{{ i.description }}</td><td class="cell">{{ i.economic_impact | money }}</td></tr>
              }
            </tbody>
          </table>
        </section>
      }
    } @else {
      <p class="empty">Cargando…</p>
    }
  `,
  styles: [`
    .page-header { margin-bottom: var(--space-6); }
    .page-title { font-size: 1.25rem; font-weight: 600; margin: 0; color: var(--ink); }
    .totals { display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-4); margin-bottom: var(--space-6); }
    .total-card { padding: var(--space-4); background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius-lg); }
    .total-label { display: block; font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--ink-subtle); margin-bottom: var(--space-1); }
    .total-value { font-size: 1.125rem; font-weight: 700; color: var(--ink); }
    .section-label { font-size: 0.875rem; font-weight: 600; margin: var(--space-6) 0 var(--space-3); color: var(--ink); }
    .table { width: 100%; border-collapse: collapse; }
    .row { cursor: pointer; }
    .row:hover { background: var(--surface-muted); }
    .cell { padding: var(--space-2) var(--space-4); font-size: 0.875rem; border-bottom: 1px solid var(--border); color: var(--ink); }
    th { padding: var(--space-2) var(--space-4); font-size: 0.75rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--ink-subtle); text-align: left; border-bottom: 1px solid var(--border-strong); }
    .empty { color: var(--ink-subtle); font-size: 0.875rem; }
  `],
})
export class AreaDetail {
  readonly d = signal<AreaDashboard | null>(null);

  constructor(route: ActivatedRoute) {
    const id = route.snapshot.paramMap.get('id')!;
    getAreaDashboard(id).then(data => this.d.set(data));
  }
}
