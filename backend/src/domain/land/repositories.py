from abc import ABC, abstractmethod
from .models import Parcel
from domain.shared.value_objects import Id

class ParcelRepository(ABC):

    @abstractmethod
    def get(self, parcel_id: Id) -> Parcel:
        pass

    @abstractmethod
    def save(self, parcel: Parcel) -> None:
        pass