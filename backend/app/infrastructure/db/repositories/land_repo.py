from typing import List

from geoalchemy2.shape import to_shape
from shapely.geometry import mapping
from sqlalchemy.orm import Session

from app.domain.land.entity import Land, SpatialFeature
from app.domain.land.repository import LandRepository, SpatialFeatureRepository
from app.infrastructure.db.models import LandModel, SpatialFeatureModel


def _wkbelement_to_geojson(wkb) -> dict | None:
    """Convert a GeoAlchemy2 WKBElement to a GeoJSON-compatible dict."""
    if wkb is None:
        return None
    try:
        shape = to_shape(wkb)
        return mapping(shape)
    except Exception:
        return None


class SQLLandRepository(LandRepository):
    """Concrete implementation of LandRepository backed by PostgreSQL/PostGIS."""

    def __init__(self, db: Session) -> None:
        self.db = db

    def get_all(self) -> List[Land]:
        rows = self.db.query(LandModel).all()
        return [
            Land(
                id=row.id,
                name=row.name,
                total_area_m2=row.total_area_m2,
                geom=_wkbelement_to_geojson(row.geom),
            )
            for row in rows
        ]


class SQLSpatialFeatureRepository(SpatialFeatureRepository):
    """Concrete implementation of SpatialFeatureRepository backed by PostgreSQL/PostGIS."""

    def __init__(self, db: Session) -> None:
        self.db = db

    def get_all(self) -> List[SpatialFeature]:
        rows = self.db.query(SpatialFeatureModel).all()
        return [
            SpatialFeature(
                id=row.id,
                finca_id=row.finca_id,
                geom=_wkbelement_to_geojson(row.geom),
                type=row.type,
                name=row.name,
                properties=row.properties or {},
            )
            for row in rows
        ]
