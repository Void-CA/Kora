import { Component, signal } from '@angular/core';
import { SlicePipe } from '@angular/common';
import { ActivatedRoute } from '@angular/router';
import {
  getCycle, getProfitability, getCycleVariance, getCycleTimeline,
  getSchedule, addPlannedActivity, registerActivity,
  listRevenueForCycle, createBudget,
  listIncidencesForCycle, registerIncidence,
  listSoilForArea, linkSoilToCycle,
  CycleDetail as CycleDetailType, Profitability, CycleVariance,
  ScheduleSummary, PlannedSuggestion,
} from '../../api/kora-api';
import { TimestampPipe } from '../../shared/pipes/timestamp.pipe';
import { MoneyPipe } from '../../shared/pipes/money.pipe';
import { VarianceChart } from '../variance-chart/variance-chart';
import { CropCycleHeader } from '../../components/domain/campaign/crop-cycle-header';
import { ProfitabilitySummary } from '../../components/domain/campaign/profitability-summary';
import { BudgetBar } from '../../components/domain/campaign/budget-bar';

@Component({
  selector: 'app-cycle-detail',
  imports: [SlicePipe, TimestampPipe, MoneyPipe, VarianceChart, CropCycleHeader, ProfitabilitySummary, BudgetBar],
  template: `
    @if (cycle(); as c) {
      <kora-crop-cycle-header
        crop="Campaña" [field]="c.summary.area_id"
        [period]="(c.summary.period_start | ts) + ' — ' + (c.summary.period_end | ts)"
        status="active" />

      <div class="tabs">
        @for (tab of tabs; track tab.id; let i = $index) {
          <button class="tab" [class.tab--active]="activeTab() === i"
                  (click)="activeTab.set(i)">
            {{ tab.label }}
          </button>
        }
      </div>

      <!-- Tab 0: Resumen -->
      @if (activeTab() === 0) {
        @if (profit(); as p) {
          <div class="summary-section">
            <kora-profitability-summary
              [baseline]="p.baseline" [spent]="p.spent"
              [revenue]="p.revenue" [profit]="p.profit"
              [roi]="p.roi_percent + '%'" profitSign="+" />
          </div>
        }
        @if (variance(); as v) {
          <app-variance-chart [variance]="v" />
        }
      }

      <!-- Tab 1: Cronograma -->
      @if (activeTab() === 1) {
        @if (schedule(); as s) {
          <table class="table">
            <thead><tr><th>Actividad</th><th>Día relativo</th><th>Estado</th></tr></thead>
            <tbody>
              @for (p of s.planned; track p.id) {
                <tr><td class="cell">{{ p.category }}</td><td class="cell">+{{ p.relative_day }}</td><td class="cell">Planificado</td></tr>
              }
            </tbody>
          </table>
        }
        <details class="form-wrap">
          <summary class="form-toggle">+ Agregar actividad planificada</summary>
          <div class="form-row">
            <select #cat class="input" style="flex:1">
              <option value="Sowing">Siembra</option>
              <option value="Maintenance">Mantenimiento</option>
              <option value="SanitaryControl">Control sanitario</option>
              <option value="Harvest">Cosecha</option>
            </select>
            <input #day type="number" class="input" placeholder="Día relativo" style="width:120px" />
            <button class="btn btn--primary" (click)="addPlanned(cat.value, +day.value)">Agregar</button>
          </div>
        </details>
      }

      <!-- Tab 2: Actividades -->
      @if (activeTab() === 2) {
        @if (c.activities.length > 0) {
          <table class="table">
            <thead><tr><th>Tipo</th><th>Momento</th><th>Integridad</th></tr></thead>
            <tbody>
              @for (a of c.activities; track a.id) {
                <tr><td class="cell">{{ a.category }}</td><td class="cell">{{ a.timestamp | ts }}</td>
                  <td class="cell">@for (i of a.integrity; track i) { <span class="badge">{{ i }}</span> }</td>
                </tr>
              }
            </tbody>
          </table>
        } @else {
          <p class="empty">Sin actividades registradas.</p>
        }
        <details class="form-wrap">
          <summary class="form-toggle">+ Registrar actividad</summary>
          <div class="form-row">
            <select #actCat class="input" style="flex:1">
              <option value="Sowing">Siembra</option>
              <option value="Maintenance">Mantenimiento</option>
              <option value="SanitaryControl">Control sanitario</option>
              <option value="Harvest">Cosecha</option>
            </select>
            <input #actTs type="number" class="input" placeholder="Timestamp" style="width:160px" />
            <select #actMode class="input" style="width:130px">
              <option value="suggested">Sugerido</option>
              <option value="emergent">Emergente</option>
            </select>
            <button class="btn btn--primary" (click)="doRegisterActivity(actCat.value, +actTs.value, actMode.value)">Registrar</button>
          </div>
          @if (lastSuggestions().length > 0) {
            <div class="suggestions">
              <p class="suggestions__label">¿Corresponde a una actividad planificada?</p>
              @for (s of lastSuggestions(); track s.planned_id) {
                <button class="btn btn--ghost" (click)="confirmMatch(s)">{{ s.category }} día +{{ s.relative_day }} (desviación {{ s.drift_days }}s)</button>
              }
            </div>
          }
        </details>
      }

      <!-- Tab 3: Finanzas -->
      @if (activeTab() === 3) {
        @if (profit(); as p) {
          <div class="grid-4">
            <div class="metric"><span class="metric__label">Presupuesto</span><span class="metric__value">{{ p.baseline | money }}</span></div>
            <div class="metric"><span class="metric__label">Gastado</span><span class="metric__value">{{ p.spent | money }}</span></div>
            <div class="metric"><span class="metric__label">Ingresos</span><span class="metric__value">{{ p.revenue | money }}</span></div>
            <div class="metric"><span class="metric__label">Ganancia</span><span class="metric__value">{{ p.profit | money }}</span></div>
          </div>
        }
        @if (revenues(); as r) {
          <h3 class="section-label">Ingresos</h3>
          <table class="table">
            <thead><tr><th>Fuente</th><th>Monto</th><th>Fecha</th></tr></thead>
            <tbody>@for (r of revenues(); track r.id) {<tr><td class="cell">{{ r.source }}</td><td class="cell">{{ r.amount | money }}</td><td class="cell">{{ r.received_at | ts }}</td></tr>}</tbody>
          </table>
        }
        <details class="form-wrap">
          <summary class="form-toggle">+ Registrar ingreso</summary>
          <div class="form-row">
            <input #revAmt type="text" class="input" placeholder="Monto" style="width:120px" />
            <select #revSrc class="input" style="width:120px"><option value="Harvest">Cosecha</option><option value="Sale">Venta</option></select>
            <input #revTs type="number" class="input" placeholder="Timestamp" style="width:160px" />
            <button class="btn btn--primary" (click)="addRevenue(revAmt.value, revSrc.value, +revTs.value)">Agregar</button>
          </div>
        </details>
        <details class="form-wrap">
          <summary class="form-toggle">+ Crear presupuesto</summary>
          <div class="form-col">
            <div class="form-row">
              <input #budgetBase type="text" class="input" placeholder="Monto total" style="width:140px" />
              <input #budgetStart type="number" class="input" placeholder="Inicio" style="flex:1" />
              <input #budgetEnd type="number" class="input" placeholder="Fin" style="flex:1" />
            </div>
            <button class="btn btn--primary" (click)="doCreateBudget(budgetBase.value, +budgetStart.value, +budgetEnd.value)">Crear presupuesto</button>
          </div>
        </details>

      }

      <!-- Tab 4: Sanidad -->
      @if (activeTab() === 4) {
        @if (incidences(); as list) {
          <table class="table">
            <thead><tr><th>Tipo</th><th>Severidad</th><th>Descripción</th><th>Impacto</th></tr></thead>
            <tbody>@for (i of list; track i.id) {<tr><td class="cell">{{ i.kind }}</td><td class="cell">{{ i.severity }}</td><td class="cell">{{ i.description }}</td><td class="cell">{{ i.economic_impact | money }}</td></tr>}</tbody>
          </table>
        }
        <details class="form-wrap">
          <summary class="form-toggle">+ Registrar incidencia</summary>
          <div class="form-col">
            <div class="form-row">
              <select #incKind class="input" style="flex:1"><option value="Pest">Plaga</option><option value="Disease">Enfermedad</option></select>
              <select #incSev class="input" style="flex:1"><option value="Low">Baja</option><option value="Medium">Media</option><option value="High">Alta</option><option value="Critical">Crítica</option></select>
            </div>
            <input #incDesc class="input" placeholder="Descripción" />
            <input #incAct class="input" placeholder="Acción tomada" />
            <div class="form-row">
              <input #incTs type="number" class="input" placeholder="Timestamp" style="width:160px" />
              <input #incImpact type="text" class="input" placeholder="Impacto económico (opcional)" style="flex:1" />
              <button class="btn btn--primary" (click)="doRegisterIncidence(incKind.value, incSev.value, incDesc.value, incAct.value, +incTs.value, incImpact.value)">Registrar</button>
            </div>
          </div>
        </details>
      }

      <!-- Tab 5: Suelo -->
      @if (activeTab() === 5) {
        @if (soilAnalyses(); as list) {
          <table class="table">
            <thead><tr><th>Fecha</th><th>Calidad</th><th>Métricas</th><th>Costo</th></tr></thead>
            <tbody>@for (s of list; track s.id) {<tr><td class="cell">{{ s.sampled_at | ts }}</td><td class="cell">{{ s.quality }}</td><td class="cell">{{ s.metric_count }}</td><td class="cell">{{ s.cost | money }}</td></tr>}</tbody>
          </table>
        }
        <details class="form-wrap">
          <summary class="form-toggle">+ Vincular análisis de suelo</summary>
          <div class="form-row">
            <input #soilId class="input" placeholder="ID del análisis" style="flex:1" />
            <select #soilKind class="input" style="width:130px"><option value="Previo">Previo</option><option value="Seguimiento">Seguimiento</option><option value="Posterior">Posterior</option></select>
            <button class="btn btn--primary" (click)="doLinkSoil(soilId.value, soilKind.value)">Vincular</button>
          </div>
        </details>
      }

    } @else {
      <p class="empty">Cargando…</p>
    }
  `,
  styles: [`
    .page-header { display: flex; align-items: baseline; gap: var(--space-3); margin-bottom: var(--space-4); }
    .page-title { font-size: 1.25rem; font-weight: 600; margin: 0; color: var(--ink); }
    .page-period { font-size: 0.8125rem; color: var(--ink-subtle); }
    .tabs { display: flex; gap: 0; margin-bottom: var(--space-6); border-bottom: 1px solid var(--border); overflow-x: auto; }
    .tab { padding: var(--space-2) var(--space-4); font-size: 0.8125rem; border: none; background: none; color: var(--ink-muted); cursor: pointer; white-space: nowrap; border-bottom: 2px solid transparent; transition: color 0.12s, border-color 0.12s; }
    .tab:hover { color: var(--ink); }
    .tab--active { color: var(--ink); font-weight: 500; border-bottom-color: var(--ink); }
    .tab:disabled { opacity: 0.4; cursor: default; }
    .grid-4 { display: grid; grid-template-columns: repeat(auto-fill, minmax(160px, 1fr)); gap: var(--space-4); margin-bottom: var(--space-6); }
    .metric { padding: var(--space-4); background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius-lg); }
    .metric__label { display: block; font-size: 0.6875rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--ink-subtle); margin-bottom: var(--space-1); }
    .metric__value { font-size: 1rem; font-weight: 700; color: var(--ink); }
    .section-label { font-size: 0.875rem; font-weight: 600; margin: var(--space-4) 0 var(--space-2); color: var(--ink); }
    .table { width: 100%; border-collapse: collapse; margin-bottom: var(--space-4); }
    .cell { padding: var(--space-2) var(--space-3); font-size: 0.8125rem; border-bottom: 1px solid var(--border); color: var(--ink); }
    th { padding: var(--space-2) var(--space-3); font-size: 0.6875rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--ink-subtle); text-align: left; border-bottom: 1px solid var(--border-strong); }
    .badge { display: inline-block; font-size: 0.6875rem; padding: 1px 6px; border-radius: 999px; background: var(--surface-muted); color: var(--ink-muted); margin-right: var(--space-1); }
    .empty { color: var(--ink-subtle); font-size: 0.875rem; padding: var(--space-6) 0; }
    .form-wrap { margin-top: var(--space-4); border: 1px solid var(--border); border-radius: var(--radius); padding: var(--space-3); background: var(--surface); }
    .form-toggle { font-size: 0.8125rem; font-weight: 500; color: var(--state-info); cursor: pointer; }
    .form-row { display: flex; gap: var(--space-2); margin-top: var(--space-3); align-items: center; flex-wrap: wrap; }
    .form-col { display: flex; flex-direction: column; gap: var(--space-2); margin-top: var(--space-3); }
    .input { padding: var(--space-2) var(--space-3); font-size: 0.8125rem; border: 1px solid var(--border); border-radius: var(--radius); background: var(--surface); color: var(--ink); }
    .btn { padding: var(--space-2) var(--space-4); border-radius: var(--radius); font-size: 0.8125rem; font-weight: 500; border: 1px solid transparent; cursor: pointer; }
    .btn--primary { background: var(--ink); color: var(--surface); }
    .btn--ghost { background: transparent; color: var(--ink-muted); border-color: var(--border); }
    .suggestions { margin-top: var(--space-3); display: flex; flex-direction: column; gap: var(--space-2); }
    .suggestions__label { font-size: 0.75rem; color: var(--ink-muted); margin: 0; }
  `],
})
export class CycleDetail {
  readonly activeTab = signal(0);
  readonly tabs = [
    { id: 0, label: 'Resumen' },
    { id: 1, label: 'Cronograma' },
    { id: 2, label: 'Actividades' },
    { id: 3, label: 'Finanzas' },
    { id: 4, label: 'Sanidad' },
    { id: 5, label: 'Suelo' },
  ];

