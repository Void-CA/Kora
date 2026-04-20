// agriculture/area.rs
use crate::shared_kernel::space::{Measurement, Polygon};
use crate::shared_kernel::ids::AreaId;

#[derive(Debug)]
pub enum AreaClassification {
    Productive,
    Greenhouse,
    Storage, // El almacén PODRIA tener su propia lógica
}

#[derive(Debug)]
pub struct Area {
    id: AreaId,
    parent_area_id: Option<AreaId>, // Jerarquía: "Esta área está dentro del Sector 1"
    name: String,
    classification: AreaClassification,
    measurement: Measurement,
    geometry: Polygon,
}

impl Area {
    pub fn new(
        id: AreaId,
        name: String,
        classification: AreaClassification,
        measurement: Measurement,
        geometry: Polygon,
    ) -> Self {
        Self {
            id,
            parent_area_id: None,
            name,
            classification,
            measurement,
            geometry,
        }
    }

    pub fn id(&self) -> &AreaId {
        &self.id
    }
}