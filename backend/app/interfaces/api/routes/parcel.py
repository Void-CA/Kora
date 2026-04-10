from fastapi import APIRouter, Depends
from app.interfaces.api.schemas.parcel.create import ParcelCreate
from app.application.parcel.create import CreateParcel
from app.infrastructure.db.session import get_db
from app.infrastructure.db.repositories.parcel_repo import SQLParcelRepository

router = APIRouter()

def get_repo():
    conn = get_db()
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