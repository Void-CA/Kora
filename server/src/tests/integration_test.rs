// Test de Integración: Flujo completo Plan → Ejecutar → Analizar
// Este test valida si el dominio actual soporta flujos de negocio reales.

use crate::agriculture::domain::{CropCycle, Schedule, ScheduleAnchor, PlannedActivity, Activity, ActivityStatus, ActivityCategory, IntegrityStatus};
use crate::finance::domain::{Budget, ExpenseCategory, Expense};
use crate::shared_kernel::ids::{CycleId, CropId, AreaId, BudgetId};
use crate::shared_kernel::time::Period;
use crate::shared_kernel::money::{Money, Currency, ExchangeRateProvider, RateError};
use rust_decimal::Decimal;
use std::str::FromStr;
// --- Mocks y Helpers ---
struct ProveedorTasa;
impl ExchangeRateProvider for ProveedorTasa {
    fn get_rate(&self, from: Currency, to: Currency) -> Result<Decimal, RateError> {
        match (from, to) {
            (Currency::USD, Currency::NIO) => Ok(Decimal::from(36)), // 1 USD = 36 NIO
            (Currency::NIO, Currency::USD) => Ok(Decimal::from_str("0.0277").unwrap()),
            _ => Err(RateError::RateNotAvailable(from, to)),
        }
    }
}

fn configurar_flujo() -> (CropCycle, Schedule, Budget) {
    // 1. PLAN: Crear Ciclo
    let periodo = Period::new(1000, 2000).unwrap();
    let mut ciclo = CropCycle::new(
        CropId("crop-maiz-1".to_string()),
        AreaId("area-lote-a".to_string()),
        periodo,
    );

    // 2. PLAN: Crear Cronograma anclado a la Fecha de Siembra
    let mut cronograma = Schedule::new(
        ciclo.id().clone(),
        ScheduleAnchor::SowingDate,
        1500, // fecha ancla (timestamp)
    );

    // Planificado: Siembra en el día 0 (timestamp 1500)
    cronograma.add_planned_activity(PlannedActivity {
        category: ActivityCategory::Sowing,
        relative_day: 0,
        status: ActivityStatus::Planned,
    });

    // Planificado: Fertilización en el día +15 (timestamp 1515)
    cronograma.add_planned_activity(PlannedActivity {
        category: ActivityCategory::Maintenance, // Usamos Maintenance como proxy para Fertilización
        relative_day: 15,
        status: ActivityStatus::Planned,
    });

    // 3. PLAN: Crear Presupuesto para este ciclo
    let linea_base = Money::new(Decimal::from(1000), Currency::USD);
    let proveedor = Box::new(ProveedorTasa);
    let presupuesto = Budget::new(
        ciclo.id().clone(),
        Period::new(900, 2500).unwrap(), // Periodo más amplio que el ciclo
        linea_base,
        proveedor,
    );

    (ciclo, cronograma, presupuesto)
}

// --- TEST DE INTEGRACIÓN ---

#[cfg(test)]
mod tests_integracion {
    use super::*;

    #[test]
    fn flujo_completo_plan_ejecutar_analizar() {
        let (mut ciclo, cronograma, mut presupuesto) = configurar_flujo();

        // --- EJECUTAR: Registrar actividades (la "Realidad") ---

        // Actividad 1: Siembra en el día 0 (timestamp 1500) -> VÁLIDO (coincide con cronograma)
        let actividad1 = Activity::new(1500, ActivityCategory::Sowing);
        let registro1 = ciclo.register_activity(actividad1).unwrap();
        assert_eq!(registro1.integrity[0], IntegrityStatus::Valid);

        // Actividad 2: Fertilización en el día +20 (timestamp 1520) -> VÁLIDO (dentro del periodo)
        // pero está "Desplanificada" (el cronograma decía día +15)
        let actividad2 = Activity::new(1520, ActivityCategory::Maintenance);
        let registro2 = ciclo.register_activity(actividad2).unwrap();
        assert_eq!(registro2.integrity[0], IntegrityStatus::Valid); 
        // NOTA: No podemos marcarla como "Desplanificada" porque no hay lógica que compare cronograma vs actividad.
        // ESTO ES LO QUE FALTA.

        // Actividad 3: Cosecha fuera del periodo del ciclo -> FUERA_DE_PERIODO
        let actividad3 = Activity::new(2500, ActivityCategory::Harvest);
        let registro3 = ciclo.register_activity(actividad3).unwrap();
        assert_eq!(registro3.integrity[0], IntegrityStatus::OutsidePeriod);

        // --- EJECUTAR: Registrar gastos contra el presupuesto ---
        let gasto1 = Money::new(Decimal::from(300), Currency::USD);
        assert!(presupuesto.register_expense(&gasto1).is_ok());

        let gasto2 = Money::new(Decimal::from(800), Currency::USD); // Excede la línea base de 1000
        assert!(presupuesto.register_expense(&gasto2).is_ok()); // ¡No bloquea! (filosofía imperfección controlada)

        // --- ANALIZAR: Qué podemos hacer? Qué falta? ---

        // 1. Varianza de Presupuesto: SÍ podemos calcular esto.
        let varianza = presupuesto.get_variance().unwrap();
        assert!(varianza.amount > Decimal::ZERO); // Sobre gasto (300 + 800 > 1000)
        println!("Varianza de Presupuesto: {:?} sobre presupuesto", varianza);

        // 2. Cronograma vs Realidad: NO podemos comparar fácilmente.
        // Tenemos: cronograma.activities (planificadas) y ciclo.executed_activities (reales)
        // Pero no hay un servicio que las relacione.
        
        // VERIFICACIÓN MANUAL (revela la brecha):
        // Planificado: Siembra @ día relativo 0 (ts 1500)
        // Realidad: registro1 @ ts 1500 -> COINCIDE
        // Planificado: Fertilización @ día relativo 15 (ts 1515)
        // Realidad: registro2 @ ts 1520 -> NO COINCIDE (retraso de 5 días)
        
        // Para hacer esto correctamente, necesitamos un VarianceService que:
        // - Tome Schedule + Vec<ActivityRecord>
        // - Relacione category + timestamp vs relative_day
        // - Produzca VarianceReport { a_tiempo: Vec<..>, retrasado: Vec<..>, desplanificado: Vec<..> }

        // 3. Remanente del presupuesto
        let remanente = presupuesto.get_remaining().unwrap();
        assert!(remanente.amount < Decimal::ZERO); // Negativo = sobre gasto
        println!("Remanente: {:?} (negativo = sobre gasto)", remanente);

        // --- CONCLUSIÓN DE ESTE TEST ---
        // El dominio soporta:
        // ✅ Creación de ciclos, cronogramas, presupuestos
        // ✅ Registro de actividades con estado de integridad
        // ✅ Registro de gastos sin bloqueo
        // ✅ Cálculo de varianza de presupuesto
        //
        // FALTA para el análisis completo "Plan vs Realidad":
        // ❌ VarianceService para comparar Schedule.activities con CropCycle.executed_activities
        // ❌ Lógica para marcar ActivityRecord con Unplanned si no coincide con PlannedActivity
        // ❌ Lógica para calcular "retraso" (diferencia entre relative_day y timestamp real)
    }
}
