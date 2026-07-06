import { Component, input } from '@angular/core';
import { RouterLink } from '@angular/router';
import { FieldStatus, FieldHealthStatus } from './field-status';

export type FieldSummaryVariant = 'compact' | 'default' | 'expanded';

export interface FieldPhase {
  name: string;
  status: 'done' | 'current' | 'pending';
  day: number;
  total: number;
}

@Component({
  selector: 'kora-field-summary',
  imports: [RouterLink, FieldStatus],
  template: `
    <div class="field" [class.field--compact]="variant()==='compact'" [class.field--expanded]="variant()==='expanded'">
      <a class="field__link" [routerLink]="['/fields', id()]">
        <div class="field__head">
          <span class="field__name">{{ name() }}</span>
          <div class="field__meta">
            <span class="field__crop">{{ crop() }}</span>
            <span class="field__ha">{{ hectares() }} ha</span>
          </div>
        </div>
        <div class="field__bar"><div class="field__fill" [style.width.%]="progress()"></div></div>
        <div class="field__foot">
          <span>{{ daysToHarvest() }} días a cosecha</span>
          <span>{{ lastActivity() }}</span>
        </div>
        @if (variant() === 'expanded') {
          <div class="field__extras">
            @if (cost(); as c) { <span class="field__extra">Costo: {{ c }}</span> }
            @if (responsible(); as r) { <span class="field__extra">Resp: {{ r }}</span> }
          </div>
          @if (phases().length > 0) {
            <div class="field__phases">
              @for (p of phases(); track p.name) {
                <span class="phase" [class.phase--done]="p.status==='done'" [class.phase--current]="p.status==='current'">
                  {{ p.name }}
                  @if (p.status === 'current') { <span class="phase__day">{{ p.day }}/{{ p.total }}</span> }
                </span>
              }
            </div>
          }
        }
      </a>
      <div class="field__status"><kora-field-status [status]="health()" /></div>
    </div>
  `,
  styleUrl: './field-summary.component.scss',
})
export class FieldSummary {
  readonly id = input.required<string>();
  readonly name = input.required<string>();
  readonly crop = input.required<string>();
  readonly hectares = input.required<number>();
  readonly progress = input.required<number>();
  readonly daysToHarvest = input.required<number>();
  readonly lastActivity = input.required<string>();
  readonly health = input<FieldHealthStatus>('ok');
  readonly variant = input<FieldSummaryVariant>('default');
  readonly cost = input<string>('');
  readonly responsible = input<string>('');
  readonly phases = input<FieldPhase[]>([]);
}
