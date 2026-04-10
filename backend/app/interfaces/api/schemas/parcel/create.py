from pydantic import BaseModel

class ParcelCreate(BaseModel):
    name: str
    geometry: str