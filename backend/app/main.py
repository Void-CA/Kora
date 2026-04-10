from interfaces.api.routes import parcel
from fastapi import FastAPI
from shared.config import settings

app = FastAPI()

@app.get("/health")
def health():
    return {
        "status": "ok",
        "db": settings.DATABASE_URL is not None
    }

app.include_router(parcel.router)