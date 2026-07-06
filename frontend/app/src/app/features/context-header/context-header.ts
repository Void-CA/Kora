import { Component, signal, OnDestroy, inject } from '@angular/core';
import { Router, Event as RouterEvent, NavigationEnd } from '@angular/router';
import { Subscription } from 'rxjs';
import { getField } from '../../api/field.api';
import { getCycle } from '../../api/cycle.api';

@Component({
  selector: 'app-context-header',
  template: `
    @if (ctx(); as ctx) {
      <div class="context-bar">
        <div class="context-bar__breadcrumb">
          <span class="context-section">{{ ctx.section }}</span>
          @if (ctx.name) {
            <span class="context-sep">/</span>
            <span class="context-name">{{ ctx.name }}</span>
          }
        </div>
        @if (ctx.detail) {
          <div class="context-detail">
            @for (d of ctx.detail; track d.label) {
              <span class="context-detail__item">
                <span class="context-detail__label">{{ d.label }}</span>
                <span class="context-detail__value">{{ d.value }}</span>
              </span>
            }
          </div>
        }
      </div>
    }
  `,
  styles: [`
    .context-bar { display: flex; align-items: center; justify-content: space-between; gap: var(--space-4); padding: var(--space-2) 0 var(--space-3); margin-bottom: var(--space-6); border-bottom: 1px solid var(--border); }
    .context-bar__breadcrumb { display: flex; align-items: baseline; gap: var(--space-2); font-size: 0.8125rem; }
    .context-section { color: var(--ink-muted); font-weight: 500; }
    .context-sep { color: var(--ink-subtle); }
    .context-name { color: var(--ink); font-weight: 600; }
    .context-detail { display: flex; gap: var(--space-4); }
    .context-detail__item { display: flex; align-items: baseline; gap: var(--space-1); font-size: 0.75rem; }
    .context-detail__label { color: var(--ink-subtle); }
    .context-detail__value { color: var(--ink-muted); font-weight: 500; }
  `],
})
export class ContextHeader implements OnDestroy {
  private router = inject(Router);
  private sub: Subscription;
  private currentUrl = '';

  readonly ctx = signal<{ section: string; name: string | null; detail: { label: string; value: string }[] | null } | null>(null);

  constructor() {
    this.sub = this.router.events.subscribe((e: RouterEvent) => {
      if (e instanceof NavigationEnd && e.url !== this.currentUrl) {
        this.currentUrl = e.url;
        this.updateContext(e.url);
      }
    });
  }

  ngOnDestroy(): void {
    this.sub.unsubscribe();
  }

  private updateContext(url: string): void {
    const fieldMatch = url.match(/^\/fields\/(.+)/);
    const historyMatch = url.match(/^\/history\/(.+)/);

    if (fieldMatch) {
      const id = fieldMatch[1];
      this.ctx.set({ section: 'Campos', name: null, detail: null });
      getField(id).then(f => {
        if (this.currentUrl !== `/fields/${id}`) return;
        this.ctx.set({ section: 'Campos', name: f.name, detail: [
          { label: 'cultivo', value: f.crop },
          { label: 'ha', value: String(f.hectares) },
          { label: 'última', value: f.last_activity },
        ]});
      }).catch(() => {});
    } else if (historyMatch) {
      const id = historyMatch[1];
      this.ctx.set({ section: 'Historial', name: null, detail: null });
      getCycle(id).then(c => {
        if (this.currentUrl !== `/history/${id}`) return;
        this.ctx.set({ section: 'Historial', name: `Campaña ${c.summary.id.slice(0, 8)}…`, detail: [
          { label: 'actividades', value: String(c.activities.length) },
          { label: 'planificadas', value: String(c.planned_activities.length) },
        ]});
      }).catch(() => {});
    } else if (url.startsWith('/fields')) {
      this.ctx.set({ section: 'Campos', name: null, detail: null });
    } else if (url.startsWith('/history')) {
      this.ctx.set({ section: 'Historial', name: null, detail: null });
    } else if (url.startsWith('/operations')) {
      this.ctx.set({ section: 'Trabajo', name: null, detail: null });
    } else if (url.startsWith('/team')) {
      this.ctx.set({ section: 'Equipo', name: null, detail: null });
    } else if (url.startsWith('/finances')) {
      this.ctx.set({ section: 'Finanzas', name: null, detail: null });
    } else if (url === '/') {
      this.ctx.set(null);
    } else {
      this.ctx.set(null);
    }
  }
}
