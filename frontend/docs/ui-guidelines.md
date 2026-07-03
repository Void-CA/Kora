# Kora — Directrices de Interfaz

> La identidad del producto es más importante que la identidad de marca.
> Un agricultor puede olvidar el color del logo, pero recuerda si el sistema
> le permitía entender qué pasaba en su finca.

---

## La directriz rectora

> **Cada pantalla debe ayudar al usuario a tomar una decisión,
> no solo a consultar o registrar datos.**

Esa regla afecta al dashboard, los formularios, los mapas y los reportes.
Hace que la interfaz esté orientada al trabajo real del usuario,
en lugar de limitarse a representar la estructura de la base de datos.

---

## La identidad de Kora

Cuatro palabras:

> **Estado. Contexto. Tiempo. Espacio.**

No CRUD. No tablas en el trabajo diario. No formularios por defecto.

Todo debe responder una de estas preguntas:

- ¿Qué está ocurriendo?
- ¿Dónde ocurre?
- ¿Cuándo ocurre?
- ¿Qué debería hacer ahora?

Kora no es "otro ERP agrícola". Kora es un **centro de operaciones agrícolas**.

---

## El dashboard es una agenda inteligente

El dashboard de Kora no es un tablero de KPIs.

No mostramos:

```
Clientes: 20
Ventas: 10
Pedidos: 35
```

Mostramos:

```
Hoy
• 3 cultivos requieren riego
• 1 fertilización vence mañana
• 2 lotes presentan retraso
• Temperatura elevada en Campo Norte
```

El dashboard se construye alrededor de **preguntas**, no de widgets:

| Pregunta | Componente |
|---|---|
| ¿Qué necesita mi atención? | Lista de atención |
| ¿Qué debo hacer ahora? | Action Card |
| ¿Qué cambió desde ayer? | Timeline |
| ¿Hay algo crítico? | Alerta |

Esto permite que el dashboard evolucione sin romper la metáfora:
si surge una nueva pregunta útil, se agrega su componente;
si una pregunta deja de aplicar, se quita sin tocar las demás.

---

## Principios de diseño

### 1. El dashboard no es un resumen

Es una agenda inteligente. Responde "¿qué debo hacer hoy?".

### 2. Todo gira alrededor del tiempo

La agricultura es temporal. Usar timelines, calendarios, progreso y ciclos.

Una barra de progreso de fases del cultivo comunica más que una tabla:

```
Preparación   ████████████ done
Siembra       ████████ done
Crecimiento   ██ ahora
Floración     ░░░░░░░░
Cosecha       ░░░░░░░░
```

### 3. El mapa siempre cerca

No todo es GIS, pero el usuario debe poder responder "¿dónde está esto?"
sin abrir otra pantalla. Mapas pequeños, contextuales, embebidos.

### 4. Las entidades tienen identidad

Nunca `Campo #34`. Siempre `Campo Norte`, con hectáreas, cultivo,
estado de salud y última actividad. Cada entidad importante es una ficha.

### 5. El color representa estado

Nunca decoración. Cuatro colores, uno por estado:

- Verde — todo correcto
- Amarillo — requiere atención
- Rojo — acción inmediata
- Azul — información

No usar cinco tonos de verde "porque es agricultura".

### 6. Información jerárquica

Cada pantalla responde en orden:

1. ¿Qué pasa? — vista superficial
2. ¿Por qué pasa? — al expandir
3. ¿Qué puedo hacer? — acciones

Nunca al revés.

### 7. La navegación sigue el trabajo

No a las entidades. El flujo mental del agricultor:

```
Inicio
Operaciones
  Hoy
  Pendientes
  Alertas
Planificación
  Calendario
  Ciclos
  Cronograma
Campos
  Vista mapa
  Vista lista
  Parcelas
Inventario
Análisis
  Producción
  Costos
  Rendimiento
Reportes
Configuración
```

### 8. La IA nunca reemplaza al usuario

La IA sugiere. Nunca decide.

```
Probablemente convenga fertilizar esta semana.
```

Nunca:

```
Se programó una fertilización.
```

### 9. Todo tiene contexto

Una tarea muestra campo, lote, cultivo, fecha y prioridad
sin abrir tres ventanas.

### 10. Las tablas son el último recurso

