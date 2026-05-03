# Estrategia de Puertos y Adaptadores — Kora

> Documento vivo. Define cómo llevar el dominio estable a usable mediante repositorios, interfaces y casos de uso.

## Índice
1. [Filosofía Arquitectónica](#1-filosofía-arquitectónica)
2. [Estado Actual](#2-estado-actual)
3. [Patrón de Conexión Entre Contextos](#3-patrón-de-conexión-entre-contextos)
4. [Estrategia por Módulo](#4-estrategia-por-módulo)
5. [Orden Recomendado de Implementación](#5-orden-recomendado-de-implementación)
6. [Consideraciones Transversales](#6-consideraciones-transversales)
7. [Manejo de Errores](#7-manejo-de-errores)
8. [Contract Testing](#8-contract-testing)

---

## 1. Filosofía Arquitectónica

Kora usa **Arquitectura Hexagonal (Puertos y Adaptadores)** con **DDD**.

### Capas

```
┌─────────────────────────────────────────────────────┐
│                   INFRASTRUCTURE                      │
│  (Adaptadores secundarios: BD, APIs, Adapters)       │
└──────────────────┬──────────────────────────────────┘
                   │ implementa puertos
┌──────────────────▼──────────────────────────────────┐
│                   APPLICATION                         │
│  Casos de Uso + Puertos de Aplicación + DTOs        │
│  Servicios de Aplicación (reutilizables)             │
└──────────────────┬──────────────────────────────────┘
                   │ usa
┌──────────────────▼──────────────────────────────────┐
│                     DOMAIN                            │
│  Entidades, Agregados, Servicios de Dominio           │
│  Puertos de Dominio (abstracciones para lógica)      │
└─────────────────────────────────────────────────────┘
```

### Reglas de Oro (RELAJADAS Y PRECISAS)

> **1. El dominio NO conoce la aplicación ni la infraestructura específica.**
> **2. La aplicación conoce el dominio y define PUERTOS de aplicación (traits para persistencia, etc.).**
> **3. La infraestructura conoce la aplicación y el dominio, e IMPLEMENTA puertos.**
> **4. El contexto proveedor NO expone su modelo interno. Solo devuelve tipos de `shared_kernel` o primitivos del contrato.**
> **5. Los PUERTOS DE DOMINIO (para lógica pura) SÍ pueden vivir en `domain/`. Los ADAPTADORES (que implementan puertos de OTRO contexto) NUNCA viven en `domain/`.**

---

## 2. Estado Actual

### ✅ Completado (Dominio Estable)

| Componente | Estado | Ubicación |
|-------------|--------|-----------|
| `CropCycle` (Aggregate Root) | ✅ Implementado | `agriculture/domain/cycle.rs` |
| `Schedule` (Entity) | ✅ Implementado | `agriculture/domain/planning.rs` |
| `Budget` (Aggregate Root) | ✅ Implementado | `finance/domain/budget.rs` |
| `VarianceService` | ✅ Implementado (función pura) | `agriculture/domain/services/variance_service.rs` |
| `EconomicVarianceService` | ✅ Implementado (función pura) | `agriculture/domain/services/economic_variance.rs` |
| `EconomicDataProvider` (Puerto de Dominio) | ✅ Definido en dominio | `agriculture/domain/services/economic_variance.rs` |

### 🔄 En Progreso (Capa de Aplicación)

| Componente | Estado | Ubicación |
|-------------|--------|-----------|
| `CropCycleRepository` (Puerto de Aplicación) | ✅ Definido | `agriculture/application/ports/cycle_repository.rs` |
| `ScheduleRepository` (Puerto de Aplicación) | ✅ Definido | `agriculture/application/ports/schedule_repository.rs` |
| `InMemoryCropCycleRepository` | ✅ Implementado | `agriculture/application/ports/cycle_repository.rs` |
| `InMemoryScheduleRepository` | ✅ Implementado | `agriculture/application/ports/schedule_repository.rs` |
| `analyze_variance` (Caso de Uso) | ✅ Implementado | `agriculture/application/use_cases/analyze_variance.rs` |
| `schedule_crop_cycle` (Caso de Uso) | 🔄 Esqueleto | `agriculture/application/use_cases/schedule_crop_cycle.rs` |

### ✅ CORRECCIÓN APLICADA: Ubicación de `FinanceEconomicProvider`

| Componente | Estado | Ubicación |
|-------------|--------|-----------|
| `FinanceEconomicProvider` | ✅ Movido a infraestructura | `finance/infrastructure/adapters/economic_provider.rs` |
| Resolución de `budget_id` | ✅ Vía `BudgetRepository` inyectado | Ver sección 3 |

### ❌ Pendiente (Infraestructura y Otros Contextos)

| Componente | Estado | Ubicación Esperada |
|-------------|--------|-------------------|
| Repositorios PostGIS/SQL | ❌ Pendiente | `agriculture/infrastructure/repositories/` |
| Casos de uso para `finance` | ❌ Pendiente | `finance/application/use_cases/` |
| Casos de uso para `labor` | ❌ Pendiente | `labor/application/use_cases/` |
| **Capa de Integración** | ❌ Pendiente | `server/src/integration/` |

---

## 3. Patrón de Conexión Entre Contextos (CORREGIDO)

### Reglas del Patrón (EXPLÍCITAS Y RELAJADAS)

1. **Puertos de Dominio** (para lógica pura del dominio): Viven en `domain/` del contexto que LOS NECESITA.
   - Ej: `EconomicDataProvider` vive en `agriculture/domain/` porque `EconomicVarianceService` (dominio) lo necesita.
2. **Puertos de Aplicación** (para persistencia, casos de uso): Viven en `application/ports/` del contexto que LOS NECESITA.
   - Ej: `CropCycleRepository` vive en `agriculture/application/ports/`.
3. **Adaptadores** (implementan puertos de OTRO contexto): NUNCA en `domain/`. Viven en `infrastructure/adapters/` del contexto PROVEEDOR.
4. **El adaptador NO debe exponer entidades del dominio proveedor. Solo devuelve tipos de `shared_kernel` o primitivos del contrato.**
5. **Resolución de IDs**: El adaptador inyecta el repositorio correspondiente para resolver IDs a entidades internas.

### Ejemplo: Agriculture → Finance (Datos Económicos) - CORREGIDO

```
agriculture/domain/services/economic_variance.rs
    ↓ define PUERTO DE DOMINIO (cliente lo necesita para lógica pura)
pub trait EconomicDataProvider {
    fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money>;
    fn get_actual_cost(&self, record_id: &str) -> Option<Money>;
}

finance/infrastructure/adapters/economic_provider.rs  ← CORRECTO: en infrastructure/, no domain/
    ↓ implementa puerto (servidor provee datos)
pub struct FinanceEconomicProvider {
    budget_id: BudgetId,  ← Solo guarda el ID, no la entidad
    budget_repo: Arc<dyn BudgetRepository>,  ← Inyecta dependencia para resolver
}

impl EconomicDataProvider for FinanceEconomicProvider {
    fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money> {
        // Resolución: ID → Entidad vía repositorio
        let budget = self.budget_repo.find_by_id(&self.budget_id)?;
        // Solo devuelve Money (shared_kernel), no Budget
        budget.get_planned_cost(&planned_id.0)
    }
    
    fn get_actual_cost(&self, record_id: &str) -> Option<Money> {
        let budget = self.budget_repo.find_by_id(&self.budget_id)?;
        budget.get_actual_cost_for_activity(record_id)
    }
}
```

### ¿Por qué `budget_repo` y no `budget: Budget`?

1. **Desacoplamiento**: Agriculture no "sabe" que existe `Budget` como estructura interna.
2. **Flexibilidad**: El repositorio puede ser en memoria, PostGIS, o un mock.
3. **Ciclo de vida**: El adaptador no posee el `Budget`, solo sabe cómo obtenerlo.

---

## 4. Estrategia por Módulo

### Tipos de Componentes en Application

#### A. Puertos de Aplicación (Ports) — `application/ports/`
Traits que definen contratos para persistencia. Viven en el contexto que los necesita.

#### B. DTOs (Data Transfer Objects) — `application/dtos/`
Contratos de entrada/salida para casos de uso. **NO son entidades del dominio**.
- Separan datos de dependencias.

#### C. Casos de Uso (Use Cases) — `application/use_cases/`
Orquestan una acción CONCRETA del usuario. Usan DTOs.
- **Input**: Separa `data: InputDTO` de `dependencies: Dependencies`.

#### D. Servicios de Aplicación (Application Services) — `application/services/`
Lógica reutilizable entre casos de uso. Ej: `VarianceApplicationService`.

### 4.1 Contexto: `agriculture` (Core)

#### A. Puertos Necesarios

**Dominio (en `domain/`):**
- ✅ `EconomicDataProvider` (para `EconomicVarianceService`)

**Aplicación (en `application/ports/`):**
Ya definidos:
- ✅ `CropCycleRepository`
- ✅ `ScheduleRepository`

Pendientes:
- ❌ `ActivityRepository` — para persistir `ActivityRecord`
- ❌ `AnalysisRepository` — para persistir resultados de análisis
- ❌ `FarmRepository` — para gestión de fincas
- ❌ `AreaRepository` — para gestión de parcelas

#### B. DTOs (en `application/dtos/`)

**Input DTOs** (solo datos, no dependencias):
```rust
// agriculture/application/dtos/analyze_variance_dto.rs
pub struct AnalyzeVarianceInputDTO {
    pub cycle_id: CycleId,
    pub schedule_id: ScheduleId,
    pub config: VarianceConfigDTO,
}
```

**Dependencies struct** (separada del DTO):
```rust
// agriculture/application/use_cases/analyze_variance.rs
pub struct AnalyzeVarianceDependencies {
    pub economic_provider: Arc<dyn EconomicDataProvider>,
    pub cycle_repo: Arc<dyn CropCycleRepository>,
    pub schedule_repo: Arc<dyn ScheduleRepository>,
}

pub fn execute(
    dto: AnalyzeVarianceInputDTO,
    deps: AnalyzeVarianceDependencies,
) -> AnalyzeVarianceOutputDTO {
    // 1. Obtener entidades desde repos (usando DTO data)
    let cycle = deps.cycle_repo.find_by_id(&dto.cycle_id)?;
    let schedule = deps.schedule_repo.find_by_id(&dto.schedule_id)?;
    
    // 2. Ejecutar lógica de dominio
    let timing_report = VarianceService::analyze_with_config(&schedule, &cycle, &dto.config);
    
    // 3. Análisis económico (si hay provider)
    let economic_report = deps.economic_provider.as_ref().map(|p| {
        EconomicVarianceService::analyze_costs(&timing_report.matched, p)
    });
    
    // 4. Retornar DTO de salida
    AnalyzeVarianceOutputDTO { timing_report, economic_report }
}
```

**Por qué separar DTOs de Dependencias:**
- El DTO es serializable (para API/JSON).
- Las dependencias son para inyección de arquitectura.
- Claridad total: qué es DATOS vs qué es INFRAESTRUCTURA.

#### C. Casos de Uso

| Caso de Uso | Input DTO | Dependencies | Output DTO | Servicios Usados |
|-------------|-----------|-------------|------------|------------------|
| `analyze_variance` | `AnalyzeVarianceInputDTO` | `AnalyzeVarianceDependencies` | `AnalyzeVarianceOutputDTO` | `VarianceService`, `EconomicVarianceService` |
| `schedule_crop_cycle` | `ScheduleCropCycleInputDTO` | `ScheduleDependencies` | `ScheduleDTO` | `Schedule` (entity) |
| `register_activity` | `RegisterActivityInputDTO` | `RegisterActivityDependencies` | `ActivityRecordDTO` | `CropCycle.register_activity()` |

#### D. Servicios de Aplicación (en `application/services/`)

- ❌ `VarianceApplicationService` — reutilizable por `get_cycle_status`, `analyze_variance`, etc.

#### E. Infraestructura (en `infrastructure/repositories/`)

Implementar para PostGIS/PostgreSQL:
- `PostGisCropCycleRepository`
- `PostGisScheduleRepository`
- `PostGisActivityRepository`

### 4.2 Contexto: `finance` (Supporting)

#### A. Puertos Necesarios

En `finance/application/ports/`:
- ❌ `BudgetRepository`
- ❌ `ExpenseRepository`

#### B. DTOs

```rust
pub struct CreateBudgetInputDTO {
    pub cycle_id: CycleId,
    pub period: PeriodDTO,
    pub baseline: MoneyDTO,
}
```

#### C. Casos de Uso (SOLO INTENCIONES DEL USUARIO)

En `finance/application/use_cases/`:
- ❌ `create_budget` — crear presupuesto para un ciclo
- ❌ `register_expense` — registrar gasto
- ❌ `get_budget_variance` — consultar varianza financiera
- ~~❌ `provide_economic_data`~~ — **ELIMINADO**: no es caso de uso, es adaptador en `infrastructure/adapters/`

#### D. Adaptadores (en `finance/infrastructure/adapters/`)

- ✅ `FinanceEconomicProvider` ← **YA MOVIDO ACÁ**
  - Usa `BudgetRepository` inyectado
  - No expone `Budget` directamente
  - Solo devuelve `Money` (shared_kernel)

### 4.3 Contexto: `labor` (Supporting)

#### A. Puertos Necesarios

En `labor/application/ports/`:
- ❌ `WorkerRepository`
- ❌ `WorkRecordRepository`
- ❌ `LaborCostProvider` (puerto para agriculture, definido en `agriculture/application/ports/`)

#### B. Casos de Uso

En `labor/application/use_cases/`:
- ❌ `register_worker`
- ❌ `record_work`
- ❌ `get_labor_cost_for_cycle` — expone datos para agriculture

#### C. Adaptadores (en `labor/infrastructure/adapters/`)

- ❌ `LaborCostAdapter` — implementa puerto de agriculture
  - Usa `WorkerRepository` y `WorkRecordRepository`
  - Solo devuelve `Money` o DTOs

### 4.4 Contexto: `shared_kernel` (El Estabilizador)

NO tiene aplicación ni infraestructura. Solo define:
- `Money`, `Currency`, `ExchangeRateProvider`
- `AreaId`, `CycleId`, `CropId`, etc.
- `Period`, `Area`, `Measurement`

---

## 5. Orden Recomendado de Implementación

### Fase 1: Agriculture (Core) — **EN PROGRESO**

#### Paso 1: Separar DTOs de Dependencias en `analyze_variance`
- ❌ Crear `application/dtos/analyze_variance_dto.rs`
- ❌ Refactorizar `analyze_variance.rs` para usar `InputDTO` + `Dependencies`

#### Paso 2: Definir Puertos Faltantes
En `agriculture/application/ports/`:
- ❌ `activity_repository.rs`
- ❌ `farm_repository.rs`
- ❌ `area_repository.rs`

#### Paso 3: Implementar Repos en Memoria
En `agriculture/application/ports/` (mismo archivo, para testing):
- `InMemoryActivityRepository`
- `InMemoryFarmRepository`
- `InMemoryAreaRepository`

#### Paso 4: Crear Casos de Uso Faltantes
En `agriculture/application/use_cases/`:
- ❌ `register_activity.rs` — usar DTO + Dependencies
- ❌ `register_farm.rs`
- ❌ `register_area.rs`
- ❌ `get_cycle_status.rs`

#### Paso 5: Infraestructura Real (PostGIS)
En `agriculture/infrastructure/repositories/`:
- ❌ `postgis_crop_cycle_repository.rs`
- ❌ `postgis_schedule_repository.rs`
- ❌ `postgis_activity_repository.rs`

---

### Fase 2: Finance (Supporting)

#### Paso 1: Puertos
En `finance/application/ports/`:
- ❌ `budget_repository.rs`
- ❌ `expense_repository.rs`

#### Paso 2: DTOs
En `finance/application/dtos/`:
- ❌ `create_budget_dto.rs`
- ❌ `register_expense_dto.rs`

#### Paso 3: Casos de Uso (solo intenciones del usuario)
En `finance/application/use_cases/`:
- ❌ `create_budget.rs`
- ❌ `register_expense.rs`
- ❌ `get_budget_variance.rs`

#### Paso 4: Adaptadores (en `finance/infrastructure/adapters/`)
- ✅ `economic_provider.rs` ← **YA CORREGIDO**
- ❌ `postgre_budget_repository.rs`

---

### Fase 3: Labor (Supporting)

#### Paso 1: Puertos
En `labor/application/ports/`:
- ❌ `worker_repository.rs`
- ❌ `work_record_repository.rs`

#### Paso 2: Casos de Uso
En `labor/application/use_cases/`:
- ❌ `register_worker.rs`
- ❌ `record_work.rs`

#### Paso 3: Adaptadores
En `labor/infrastructure/adapters/`:
- ❌ `labor_cost_adapter.rs` (implementa puerto de agriculture)

---

### Fase 4: Integración Transversal (Naming Ajustado)

Casos de uso que cruzan múltiples contextos. **No pertenecen a un solo bounded context**.

#### Ubicación: `server/src/integration/` (mejor que `orchestration/`)

```
server/src/integration/
├── mod.rs
├── start_cycle.rs        // Crea CropCycle + Schedule + Budget
├── close_cycle.rs        // Cierra ciclo, genera reporte final
└── get_full_variance.rs  // Varianza completa: tiempo + costo + mano de obra
```

| Caso de Uso de Integración | Contextos Involucrados | Descripción |
|-----------------------------|-----------------------|-------------|
| `start_cycle` | agriculture + finance | Crea ciclo, cronograma y presupuesto atómicamente |
| `close_cycle` | agriculture + finance + labor | Cierra ciclo, genera reporte final |
| `get_full_variance` | agriculture + finance + labor | Varianza completa: tiempo + costo + mano de obra |

---

## 6. Consideraciones Transversales

### 6.1 Conexión Agriculture ↔ Finance (Corregido)

```
Agriculture Application Layer
    ↓ usa puerto (trait definido en agriculture domain/)
EconomicDataProvider (trait)
    ↑ implementa adaptador
Finance Infrastructure Layer (finance/infrastructure/adapters/)
    ↓ usa repositorio de aplicación
BudgetRepository (trait en finance/application/ports/)
    ↑ implementa
PostgreBudgetRepository / InMemoryBudgetRepository
    ↓ consulta
PostgreSQL / InMemory
```

**Diferencia clave:** El adaptador ya NO tiene `budget: Budget` directo. Usa el repositorio para resolver `budget_id`.

### 6.2 Inyección de Dependencias (CORREGIDO)

#### Dominio: Generics (monomorfismo, compile-time)
```rust
// En domain/services/ (funciones puras)
pub fn analyze_with_config<P: SomeTrait>(...) { ... }
```

#### Aplicación: `dyn Trait` (polimorfismo dinámico) + Separación DTO/Dependencies
```rust
// En application/use_cases/ (flexible, preparado para DI)
use std::sync::Arc;

pub struct AnalyzeVarianceDependencies {
    pub economic_provider: Option<Arc<dyn EconomicDataProvider>>,
    pub cycle_repo: Arc<dyn CropCycleRepository>,
    pub schedule_repo: Arc<dyn ScheduleRepository>,
}

pub fn execute(
    dto: AnalyzeVarianceInputDTO,  // Solo datos
    deps: AnalyzeVarianceDependencies,  // Solo dependencias
) -> AnalyzeVarianceOutputDTO {
    // ...
}
```

**Por qué:**
- `dyn Trait` permite cambiar implementaciones en runtime.
- Prepara el sistema para frameworks de DI (Axum, Actix, etc.).
- Los generics quedan para lógica de dominio pura.
- Separar DTO de Dependencies = claridad arquitectónica total.

### 6.3 Application Services vs Use Cases

| Componente | Rol | Ejemplo |
|------------|-----|---------|
| **Application Service** | Lógica reutilizable entre casos | `VarianceApplicationService` (usado por `analyze_variance` y `get_cycle_status`) |
| **Use Case** | Orquesta una acción CONCRETA del usuario | `analyze_variance`, `start_cycle` |

---

## 7. Manejo de Errores (NUEVO)

### Estrategia Unificada

#### 1. Dominio: Errores de Negocio
- `AgricultureError`, `FinanceError` (ya definidos)
- Son enums que representan casos de fallo de negocio.

#### 2. Aplicación: Wrapper Unificado
```rust
// server/src/application/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] Box<dyn std::error::Error + Send + Sync>),
    
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    
    #[error("Provider error: {0}")]
    Provider(#[from] ProviderError),
}

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Connection error: {0}")]
    Connection(String),
}
```

#### 3. Infraestructura: Mapeo de Errores
```rust
// En infrastructure/repositories/
impl CropCycleRepository for PostgisCropCycleRepository {
    fn find_by_id(&self, id: &CycleId) -> Option<CropCycle> {
        // Mapear error de BD a RepositoryError
        // Propagar via ApplicationError
    }
}
```

#### 4. Casos de Uso: Retornan `Result<OutputDTO, ApplicationError>`
```rust
pub fn execute(
    dto: AnalyzeVarianceInputDTO,
    deps: AnalyzeVarianceDependencies,
) -> Result<AnalyzeVarianceOutputDTO, ApplicationError> {
    let cycle = deps.cycle_repo.find_by_id(&dto.cycle_id)
        .ok_or_else(|| ApplicationError::Repository(RepositoryError::NotFound(id.0.clone())))?;
    // ...
}
```

---

## 8. Contract Testing (NUEVO)

### ¿Por qué?

Asegura que los adaptadores respeten los contratos (puertos) sin depender de la implementación concreta. Crítico cuando:
- Cambias de una BD a otra.
- Mockeas proveedores externos.
- Tienes múltiples implementaciones de un puerto.

### Estrategia para Kora

#### 1. Tests de Contrato para Puertos de Aplicación
```rust
// agriculture/application/ports/cycle_repository.rs
#[cfg(test)]
mod contract_tests {
    use super::*;
    
    // TODO: GENERIC TEST FUNCTION
    // fn test_crop_cycle_repo_contract<R: CropCycleRepository>(repo: &mut R) {
    //     // 1. Save
    //     let cycle = CropCycle::new(...);
    //     repo.save(cycle.clone());
    //     
    //     // 2. Find by ID
    //     let found = repo.find_by_id(cycle.id());
    //     assert!(found.is_some());
    //     assert_eq!(found.unwrap().id(), cycle.id());
    //     
    //     // 3. Not found
    //     let not_found = repo.find_by_id(&CycleId("non-existent".to_string()));
    //     assert!(not_found.is_none());
    // }
    
    // #[test]
    // fn inmemory_repo_satisfies_contract() {
    //     let mut repo = InMemoryCropCycleRepository::new();
    //     test_crop_cycle_repo_contract(&mut repo);
    // }
}
```

#### 2. Tests de Contrato para Puertos de Dominio (como `EconomicDataProvider`)
```rust
// agriculture/domain/services/economic_variance.rs
#[cfg(test)]
mod contract_tests {
    use super::*;
    
    // fn test_economic_data_provider_contract<P: EconomicDataProvider>(provider: &P) {
    //     // Test: get_planned_cost returns Some for known ID
    //     // Test: get_planned_cost returns None for unknown ID
    //     // Test: get_actual_cost returns Some for known ID
    // }
}
```

#### 3. Integración: Verificar que Adaptador cumple Contrato
```rust
// finance/infrastructure/adapters/economic_provider.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn finance_adapter_satisfies_economic_data_provider_contract() {
    //     let repo = InMemoryBudgetRepository::new();
    //     let provider = FinanceEconomicProvider::new(budget_id, Arc::new(repo));
    //     test_economic_data_provider_contract(&provider);
    // }
}
```

---

## Resumen de Próximos Pasos Inmediatos (CORREGIDOS)

1. ✅ **Mover `FinanceEconomicProvider`** de `finance/domain/adapters/` a `finance/infrastructure/adapters/` ← HECHO
2. ✅ **Corregir `FinanceEconomicProvider`** para que use `BudgetRepository` y resuelva `budget_id` ← HECHO
3. ❌ **Separar DTOs de Dependencias** en `analyze_variance` (crear `application/dtos/`)
4. ❌ **Cambiar `Input` structs** para usar `InputDTO` + `Dependencies` en todos los casos de uso
5. ❌ **Crear capa de integración** `server/src/integration/` (en lugar de `orchestration/`)
6. ❌ **Implementar estrategia de errores unificada** (`ApplicationError`)
7. ❌ **Agregar contract tests** para puertos y adaptadores
8. ❌ **Definir `activity_repository.rs`**, `farm_repository.rs`, `area_repository.rs`
9. ❌ **Crear casos de uso básicos para `finance`** (solo intenciones del usuario)

---

## Conclusión

Tu revisión fue **QUIRÚRGICA**. Los ajustes clave:
1. ✅ Relajar regla de puertos (dominio SÍ puede tener puertos para lógica pura)
2. ✅ `budget_id` se resuelve vía `BudgetRepository` (ya no exponemos `Budget`)
3. ✅ Separar DTOs de Dependencias (claridad total)
4. ✅ Naming: `integration/` en lugar de `orchestration/`
5. ✅ Estrategia de errores unificada (`ApplicationError`)
6. ✅ Contract testing para asegurar cumplimiento de contratos

Con estos cambios, Kora tendrá una arquitectura hexagonal REAL, portable y mantenible.

---

*Documento actualizado el 2026-05-02 con ajustes arquitectónicos precisos tras revisión.*
