from dataclasses import dataclass

@dataclass
class Parcel:
    id: int | None
    name: str
    geometry: str  # WKT por ahora

    def validate(self):
        if not self.name:
            raise ValueError("Parcel must have a name")

        if not self.geometry:
            raise ValueError("Parcel must have geometry")