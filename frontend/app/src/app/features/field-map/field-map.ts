import { Component, input, ElementRef, viewChild, afterNextRender } from '@angular/core';
import L from 'leaflet';

@Component({
  selector: 'app-field-map',
  template: `
    <div #mapContainer class="map" [style.aspect-ratio]="aspectRatio()"></div>
    @if (caption()) {
      <figcaption class="map__caption">{{ caption() }}</figcaption>
    }
  `,
  styles: `
    :host { display: block; }

    .map {
      width: 100%;
      min-height: 200px;
      border-radius: var(--radius);
      overflow: hidden;
      border: 1px solid var(--border);
    }

    .map__caption {
      font-size: 0.8125rem;
      color: var(--ink-muted);
      text-align: center;
      margin-top: var(--space-2);
    }
  `,
})
export class FieldMap {
  readonly caption = input<string>('');
  readonly aspectRatio = input<string>('4 / 3');
  readonly centerLat = input<number>(12.0);
  readonly centerLng = input<number>(-70.0);
  readonly zoom = input<number>(11);

  readonly mapContainer = viewChild.required<ElementRef<HTMLElement>>('mapContainer');

  private map: L.Map | null = null;

  constructor() {
    afterNextRender(() => {
      this.initMap();
    });
  }

  private initMap(): void {
    const el = this.mapContainer().nativeElement;
    if (!el || this.map) return;

    this.map = L.map(el, {
      center: [this.centerLat(), this.centerLng()],
      zoom: this.zoom(),
      zoomControl: false,
      attributionControl: false,
    });

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      maxZoom: 19,
    }).addTo(this.map);
  }
}
