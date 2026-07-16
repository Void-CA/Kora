use axum::extract::State;
use axum::Json;

use crate::common::error::ApiError;
use crate::config::AppState;

use super::dto::{CreateEmployeeRequest, EmployeeResponse};

pub async fn list_employees(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeResponse>>, ApiError> {
    let employees = kora_domain::features::employees::use_cases::list_employees(
        &*state.employee_repo,
    )
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(employees.into_iter().map(Into::into).collect()))
}

pub async fn create_employee(
    State(state): State<AppState>,
    Json(body): Json<CreateEmployeeRequest>,
) -> Result<Json<EmployeeResponse>, ApiError> {
    let employee = kora_domain::features::employees::use_cases::create_employee(
        &*state.employee_repo,
        body.name,
    )
    .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    Ok(Json(employee.into()))
}
