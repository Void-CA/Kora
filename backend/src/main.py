from fastapi import FastAPI
from interfaces.api.routes import parcel

app = FastAPI()

app.include_router(parcel.router)