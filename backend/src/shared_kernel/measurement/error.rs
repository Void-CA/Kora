use core::fmt;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeasurementError {
    MustBePositive,
    
}

impl fmt::Display for MeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeasurementError::MustBePositive => write!(f, "Value must be positive"),
        }
    }
}