# Backend Architecture – Coras

## 🧭 Propósito

Este backend está diseñado siguiendo una arquitectura orientada a dominio (Domain-Driven Design) con una separación clara de responsabilidades entre capas.

El objetivo principal es:

* Mantener la lógica de negocio independiente de frameworks
* Garantizar consistencia mediante tipado fuerte
* Facilitar escalabilidad y mantenibilidad a largo plazo
* Reducir errores moviéndolos lo más posible a tiempo de desarrollo (type checking)

---

## 🧱 Estructura del proyecto

```text
/backend
  /src

    /domain          # Núcleo del sistema (reglas de negocio)
    /application     # Casos de uso (orquestación)
    /infrastructure  # Implementaciones concretas (Django, DB)
    /interfaces      # Entrada/salida (HTTP, DRF)

  manage.py
```

---

## 🧠 Filosofía general

El sistema está organizado bajo esta regla:

> El dominio es el centro. Todo lo demás son adaptadores.

---

## 🔷 Capas del sistema

### 1. Domain (`/domain`)

Contiene la lógica de negocio pura.

**Incluye:**

* Entidades (ej: `Parcel`)
* Value Objects (ej: `Id`, `Area`)
* Interfaces (ej: `ParcelRepository`)

**No incluye:**

* Django
* DRF
* Base de datos
* Requests/Responses

👉 Esta capa debe poder ejecutarse sin ningún framework.

---

### 2. Application (`/application`)

Define los **casos de uso** del sistema.

**Responsabilidades:**

* Orquestar operaciones del dominio
* Convertir inputs en objetos de dominio
* Coordinar repositorios

**Ejemplo:**

* `create_parcel`

**Estructura típica:**

```text
/application
  /use_cases
    create_parcel.py
```

---

### 3. Infrastructure (`/infrastructure`)

Implementa detalles técnicos.

**Incluye:**

* Modelos de Django
* Repositorios concretos
* Configuración del framework

Ejemplo:

```text
/infrastructure/django
  /apps
  /persistence
```

👉 Esta capa depende del dominio, pero el dominio NO depende de ella.

---

### 4. Interfaces (`/interfaces`)

Capa de entrada/salida.

**Incluye:**

* Vistas de DRF
* Serialización
* Mapping request → application

Ejemplo:

```text
/interfaces/drf
```

👉 No contiene lógica de negocio.

---

## 🔁 Flujo de datos

```text
HTTP Request (DRF)
   ↓
Input (Application)
   ↓
Use Case
   ↓
Domain
   ↓
Repository
   ↓
Database
```

---

## 🧩 Principios clave

### ✔ Dominio independiente

El dominio no debe importar nada de Django ni de frameworks.

---

### ✔ Tipado fuerte

Se prioriza el uso de:

* dataclasses
* value objects
* contratos explícitos

---

### ✔ Separación de responsabilidades

Cada capa tiene un rol específico y limitado.

---

### ✔ DRF como capa de transporte

Django REST Framework se utiliza únicamente para:

* manejar requests
* validar input superficialmente
* invocar casos de uso

---

## 🚫 Anti-patrones (evitar)

* Lógica de negocio en views o serializers
* Uso de `dict` sin tipar en capas internas
* Acoplar dominio a Django
* Reutilizar objetos de transporte como dominio

---

## 🧪 Testing (visión)

* Domain → testeable sin Django
* Application → testeable con mocks de repositorios
* Infrastructure → testeable con integración

---

## ⚙️ Tipado

Se utiliza:

* `mypy`
* django-stubs
* `djangorestframework-stubs`

Para asegurar consistencia y detectar errores temprano.

---

## 📌 Nota final

Este proyecto no está diseñado como un CRUD tradicional.

Es un sistema donde:

* las reglas de negocio son explícitas
* los estados inválidos se previenen
* la complejidad se controla mediante diseño

---

## 🚀 Evolución esperada

A medida que el sistema crezca:

* Los casos de uso se volverán más complejos
* Se introducirán múltiples dominios (production, finance, etc.)
* Se integrará analítica como capa separada

---

## 🧠 Mentalidad requerida

Trabajar en este proyecto implica:

* Pensar en términos de dominio, no de endpoints
* Modelar estados válidos explícitamente
* Priorizar claridad sobre rapidez inicial

---
