use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpaceError {}

#[derive(Error, Debug)]
pub enum MeasurementError {
    #[error("Value must be positive")]
    MustBePositive,
}
