from fastapi import APIRouter, Depends
from application.parcel.create import CreateParcel
from interfaces.api.schemas.parcel.create import ParcelCreate
from infrastructure.db.repositories.parcel_repo import SQLParcelRepository

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