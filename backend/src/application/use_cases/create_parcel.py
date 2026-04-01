from domain.land.models import Parcel
from domain.land.repositories import ParcelRepository
from domain.shared.value_objects import Id, Area
from dataclasses import dataclass

@dataclass(frozen=True)  
class CreateParcelInput:
    id: int
    name: str
    area: float


def create_parcel(repo: ParcelRepository, data: CreateParcelInput) -> Parcel:
    parcel = Parcel(
        id=Id(data.id),
        name=data.name,
        area=Area(data.area)
    )

    repo.save(parcel)

    return parcel


