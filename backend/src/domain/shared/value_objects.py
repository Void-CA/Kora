from dataclasses import dataclass

@dataclass(frozen=True)
class Id:
    value: int

@dataclass(frozen=True)
class Area:
    hectares: float

    def __post_init__(self) -> None:
        if self.hectares <= 0:
            raise ValueError("Area must be positive")