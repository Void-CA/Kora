Casos de Uso — Kora (Versión 2.0)
1. Planificación (Nuevos)
1.1 Crear Presupuesto del Ciclo (Nuevo)
Actor: Administrador / Dueño.
Descripción: Permite definir el techo financiero de un ciclo, incluso antes de que este comience (gastos pre-operativos).
Flujo:
Define el Periodo Financiero (puede empezar meses antes de la siembra).
Asigna rubros estimados (Semillas, Fertilizantes, Mano de Obra, Preparación de Suelo).
Establece el monto total proyectado.
Resultado: Un Budget que servirá de base para medir desviaciones de costos.
1.2 Diseñar Cronograma de Actividades (Nuevo)
Actor: Encargado Agrónomo.
Descripción: Crea la "hoja de ruta" de tareas para un cultivo específico.
Flujo:
Selecciona un Ancla Temporal (ej. Fecha de Siembra).
Añade actividades proyectadas indicando el día relativo (ej. Día +15: Fertilización).
Asigna recursos estimados (insumos y jornales) a cada actividad del cronograma.
Resultado: Un Schedule (v1) que define el "deber ser" del ciclo.

2. Operaciones y Ejecución (Actualizados)
2.1 Registrar Ciclo de Cultivo (Ajustado)
Descripción: Activa el ciclo biológico vinculándolo a un Plan.
Cambio Clave: Ahora el ciclo debe estar vinculado a un Schedule y un Budget.
Flujo: Igual al anterior, pero el sistema valida que el área esté libre en el Period seleccionado (Domain Service: CropPlanningService).
2.2 Registrar Actividad (Control de Ejecución)
Descripción: Registra una tarea real, vinculándola opcionalmente a la planificación.
Flujo:
Selecciona el ciclo.
Nuevo: El sistema sugiere actividades del Schedule que deberían ocurrir en esa fecha.
El usuario confirma si es una actividad planificada o una "emergente" (fuera de plan).
Ingresa costos reales y recursos usados.
Resultado: Permite medir la Varianza de Tiempo (¿Se hizo cuando se planeó?).
2.3 Registrar Gasto (Control Financiero)
Cambio Clave: Todo gasto registrado se resta del Budget correspondiente para ver el saldo disponible en tiempo real.

3. Análisis Técnico (Refinado)
3.1 Registrar Análisis de Suelo (Ajustado)
Descripción: Registro estructurado de la salud del suelo.
Cambio Clave: Las mediciones no son texto libre.
Flujo:
Selecciona área y fecha.
Selecciona el Nivel de Calidad (Básico, Completo, Satelital).
Ingresa valores para métricas predefinidas (MetricKind: pH, N, P, K, etc.).
Asigna el costo del análisis (se vincula al presupuesto de la finca).

4. Gestión de Incidencias y Personal (Sin Cambios Críticos)
4.1 Registrar Incidencia Sanitaria (Fitosanitario)
Permite rastrear plagas y enfermedades. Se añade el impacto económico (costo extra no presupuestado).
4.2 Gestión de Trabajadores y Planilla
Control de jornales y roles. Estos costos se cruzan con el Schedule para ver si la mano de obra estimada coincide con la real.

5. Inteligencia de Negocio (Nuevos y Ajustados)
5.1 Consultar Análisis de Varianza (Nuevo)
Actor: Dueño / Stakeholder.
Descripción: La joya de la corona. Compara el Plan vs. la Realidad.
El sistema muestra:
Varianza de Costo: (Presupuestado - Real).
Varianza de Cronograma: (Fecha Planificada - Fecha Real).
Desempeño: Identifica qué actividades "quemaron" más presupuesto del previsto.
5.2 Consultar Historial Espacio-Temporal del Área (Ajustado)
Muestra la evolución química del suelo (vía análisis históricos) en paralelo con el rendimiento de los ciclos realizados.
Pregunta que responde: "¿Está el suelo mejorando o degradándose tras estos últimos 3 ciclos de maíz?"

Resumen de la Evolución de Kora
Versión
Enfoque
Valor Principal
v1.0
Registro (Log)
Trazabilidad de "qué pasó".
v2.0 (Actual)
Gestión (Control)
Comparación de "qué planeamos" vs "qué pasó".


