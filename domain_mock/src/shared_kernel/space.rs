// shared_kernel/space.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    hectares: f64,
}

impl Measurement {
    pub fn new(hectares: f64) -> Result<Self, &'static str> {
        if hectares <= 0.0 {
            return Err("Measurement must be greater than zero");
        }
        Ok(Self { hectares })
    }
}

// Representación simplificada de la geometría (Sección 4.1 del PRD)
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    // Aquí iría un Vec de Coordenadas, GIS data, etc.
    coordinates: Vec<(f64, f64)>,
}