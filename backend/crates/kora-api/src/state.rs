use std::sync::{Arc, Mutex};
use kora_domain::ports::cycle_repository::CropCycleRepository;
use kora_domain::ports::schedule_repository::ScheduleRepository;
use kora_domain::ports::budget_repository::BudgetRepository;
use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::agriculture::farm::Farm;
use kora_kernel::ids::AreaId;

pub struct AppState {
    pub cycle_repo: Arc<Mutex<Box<dyn CropCycleRepository + Send>>>,
    pub schedule_repo: Arc<Mutex<Box<dyn ScheduleRepository + Send>>>,
    pub budget_repo: Arc<Mutex<Box<dyn BudgetRepository + Send>>>,
    pub farms: Vec<Farm>,
}

impl AppState {
    pub fn new() -> Self {
        use crate::adapters::in_memory_repositories::{
            InMemoryBudgetRepository, InMemoryCropCycleRepository, InMemoryScheduleRepository,
        };
        let state = Self {
            cycle_repo: Arc::new(Mutex::new(Box::new(InMemoryCropCycleRepository::new()))),
            schedule_repo: Arc::new(Mutex::new(Box::new(InMemoryScheduleRepository::new()))),
            budget_repo: Arc::new(Mutex::new(Box::new(InMemoryBudgetRepository::new()))),
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
    }
}
