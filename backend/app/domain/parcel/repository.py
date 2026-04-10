from abc import ABC, abstractmethod
from .entity import Parcel

class ParcelRepository(ABC):

    @abstractmethod
    def save(self, parcel: Parcel) -> Parcel:
        pass

    @abstractmethod
    def get(self, parcel_id: int) -> Parcel | None:
        pass