Casos de Uso — Kora (Versión Inicial)

1. Registrar Ciclo de Cultivo
Actor: Usuario (administrador / encargado)
Descripción:
 Permite registrar un nuevo ciclo productivo en un área específica.
Flujo principal:
Selecciona un área
Selecciona el cultivo
Ingresa fecha de inicio
(Opcional) ingresa fecha estimada de fin
Selecciona o redacta estrategias aplicadas:
riego por goteo
fertiriego
cobertura, etc.
Ingresa parámetros del ciclo:
distancia entre plantas
distancia entre filas
densidad
tipo de suelo (si no viene de análisis)
Guarda el ciclo
Resultado:
Ciclo registrado y disponible para:
gastos
actividades
análisis


2. Registrar Análisis de Suelo
Actor: Usuario
Descripción:
 Permite registrar un análisis técnico del suelo asociado a un área.
Flujo principal:
Selecciona área
Ingresa fecha del análisis
Ingresa mediciones:
pH
nitrógeno
fósforo
potasio
otros
(Opcional) agrega observaciones
Guarda análisis
Resultado:
Análisis disponible para:
consulta histórica
asociación a ciclos


3. Asociar Análisis de Suelo a un Ciclo
Actor: Usuario
Descripción:
 Permite vincular un análisis de suelo a un ciclo de cultivo.
Flujo principal:
Selecciona ciclo
Selecciona análisis de suelo existente
Indica tipo:
previo
seguimiento
posterior
Guarda relación
Resultado:
El ciclo queda contextualizado con condiciones del suelo

4. Registrar Gasto
Actor: Usuario
Descripción:
 Permite registrar un gasto asociado a un área o ciclo.
Flujo principal:
Selecciona:
ciclo (opcional)
área (obligatorio si no hay ciclo)
Ingresa:
monto
fecha
tipo (semillas, insumos, etc.)
(Opcional) descripción
Guarda gasto
Resultado:
Gasto disponible para análisis financiero

5. Registrar Gasto de Planilla
Actor: Usuario
Descripción:
 Permite registrar costos de personal asociados a un ciclo o área.
Flujo principal:
Selecciona ciclo o área
Selecciona trabajador (o lo crea)
Ingresa:
monto pagado
fecha
rol (opcional)
Guarda registro
Resultado:
Costos laborales integrados al análisis

6. Registrar Trabajador
Actor: Usuario
Descripción:
 Permite registrar una persona que trabaja en la finca.
Flujo principal:
Ingresa:
nombre
información básica
Guarda trabajador
Resultado:
Disponible para asignaciones y planilla

7. Registrar Actividad en un Ciclo
Actor: Usuario
Descripción:
 Permite registrar acciones realizadas durante un ciclo.
Flujo principal:
Selecciona ciclo
Selecciona tipo de actividad:
fertilización
fumigación
monitoreo
Ingresa:
fecha
(opcional) descripción
Guarda actividad
Resultado:
Historial operativo del ciclo

8. Registrar Incidencia Sanitaria
Actor: Usuario
Descripción:
 Permite registrar problemas fitosanitarios.
Flujo principal:
Selecciona ciclo
Ingresa:
tipo (plaga, enfermedad)
descripción
fecha
acción tomada
Guarda incidencia
Resultado:
Historial de riesgos y control sanitario

9. Consultar Rentabilidad por Ciclo
Actor: Usuario
Descripción:
 Permite analizar la rentabilidad de un ciclo específico.
Flujo principal:
Selecciona ciclo
El sistema calcula:
total de gastos
(futuro: ingresos)
Muestra resumen
Resultado:
Visión financiera del ciclo


10. Consultar Historial de un Área
Actor: Usuario
Descripción:
 Permite visualizar todo lo ocurrido en un área.
Flujo principal:
Selecciona área
El sistema muestra:
ciclos realizados
análisis de suelo
gastos
actividades
Resultado:
Trazabilidad completa del área

