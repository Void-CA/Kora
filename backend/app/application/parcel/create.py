from domain.parcel.entity import Parcel
from domain.parcel.repository import ParcelRepository

class CreateParcel:

    def __init__(self, repo: ParcelRepository):
        self.repo = repo

    def execute(self, name: str, geometry: str) -> Parcel:
        parcel = Parcel(id=None, name=name, geometry=geometry)

        parcel.validate()

        return self.repo.save(parcel)