from dataclasses import dataclass
from domain.shared.value_objects import Id, Area

@dataclass(frozen=True)
class Parcel:
    id: Id
    name: str
    area: Area