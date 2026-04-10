from fastapi import FastAPI

from app.interfaces.api.routes import parcel, land
from app.shared.config import settings

app = FastAPI(
    title="Kora API",
    description="Spatial land management API powered by FastAPI + PostGIS.",
    version="0.1.0",
)


@app.get("/health", tags=["health"])
def health():
    return {
        "status": "ok",
        "db": settings.DATABASE_URL is not None,
    }


# Existing parcel routes (kept for backward-compatibility)
app.include_router(parcel.router, prefix="/api") 

# New land / spatial-feature routes
app.include_router(land.router, prefix="/api")