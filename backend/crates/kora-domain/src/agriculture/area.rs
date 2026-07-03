use kora_kernel::polygon::Polygon;
use kora_kernel::area_unit::AreaMeasurement;
use kora_kernel::ids::AreaId;
use crate::agriculture::error::AgricultureError;

#[derive(Debug)]
pub enum AreaClassification {
    Productive,
    Greenhouse,
    Storage,
}

#[derive(Debug)]
pub struct Area {
    id: AreaId,
    parent_area_id: Option<AreaId>,
    name: String,
    classification: AreaClassification,
    measurement: AreaMeasurement,
    geometry: Polygon,
}

impl Area {
    pub fn new(
        id: AreaId,
        name: String,
        classification: AreaClassification,
        measurement: AreaMeasurement,
        geometry: Polygon,
    ) -> Result<Self, AgricultureError> {
        let max_theoretical_ha = geometry.calculate_geodesic_sq_meters() / 10_000.0;
        let declared_ha = measurement.value_in_hectares();

        if declared_ha > max_theoretical_ha {
            return Err(AgricultureError::ProductiveAreaExceedsBounds {
                declared_ha,
                max_theoretical_ha,
            });
        }

        Ok(Self {
            id,
            parent_area_id: None,
            name,
            classification,
            measurement,
            geometry,
        })
    }

    pub fn id(&self) -> &AreaId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn measurement(&self) -> &AreaMeasurement {
        &self.measurement
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kora_kernel::polygon::Polygon;
    use kora_kernel::area_unit::{AreaMeasurement, AreaUnit};
    use geo::GeodesicArea;
    use geo_types::polygon;

    /// Returns the geodesic area of the test polygon in hectares.
    /// Self-calibrating: computes from the actual polygon so it tolerates
    /// variations in the geodesic algorithm.
    fn polygon_max_ha() -> f64 {
        let raw = polygon![
            (x: -70.0, y: 12.0),
            (x: -70.001, y: 12.0),
            (x: -70.001, y: 12.001),
            (x: -70.0, y: 12.001),
            (x: -70.0, y: 12.0),
        ];
        raw.geodesic_area_unsigned() / 10_000.0
    }

    fn test_polygon() -> Polygon {
        Polygon::new(polygon![
            (x: -70.0, y: 12.0),
            (x: -70.001, y: 12.0),
            (x: -70.001, y: 12.001),
            (x: -70.0, y: 12.001),
            (x: -70.0, y: 12.0),
        ])
        .unwrap()
    }

    #[test]
    fn area_within_bounds_succeeds() {
        let max_ha = polygon_max_ha();
        // Declare half the actual area → should always pass
        let area = Area::new(
            AreaId::new(),
            "Lote A".into(),
            AreaClassification::Productive,
            AreaMeasurement::new(max_ha * 0.5, AreaUnit::Hectares).unwrap(),
            test_polygon(),
        );
        assert!(area.is_ok());
    }

    #[test]
    fn area_exceeding_bounds_fails() {
        let max_ha = polygon_max_ha();
        // Declare double the actual area → should always fail
        let area = Area::new(
            AreaId::new(),
            "Lote A".into(),
            AreaClassification::Productive,
            AreaMeasurement::new(max_ha * 2.0, AreaUnit::Hectares).unwrap(),
            test_polygon(),
        );
        assert!(area.is_err());
        match area.unwrap_err() {
            AgricultureError::ProductiveAreaExceedsBounds { .. } => {}
            _ => panic!("expected ProductiveAreaExceedsBounds"),
        }
    }
}
