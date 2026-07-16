use std::sync::Arc;

use sqlx::PgPool;

use crate::features::employees::repository::PgEmployeeRepository;
use kora_domain::features::employees::repository::EmployeeRepository;
use kora_domain::features::employees::repository::WorkLogRepository;

#[derive(Clone)]
pub struct AppState {
    pub employee_repo: Arc<PgEmployeeRepository>,
}

impl AppState {
    pub fn employee_repo(&self) -> &dyn EmployeeRepository {
        self.employee_repo.as_ref() as &dyn EmployeeRepository
    }

    pub fn work_log_repo(&self) -> &dyn WorkLogRepository {
        self.employee_repo.as_ref() as &dyn WorkLogRepository
    }
}

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "8000".into())
                .parse()
                .expect("APP_PORT must be a number"),
        }
    }
}

pub fn build_state(pool: PgPool) -> AppState {
    AppState {
        employee_repo: Arc::new(PgEmployeeRepository::new(pool)),
    }
}
