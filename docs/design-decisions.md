# Decisiones de Diseño - Kora v2.0

> Documento vivo de arquitectura. Registra las decisiones técnicas tomadas, su contexto y consideraciones para el futuro.

## Índice
1. [Money Type](#1-money-type)
2. [Budget Aggregate](#2-budget-aggregate)
3. [CropCycle & ActivityRecord](#3-cropcycle--activityrecord)
4. [Schedule](#4-schedule)
5. [Filosofía Transversal](#5-filosofía-transversal-imperfection-controlled)
6. [Consideraciones para el Futuro](#6-consideraciones-para-el-futuro)

---

## 1. Money Type

### ¿Qué hace?
- Representa un valor monetario en el sistema.
- Permite operaciones matemáticas (`add`, `subtract`) y conversión entre monedas (`convert_to`).
- Es usado por `Budget`, `Expense` y cualquier entidad financiera.

### Decisiones Tomadas
| Decision | Elección | Alternativas Rechazadas | Razón |
|----------|-----------|----------------------|--------|
| **Almacenamiento** | `rust_decimal::Decimal` | `f64` (rechazado: errores de redondeo) | El dinero requiere precisión decimal exacta. `f64` es inaceptable para contabilidad. |
| **Monedas** | `enum Currency { USD, NIO }` | `String` (rechazado: typos, validación débil) | Con 2 monedas fijas (USD/NIO), un enum da type-safety en compile-time. |
| **Conversión** | `trait ExchangeRateProvider` | Función `convert(monto, de, a)` (rechazado: acopla fuente de tasas) | Desacopla la lógica de negocio de dónde vienen las tasas. |

### Lo que HACE
- ✅ Operaciones aritméticas con `Decimal`.
- ✅ Conversión usando un `ExchangeRateProvider` (manual o API).
- ✅ Protección contra operaciones entre monedas distintas (`RateError::CurrencyMismatch`).

### Lo que NO HACE (y por qué)
- ❌ **No almacena tasas de cambio internamente**: Eso es responsabilidad de quien implemente `ExchangeRateProvider`.
- ❌ **No hace llamadas HTTP**: La implementación de API (Banco Central) vive fuera del kernel, en infraestructura.
- ❌ **No soporta más de 2 monedas por ahora**: El `enum` es cerrado. Para escalar, cambiar a `String` + validación.

### Consideraciones para el Futuro
1. **`Decimal` vs `BigDecimal`**: `rust-decimal` es suficiente para 2-3 decimales. Si necesitás precisión arbitraria (crypto?), evaluar `bigdecimal`.
2. **Monedas dinámicas**: Si Kora se expande a otros países, cambiar `enum` por `struct Currency { code: String, symbol: String }` y tabla de monedas soportadas.
3. **Historial de tasas**: `ExchangeRateProvider` debería poder consultar tasas a una fecha específica para auditoría.

---

## 2. Budget Aggregate

### ¿Qué hace?
- Es el **Aggregate Root** financiero de un ciclo (doc v2: "contenedor financiero").
- Mantiene una "Foto Inicial" (`baseline`) y una "Foto Actual" (`current_expenses`).
- Su `period` puede ser más amplio que el ciclo biológico (captura gastos pre-operativos).

### Decisiones Tomadas
| Decision | Elección | Alternativas Rechazadas | Razón |
|----------|-----------|----------------------|--------|
| **Referencia a Ciclo** | `cycle_id: CycleId` (no importa dominio agriculture) | `crop_cycle: CropCycle` (rechazado: acopla contextos) | Mantiene aislación de Bounded Context. |
| **Tasas de cambio** | `rate_provider: Box<dyn ExchangeRateProvider>` | `HashMap<(Currency, Currency), Decimal>` (rechazado: no permite API) | Permite inyectar manual o API según el entorno. |
| **Comportamiento ante exceso** | **NO bloquea** gastos que superen el presupuesto | `if current > baseline { return Err(...) }` (rechazado: viola filosofía) | "La vida real no se detiene". Se registra y se analiza después. |

### Lo que HACE
- ✅ `register_expense(amount)`: Actualiza `current_expenses` sin bloquear.
- ✅ `get_remaining()`: Calcula `baseline - current` (cuánto queda).
- ✅ `get_variance()`: Calcula `current - baseline` (desviación: positivo = sobre gasto).

### Lo que NO HACE
- ❌ **No persiste aún**: No hay repositorios implementados (próxima etapa).
- ❌ **No genera alertas**: La lógica de "Excedente" va en la capa de análisis, no en el dominio.
- ❌ **No valida timestamps**: No verifica si un gasto pertenece al `period` del presupuesto (filosofía: aceptar lo que pasó).

### Consideraciones para el Futuro
1. **Repositorio**: Crear `BudgetRepository` con trait `fn save(&self, budget: &Budget)` y `fn find_by_cycle(&self, cycle_id: &CycleId)`.
2. **Alertas**: Crear un `BudgetService` que al llamar `register_expense` verifique si `get_variance() > 0` y emita un `Event::BudgetExceeded`.
3. **Multi-moneda en Budget**: ¿El `baseline` y los gastos pueden ser en monedas distintas? Si sí, `current_expenses` debe ser un `Vec<(Money, ExpenseId)>` y todas las operaciones usan `convert_to`.

---

## 3. CropCycle & ActivityRecord

### ¿Qué hace?
- `CropCycle` es el Aggregate Root del contexto agriculture.
- `register_activity()` permite registrar actividades que **sucedieron en el campo**, aceptando que la realidad es imperfecta.

### Decisiones Tomadas
| Decision | Elección | Alternativas Rechazadas | Razón |
|----------|-----------|----------------------|--------|
| **Manejo de desviaciones** | `IntegrityStatus` enum (`Valid`, `OutsidePeriod`, `Unplanned`) | `bool is_outside_period` (rechazado: no escala a otros estados) | Un vector de estados es extensible para futuros chequeos. |
| **Estructura de registro** | `ActivityRecord { activity, integrity }` | Modificar `Activity` directamente (rechazado: mezcla datos con metadatos) | Separa el "qué" (Activity) del "cómo se registró" (IntegrityStatus). |
| **Bloqueo por periodo** | **NO bloquea** si la actividad está fuera del periodo | `return Err(ActivityOutsideCyclePeriod)` (rechazado: violaba doc v2) | "El Plan no es la verdad; es el Marco de Referencia". |

### Lo que HACE
- ✅ `register_activity()`: Acepta actividades fuera de periodo, las marca con `IntegrityStatus::OutsidePeriod`.
- ✅ `close_cycle()`: Impide nuevos registros (único caso de bloqueo legítimo: ciclo cerrado).
- ✅ `executed_activities`: Ahora es `Vec<ActivityRecord>` para trackear integridad.

### Lo que NO HACE
- ❌ **No compara con Schedule**: No verifica si la actividad estaba planificada (eso lo hará `VarianceService` en el futuro).
- ❌ **No persiste**: Al igual que Budget, no hay repositorio aún.

### Consideraciones para el Futuro
1. **VarianceService**: Motor que compare `Schedule` (planificado) vs `executed_activities` (realidad) para generar el análisis de desviación.
2. **Nuevos IntegrityStatus**: `Late` (dentro del periodo pero después de la fecha planificada), `Incomplete` (sin insumos registrados).
3. **Snapshots de Schedule**: La doc menciona "Cada cambio en el plan genera un Snapshot". Implementar `ScheduleVersion` para trazabilidad.

---

## 4. Schedule

### ¿Qué hace?
- Es una **Entity** (no Aggregate Root) que define el "deber ser" del ciclo.
- Usa **Anclas Temporales** (`ScheduleAnchor`) para definir cuándo deben ocurrir las actividades.

### Decisiones Tomadas
| Decision | Elección | Alternativas Rechazadas | Razón |
|----------|-----------|----------------------|--------|
| **Referencia temporal** | `ScheduleAnchor` enum (`CycleStart`, `SowingDate`, `HarvestStart`) | Timestamp fijo (rechazado: no permite flexibilidad agronómica) | Permite al agrónomo decir "Fumigación: Día +15 desde Siembra". |
| **Actividades planificadas** | `PlannedActivity { category, relative_day, status }` | `Activity` directamente (rechazado: una cosa es el plan, otra la ejecución) | Separa intención (Plan) de realidad (Activity). |

### Lo que HACE
- ✅ Define actividades relativas a un hito (`relative_day: i32`).
- ✅ Maneja estados de planificación (`Planned`, `InProgress`, `Completed`, `Skipped`).
- ✅ Vinculado a un `cycle_id` (pero no importa el dominio agriculture).

### Lo que NO HACE
- ❌ **No se compara con la realidad**: No tiene lógica para contrastar con `executed_activities`.
- ❌ **No versiona**: La doc v2 dice "Cada cambio genera un Snapshot", pero `version: u32` está declarado y no se usa aún.

### Consideraciones para el Futuro
1. **Motor de comparación**: Crear `VarianceService.compare(schedule: &Schedule, activities: &[ActivityRecord]) -> Vec<VarianceReport>`.
2. **Versionado**: Al modificar el `Schedule`, clonar el actual, incrementar `version`, guardar el anterior en `ScheduleHistory`.
3. **Recursos estimados**: Las actividades planificadas deberían poder tener `estimated_cost: Money` y `estimated_labor: i32` para comparar con la realidad.

---

## 5. Filosofía Transversal: "Imperfection-Controlled"

Esta es la filosofía central de Kora (doc v2):

> "Kora acepta que la agricultura es caótica. Su misión no es obligar al usuario a ser perfecto, sino hacer visible la imperfección para que pueda ser gestionada."

### ¿Cómo se aplica?
| Componente | Antes (Incorrecto) | Ahora (Correcto) |
|------------|-------------------|----------------|
| `CropCycle` | Bloqueaba actividades fuera de periodo | Las acepta, marca con `IntegrityStatus` |
| `Budget` | (No existía) | Acepta gastos que superen baseline, no bloquea |
| `Schedule` | (No se tocó) | Se mantiene como referencia, no como ley |

### Mantra de Kora
> "Registra lo que pasó, compáralo con lo que querías, y aprende de la diferencia."

---

## 6. Consideraciones para el Futuro

### 6.1 Persistence (PostGIS)
- **La doc promete**: PostgreSQL + PostGIS para precisión geométrica.
- **Estado actual**: No hay nada de persistencia. Todo vive en memoria.
- **Siguiente paso**: Definir repositorios (`CropCycleRepository`, `BudgetRepository`, `ScheduleRepository`) con traits y migraciones de base de datos.

### 6.2 VarianceService (El "Diferencial" de Kora)
- **La doc promete**: "El valor diferencial de Kora ocurre al contrastar los datos: Planificado vs. Ejecutado".
- **Estado actual**: `Schedule` y `CropCycle` viven en contextos distintos.
- **Siguiente paso**: Implementar `VarianceService` que use `ExchangeRateProvider` para unificar monedas y compare `Schedule` vs `executed_activities` vs `Budget`.

#### HALLAZGO CRÍTICO DEL TEST DE INTEGRACIÓN (tests/integration_test.rs):
El flujo **Plan → Execute → Analyze** reveló:
1. ✅ **Lo que SÍ funciona**: 
   - Creación de `CropCycle`, `Schedule`, `Budget`.
   - Registro de actividades con `IntegrityStatus` (Valid, OutsidePeriod).
   - Registro de gastos sin bloqueo (filosofía imperfección controlada).
   - Cálculo de varianza de presupuesto (`get_variance()`).

2. ❌ **Lo que FALTA (la brecha real)**:
   - **No hay `VarianceService`**: No podemos comparar `Schedule.activities` (planificado) con `CropCycle.executed_activities` (realidad).
   - **No hay matching de actividades**: No hay lógica que diga "esta actividad real coincidió con esta planificada".
   - **No hay cálculo de retraso**: `Schedule` usa `relative_day`, pero no hay forma de comparar con `Activity.timestamp`.
   - **`IntegrityStatus::Unplanned` no se asigna**: No hay servicio que detecte actividades reales sin planificación.

3. **Lo que necesitamos para completar el "Diferencial"**:
   ```rust
   // Estructura propuesta para VarianceService
   pub struct VarianceReport {
       on_time: Vec<ActivityRecord>,      // Coinciden con Schedule
       delayed: Vec<(PlannedActivity, ActivityRecord)>, // Hubo retraso
       unplanned: Vec<ActivityRecord>,  // No estaban en Schedule
       budget_variance: Money,          // Presupuesto vs Gastado
   }
   
   // El servicio necesita:
   // 1. Matchear por category + timestamp vs relative_day (usando anchor_date)
   // 2. Calcular "días de retraso"
   // 3. Asignar IntegrityStatus::Unplanned a lo que no matchee
   // 4. Usar Budget.get_variance() para lo financiero
   ```

### 6.3 API / UI
- **La doc promete**: "Inteligencia de Negocio", "Plataforma de Inteligencia Agrícola".
- **Estado actual**: Solo dominio en Rust. No hay endpoints ni UI.
- **Siguiente paso**: Definir API REST/GraphQL, conectar con un frontend que muestre mapas interactivos (OLTP) y dashboards de análisis (OLAP).

### 6.4 Testing
- **Estado actual**: Tests unitarios en cada módulo (`#[cfg(test)]`).
- **Siguiente paso**: Integrar con `cargo test`, añadir tests de integración (¿cómo interactúan Budget y CropCycle?) y tests E2E cuando haya UI.

### 6.5 Event Sourcing (Opcional)
- **La doc dice**: "Inmutabilidad Histórica: Los planes no se borran; se versionan".
- **Consideración**: Evaluar si Kora se beneficia de un Event Store (ej. `ActivityRegistered`, `BudgetExceeded`) para reconstruir el estado y auditar.

---

## Resumen de Archivos Clave

| Archivo | Rol | Estado |
|---------|-----|--------|
| `shared_kernel/money.rs` | Money type, Currency, ExchangeRateProvider | ✅ Implementado |
| `shared_kernel/ids.rs` | BudgetId, ExpenseId, CycleId, etc. | ✅ Implementado |
| `finance/domain/budget.rs` | Budget aggregate | ✅ Implementado |
| `finance/domain/expense.rs` | Expense entity | ✅ Implementado |
| `agriculture/domain/cycle.rs` | CropCycle aggregate | ✅ Implementado (con ActivityRecord) |
| `agriculture/domain/planning.rs` | Schedule entity | ✅ Implementado (básico) |
| `finance/error.rs` | FinanceError enum | ✅ Implementado |
| `agriculture/domain/activity.rs` | ActivityRecord, IntegrityStatus | ✅ Implementado |

---

## 7. Test de Integración (Flujo End-to-End)

### ¿Qué hizo?
- Se creó `server/tests/integration_test.rs` para validar el flujo: **Plan → Execute → Analyze**.
- Usa `CropCycle`, `Schedule`, `Budget` y `ExchangeRateProvider` juntos.

### Hallazgos Críticos (lo que FALTA):
1. **NO hay `VarianceService`**: No hay forma de comparar `Schedule.activities` (planificado) con `CropCycle.executed_activities` (realidad).
2. **`IntegrityStatus::Unplanned` no se asigna**: No hay lógica que diga "esta actividad real no estaba en el cronograma".
3. **No hay cálculo de retraso**: `Schedule` usa `relative_day`, pero no hay forma de comparar con `Activity.timestamp`.
4. **No hay matching de actividades**: No se puede decir "esta actividad real corresponde a esta planificada".

### Lo que SÍ funciona:
- ✅ Creación de ciclos, cronogramas y presupuestos.
- ✅ Registro de actividades con `IntegrityStatus`.
- ✅ Registro de gastos sin bloqueo (filosofía imperfección controlada).
- ✅ Cálculo de varianza de presupuesto (`get_variance()`).

### Siguiente paso recomendado:
Implementar **`VarianceService`** que:
1. Tome `Schedule` + `Vec<ActivityRecord>`.
2. Matchee por `category` y `timestamp` vs `relative_day` (usando `anchor_date`).
3. Produzca `VarianceReport { on_time, delayed, unplanned }`.

---

*Documento generado el 2026-05-01 tras completar ciclos SDD para CropCycle y Budget.*
