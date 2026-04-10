from typing import List

from app.domain.land.entity import SpatialFeature
from app.domain.land.repository import SpatialFeatureRepository


class GetAllSpatialFeatures:
    """Use case: retrieve every SpatialFeature record."""

    def __init__(self, repo: SpatialFeatureRepository) -> None:
        self.repo = repo

    def execute(self) -> List[SpatialFeature]:
        return self.repo.get_all()
