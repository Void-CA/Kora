from fastapi import APIRouter, Depends
from app.application.use_cases.create_parcel import CreateParcel
from app.interfaces.api.schemas.parcel import ParcelCreate
from app.infrastructure.db.repositories.parcel_repo import SQLParcelRepository

router = APIRouter()

def get_repo():
    # aquí inyectas tu session real
    return SQLParcelRepository(session=...)

@router.post("/parcels")
def create_parcel(dto: ParcelCreate, repo=Depends(get_repo)):
    use_case = CreateParcel(repo)

    parcel = use_case.execute(
        name=dto.name,
        geometry=dto.geometry
    )

    return {
        "id": parcel.id,
        "name": parcel.name
    }