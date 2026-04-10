from typing import List

from app.domain.land.entity import Land
from app.domain.land.repository import LandRepository


class GetAllLands:
    """Use case: retrieve every Land record."""

    def __init__(self, repo: LandRepository) -> None:
        self.repo = repo

    def execute(self) -> List[Land]:
        return self.repo.get_all()
