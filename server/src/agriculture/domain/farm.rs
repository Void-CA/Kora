// agriculture/domain/farm.rs
use crate::shared_kernel::ids::{FarmId, AreaId};
use super::area::Area;

#[derive(Debug)]
pub struct Farm {
    id: FarmId,
    areas: Vec<Area>, // Farm sigue administrando la topología de la finca
}

impl Farm {
    pub fn new(id: FarmId) -> Self {
        Self {
            id,
            areas: Vec::new(),
        }
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