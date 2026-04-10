import os

class Settings:
    DATABASE_URL = os.getenv("DATABASE_URL")
    APP_PORT = int(os.getenv("APP_PORT", 8000))

settings = Settings()