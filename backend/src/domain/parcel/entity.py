from dataclasses import dataclass

@dataclass
class Parcel:
    id: int | None
    name: str
    geometry: str 

    def validate(self):
        if not self.name:
            raise ValueError("Parcel must have a name")