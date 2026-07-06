import { Component, input } from '@angular/core';

@Component({
  selector: 'app-mini-map',
  template: `
    <figure class="mini-map" [attr.aria-label]="ariaLabel()">
      <div class="mini-map__placeholder" aria-hidden="true">
        <span>Mapa</span>
      </div>
      <figcaption class="mini-map__caption">{{ caption() }}</figcaption>
    </figure>
  `,
  styles: `
    :host { display: block; }

    .mini-map {
      margin: 0;
      display: flex;
      flex-direction: column;
      gap: var(--space-2);
    }

    .mini-map__placeholder {
      width: 100%;
      aspect-ratio: 4 / 3;
      background:
        linear-gradient(135deg, var(--surface-muted), var(--surface-sunken));
      border: 1px dashed var(--border-strong);
      border-radius: var(--radius);
      display: grid;
      place-items: center;
      color: var(--ink-subtle);
      font-size: 0.75rem;
      letter-spacing: 0.05em;
      text-transform: uppercase;
    }

    .mini-map__caption {
      font-size: 0.8125rem;
      color: var(--ink-muted);
      text-align: center;
    }
  `,
})
export class MiniMap {
  readonly caption = input.required<string>();
  readonly ariaLabel = input<string>('Ubicación');
}
