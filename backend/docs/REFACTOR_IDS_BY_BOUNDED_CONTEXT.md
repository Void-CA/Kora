# Refactor: IDs por Bounded Context

**Fecha**: 3 de mayo de 2026  
**Autor**: Refactorización guiada por arquitectura DDD  
**Estado**: ✅ Completado - 32 tests pasando

---

## Objetivo

Refactorizar los IDs (identificadores de dominio) moviéndolos desde `shared_kernel` hacia sus respectivos Bounded Contexts, siguiendo los principios de Domain-Driven Design (DDD).

**Regla clave**: "Compartido ≠ usado en múltiples lugares". El shared kernel solo debe contener conceptos verdaderamente compartidos.

---

## Principios Arquitectónicos Aplicados

1. **DDD Bounded Contexts**: Cada contexto delimita su propio modelo, incluyendo IDs
2. **Sin dependencias cruzadas**: Los contextos se integran vía puertos (ports) y adaptadores, NO compartiendo tipos
3. **Traducción en el adaptador**: Los adaptadores traducen IDs en el límite del contexto, no adoptan tipos de otros dominios
4. **Capas claras**:
   - **Application layer**: usa `dyn Trait` (dynamic dispatch)
   - **Domain layer**: usa genéricos (static dispatch)

---

## Cambios Realizados

### 1. Creación de módulos de IDs por contexto

#### `agriculture/domain/ids.rs` (5 IDs)
```rust
pub struct PlannedActivityId(String);
pub struct ActivityRecordId(String);
pub struct ScheduleId(String);
pub struct ActivityId(String);
pub struct FarmId(String);
```

#### `finance/domain/ids.rs` (2 IDs)
```rust
pub struct BudgetId(String);
pub struct ExpenseId(String);
```

#### `labor/domain/ids.rs` (1 ID)
```rust
pub struct WorkerId(String);
```

### 2. Limpieza de `shared_kernel/ids.rs`

**Antes**: 11 IDs (mezclados de todos los contextos)  
**Después**: 3 IDs verdaderamente compartidos

```rust
pub struct CycleId(String);    // Usado por agriculture Y finance
pub struct CropId(String);     // Usado por agriculture Y finance
pub struct AreaId(String);     // Usado por agriculture Y finance
```

### 3. Actualización de todas las importaciones

Se actualizaron 10+ archivos para usar las nuevas rutas de módulos:

| Archivo | Cambio |
|---------|--------|
| `agriculture/domain/activity.rs` | `super::ids::*` |
| `agriculture/domain/cycle.rs` | `super::ids::*` |
| `agriculture/domain/services/economic_variance.rs` | `super::super::domain::ids::*` |
| `finance/domain/budget.rs` | `super::super::domain::ids::BudgetId` |
| `finance/infrastructure/adapters/agriculture_economic_provider.rs` | `agriculture::domain::ids::*` |
| `labor/domain/worker.rs` | `super::ids::WorkerId` |

### 4. Cambios en la API de Budget (Evitar dependencias cruzadas)

**Problema**: `Budget` usaba tipos de `agriculture` en sus HashMaps:
```rust
// ANTES (incorrecto - cross-context dependency)
pub struct Budget {
    planned_costs: HashMap<PlannedActivityId, Money>,  // Tipo de agriculture
    actual_costs: HashMap<ActivityRecordId, Money>,     // Tipo de agriculture
}
```

**Solución**: Usar `&str` (el adaptador traduce):
```rust
// DESPUÉS (correcto - sin dependencias cruzadas)
pub struct Budget {
    planned_costs: HashMap<String, Money>,
    actual_costs: HashMap<String, Money>,
}

impl Budget {
    pub fn get_planned_cost(&self, planned_id: &str) -> Option<Money> { ... }
    pub fn get_actual_cost_for_activity(&self, record_id: &str) -> Option<Money> { ... }
}
```

### 5. FinanceEconomicProvider (Adaptador)

