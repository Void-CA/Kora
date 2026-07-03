use crate::agriculture::drift::{
    VarianceReport as TimingReport, EconomicVarianceReport, VarianceConfig,
    VarianceService, EconomicVarianceService,
};
use crate::agriculture::planning::Schedule;
use crate::agriculture::cycle::CropCycle;
use crate::ports::economic_data_provider::EconomicDataProvider;

pub struct AnalyzeVarianceInput {
    pub schedule: Schedule,
    pub cycle: CropCycle,
    pub config: VarianceConfig,
    pub economic_provider: Option<Box<dyn EconomicDataProvider>>,
}

pub struct AnalyzeVarianceOutput {
    pub timing_report: TimingReport,
    pub economic_report: Option<EconomicVarianceReport>,
}

pub fn execute(input: AnalyzeVarianceInput) -> AnalyzeVarianceOutput {
    let timing_report = VarianceService::analyze_with_config(
        &input.schedule,
        &input.cycle,
        &input.config,
    );

    let economic_report = input.economic_provider.as_ref().map(|provider| {
        EconomicVarianceService::analyze_costs(&timing_report.matched, provider.as_ref())
    });

    AnalyzeVarianceOutput { timing_report, economic_report }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agriculture::planning::{Schedule, ScheduleAnchor, PlannedActivity, ActivityStatus};
    use crate::agriculture::cycle::CropCycle;
    use crate::agriculture::activity::{Activity, ActivityCategory};
    use crate::agriculture::drift::TimingVariance;
    use crate::shared_kernel::ids::{CycleId, CropId, AreaId};
    use crate::agriculture::ids::PlannedActivityId;
    use crate::shared_kernel::period::Period;
    use crate::shared_kernel::money::{Money, Currency};
    use crate::ports::economic_data_provider::EconomicDataProvider;
    use rust_decimal::Decimal;

    struct MockProvider;
    impl EconomicDataProvider for MockProvider {
        fn get_planned_cost(&self, _: &PlannedActivityId) -> Option<Money> {
            Some(Money::new(Decimal::from(100), Currency::USD))
        }
        fn get_actual_cost(&self, _: &crate::agriculture::ids::ActivityRecordId) -> Option<Money> {
            Some(Money::new(Decimal::from(120), Currency::USD))
        }
    }

    #[test]
    fn timing_only_flow() {
        let mut schedule = Schedule::new(CycleId::new(), ScheduleAnchor::SowingDate, 1500);
        schedule.add_planned_activity(PlannedActivity {
            id: PlannedActivityId::new(),
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: ActivityStatus::Planned,
        });

        let period = Period::new(1000, 2000).unwrap();
        let mut cycle = CropCycle::new(CropId::new(), AreaId::new(), period);
        cycle.register_activity(Activity::new(1500, ActivityCategory::Sowing)).unwrap();

        let output = execute(AnalyzeVarianceInput {
            schedule,
            cycle,
            config: VarianceConfig { temporal_tolerance_days: 5, enable_confidence_scoring: true },
            economic_provider: None,
        });

        assert_eq!(output.timing_report.matched.len(), 1);
        assert_eq!(output.timing_report.matched[0].variance, TimingVariance::OnTime);
        assert!(output.economic_report.is_none());
    }

    #[test]
    fn timing_and_economic_flow() {
        let mut schedule = Schedule::new(CycleId::new(), ScheduleAnchor::SowingDate, 1500);
        schedule.add_planned_activity(PlannedActivity {
            id: PlannedActivityId::new(),
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: ActivityStatus::Planned,
        });

        let period = Period::new(1000, 2000).unwrap();
        let mut cycle = CropCycle::new(CropId::new(), AreaId::new(), period);
        cycle.register_activity(Activity::new(1500, ActivityCategory::Sowing)).unwrap();

        let output = execute(AnalyzeVarianceInput {
            schedule,
            cycle,
            config: VarianceConfig { temporal_tolerance_days: 5, enable_confidence_scoring: false },
            economic_provider: Some(Box::new(MockProvider)),
        });

        assert!(output.economic_report.is_some());
        let econ = output.economic_report.unwrap();
        assert_eq!(econ.matched[0].cost_variance.as_ref().unwrap().variance.amount, Decimal::from(20));
    }
}
