# Decisiones de Diseño — Kora

> Documento vivo de arquitectura. Refleja el estado actual del código, no aspiraciones.

## Estructura actual (post-simplificación)

```
backend/src/
  shared_kernel/    # Tipos transversales (Money, Period, Polygon, IDs)
  agriculture/      # Contexto agrícola (CropCycle, Schedule, Drift)
  finance/          # Contexto financiero (Budget, Expense)
  ports/            # Traits de repositorio y providers (interfaces puras)
  adapters/         # Implementaciones concretas (in-memory, bridges)
  analyze_variance.rs   # Use case único "Plan vs Realidad"
  main.rs           # Entry point (pendiente de endpoints)
```

Principio rector: las abstracciones aparecen **como consecuencia del conocimiento adquirido**, no como apuesta sobre el futuro. No hay capas `domain/application/infrastructure` anidadas porque no había suficiente código real que las justificara.

## Lo implementado ✅

| Componente | Archivo | Tests |
|---|---|---|
| Money, Currency, ExchangeRateProvider | `shared_kernel/money.rs` | 5 |
| Period (inmutable, con overlaps/contains) | `shared_kernel/period.rs` | 8 |
| Polygon geoespacial | `shared_kernel/polygon.rs` | — |
| AreaMeasurement (ha/m²/acres) | `shared_kernel/area_unit.rs` | 5 |
| IDs compartidos (CycleId, CropId, AreaId) | `shared_kernel/ids.rs` | — |
| CropCycle + register_activity | `agriculture/cycle.rs` | 3 |
| Activity/ActivityRecord + IntegrityStatus | `agriculture/activity.rs` | 3 |
| Schedule + PlannedActivity | `agriculture/planning.rs` | 3 |
| Farm + validación de jerarquía | `agriculture/farm.rs` | 2 |
| Area + invariante ProductiveAreaExceedsBounds | `agriculture/area.rs` | 2 |
| Crop | `agriculture/crop.rs` | — |
| CropPlanningService (colisión espacio-temporal) | `agriculture/planning_service.rs` | 4 |
| Drift: VarianceService (timing) + EconomicVarianceService (costos) | `agriculture/drift.rs` | 8 |
| Budget + plan_cost/record_actual_cost + multi-moneda | `finance/budget.rs` | 4 |
| Expense | `finance/expense.rs` | — |
| FinanceError + From<RateError> | `finance/error.rs` | 2 |
| AnalyzeVariance use case (timing + optional economic) | `analyze_variance.rs` | 2 |
| FinanceEconomicProvider (puente agriculture↔finance) | `adapters/finance_economic_provider.rs` | 3 |
| InMemory repositories (test) | `adapters/in_memory_repositories.rs` | — |

**Total: 51 tests, 0 warnings.**

## Decisiones activas

### 1. Money type
- `rust_decimal::Decimal` (no f64) para precisión contable.
- `Currency` enum cerrado (USD/NIO). Si se expande, migrar a `struct Currency { code: String }`.
- `ExchangeRateProvider` trait desacopla fuente de datos del dominio.
- `add`/`subtract` rechazan monedas distintas en compile-time.

### 2. Imperfección controlada
Kora NO bloquea registros "incorrectos". Los marca y analiza después.

- `CropCycle.register_activity()` acepta actividades fuera de periodo → `IntegrityStatus::OutsidePeriod`.
- `Budget.register_expense()` no rechaza gastos que superen el baseline.

### 3. Análisis de varianza (Drift)
Dos servicios en un mismo archivo (`agriculture/drift.rs`):

- **VarianceService**: compara Schedule (plan) vs CropCycle (ejecución). Produce VarianceReport con matched/unplanned/missing y confidence scoring.
- **EconomicVarianceService**: enriquece el reporte con costos planificados vs reales, usando `EconomicDataProvider` trait (implementado por finance vía `FinanceEconomicProvider`).

### 4. Puente entre bounded contexts
`FinanceEconomicProvider` implementa `EconomicDataProvider` (definido en `ports/`). Agriculture consulta costos sin importar el módulo finance.

## Lo que NO está (y por qué)

| Feature | Estado | Motivo |
|---|---|---|
| Persistencia PostGIS | No implementado | El dominio no tiene repositorios reales; los InMemory son suficientes para validar la lógica |
| HTTP API | No implementado | Sin endpoints todavía; el dominio es un crate puro |
| Frontend | No implementado | `frontend/` existe como carpeta vacía (previsión, no deuda) |
| Labor context (Worker, WorkRecord) | Eliminado en simplificación | No tenía casos de uso; se reintroduce cuando una feature lo demande |
| Analysis/Suelo (MetricKind, AnalysisMetric) | Eliminado en simplificación | Idem; se reintroduce con el UC "Registrar Análisis de Suelo" |
| Eventsourcing / Schedule snapshots | No implementado | Se evalúa cuando haya más de un ciclo en la misma área |

## Roadmap tentativo

1. Agregar endpoint HTTP (axum o actix) exponiendo `analyze_variance`
2. Persistencia real con sqlx + PostGIS (implementar repos en `adapters/`)
3. UC "Registrar Actividad" con sugerencias del Schedule
4. UC "Crear Presupuesto" multi-rubro
5. Dashboard de Drift (timeline + costos)
6. Análisis de suelo con historial por área

---

*Última actualización: post-simplificación 2026-07-02*
*De 149 warnings / 4 contextos inflados / 22 tests → 0 warnings / 2 bounded contexts planos / 51 tests*
