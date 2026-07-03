use crate::agriculture::ids::FarmId;
use crate::shared_kernel::ids::AreaId;
use crate::agriculture::area::Area;

#[derive(Debug)]
pub struct Farm {
    id: FarmId,
    areas: Vec<Area>,
}

impl Farm {
    pub fn new(id: FarmId) -> Self {
        Self { id, areas: Vec::new() }
    }

    pub fn add_area(&mut self, area: Area) {
        self.areas.push(area);
    }

    pub fn has_area(&self, area_id: &AreaId) -> bool {
        self.areas.iter().any(|a| a.id() == area_id)
    }

    pub fn id(&self) -> &FarmId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared_kernel::area_unit::{AreaMeasurement, AreaUnit};
    use crate::shared_kernel::polygon::Polygon;
    use geo_types::polygon;

    fn dummy_polygon() -> Polygon {
        Polygon::new(polygon![
            (x: -70.0, y: 12.0),
            (x: -70.001, y: 12.0),
            (x: -70.001, y: 12.001),
            (x: -70.0, y: 12.001),
            (x: -70.0, y: 12.0),
        ])
        .unwrap()
    }

    fn dummy_area(id: &AreaId) -> Area {
        Area::new(
            id.clone(),
            "Lote A".into(),
            crate::agriculture::area::AreaClassification::Productive,
            AreaMeasurement::new(1.0, AreaUnit::Hectares).unwrap(),
            dummy_polygon(),
        )
        .unwrap()
    }

    #[test]
    fn farm_has_area_returns_true_for_existing() {
        let mut f = Farm::new(FarmId::new());
        let area_id = AreaId::new();
        f.add_area(dummy_area(&area_id));
        assert!(f.has_area(&area_id));
    }

    #[test]
    fn farm_has_area_returns_false_for_unknown() {
        let f = Farm::new(FarmId::new());
        assert!(!f.has_area(&AreaId::new()));
    }
}
