use std::sync::Arc;

use crate::features::employees::repository::PgEmployeeRepository;
use kora_domain::features::employees::repository::EmployeeRepository;

#[derive(Clone)]
pub struct AppState {
    pub employee_repo: Arc<dyn EmployeeRepository>,
}

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "8000".into())
                .parse()
                .expect("APP_PORT must be a number"),
        }
    }
}

pub fn build_state() -> AppState {
    AppState {
        employee_repo: Arc::new(PgEmployeeRepository),
    }
}
