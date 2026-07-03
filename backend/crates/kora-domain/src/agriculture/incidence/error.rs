#[derive(Debug, PartialEq)]
pub enum IncidenceError {
    EmptyDescription,
    EmptyActionTaken,
    EmptyCycleId,
}
