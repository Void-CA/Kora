PYTHON = python
DJANGO_SETTINGS = infrastructure.django.config.settings


SRC = src


run:
	DJANGO_SETTINGS_MODULE=$(DJANGO_SETTINGS) $(PYTHON) manage.py runserver

# 🧪 Migrations

makemigrations:
	DJANGO_SETTINGS_MODULE=$(DJANGO_SETTINGS) $(PYTHON) manage.py makemigrations

migrate:
	DJANGO_SETTINGS_MODULE=$(DJANGO_SETTINGS) $(PYTHON) manage.py migrate

# 🔍 Type checking

typecheck:
	DJANGO_SETTINGS_MODULE=$(DJANGO_SETTINGS) $(PYTHON) -m mypy $(SRC)

# 🧹 Limpieza

clean:
	rm -rf .mypy_cache **pycache** */**pycache**

# 🔥 Full check (lo que vas a usar siempre)

check: typecheck

# 📡 Test endpoint rápido (opcional)

test-parcel:
	curl -X POST http://127.0.0.1:8000/parcels/ 
	-H "Content-Type: application/json" 
	-d '{"id":1,"name":"Parcela Norte","area":12.5}'
