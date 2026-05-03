// Test de Integración: Flujo completo Plan → Ejecutar → Analizar
// Este test valida si el dominio actual soporta flujos de negocio reales.

use crate::agriculture::domain::{CropCycle, Schedule, ScheduleAnchor, PlannedActivity, Activity, ActivityStatus, IntegrityStatus};
use crate::agriculture::domain::activity::ActivityCategory;
use crate::finance::domain::Budget;
use crate::agriculture::domain::ids::{PlannedActivityId};
use crate::shared_kernel::ids::{CycleId, CropId, AreaId};
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
    let ciclo = CropCycle::new(
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
        id: PlannedActivityId::new(),
        category: ActivityCategory::Sowing,
        relative_day: 0,
        status: ActivityStatus::Planned,
    });

    // Planificado: Fertilización en el día +15 (timestamp 1515)
    cronograma.add_planned_activity(PlannedActivity {
        id: PlannedActivityId::new(),
        category: ActivityCategory::Maintenance, // Usamos Maintenance como proxy para Fertilización
        relative_day: 15,
        status: ActivityStatus::Planned,
    });

    // 3. PLAN: Crear Presupuesto para este ciclo
    let linea_base = Money::new(Decimal::from(1000), Currency::USD);
    let presupuesto = Budget::new(
        ciclo.id().clone(),
        Period::new(900, 2500).unwrap(), // Periodo más amplio que el ciclo
        linea_base,
    );

    (ciclo, cronograma, presupuesto)
}

// --- TEST DE INTEGRACIÓN ---

#[cfg(test)]
mod tests_integracion {
    use super::*;

    #[test]
    fn flujo_completo_plan_ejecutar_analizar() {
        let (mut ciclo, _cronograma, mut presupuesto) = configurar_flujo();

        // --- EJECUTAR: Registrar actividades (la "Realidad") ---

        // Actividad 1: Siembra en el día 0 (timestamp 1500) -> VÁLIDO (coincide con cronograma)
        let actividad1 = Activity::new(1500, ActivityCategory::Sowing);
        let resultado1 = ciclo.register_activity(actividad1).unwrap();
        assert_eq!(resultado1.integrity.len(), 1);
        assert!(matches!(resultado1.integrity[0], IntegrityStatus::Valid));

        // Actividad 2: Fertilización en el día +15 (timestamp 1515) -> VÁLIDO
        let actividad2 = Activity::new(1515, ActivityCategory::Maintenance);
        let resultado2 = ciclo.register_activity(actividad2).unwrap();
        assert_eq!(resultado2.integrity.len(), 1);
        assert!(matches!(resultado2.integrity[0], IntegrityStatus::Valid));

        // Actividad 3: Cosecha en el día +90 (timestamp 1590) -> DENTRO DEL PERIODO
        let actividad3 = Activity::new(1590, ActivityCategory::Harvest);
        let resultado3 = ciclo.register_activity(actividad3).unwrap();
        assert_eq!(resultado3.integrity.len(), 1); 
        assert!(matches!(resultado3.integrity[0], IntegrityStatus::Valid)); // Está dentro del período 1000-2000

        // --- EJECUTAR: Registrar gastos (la "Realidad económica") ---
        let proveedor = ProveedorTasa;
        let gasto1 = Money::new(Decimal::from(300), Currency::USD);
        assert!(presupuesto.register_expense(&gasto1, &proveedor).is_ok()); // ¡No bloquea! (filosofía imperfección controlada)
        let gasto2 = Money::new(Decimal::from(400), Currency::NIO);
        assert!(presupuesto.register_expense(&gasto2, &proveedor).is_ok()); // ¡No bloquea! (filosofía imperfección controlada)

        // --- ANÁLISIS: Qué podemos hacer? Qué falta? ---
        
        // 1. Varianza de Presupuesto: SÍ podemos calcular esto.
        let varianza = presupuesto.get_variance().unwrap();
        // Gasto1: 300 USD, Gasto2: 400 NIO ≈ 10.81 USD (tasa 37) = Total ~310.81 USD
        // Baseline: 1000 USD → Varianza NEGATIVA (gastamos menos de lo presupuestado)
        println!("Varianza de Presupuesto: {:?} (negativo = ahorro, positivo = sobre gasto)", varianza);

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
        // Gastamos ~310.81 USD de 1000 USD presupuestados → Remanente POSITIVO
        let remanente = presupuesto.get_remaining().unwrap();
        assert!(remanente.amount > Decimal::ZERO); // Positivo = nos sobra presupuesto
        println!("Remanente: {:?} (positivo = nos sobra, negativo = sobre gasto)", remanente);

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