  readonly cycleId: string;
  readonly cycle = signal<CycleDetailType | null>(null);
  readonly profit = signal<Profitability | null>(null);
  readonly variance = signal<CycleVariance | null>(null);
  readonly schedule = signal<ScheduleSummary | null>(null);
  readonly revenues = signal<any[]>([]);
  readonly incidences = signal<any[]>([]);
  readonly soilAnalyses = signal<any[]>([]);
  readonly lastSuggestions = signal<PlannedSuggestion[]>([]);
  private areaId = '';

  constructor(route: ActivatedRoute) {
    this.cycleId = route.snapshot.paramMap.get('id')!;
    this.loadAll();
  }

  private loadAll(): void {
    getCycle(this.cycleId).then(d => {
      this.cycle.set(d);
      this.areaId = d.summary.area_id;
      listSoilForArea(this.areaId).then(s => this.soilAnalyses.set(s)).catch(() => {});
    });
    getProfitability(this.cycleId).then(d => this.profit.set(d)).catch(() => {});
    getCycleVariance(this.cycleId).then(d => this.variance.set(d)).catch(() => {});
    getSchedule(this.cycleId).then(d => this.schedule.set(d)).catch(() => {});
    listRevenueForCycle(this.cycleId).then(d => this.revenues.set(d)).catch(() => {});
    listIncidencesForCycle(this.cycleId).then(d => this.incidences.set(d)).catch(() => {});
  }

