use std::str::FromStr;
use std::sync::{Arc, Mutex};
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_domain::ports::schedule_repository::ScheduleRepository;
use kora_domain::ports::budget_repository::BudgetRepository;
use kora_domain::ports::soil_analysis_repository::SoilAnalysisRepository;
use kora_domain::ports::worker_repository::WorkerRepository;
use kora_domain::ports::payroll_entry_repository::PayrollEntryRepository;
use kora_domain::ports::sanitary_incidence_repository::SanitaryIncidenceRepository;
use kora_domain::ports::revenue_repository::RevenueRepository;
use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::farm::Farm;
use kora_kernel::ids::AreaId;

pub struct AppState {
    pub cycle_repo: Arc<Mutex<Box<dyn CropCycleRepository + Send>>>,
    pub schedule_repo: Arc<Mutex<Box<dyn ScheduleRepository + Send>>>,
    pub budget_repo: Arc<Mutex<Box<dyn BudgetRepository + Send>>>,
    pub soil_repo: Arc<Mutex<Box<dyn SoilAnalysisRepository + Send>>>,
    pub worker_repo: Arc<Mutex<Box<dyn WorkerRepository + Send>>>,
    pub payroll_repo: Arc<Mutex<Box<dyn PayrollEntryRepository + Send>>>,
    pub incidence_repo: Arc<Mutex<Box<dyn SanitaryIncidenceRepository + Send>>>,
    pub revenue_repo: Arc<Mutex<Box<dyn RevenueRepository + Send>>>,
    pub farms: Vec<Farm>,
}

impl AppState {
    pub fn new() -> Self {
        use crate::adapters::in_memory_repositories::{
            InMemoryBudgetRepository, InMemoryCropCycleRepository, InMemoryScheduleRepository,
        };
        use crate::adapters::soil_in_memory::InMemorySoilAnalysisRepository;
        use crate::adapters::worker_in_memory::InMemoryWorkerRepository;
        use crate::adapters::payroll_in_memory::InMemoryPayrollEntryRepository;
        use crate::adapters::incidence_in_memory::InMemorySanitaryIncidenceRepository;
        use crate::adapters::revenue_in_memory::InMemoryRevenueRepository;
        let state = Self {
            cycle_repo: Arc::new(Mutex::new(Box::new(InMemoryCropCycleRepository::new()))),
            schedule_repo: Arc::new(Mutex::new(Box::new(InMemoryScheduleRepository::new()))),
            budget_repo: Arc::new(Mutex::new(Box::new(InMemoryBudgetRepository::new()))),
            soil_repo: Arc::new(Mutex::new(Box::new(InMemorySoilAnalysisRepository::new()))),
            worker_repo: Arc::new(Mutex::new(Box::new(InMemoryWorkerRepository::new()))),
            payroll_repo: Arc::new(Mutex::new(Box::new(InMemoryPayrollEntryRepository::new()))),
            incidence_repo: Arc::new(Mutex::new(Box::new(InMemorySanitaryIncidenceRepository::new()))),
            revenue_repo: Arc::new(Mutex::new(Box::new(InMemoryRevenueRepository::new()))),
            farms: seed::build_farms(),
        };
        seed::seed_via_use_cases(&state);
        state
    }

    pub fn farm_for_area(&self, area_id: &AreaId) -> Option<&Farm> {
        self.farms.iter().find(|f| f.has_area(area_id))
    }

    pub fn list_cycles(&self) -> Vec<CropCycle> {
        self.cycle_repo.lock().unwrap().all()
    }
}

pub mod seed {
    use super::*;
    use kora_domain::agriculture::area::{Area, AreaClassification};
    use kora_kernel::area_unit::{AreaMeasurement, AreaUnit};
    use rust_decimal::Decimal;
    use geo_types::polygon;

    fn dummy_polygon() -> kora_kernel::polygon::Polygon {
        kora_kernel::polygon::Polygon::new(polygon![
            (x: -70.0, y: 12.0),
            (x: -70.001, y: 12.0),
            (x: -70.001, y: 12.001),
            (x: -70.0, y: 12.001),
            (x: -70.0, y: 12.0),
        ]).unwrap()
    }

    pub fn build_farms() -> Vec<Farm> {
        let mut farm = Farm::new(kora_domain::agriculture::ids::FarmId::new());
        farm.add_area(
            Area::new(
                AreaId("area-campo-norte".into()),
                "Campo Norte".into(),
                AreaClassification::Productive,
                AreaMeasurement::new(12.0, AreaUnit::Hectares).unwrap(),
                dummy_polygon(),
            ).unwrap()
        );
        farm.add_area(
            Area::new(
                AreaId("area-campo-sur".into()),
                "Campo Sur".into(),
                AreaClassification::Productive,
                AreaMeasurement::new(8.0, AreaUnit::Hectares).unwrap(),
                dummy_polygon(),
            ).unwrap()
        );
        vec![farm]
    }

