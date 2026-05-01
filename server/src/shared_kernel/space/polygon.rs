// shared_kernel/space/polygon.rs
use geo_types::Polygon as GeoPolygon;
use geo::{GeodesicArea, Intersects};
use super::error::SpaceError;

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    inner: GeoPolygon<f64>,
}

impl Polygon {
    pub fn new(geo_polygon: GeoPolygon<f64>) -> Result<Self, SpaceError> {
        Ok(Self { inner: geo_polygon })
    }

    pub fn intersects(&self, other: &Polygon) -> bool {
        self.inner.intersects(&other.inner)
    }

    /// Calcula el área matemática del polígono en metros cuadrados
    pub fn calculate_geodesic_sq_meters(&self) -> f64 {
        self.inner.geodesic_area_unsigned()
    }
}