  addPlanned(category: string, relativeDay: number): void {
    if (!relativeDay && relativeDay !== 0) return;
    addPlannedActivity({ cycle_id: this.cycleId, category, relative_day: relativeDay })
      .then(() => getSchedule(this.cycleId).then(d => this.schedule.set(d)));
  }

  doRegisterActivity(category: string, timestamp: number, mode: string): void {
    if (!timestamp) return;
    registerActivity(this.cycleId, { cycle_id: this.cycleId, timestamp, category, mode: mode as 'suggested' | 'emergent' | 'confirm_match' })
      .then(res => {
        this.lastSuggestions.set(res.suggestions);
        return getCycle(this.cycleId).then(d => this.cycle.set(d));
      });
  }

  confirmMatch(suggestion: PlannedSuggestion): void {
    registerActivity(this.cycleId, {
      cycle_id: this.cycleId,
      timestamp: suggestion.expected_timestamp,
      category: suggestion.category,
      mode: 'confirm_match',
      match_against: suggestion.planned_id,
    }).then(() => {
      this.lastSuggestions.set([]);
      return getCycle(this.cycleId).then(d => this.cycle.set(d));
    });
  }

  addRevenue(amount: string, source: string, receivedAt: number): void {
    if (!amount || !receivedAt) return;
    fetch(`http://localhost:8000/api/revenue`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ cycle_id: this.cycleId, amount, currency: 'USD', received_at: receivedAt, source }),
    }).then(() => listRevenueForCycle(this.cycleId).then(d => this.revenues.set(d)));
  }

  doRegisterIncidence(kind: string, severity: string, description: string, actionTaken: string, detectedAt: number, impact: string): void {
    if (!description || !actionTaken || !detectedAt) return;
    registerIncidence({
      cycle_id: this.cycleId, kind, severity, description, action_taken: actionTaken, detected_at: detectedAt,
      ...(impact ? { economic_impact_amount: impact, economic_impact_currency: 'USD' } : {}),
    }).then(() => listIncidencesForCycle(this.cycleId).then(d => this.incidences.set(d)));
  }

  doCreateBudget(amount: string, start: number, end: number): void {
    if (!amount || !start || !end) return;
    createBudget({ cycle_id: this.cycleId, period_start: start, period_end: end, baseline_amount: amount, baseline_currency: 'USD', estimated_lines: [] })
      .then(() => getProfitability(this.cycleId).then(d => this.profit.set(d)));
  }

  doLinkSoil(analysisId: string, kind: string): void {
    if (!analysisId) return;
    linkSoilToCycle({ analysis_id: analysisId, cycle_id: this.cycleId, kind })
      .then(() => {});
  }
}
