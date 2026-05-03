// agriculture/application/use_cases/analyze_variance.rs
use crate::shared_kernel::money::Money;
use crate::agriculture::domain::services::variance_service::VarianceReport as TimingReport;
use crate::agriculture::domain::services::economic_variance::{
    EconomicDataProvider, 
    EconomicVarianceReport
};
use crate::agriculture::domain::planning::Schedule;
use crate::agriculture::domain::cycle::CropCycle;
use crate::agriculture::domain::services::variance_service::VarianceConfig;

/// Input: Acepta tanto timing-only como economic analysis
/// REGLA: Application layer usa dyn Trait (dynamic dispatch)
pub struct AnalyzeVarianceInput {
    pub schedule: Schedule,
    pub cycle: CropCycle,
    pub config: VarianceConfig,
    pub economic_provider: Option<Box<dyn EconomicDataProvider>>,
}

/// Output: Both reports (economic is Optional)
pub struct AnalyzeVarianceOutput {
    pub timing_report: TimingReport,
    pub economic_report: Option<EconomicVarianceReport>,
}

/// Use Case: Analyze Variance (Timing + Optional Economic)
pub fn execute(input: AnalyzeVarianceInput) -> AnalyzeVarianceOutput {
    let timing_report = crate::agriculture::domain::services::variance_service::VarianceService::analyze_with_config(
        &input.schedule,
        &input.cycle,
        &input.config,
    );

    let economic_report = input.economic_provider.as_ref().map(|provider| {
        crate::agriculture::domain::services::economic_variance::EconomicVarianceService::analyze_costs(
            &timing_report.matched,
            provider.as_ref(),
        )
    });

    AnalyzeVarianceOutput {
        timing_report,
        economic_report,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agriculture::domain::{
        planning::{Schedule, ScheduleAnchor, PlannedActivity, ActivityStatus},
        cycle::CropCycle,
        activity::{Activity, ActivityCategory}
    };
    use crate::agriculture::domain::services::variance_service::{TimingVariance, VarianceConfig};
    use crate::shared_kernel::ids::{CycleId, CropId, AreaId};
    use crate::agriculture::domain::ids::PlannedActivityId;
    use crate::shared_kernel::time::Period;
    use crate::shared_kernel::money::{Money, Currency};
    use rust_decimal::Decimal;
    use crate::agriculture::domain::services::economic_variance::EconomicDataProvider;

    // Test: Complete flow - Timing + Economic analysis
    #[test]
    fn use_case_analyze_variance_complete_flow() {
        // --- GIVEN: Schedule with planned activities ---
        let mut schedule = Schedule::new(
            CycleId::new(),
            ScheduleAnchor::SowingDate,
            1500,  // anchor date
        );

        let planned_1 = PlannedActivityId::new();
        schedule.add_planned_activity(PlannedActivity {
            id: planned_1.clone(),
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: ActivityStatus::Planned,
        });

        // --- GIVEN: CropCycle with executed activities ---
        let period = Period::new(1000, 2000).unwrap();
        let mut cycle = CropCycle::new(
            CropId::new(),
            AreaId::new(),
            period,
        );

        // Activity 1: Sowing ON TIME (timestamp 1500)
        let activity1 = Activity::new(1500, ActivityCategory::Sowing);
        let _record1 = cycle.register_activity(activity1).unwrap();

        // --- WHEN: Execute use case (timing only first) ---
        let input = AnalyzeVarianceInput {
            schedule,
            cycle,
            config: VarianceConfig {
                temporal_tolerance_days: 5,
                enable_early_detection: true,
                enable_confidence_scoring: true,
            },
            economic_provider: None,  // No economic analysis yet
        };

        let output = execute(input);

        // --- THEN: Verify timing report ---
        assert_eq!(output.timing_report.matched.len(), 1);
        assert_eq!(output.timing_report.unplanned.len(), 0);
        assert!(output.economic_report.is_none());  // No provider given

        // Verify timing details
        let matched = &output.timing_report.matched[0];
        assert_eq!(matched.planned_id.as_str(), planned_1.as_str());
        assert_eq!(matched.variance, TimingVariance::OnTime);
    }

    // Test: Use case with economic analysis
    #[test]
    fn use_case_with_economic_analysis() {
        // --- GIVEN: Schedule ---
        let mut schedule = Schedule::new(
            CycleId::new(),
            ScheduleAnchor::SowingDate,
            1500,
        );

        let planned_1 = PlannedActivityId::new();
        schedule.add_planned_activity(PlannedActivity {
            id: planned_1.clone(),
            category: ActivityCategory::Sowing,
            relative_day: 0,
            status: ActivityStatus::Planned,
        });

        // --- GIVEN: CropCycle ---
        let period = Period::new(1000, 2000).unwrap();
        let mut cycle = CropCycle::new(
            CropId::new(),
            AreaId::new(),
            period,
        );

        let activity1 = Activity::new(1500, ActivityCategory::Sowing);
        let _record1 = cycle.register_activity(activity1).unwrap();

        // --- GIVEN: Mock EconomicDataProvider ---
        struct MockProvider;
        impl EconomicDataProvider for MockProvider {
            fn get_planned_cost(&self, _planned_id: &crate::agriculture::domain::ids::PlannedActivityId) -> Option<Money> {
                Some(Money::new(Decimal::from(100), Currency::USD))
            }
            fn get_actual_cost(&self, _record_id: &crate::agriculture::domain::ids::ActivityRecordId) -> Option<Money> {
                Some(Money::new(Decimal::from(120), Currency::USD))
            }
        }

        // --- WHEN: Execute use case WITH economic provider ---
        let input = AnalyzeVarianceInput {
            schedule,
            cycle,
            config: VarianceConfig {
                temporal_tolerance_days: 5,
                enable_early_detection: false,
                enable_confidence_scoring: false,
            },
            economic_provider: Some(Box::new(MockProvider)),
        };

        let output = execute(input);

        // --- THEN: Verify BOTH reports ---
        // Timing report
        assert_eq!(output.timing_report.matched.len(), 1);

        // Economic report
        assert!(output.economic_report.is_some());
        let econ_report = output.economic_report.unwrap();
        assert_eq!(econ_report.matched.len(), 1);
        
        let cv = econ_report.matched[0].cost_variance.as_ref().unwrap();
        assert_eq!(cv.planned_cost.amount, Decimal::from(100));
        assert_eq!(cv.actual_cost.amount, Decimal::from(120));
        assert_eq!(cv.variance.amount, Decimal::from(20));  // Overcost $20
    }
}
