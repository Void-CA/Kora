use axum::extract::{Path, State};
use axum::Json;

use crate::common::error::ApiError;
use crate::config::AppState;

use super::dto::{
    CreateEmployeeRequest, CreateWorkLogRequest, EmployeeResponse, WorkLogResponse,
};
use uuid::Uuid;

pub async fn list_employees(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeResponse>>, ApiError> {
    let employees =
        kora_domain::features::employees::use_cases::list_employees(state.employee_repo())
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(employees.into_iter().map(Into::into).collect()))
}

pub async fn create_employee(
    State(state): State<AppState>,
    Json(body): Json<CreateEmployeeRequest>,
) -> Result<Json<EmployeeResponse>, ApiError> {
    let employee = kora_domain::features::employees::use_cases::create_employee(
        state.employee_repo(),
        body.name,
    )
    .await
    .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    Ok(Json(employee.into()))
}

pub async fn list_work_logs(
    State(state): State<AppState>,
    Path(employee_id): Path<Uuid>,
) -> Result<Json<Vec<WorkLogResponse>>, ApiError> {
    let employee_id =
        kora_domain::features::employees::employee::EmployeeId(employee_id);

    let logs = kora_domain::features::employees::use_cases::list_work_logs(
        state.work_log_repo(),
        employee_id,
    )
    .await
    .map_err(|e| ApiError::NotFound(e.to_string()))?;

    Ok(Json(logs.into_iter().map(Into::into).collect()))
}

pub async fn create_work_log(
    State(state): State<AppState>,
    Json(body): Json<CreateWorkLogRequest>,
) -> Result<Json<WorkLogResponse>, ApiError> {
    let employee_id =
        kora_domain::features::employees::employee::EmployeeId(body.employee_id);

    let log = kora_domain::features::employees::use_cases::register_work_log(
        state.work_log_repo(),
        employee_id,
        body.hours,
        body.worked_on,
    )
    .await
    .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    Ok(Json(WorkLogResponse {
        id: log.id.0,
        employee_id: log.employee_id.0,
        worked_on: log.worked_on,
        hours: log.hours,
    }))
}
