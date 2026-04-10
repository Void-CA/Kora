from abc import ABC, abstractmethod
from typing import List

from .entity import Land, SpatialFeature


class LandRepository(ABC):
    """Port (interface) for Land persistence operations."""

    @abstractmethod
    def get_all(self) -> List[Land]:
        """Return every Land record in the database."""
        ...


class SpatialFeatureRepository(ABC):
    """Port (interface) for SpatialFeature persistence operations."""

    @abstractmethod
    def get_all(self) -> List[SpatialFeature]:
        """Return every SpatialFeature record in the database."""
        ...