El adaptador traduce IDs en el límite del contexto:

```rust
impl EconomicDataProvider for FinanceEconomicProvider {
    fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money> {
        // Traduce de agriculture ID a &str
        self.budget.get_planned_cost(planned_id.as_str())
    }
    
    fn get_actual_cost(&self, record_id: &ActivityRecordId) -> Option<Money> {
        // Traduce de agriculture ID a &str
        self.budget.get_actual_cost_for_activity(record_id.as_str())
    }
}
```

---

## Corrección de Errores

### Error E0592: Duplicate `as_str`
**Causa**: `as_str()` definido tanto en `activity.rs` como en `ids.rs`  
**Solución**: Eliminado de `activity.rs`, mantenido en `ids.rs` (single responsibility)

### Error E0425/E0432: Unresolved imports
**Causa**: Imports apuntando a rutas antiguas de `shared_kernel::ids`  
**Solución**: Actualizadas todas las importaciones a los nuevos módulos

### Error en tests: `from_str` not in scope
**Causa**: `rust_decimal::Decimal::from_str()` requiere trait `FromStr` en scope  
**Solución**: Agregado `use std::str::FromStr;` en el módulo de tests

---

## Tests Restaurados

Se restauraron 7 tests unitarios en `finance/domain/budget.rs` que habían sido eliminados previamente:

1. `budget_new_initializes_correctly`
2. `register_expense_updates_current`
3. `register_expense_exceeding_budget_no_block`
4. `get_remaining_under_budget`
5. `get_variance_over_budget`
6. `plan_cost_stores_planned_activity`
7. `record_actual_cost_stores_activity_record`

**Total de tests**: 32 pasando ✅

---

## Estructura Final de Archivos

```
src/
├── shared_kernel/
│   └── ids.rs              # 3 IDs: CycleId, CropId, AreaId
├── agriculture/
│   └── domain/
│       └── ids.rs          # 5 IDs: PlannedActivityId, ActivityRecordId, ScheduleId, ActivityId, FarmId
├── finance/
│   ├── domain/
│   │   └── ids.rs          # 2 IDs: BudgetId, ExpenseId
│   └── infrastructure/
│       └── adapters/
│           └── agriculture_economic_provider.rs  # Traduce IDs en el límite
└── labor/
    └── domain/
        └── ids.rs          # 1 ID: WorkerId
```

---

## Beneficios Obtenidos

1. **Alineación con DDD**: Cada Bounded Context tiene su propio modelo de IDs
2. **Sin acoplamiento**: Finance no depende de tipos de Agriculture
3. **Claridad conceptual**: Es obvio qué IDs pertenecen a qué contexto
4. **Escalabilidad**: Agregar nuevos IDs no requiere tocar `shared_kernel`
5. **Integración limpia**: Los adaptadores traducen en el límite, no hay fugas de tipos

---

## Decisiones de Diseño

| Decisión | Justificación |
|----------|---------------|
| IDs como `String` interno (UUID v4) | Simplicidad, compatibilidad con bases de datos, serialización fácil |
| Métodos `new()` y `as_str()` en todos los IDs | API consistente, conversión fácil a/desde strings |
| Budget usa `&str` en HashMaps | Evita dependencia de tipos de agriculture, el adaptador traduce |
| Application usa `dyn Trait` | Flexibilidad para testing y múltiples implementaciones |
| Domain usa genéricos | Rendimiento (static dispatch) en lógica crítica |

---

## Verificación

```bash
$ cd /home/void/projects/web/Kora/backend
$ cargo test
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured
```

---

## Notas para el Futuro

- Al agregar nuevos IDs, preguntarse: "¿Es verdaderamente compartido (usado por múltiples contextos)?" Si no, va en su propio contexto.
- Los adaptadores SIEMPRE deben traducir en el límite. No hacer que un contexto "conozca" los tipos de otro.
- Si dos contextos necesitan el mismo concepto (ej. Money), ese sí va en shared_kernel.
