use async_trait::async_trait;
use sqlx::PgPool;

use kora_domain::features::employees::employee::{Employee, EmployeeId};
use kora_domain::features::employees::errors::EmployeeError;
use kora_domain::features::employees::repository::EmployeeRepository;
use kora_domain::features::employees::repository::WorkLogRepository;
use kora_domain::features::employees::work_log::{WorkLog, WorkLogId};

pub struct PgEmployeeRepository {
    pool: PgPool,
}

impl PgEmployeeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmployeeRepository for PgEmployeeRepository {
    async fn insert(&self, employee: &Employee) -> Result<(), EmployeeError> {
        sqlx::query("INSERT INTO employees (id, name, active) VALUES ($1, $2, $3)")
            .bind(employee.id.0)
            .bind(&employee.name)
            .bind(employee.active)
            .execute(&self.pool)
            .await
            .map_err(|e| EmployeeError::NotFound(e.to_string()))?;
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Employee>, EmployeeError> {
        sqlx::query_as::<_, (uuid::Uuid, String, bool)>(
            "SELECT id, name, active FROM employees ORDER BY name",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| EmployeeError::NotFound(e.to_string()))
        .map(|rows| {
            rows.into_iter()
                .map(|(id, name, active)| Employee {
                    id: EmployeeId(id),
                    name,
                    active,
                })
                .collect()
        })
    }
}

#[async_trait]
impl WorkLogRepository for PgEmployeeRepository {
    async fn insert(&self, work_log: &WorkLog) -> Result<(), EmployeeError> {
        sqlx::query(
            "INSERT INTO work_logs (id, employee_id, worked_on, hours) VALUES ($1, $2, $3, $4)",
        )
        .bind(work_log.id.0)
        .bind(work_log.employee_id.0)
        .bind(work_log.worked_on)
        .bind(work_log.hours)
        .execute(&self.pool)
        .await
        .map_err(|e| EmployeeError::NotFound(e.to_string()))?;
        Ok(())
    }

    async fn list_by_employee(
        &self,
        employee_id: EmployeeId,
    ) -> Result<Vec<WorkLog>, EmployeeError> {
        sqlx::query_as::<_, (uuid::Uuid, uuid::Uuid, chrono::NaiveDate, f64)>(
            "SELECT id, employee_id, worked_on, hours FROM work_logs WHERE employee_id = $1 ORDER BY worked_on DESC",
        )
        .bind(employee_id.0)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| EmployeeError::NotFound(e.to_string()))
        .map(|rows| {
            rows.into_iter()
                .map(|(id, eid, worked_on, hours)| WorkLog {
                    id: WorkLogId(id),
                    employee_id: EmployeeId(eid),
                    worked_on,
                    hours,
                })
                .collect()
        })
    }
}
