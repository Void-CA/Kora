from domain.land.models import Parcel
from domain.land.repositories import ParcelRepository
from domain.shared.value_objects import Id, Area
from infrastructure.django.apps.parcels.models import ParcelModel

class DjangoParcelRepository(ParcelRepository):

    def get(self, parcel_id: Id) -> Parcel:
        obj: ParcelModel = ParcelModel.objects.get(id=parcel_id.value)

        return Parcel(
            id=Id(obj.id),
            name=obj.name,
            area=Area(obj.area)
        )

    def save(self, parcel: Parcel) -> None:
        ParcelModel.objects.update_or_create(
            id=parcel.id.value,
            defaults={
                "name": parcel.name,
                "area": parcel.area.hectares
            }
        )