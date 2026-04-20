// shared_kernel/time.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    start_timestamp: i64,
    end_timestamp: i64,
}

impl Period {
    /// Crea un nuevo periodo protegiendo el invariante temporal base.
    pub fn new(start_timestamp: i64, end_timestamp: i64) -> Result<Self, &'static str> {
        if start_timestamp >= end_timestamp {
            return Err("Start timestamp must be strictly before end timestamp");
        }

        Ok(Self {
            start_timestamp,
            end_timestamp,
        })
    }

    /// Usado por CropCycle para asegurar que una Activity se registra 
    /// en el momento histórico correcto.
    pub fn contains(&self, timestamp: i64) -> bool {
        timestamp >= self.start_timestamp && timestamp <= self.end_timestamp
    }

    /// Usado por Farm para prevenir colisiones espacio-temporales
    /// (dos cultivos en la misma área al mismo tiempo).
    pub fn overlaps_with(&self, other: &Period) -> bool {
        self.start_timestamp <= other.end_timestamp && self.end_timestamp >= other.start_timestamp
    }

    pub fn start(&self) -> i64 {
        self.start_timestamp
    }

    pub fn end(&self) -> i64 {
        self.end_timestamp
    }
}