// agriculture/domain/area.rs
use crate::shared_kernel::space::Polygon;
use crate::shared_kernel::measurement::AreaMeasurement;
use crate::shared_kernel::ids::AreaId;
use crate::agriculture::domain::error::AgricultureError;

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
    ) -> Result<Self, AgricultureError> { // <- Cambio clave: Retorna Result

        let max_theoretical_ha = geometry.calculate_geodesic_sq_meters() / 10_000.0;
        let declared_ha = measurement.value_in_hectares();

        // Si el usuario dice que hay 12 hectáreas productivas en un polígono de 10 ha, explota.
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
}