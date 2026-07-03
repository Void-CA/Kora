import { Component, input, computed } from '@angular/core';
import { NgxEchartsDirective, provideEchartsCore } from 'ngx-echarts';
import * as echarts from 'echarts/core';
import { BarChart } from 'echarts/charts';
import { TitleComponent, TooltipComponent, GridComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type { EChartsOption } from 'echarts';
import type { CycleVariance } from '../../api/kora-api';

echarts.use([BarChart, TitleComponent, TooltipComponent, GridComponent, CanvasRenderer]);

@Component({
  selector: 'app-variance-chart',
  imports: [NgxEchartsDirective],
  providers: [provideEchartsCore({ echarts })],
  template: `
    <div class="chart" [class.chart--loading]="!variance()">
      @if (variance(); as v) {
        <div echarts [options]="chartOptions()" class="chart__render" (chartClick)="onClick($event)"></div>
        <div class="chart__legend">
          <span class="legend__item legend__item--ontime">● a tiempo</span>
          <span class="legend__item legend__item--late">● atrasado</span>
          <span class="legend__item legend__item--early">● adelantado</span>
          <span class="legend__item legend__item--missing">● no ejecutado</span>
        </div>
        <p class="chart__summary">
          {{ v.totals.matched_count }} de {{ v.totals.matched_count + v.totals.missing_count }} actividades planificadas ejecutadas,
          {{ v.totals.unplanned_count }} no planificadas.
        </p>
      } @else {
        <p class="chart__empty">Cargando análisis de varianza…</p>
      }
    </div>
  `,
  styles: `
    :host { display: block; }

    .chart__render {
      width: 100%;
      height: 320px;
    }

    .chart__legend {
      display: flex;
      gap: var(--space-4);
      padding: var(--space-3) 0;
      font-size: 0.75rem;
      color: var(--ink-muted);
    }

    .legend__item--ontime  { color: var(--state-ok); }
    .legend__item--late    { color: var(--state-critical); }
    .legend__item--early   { color: var(--state-attention); }
    .legend__item--missing { color: var(--ink-subtle); }

    .chart__summary {
      font-size: 0.8125rem;
      color: var(--ink-muted);
      margin: 0;
      padding: var(--space-2) 0;
    }

    .chart__empty, .chart--loading {
      padding: var(--space-8);
      text-align: center;
      color: var(--ink-subtle);
    }
  `,
})
export class VarianceChart {
  readonly variance = input<CycleVariance | null>(null);

  readonly chartOptions = computed<EChartsOption>(() => {
    const v = this.variance();
    if (!v) return {};

    const categories = v.matched.map(m => m.category);
    const lateDays = v.matched.map(m =>
      m.variance.kind === 'late' ? Math.round(Math.abs(m.variance.days) * 100) / 100 : 0
    );
    const earlyDays = v.matched.map(m =>
      m.variance.kind === 'early' ? Math.round(Math.abs(m.variance.days) * 100) / 100 : 0
    );
    const onTime = v.matched.map(m =>
      m.variance.kind === 'on_time' ? 1 : 0
    );

    return {
      tooltip: {
        trigger: 'axis' as const,
        axisPointer: { type: 'shadow' as const },
        formatter: (params: unknown) => {
          const p = params as Array<{ seriesName: string; value: number; marker: string }>;
          return p.map(i => `${i.seriesName}: ${i.value} día(s)`).join('<br/>');
        },
      },
      grid: { left: '3%', right: '4%', bottom: '3%', containLabel: true },
      xAxis: {
        type: 'category' as const,
        data: categories,
        axisLabel: { fontSize: 11 },
      },
      yAxis: {
        type: 'value' as const,
        name: 'días',
        min: 0,
      },
      series: [
        {
          name: 'atrasado',
          type: 'bar' as const,
          stack: 'total',
          color: '#dc2626',
          data: lateDays,
        },
        {
          name: 'adelantado',
          type: 'bar' as const,
          stack: 'total',
          color: '#ca8a04',
          data: earlyDays,
        },
        {
          name: 'a tiempo',
          type: 'bar' as const,
          stack: 'total',
          color: '#16a34a',
          data: onTime,
        },
      ],
    };
  });

  onClick(event: unknown): void {
    // Placeholder: future detail navigation
  }
}
