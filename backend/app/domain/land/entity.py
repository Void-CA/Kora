from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Land:
    """Represents a land parcel with its geometry and metadata."""
    id: Optional[int]
    name: str
    total_area_m2: Optional[float]
    geom: Optional[str]  # WKT / GeoJSON string representation


@dataclass
class SpatialFeature:
    """Represents a spatial element (terrace, building, crop, etc.) within a Land."""
    id: Optional[int]
    finca_id: Optional[int]
    geom: Optional[str]  # WKT / GeoJSON string representation
    type: Optional[str]  # 'terrace' | 'building' | 'crop' | 'other'
    name: Optional[str]
    properties: Optional[dict] = field(default_factory=dict)


from enum import Enum

class FeatureType(str, Enum):
    TERRACE = "terrace"
    BUILDING = "building"
    CROP = "crop"
    OTHER = "other"