from typing import List

from fastapi import APIRouter, Depends
from sqlalchemy.orm import Session

from app.application.land.get import GetAllLands
from app.application.land.get_spatial_features import GetAllSpatialFeatures
from app.domain.land.entity import Land, SpatialFeature
from app.infrastructure.db.repositories.land_repo import (
    SQLLandRepository,
    SQLSpatialFeatureRepository,
)
from app.infrastructure.db.session import get_db
from app.interfaces.api.schemas.parcel.response import (
    LandListResponse,
    LandResponse,
    SpatialFeatureListResponse,
    SpatialFeatureResponse,
)

router = APIRouter(tags=["lands"])


# ---------------------------------------------------------------------------
# Dependency helpers
# ---------------------------------------------------------------------------

def get_land_repo(db: Session = Depends(get_db)) -> SQLLandRepository:
    return SQLLandRepository(db)


def get_spatial_feature_repo(db: Session = Depends(get_db)) -> SQLSpatialFeatureRepository:
    return SQLSpatialFeatureRepository(db)


# ---------------------------------------------------------------------------
# Endpoints
# ---------------------------------------------------------------------------

@router.get(
    "/lands",
    response_model=LandListResponse,
    summary="List all land parcels",
    description=(
        "Returns every record from the **land** table, including the geometry "
        "serialised as a GeoJSON MultiPolygon (SRID 32616 / UTM zone 16N)."
    ),
)
def list_lands(repo: SQLLandRepository = Depends(get_land_repo)):
    use_case = GetAllLands(repo)
    lands: List[Land] = use_case.execute()
    return LandListResponse(
        count=len(lands),
        results=[LandResponse(**land.__dict__) for land in lands],
    )


@router.get(
    "/spatial-features",
    response_model=SpatialFeatureListResponse,
    summary="List all spatial features",
    description=(
        "Returns every record from the **spatial_features** table. "
        "Each feature geometry is serialised as a GeoJSON MultiPolygon."
    ),
)
def list_spatial_features(repo: SQLSpatialFeatureRepository = Depends(get_spatial_feature_repo)):
    use_case = GetAllSpatialFeatures(repo)
    features: List[SpatialFeature] = use_case.execute()
    return SpatialFeatureListResponse(
        count=len(features),
        results=[SpatialFeatureResponse(**f.__dict__) for f in features],
    )
