1. Definición Operativa de Kora
Kora es un sistema de planificación y control operativo agrícola diseñado para medir y analizar la ejecución de ciclos de cultivo. Su valor reside en la capacidad de contrastar las proyecciones (Presupuesto/Cronograma) con la realidad del campo (Actividades/Gastos) sobre un modelo de datos inmutable y georeferenciado.

2. Lenguaje Ubicuo (Alineado con el Stakeholder)
Mantenemos la semántica técnica internamente, pero el lenguaje del dominio se "humaniza":
CropCycle (Ciclo): El periodo de vida de un cultivo en un lote.
Deviation / Gap (Desviación): El "atraso" o "sobrecosto". Internamente lo llamamos Drift, pero el usuario ve "Desviación de presupuesto" o "Retraso en calendario".
Baseline (Plan Base): El cronograma original.
Actuals (Realidad): Lo que efectivamente se registró (gastos, tareas).
Anchor (Ancla): El hito que dispara todo (ej: "Día de Siembra").

3. Bounded Contexts y sus "Puentes"
A. Contexto: agriculture (Core)
Foco: Salud biológica y logística de tareas.
Fuente de Verdad: El CropCycle es el dueño de la cronología de eventos.
Relación con la Realidad: Permite registros "Fuera de Rango" pero los marca con un Flag de Inconsistencia.
B. Contexto: finance (Supporting)
Foco: Flujo de caja y rentabilidad.
Fuente de Verdad: Los comprobantes de gasto y las planillas de pago.
Implicación: No bloquea un gasto si el presupuesto se acabó (la vida real no se detiene), pero genera una alerta de "Excedente".
C. shared_kernel (El Estabilizador)
Contiene solo: Money, Quantity, Period, AreaId, CycleId.
Regla: Si una estructura requiere lógica de negocio para cambiar, sale del kernel.
4. Aggregates e Invariantes "Resilientes"
CropCycle (Aggregate Root)

Invariante Duro (Geográfico): No puede haber solapamiento de dos ciclos productivos en el mismo espacio-tiempo. (Esto es innegociable para la trazabilidad).
Invariante Blando (Temporal): Si una actividad se registra fuera del Period del ciclo, el sistema no la rechaza, la acepta pero la marca como Outlier para que el dueño pueda corregir la fecha o el periodo del ciclo.
Schedule (Entity)
Versiones: Cada cambio en el plan genera un Snapshot.
Invariante: Una actividad planificada debe pertenecer a una categoría válida (Siembra, Riego, etc.).
Budget (Aggregate Root)
Flexibilidad: El presupuesto puede ser ajustado. El sistema guarda la "Foto Inicial" (Baseline) y la "Foto Actual".

5. La Fuente de Verdad (Single Source of Truth)
En Kora, la fuente de verdad es el Evento Registrado.
Si el trabajador anotó que aplicó urea el 10 de mayo, esa es la verdad operativa, aunque el plan dijera el 5 de mayo.
El Plan no es la verdad; es el Marco de Referencia.

6. Implicaciones en Rust (Implementación Práctica)
Para permitir esta "imperfección controlada", nuestro código en Rust cambiará de esta forma:
De Validaciones de Bloqueo a Validaciones de Estado
En lugar de que register_activity devuelva siempre Err, usaremos un patrón de Validación de Integridad:
pub enum IntegrityStatus {
    Valid,
    OutsidePeriod, // La actividad ocurrió fuera de las fechas del ciclo
    Unplanned,      // No estaba en el cronograma
}

pub struct ActivityRecord {
    pub activity: Activity,
    pub integrity: Vec<IntegrityStatus>, // Captura las inconsistencias
}



Uso de Traits para Desacoplo
// En agriculture/domain/mod.rs
pub trait CostProvider {
    fn get_total_cost_for_cycle(&self, cycle_id: &CycleId) -> Money;
}


Esto permite que el contexto de agriculture pregunte por costos sin saber que existe una base de datos de facturas en el contexto de finance.

7. Conclusión: Kora como Espejo de lo Real
Kora acepta que la agricultura es caótica. Su misión no es obligar al usuario a ser perfecto, sino hacer visible la imperfección para que pueda ser gestionada.
Mantra de Kora: "Registra lo que pasó, compáralo con lo que querías, y aprende de la diferencia."

