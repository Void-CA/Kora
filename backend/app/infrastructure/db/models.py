from sqlalchemy import Column, BigInteger, String, Float, Integer, ForeignKey, JSON
from sqlalchemy.orm import DeclarativeBase
from geoalchemy2 import Geometry


class Base(DeclarativeBase):
    pass


class LandModel(Base):
    """SQLAlchemy ORM mapping for the `land` table."""
    __tablename__ = "land"

    id             = Column(BigInteger, primary_key=True, autoincrement=True)
    name           = Column(String(120), nullable=False)
    total_area_m2  = Column(Float, nullable=True)
    geom           = Column(Geometry(geometry_type="MULTIPOLYGON", srid=32616), nullable=False)


class SpatialFeatureModel(Base):
    """SQLAlchemy ORM mapping for the `spatial_features` table."""
    __tablename__ = "spatial_features"

    id          = Column(Integer, primary_key=True, autoincrement=True)
    finca_id    = Column(Integer, ForeignKey("land.id"), nullable=True)
    geom        = Column(Geometry(geometry_type="MULTIPOLYGON", srid=32616), nullable=True)
    type        = Column(String(50), nullable=True)
    name        = Column(String(100), nullable=True)
    properties  = Column(JSON, nullable=True)
