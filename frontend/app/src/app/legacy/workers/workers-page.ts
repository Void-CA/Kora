import { Component, signal } from '@angular/core';
import { SlicePipe } from '@angular/common';
import { listWorkers, registerWorker, recordPayroll, listPayrollForCycle, WorkerSummary, PayrollEntrySummary } from '../../api/kora-api';
import { MoneyPipe } from '../../shared/pipes/money.pipe';
import { TimestampPipe } from '../../shared/pipes/timestamp.pipe';
import { getCycles, CycleSummary } from '../../api/kora-api';

@Component({
  selector: 'app-workers-page',
  imports: [MoneyPipe, TimestampPipe, SlicePipe],
  template: `
    <header class="page-header">
      <h1 class="page-title">Personal</h1>
      <span class="page-count">{{ workers().length }} trabajadores</span>
    </header>

    <div class="two-col">
      <section>
        <h2 class="section-label">Trabajadores</h2>
        <table class="table">
          <thead><tr><th>Nombre</th><th>Rol</th><th>Activo</th></tr></thead>
          <tbody>
            @for (w of workers(); track w.id) {
              <tr><td class="cell">{{ w.name }}</td><td class="cell">{{ w.role ?? '—' }}</td><td class="cell">{{ w.active ? 'Sí' : 'No' }}</td></tr>
            }
          </tbody>
        </table>
        <details class="form-wrap">
          <summary class="form-toggle">+ Registrar trabajador</summary>
          <div class="form-row">
            <input #name class="input" placeholder="Nombre" style="flex:1" />
            <select #role class="input" style="width:140px">
              <option value="">Sin rol</option><option value="Operario">Operario</option><option value="Supervisor">Supervisor</option><option value="Tractorista">Tractorista</option><option value="Tecnico">Técnico</option>
            </select>
            <button class="btn btn--primary" (click)="doRegisterWorker(name.value, role.value)">Registrar</button>
          </div>
        </details>
      </section>

      <section>
        <h2 class="section-label">Pagos</h2>
        <div class="form-row" style="margin-bottom:var(--space-3)">
          <select #cycleSelect class="input" style="flex:1">
            <option value="">Seleccionar ciclo…</option>
            @for (c of cycles(); track c.id) {
              <option value="{{ c.id }}">{{ c.id | slice:0:8 }}… ({{ c.crop_id | slice:0:8 }}…)</option>
            }
          </select>
          <button class="btn btn--ghost" (click)="loadPayroll(cycleSelect.value)">Ver pagos</button>
        </div>
        @if (payrollEntries(); as entries) {
          <table class="table">
            <thead><tr><th>Trabajador</th><th>Monto</th><th>Fecha</th></tr></thead>
            <tbody>@for (e of entries; track e.id) {<tr><td class="cell">{{ e.worker_id | slice:0:8 }}…</td><td class="cell">{{ e.amount | money }}</td><td class="cell">{{ e.paid_at | ts }}</td></tr>}</tbody>
          </table>
        }
        <details class="form-wrap">
          <summary class="form-toggle">+ Registrar pago</summary>
          <div class="form-col">
            <select #payWorker class="input">
              <option value="">Seleccionar trabajador…</option>
              @for (w of workers(); track w.id) { <option value="{{ w.id }}">{{ w.name }}</option> }
            </select>
            <div class="form-row">
              <input #payAmt type="text" class="input" placeholder="Monto" style="width:120px" />
              <input #payTs type="number" class="input" placeholder="Timestamp" style="flex:1" />
              <button class="btn btn--primary" (click)="doRecordPayroll(payWorker.value, payAmt.value, +payTs.value, cycleSelect.value)">Pagar</button>
            </div>
          </div>
        </details>
      </section>
    </div>
  `,
  styles: [`
    .page-header { display: flex; align-items: baseline; gap: var(--space-3); margin-bottom: var(--space-6); }
    .page-title { font-size: 1.25rem; font-weight: 600; margin: 0; color: var(--ink); }
    .page-count { font-size: 0.8125rem; color: var(--ink-subtle); }
    .two-col { display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-6); }
    .section-label { font-size: 0.875rem; font-weight: 600; margin: 0 0 var(--space-3); color: var(--ink); }
    .table { width: 100%; border-collapse: collapse; margin-bottom: var(--space-4); }
    .cell { padding: var(--space-2) var(--space-3); font-size: 0.8125rem; border-bottom: 1px solid var(--border); color: var(--ink); }
    th { padding: var(--space-2) var(--space-3); font-size: 0.6875rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--ink-subtle); text-align: left; border-bottom: 1px solid var(--border-strong); }
    .form-wrap { margin-top: var(--space-2); border: 1px solid var(--border); border-radius: var(--radius); padding: var(--space-3); background: var(--surface); }
    .form-toggle { font-size: 0.8125rem; font-weight: 500; color: var(--state-info); cursor: pointer; }
    .form-row { display: flex; gap: var(--space-2); margin-top: var(--space-2); align-items: center; flex-wrap: wrap; }
    .form-col { display: flex; flex-direction: column; gap: var(--space-2); margin-top: var(--space-2); }
    .input { padding: var(--space-2) var(--space-3); font-size: 0.8125rem; border: 1px solid var(--border); border-radius: var(--radius); background: var(--surface); color: var(--ink); }
    .btn { padding: var(--space-2) var(--space-4); border-radius: var(--radius); font-size: 0.8125rem; font-weight: 500; border: 1px solid transparent; cursor: pointer; }
    .btn--primary { background: var(--ink); color: var(--surface); }
    .btn--ghost { background: transparent; color: var(--ink-muted); border-color: var(--border); }
  `],
})
export class WorkersPage {
  readonly workers = signal<WorkerSummary[]>([]);
  readonly cycles = signal<CycleSummary[]>([]);
  readonly payrollEntries = signal<PayrollEntrySummary[] | null>(null);

  constructor() {
    listWorkers().then(d => this.workers.set(d));
    getCycles().then(d => this.cycles.set(d));
  }

  doRegisterWorker(name: string, role: string): void {
    if (!name.trim()) return;
    registerWorker({ name, ...(role ? { role } : {}) }).then(() => listWorkers().then(d => this.workers.set(d)));
  }

  loadPayroll(cycleId: string): void {
    if (!cycleId) return;
    listPayrollForCycle(cycleId).then(d => this.payrollEntries.set(d));
  }

  doRecordPayroll(workerId: string, amount: string, paidAt: number, cycleId: string): void {
    if (!workerId || !amount || !paidAt || !cycleId) return;
    recordPayroll({ worker_id: workerId, amount, currency: 'USD', paid_at: paidAt, cycle_id: cycleId }).then(() => {
      if (cycleId) this.loadPayroll(cycleId);
    });
  }
}
