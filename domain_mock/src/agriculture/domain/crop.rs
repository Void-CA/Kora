// agriculture/crop.rs
use crate::shared_kernel::ids::CropId;

#[derive(Debug)]
pub struct Crop {
    id: CropId,
    name: String,
}

impl Crop {
    pub fn new(id: CropId, name: String) -> Self {
        Self { id, name }
    }
    pub fn id(&self) -> &CropId {
        &self.id
    }
}