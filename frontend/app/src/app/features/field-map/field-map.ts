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
  styleUrl: './field-map.component.scss',
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
      this.loadGeoJSON();
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

  private loadGeoJSON(): void {
    if (!this.map) return;
    fetch('http://localhost:8000/api/fields/geojson')
      .then(r => r.json())
      .then(data => {
        const geoLayer = L.geoJSON(data, {
          style: (feature) => {
            const health = feature?.properties?.['health'] ?? 'ok';
            const colors: Record<string, string> = {
              ok: '#4a7c59', attention: '#b45309', critical: '#b91c1c',
            };
            return {
              fillColor: colors[health] ?? '#4a7c59',
              weight: 1,
              color: '#ffffff',
              fillOpacity: 0.5,
            };
          },
          onEachFeature: (feature, layer) => {
            const props = feature?.properties ?? {};
            layer.bindPopup(`<b>${props['name'] ?? ''}</b><br/>${props['hectares'] ?? ''} ha`);
          },
        }).addTo(this.map);
        this.map.fitBounds(geoLayer.getBounds().pad(0.1));
      })
      .catch(() => {});
  }
}
