from fastapi import APIRouter, Depends
from interfaces.api.schemas.parcel.create import ParcelCreate
from application.parcel.create import CreateParcel
from infrastructure.db.session import get_connection
from infrastructure.db.repositories.parcel_repo import SQLParcelRepository

router = APIRouter()

def get_repo():
    conn = get_connection()
    return SQLParcelRepository(conn)

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