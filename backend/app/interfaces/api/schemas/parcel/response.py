from __future__ import annotations

from typing import Any, Dict, List, Optional

from pydantic import BaseModel, ConfigDict


# ---------------------------------------------------------------------------
# Shared GeoJSON geometry fragment
# ---------------------------------------------------------------------------

class GeoJSONGeometry(BaseModel):
    """Minimal GeoJSON geometry object as returned by Shapely's `mapping()`."""
    type: str
    coordinates: Any  # nested lists; Any avoids recursive type complexity

    model_config = ConfigDict(arbitrary_types_allowed=True)


# ---------------------------------------------------------------------------
# Land response schema
# ---------------------------------------------------------------------------

class LandResponse(BaseModel):
    """Response schema for a single Land record."""
    id: int
    name: str
    total_area_m2: Optional[float] = None
    geom: Optional[GeoJSONGeometry] = None

    model_config = ConfigDict(from_attributes=True)


class LandListResponse(BaseModel):
    """Response schema for GET /lands."""
    count: int
    results: List[LandResponse]


# ---------------------------------------------------------------------------
# SpatialFeature response schema
# ---------------------------------------------------------------------------

class SpatialFeatureResponse(BaseModel):
    """Response schema for a single SpatialFeature record."""
    id: int
    finca_id: Optional[int] = None
    geom: Optional[GeoJSONGeometry] = None
    type: Optional[str] = None
    name: Optional[str] = None
    properties: Optional[Dict[str, Any]] = None

    model_config = ConfigDict(from_attributes=True)


class SpatialFeatureListResponse(BaseModel):
    """Response schema for GET /spatial-features."""
    count: int
    results: List[SpatialFeatureResponse]
