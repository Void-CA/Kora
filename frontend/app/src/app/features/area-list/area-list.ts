import { Component, signal } from '@angular/core';
import { RouterLink } from '@angular/router';
import { getFields, Field } from '../../api/kora-api';

@Component({
  selector: 'app-area-list',
  imports: [RouterLink],
  template: `
    <header class="page-header">
      <h1 class="page-title">Lotes</h1>
      <span class="page-count">{{ fields()?.length ?? 0 }} lotes</span>
    </header>

    @if (fields(); as list) {
      <div class="grid">
        @for (f of list; track f.id) {
          <a class="card" [routerLink]="['/lotes', f.id]">
            <h2 class="card__title">{{ f.name }}</h2>
            <div class="card__meta">
              <span>{{ f.hectares }} ha</span>
              <span>{{ f.crop }}</span>
              <span>{{ f.days_to_harvest }} días a cosecha</span>
            </div>
            <div class="card__health">
              @for (h of f.health; track h.label) {
                <span class="pill" [class.pill--ok]="h.status==='ok'"
                      [class.pill--attention]="h.status==='attention'"
                      [class.pill--critical]="h.status==='critical'">
                  {{ h.label }}: {{ h.value }}
                </span>
              }
            </div>
          </a>
        }
      </div>
    } @else {
      <p class="empty">Cargando lotes…</p>
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
    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
      gap: var(--space-4);
    }
    .card {
      display: flex;
      flex-direction: column;
      gap: var(--space-3);
      padding: var(--space-5);
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius-lg);
      text-decoration: none;
      color: inherit;
      transition: box-shadow 0.12s;
    }
    .card:hover { box-shadow: 0 1px 6px rgba(0,0,0,0.08); }
    .card__title {
      font-size: 1rem;
      font-weight: 600;
      margin: 0;
      color: var(--ink);
    }
    .card__meta {
      display: flex;
      flex-wrap: wrap;
      gap: var(--space-2);
      font-size: 0.8125rem;
      color: var(--ink-muted);
    }
    .card__meta span:not(:last-child)::after {
      content: '·';
      margin-left: var(--space-2);
      color: var(--ink-subtle);
    }
    .card__health {
      display: flex;
      flex-wrap: wrap;
      gap: var(--space-2);
    }
    .pill {
      font-size: 0.75rem;
      padding: 2px 8px;
      border-radius: 999px;
      background: var(--surface-muted);
      color: var(--ink-muted);
    }
    .pill--ok { background: #f0fdf4; color: #15803d; }
    .pill--attention { background: #fefce8; color: #a16207; }
    .pill--critical { background: #fef2f2; color: #b91c1c; }
    .empty {
      color: var(--ink-subtle);
      font-size: 0.875rem;
    }
  `,
})
export class AreaList {
  readonly fields = signal<Field[] | null>(null);

  constructor() {
    getFields().then(data => this.fields.set(data));
  }
}
