use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeasurementError {
    MustBePositive,
    
}