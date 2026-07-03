import { Component } from '@angular/core';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-shell',
  imports: [RouterOutlet, RouterLink, RouterLinkActive],
  template: `
    <div class="layout">
      <aside class="sidebar">
        <a class="sidebar__brand" routerLink="/">
          <span class="sidebar__logo">K</span>
          <span class="sidebar__name">Kora</span>
        </a>
        <nav class="sidebar__nav">
          <a class="sidebar__link" routerLink="/" routerLinkActive="sidebar__link--active"
             [routerLinkActiveOptions]="{ exact: true }">
            <span class="sidebar__icon">◉</span>
            Operación
          </a>
          <a class="sidebar__link" routerLink="/lotes" routerLinkActive="sidebar__link--active">
            <span class="sidebar__icon">⊞</span>
            Lotes
          </a>
          <a class="sidebar__link" routerLink="/ciclos" routerLinkActive="sidebar__link--active">
            <span class="sidebar__icon">◎</span>
            Ciclos
          </a>
          <a class="sidebar__link" routerLink="/personal" routerLinkActive="sidebar__link--active">
            <span class="sidebar__icon">◉</span>
            Personal
          </a>
        </nav>
      </aside>
      <main class="main">
        <router-outlet />
      </main>
    </div>
  `,
  styles: `
    :host { display: contents; }

    .layout {
      display: flex;
      min-height: 100vh;
    }

    .sidebar {
      width: 200px;
      flex-shrink: 0;
      background: var(--surface);
      border-right: 1px solid var(--border);
      padding: var(--space-6) var(--space-3);
      display: flex;
      flex-direction: column;
      gap: var(--space-8);
    }

    .sidebar__brand {
      display: flex;
      align-items: center;
      gap: var(--space-2);
      text-decoration: none;
      color: var(--ink);
      padding: 0 var(--space-3);
    }

    .sidebar__logo {
      width: 28px;
      height: 28px;
      display: grid;
      place-items: center;
      background: var(--ink);
      color: var(--surface);
      border-radius: var(--radius-sm);
      font-size: 0.875rem;
      font-weight: 700;
    }

    .sidebar__name {
      font-size: 1rem;
      font-weight: 600;
    }

    .sidebar__nav {
      display: flex;
      flex-direction: column;
      gap: var(--space-1);
    }

    .sidebar__link {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      padding: var(--space-2) var(--space-3);
      border-radius: var(--radius);
      font-size: 0.875rem;
      color: var(--ink-muted);
      text-decoration: none;
      transition: background 0.12s, color 0.12s;
    }

    .sidebar__link:hover {
      background: var(--surface-muted);
      color: var(--ink);
    }

    .sidebar__link--active {
      background: var(--surface-muted);
      color: var(--ink);
      font-weight: 500;
    }

    .sidebar__icon {
      width: 16px;
      text-align: center;
      font-size: 0.75rem;
      opacity: 0.5;
    }

    .main {
      flex: 1;
      padding: var(--space-8);
      overflow-x: hidden;
    }
  `,
})
export class Shell {}
