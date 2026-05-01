Sistema Kora — Definición del Sistema (v2.0)
“Según Platón, el mundo físico ocurre en el Kora, donde las Ideas abstractas logran materializarse en un espacio de existencia.”
1. Visión General
Kora es un sistema de planificación, trazabilidad y control agrícola diseñado para materializar la estrategia agronómica en resultados productivos medibles. A diferencia de los registros tradicionales, Kora utiliza un modelo híbrido que contrasta la intención (planificación) con la realidad (ejecución), permitiendo optimizar el rendimiento y la rentabilidad mediante el análisis de desviaciones.
2. El Problema: El "Punto Ciego" Agrícola
La agricultura padece de una desconexión entre lo que se planea y lo que se ejecuta. Actualmente, los productores enfrentan:
Falta de Control de Varianza: Incapacidad de saber exactamente por qué un ciclo se salió del presupuesto o del calendario.
Información de Suelo Subutilizada: Los análisis de laboratorio se archivan en lugar de guiar la nutrición del cultivo.
Invisibilidad de Costos Previos: Los gastos de preparación antes de la siembra suelen diluirse, afectando el cálculo real del ROI.
3. Enfoque del Sistema: El Espejo Espacio-Temporal
Kora se rige bajo la filosofía de que el registro histórico es más importante que el estado actual. El sistema funciona como un "espejo" donde cada Idea (Plan) se refleja en una Acción (Registro).
Dimensión Espacial: Áreas georeferenciadas (PostGIS) con jerarquía lógica.
Dimensión Temporal: Periodos que abarcan desde la preparación del suelo hasta la comercialización.
4. Componentes Evolucionados
4.1 Planificación Estratégica (Nuevo)
Cronograma de Actividades: Definición temporal de tareas basada en un "Día Ancla" (ej. fecha de siembra). Permite proyectar el uso de mano de obra y tiempo.
Presupuesto (Budget): El contenedor financiero del ciclo. Es temporalmente más amplio que el ciclo biológico, capturando costos pre-operativos y administrativos.
4.2 Ciclos de Cultivo (El Núcleo)
La unidad central que vincula:
El Plan: Cronograma y Presupuesto.
La Ejecución: Actividades reales, insumos aplicados y clima.
El Resultado: Cosecha y rentabilidad final.
4.3 Análisis de Suelo y Fertilidad (Multidimensional)
Kora trata los análisis químicos no como documentos, sino como Métricas de Salud.
Flexibilidad de Calidad: Soporta desde análisis básicos de laboratorio hasta estimaciones satelitales.
Tipado Semántico: Clasificación estricta de variables (pH, N, P, K) para permitir comparaciones históricas entre años y lotes.
4.4 Operaciones y Finanzas
Actividades Ejecutadas: Registro detallado de mano de obra, maquinaria e insumos.
Control de Incidencias: Registro de plagas/enfermedades con su impacto económico asociado.
Costos Laborales: Seguimiento de jornales y roles por área y ciclo.
5. Flujo de Control: El Análisis de Varianza (Drift)
El valor diferencial de Kora ocurre al contrastar los datos:
Planificado vs. Ejecutado: ¿Se aplicó el fertilizante en el día previsto en el cronograma?
Presupuestado vs. Gastado: ¿Por qué el costo de fumigación superó la estimación inicial?
Análisis de Suelo vs. Rendimiento: ¿La inversión en mejorar el fósforo se tradujo en más kilos?
6. Capacidades Analíticas
Kora permite responder preguntas críticas para el negocio:
Eficiencia: "¿Qué porcentaje del cronograma logramos cumplir a tiempo este semestre?"
Rentabilidad Real: "¿Cuál es el costo real por kilo, incluyendo la preparación previa al ciclo?"
Optimización: "¿Qué estrategia de riego presenta menor desviación presupuestaria en suelos con bajo pH?"
7. Principios de Diseño (Invariantes)
Tipado Fuerte: Las métricas y unidades son estrictas para evitar ruido en los datos.
Inmutabilidad Histórica: Los planes no se borran; se versionan para entender cómo cambió la estrategia.
Anclaje Temporal: Toda actividad planificada es relativa a un hito (ej. Siembra + 15 días).
Validación Espacial: No pueden existir dos ciclos productivos solapados en el mismo espacio-tiempo.
8. Arquitectura Técnica
Core: Monolito Modular en Rust para garantizar seguridad de tipos y concurrencia.
Persistencia: PostgreSQL + PostGIS para la precisión geométrica.
Naturaleza Híbrida: * OLTP: Gestión diaria de la finca.
OLAP: Motor de análisis histórico para la toma de decisiones.
9. Proyección
Kora evoluciona de un registro manual hacia una Plataforma de Inteligencia Agrícola, con el potencial de integrar imágenes satelitales y modelos predictivos basados en los años de experiencia y datos acumulados de los stakeholders.

¿Qué cambió respecto a la V1?
El Presupuesto y Cronograma ahora son secciones principales, no solo campos en el ciclo.
El Análisis de Suelo pasó de ser "un dato más" a una herramienta de comparación técnica.
El Enfoque ya no es solo "centralizar", es "optimizar mediante el análisis de desviaciones".
La Terminología es más profesional y alineada a lo que un dueño de finca (como tu papá) espera ver en un reporte de gerencia.