Se usan para reportes, exportaciones y auditorías.
El trabajo diario se hace con tarjetas, paneles y vistas específicas del dominio.

---

## Personalidad visual

No "aplicación verde". Aplicación de ingeniería.

- Mucho espacio en blanco
- Iconografía sencilla
- Pocos colores
- Mucha información contextual
- Mapas limpios
- Gráficos discretos

Referentes: Linear, Notion, GitHub, Grafana.
No: software lleno de hojas, flores y degradados.

---

## Stack técnico

| Decisión | Elección | Justificación |
|---|---|---|
| Framework | Angular 19+ standalone | Menor noise-to-value ratio |
| Estilos | SCSS + design tokens | Mantenible, suficiente para personalidad propia |
| Estado | Signals (`signal()`, `computed()`) | No introducir state manager externo hasta que un problema claro lo justifique |
| Mapas | Leaflet directo, sin wrapper | No se necesita una abstracción sobre otra abstracción |
| Formularios | Reactive Forms sólo cuando se necesiten | La filosofía es anti-CRUD |
| RxJS | Sólo para streams reales (websockets, debounced search) | Signals para estado reactivo |
| HTTP | Función por endpoint, sin `BaseApiService` prematuro | Aparece al tener 3+ endpoints |

### Reglas explícitas

- **No NgModules.** Standalone components.
- **No NgRx** ni ComponentStore hasta que Signals no alcance para un problema claro.
- **No Material/PrimeNG/AG-Grid.** El diseño pide espacio en blanco y opinion propia.
- **No Atomic Design folders** (`atoms/`, `molecules/`) hasta que el segundo componente obligue a extraer.
- **No build de placeholders vacíos** (`inventario/`, `personal/`, `clima/`). Se reintroducen cuando una feature lo demande.
- **No librería de calendario** hasta necesitar drag-and-drop real. Calendario custom sobre SVG/canvas.

---

## Design tokens

Los colores son estado, no decoración.

```scss
// Estado
--state-ok:        #16a34a;   // verde: todo correcto
--state-attention: #ca8a04;   // amarillo: requiere atención
--state-critical:  #dc2626;   // rojo: acción inmediata
--state-info:      #2563eb;   // azul: información

// Superficies
--surface:         #ffffff;
--surface-muted:   #f5f5f4;

// Texto
--ink:             #1c1917;
--ink-muted:       #57534e;
```

Nada más. Cero tonos de verde "porque es agricultura".

---

## Cómo se validan los principios

| Principio | Validación en código |
|---|---|
| 1. Dashboard = agenda | `operation-dashboard` no tiene KPIs |
| 2. Tiempo-centric | `cycle-timeline` visible desde la Fase 2 |
| 3. Mapa cerca | `mini-map` embebido, no en ruta separada |
| 4. Identidad | Componentes llamados `field-card`, no `field-detail` |
| 5. Color = estado | Tokens `--state-*`, cero uso adorno |
| 6. Jerarquía | Progressive disclosure: chip → expand → action |
| 7. Nav = trabajo | Rutas en infinitivo de operaciones |
| 8. IA sugiere | Botones `[Sugerir]`, nunca `[Auto]` |
| 9. Contexto | `context-chips` reutilizable |
| 10. Tablas último recurso | Sólo en `/reportes` |

---

## Plan de ejecución

```
Fase 0   Cimientos Angular        → 1 día
Fase 1   Dashboard (mock)         → 2 días
         ⏸  checkpoint: ¿funciona la metáfora?
Fase 2   Ficha de entidad (mock)   → 2 días
Fase 3   API mínima (backend)      → 2 días
Fase 4   Navegación                → 1 día
Fase 5   Timeline/Calendario       → 3 días
Fase 6   Mapa                      → 2 días
Fase 7   Drift dashboard           → 3 días
Fase 8   Reportes                  → fuera del MVP
```

Después de Fase 1 hay un checkpoint real.
Si la metáfora no comunicaba, no se tiran 15 días de trabajo.

### Lo que no se hace

- No arrancar con 7 capas de state management.
- No construir scaffolding CRUD.
- No crear `shared/`, `core/`, `features/` prematuro.
  Aparecen cuando el código lo pide.

---

*Documento base para el desarrollo de la UI de Kora.*
*Redactado antes de escribir una sola línea de Angular.*