    pub fn seed_via_use_cases(state: &AppState) {
        use crate::use_cases::register_cycle::{self, RegisterCycleInput};
        use crate::use_cases::register_expense::{self, RegisterExpenseInput};
        use crate::use_cases::register_activity::{self as register_activity_uc, RegisterActivityInput};
        use kora_domain::agriculture::activity::ActivityCategory;
        use kora_domain::finance::expense::ExpenseCategory;
        use kora_domain::finance::budget::Budget;
        use kora_kernel::ids::CropId;
        use kora_kernel::period::Period;
        use kora_kernel::money::{ExchangeRateProvider, Currency, Money, RateError};

        struct SameCurrencyRate;
        impl ExchangeRateProvider for SameCurrencyRate {
            fn get_rate(&self, _: Currency, _: Currency) -> Result<Decimal, RateError> {
                Ok(Decimal::from(1))
            }
        }

        let ciclo_norte = register_cycle::execute(
            state,
            RegisterCycleInput {
                crop_id: CropId::new(),
                area_id: AreaId("area-campo-norte".into()),
                period: Period::new(1_700_000_000, 1_720_000_000).unwrap(),
                planned_activities: vec![
                    (ActivityCategory::Sowing, 0),
                    (ActivityCategory::Maintenance, 15),
                    (ActivityCategory::Harvest, 90),
                ],
            },
        ).ok();

        let ciclo_sur = register_cycle::execute(
            state,
            RegisterCycleInput {
                crop_id: CropId::new(),
                area_id: AreaId("area-campo-sur".into()),
                period: Period::new(1_710_000_000, 1_730_000_000).unwrap(),
                planned_activities: vec![
                    (ActivityCategory::Sowing, 0),
                    (ActivityCategory::Maintenance, 20),
                    (ActivityCategory::Harvest, 100),
                ],
            },
        ).ok();

        if let Some(c) = &ciclo_norte {
            let budget = Budget::new(
                c.cycle.id().clone(),
                c.cycle.period().clone(),
                Money::new(Decimal::from(5000), Currency::USD),
            );
            let bid = budget.id().clone();
            state.budget_repo.lock().unwrap().save(budget);
            let _ = register_expense::execute(
                state,
                RegisterExpenseInput {
                    budget_id: bid,
                    amount: Money::new(Decimal::from(2500), Currency::USD),
                    timestamp: 1_705_000_000,
                    category: ExpenseCategory::Seeds,
                    rate_provider: Box::new(SameCurrencyRate),
                },
            );
        }
        if let Some(c) = &ciclo_sur {
            let budget = Budget::new(
                c.cycle.id().clone(),
                c.cycle.period().clone(),
                Money::new(Decimal::from(3000), Currency::USD),
            );
            state.budget_repo.lock().unwrap().save(budget);
        }

        let _ = crate::features::soil::operations::register(
            state,
            AreaId("area-campo-norte".into()), 1_690_000_000,
            kora_domain::agriculture::soil::QualityLevel::Complete,
            Money::new(Decimal::from(150), Currency::USD),
            vec![
                kora_domain::agriculture::soil::SoilMetric::new(
                    kora_domain::agriculture::soil::SoilMetricKind::Ph, Decimal::from_str("6.2").unwrap()).unwrap(),
                kora_domain::agriculture::soil::SoilMetric::new(
                    kora_domain::agriculture::soil::SoilMetricKind::Nitrogen, Decimal::from_str("2.1").unwrap()).unwrap(),
                kora_domain::agriculture::soil::SoilMetric::new(
                    kora_domain::agriculture::soil::SoilMetricKind::Phosphorus, Decimal::from_str("28").unwrap()).unwrap(),
                kora_domain::agriculture::soil::SoilMetric::new(
                    kora_domain::agriculture::soil::SoilMetricKind::Potassium, Decimal::from_str("145").unwrap()).unwrap(),
            ],
        );

        let juan = crate::features::payroll::operations::register_worker(
            state, "Juan Pérez".into(), Some(kora_domain::finance::payroll::Role::Operario),
        ).ok();
        let _ = crate::features::payroll::operations::register_worker(
            state, "Ana López".into(), Some(kora_domain::finance::payroll::Role::Supervisor),
        );

        if let (Some(juan_worker), Some(c)) = (juan, &ciclo_norte) {
            let _ = crate::features::payroll::operations::record_payroll(
                state,
                juan_worker.id().clone(),
                Money::new(Decimal::from(500), Currency::USD), 1_704_000_000,
                Some(c.cycle.id().clone()), None,
            );
        }

        if let Some(c) = &ciclo_norte {
            let planned_sowing = state
                .schedule_repo
                .lock()
                .unwrap()
                .find_by_cycle_id(c.cycle.id())
                .and_then(|s| {
                    s.activities()
                        .iter()
                        .find(|p| matches!(p.category, ActivityCategory::Sowing))
                        .cloned()
                });
            if let Some(planned) = planned_sowing {
                let _ = register_activity_uc::execute(
                    state,
                    RegisterActivityInput {
                        cycle_id: c.cycle.id().clone(),
                        timestamp: 1_700_000_000,
                        category: ActivityCategory::Sowing,
                        notes: Some("Siembra de maíz híbrido".into()),
                        mode: register_activity_uc::RegistrationMode::ConfirmMatch(planned.id),
                    },
                );
            }
            let _ = register_activity_uc::execute(
                state,
                RegisterActivityInput {
                    cycle_id: c.cycle.id().clone(),
                    timestamp: 1_703_000_000,
                    category: ActivityCategory::Maintenance,
                    notes: Some("Riego por goteo".into()),
                    mode: register_activity_uc::RegistrationMode::Emergent,
                },
            );
            let _ = crate::features::incidences::operations::register(
                state,
                c.cycle.id().clone(),
                kora_domain::agriculture::incidence::IncidenceType::Pest,
                kora_domain::agriculture::incidence::Severity::High,
                "Pulgón detectado en hojas inferiores del Lote A".into(),
                "Aplicación de imidacloprid 0.5 L/ha".into(),
                1_708_000_000,
                Some(Money::new(Decimal::from(200), Currency::USD)),
            );
            let _ = crate::features::finance::revenue::register(
                state,
                crate::features::finance::revenue::RegisterRevenueInput {
                    cycle_id: Some(c.cycle.id().clone()),
                    amount: Money::new(Decimal::from(7200), Currency::USD),
                    received_at: 1_718_000_000,
                    source: kora_domain::finance::revenue::RevenueSource::Harvest,
                },
            );
        }
    }
}
