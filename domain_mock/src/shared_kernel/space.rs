use crate::shared_kernel::error::SpaceError;

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    hectares: f64,
}

impl Measurement {
    pub fn new(hectares: f64) -> Result<Self, SpaceError> {
        if hectares <= 0.0 {
            return Err(SpaceError::InvalidMeasurement);
        }
        Ok(Self { hectares })
    }
}

// Representación simplificada de la geometría (Sección 4.1 del PRD)
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    coordinates: Vec<(f64, f64)>,
}

impl Polygon {
    pub fn new(coordinates: Vec<(f64, f64)>) -> Result<Self, SpaceError> {
        // Un polígono requiere al menos 3 vértices
        if coordinates.len() < 3 {
            return Err(SpaceError::NotEnoughVertices);
        }

        Ok(Self { coordinates })
    }
}