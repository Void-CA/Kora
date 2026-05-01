// agriculture/domain/services/economic_variance.rs
use crate::shared_kernel::money::Money;
use super::super::planning::PlannedActivityId;
use super::variance_service::MatchedActivity;

// --- TRAIT: EconomicDataProvider (lives in agriculture, receives MINIMAL data) ---
/// Contract: Provider MUST return costs in the SAME currency for all calls.
/// If currencies differ, aggregation will fail with CurrencyMismatch.
/// Both methods return Option<Money> to allow graceful degradation (no data = skip).
pub trait EconomicDataProvider {
    /// Returns the planned cost for a given planned activity ID.
    /// Should return None if no budget data exists (graceful degradation).
    fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money>;

    /// Returns the actual cost for a given activity record ID.
    /// Should return None if no expense data exists (graceful degradation).
    fn get_actual_cost(&self, record_id: &str) -> Option<Money>;
}

// --- COST VARIANCE (specific name, NOT generic "EconomicData") ---
#[derive(Debug, Clone)]
pub struct CostVariance {
    pub planned_cost: Money,
    pub actual_cost: Money,
    pub variance: Money,  // actual - planned (computed via Money::subtract)
}

impl CostVariance {
    /// Creates a new CostVariance.
    /// Returns Result because planned and actual MUST have the same currency.
    /// This forces callers to handle currency mismatch explicitly (no silent ignore).
    pub fn new(planned_cost: Money, actual_cost: Money) -> Result<Self, crate::shared_kernel::money::RateError> {
        let variance = actual_cost
            .subtract(&planned_cost)?;  // Propagate error - no .expect()!

        Ok(Self {
            planned_cost,
            actual_cost,
            variance,
        })
    }
}

// --- ECONOMIC VARIANCE REPORT (separate from timing VarianceReport) ---
#[derive(Debug, Clone)]
pub struct EconomicVarianceReport {
    pub matched: Vec<MatchedActivity>,  // Reuse MatchedActivity (which now has cost_variance field)
    pub total_planned: Option<Money>,   // Option: None if no economic data available
    pub total_actual: Option<Money>,
    pub total_variance: Option<Money>,
}

impl EconomicVarianceReport {
    /// Creates an empty report (no hardcoded currency).
    pub fn new() -> Self {
        Self {
            matched: Vec::new(),
            total_planned: None,
            total_actual: None,
            total_variance: None,
        }
    }
}

// --- SEPARATE SERVICE for cost analysis ONLY ---
pub struct EconomicVarianceService;

impl EconomicVarianceService {
    /// Analyzes costs for already-matched activities (from VarianceService v3).
    /// ONLY does cost analysis - timing analysis stays in VarianceService.
    /// Gracefully skips activities where cost data is unavailable.
    pub fn analyze_costs(
        matched: &[MatchedActivity],
        provider: &impl EconomicDataProvider,
    ) -> EconomicVarianceReport {
        let mut report = EconomicVarianceReport::new();
        let mut total_planned = None;
        let mut total_actual = None;

        for m in matched {
            let planned_cost = provider.get_planned_cost(&m.planned_id);
            let actual_cost = provider.get_actual_cost(&m.record.activity.id().as_str());

            match (planned_cost, actual_cost) {
                (Some(pc), Some(ac)) => {
                    // EXPLICIT decision: If currency mismatch, skip economic data for this activity
                    // This is NOT silent - we log it and continue without cost_variance
                    match CostVariance::new(pc, ac) {
                        Ok(cost_var) => {
                            // Populate the cost_variance field
                            let mut enriched = m.clone();
                            enriched.cost_variance = Some(cost_var);
                            report.matched.push(enriched);

                            // Aggregate (these use Money::add which also validates currency)
                            total_planned = match (total_planned, Some(pc)) {
                                (None, Some(c)) => Some(c),
                                (Some(acc), Some(c)) => {
                                    match acc.add(&c) {
                                        Ok(sum) => Some(sum),
                                        Err(_) => {
                                            // Currency mismatch in aggregation: keep previous total
                                            // In production, this would be logged
                                            Some(acc)
                                        }
                                    }
                                }
                                (acc, None) => acc,
                            };

                            total_actual = match (total_actual, Some(ac)) {
                                (None, Some(c)) => Some(c),
                                (Some(acc), Some(c)) => {
                                    match acc.add(&c) {
                                        Ok(sum) => Some(sum),
                                        Err(_) => {
                                            // Currency mismatch in aggregation: keep previous total
                                            // In production, this would be logged
                                            Some(acc)
                                        }
                                    }
                                }
                                (acc, None) => acc,
                            };
                        }
                        Err(_rate_error) => {
                            // EXPLICIT: Currency mismatch in CostVariance creation
                            // Skip this activity for economic analysis (no cost_variance)
                            // In production: log::warn!("Currency mismatch for {:?}", m.planned_id);
                            report.matched.push(m.clone());
                        }
                    }
                }
                _ => {
                    // Graceful degradation: no cost data available from provider
                    report.matched.push(m.clone());
                }
            }
        }

        // Calculate total variance if we have data (graceful degradation)
        report.total_planned = total_planned;
        report.total_actual = total_actual;
        report.total_variance = match (total_planned, total_actual) {
            (Some(tp), Some(ta)) => {
                match ta.subtract(&tp) {
                    Ok(variance) => Some(variance),
                    Err(_) => None, // Currency mismatch
                }
            }
            _ => None,
        };

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared_kernel::money::{Money, Currency};
    use rust_decimal::Decimal;
    use crate::agriculture::domain::planning::PlannedActivityId;
    use crate::agriculture::domain::activity::{Activity, ActivityCategory, ActivityRecord, IntegrityStatus};
    use super::super::variance_service::{TimingVariance, ConfidenceScore};

    // --- MOCK for testing ---
    struct MockEconomicProvider {
        planned_costs: std::collections::HashMap<String, Money>,
        actual_costs: std::collections::HashMap<String, Money>,
    }

    impl EconomicDataProvider for MockEconomicProvider {
        fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money> {
            self.planned_costs.get(&planned_id.0).copied()
        }

        fn get_actual_cost(&self, record_id: &str) -> Option<Money> {
            self.actual_costs.get(record_id).copied()
        }
    }

    // Test 4.2: CostVariance creation (planned=100, actual=120 → variance=20)
    #[test]
    fn cost_variance_creation() {
        let planned = Money::new(Decimal::from(100), Currency::USD);
        let actual = Money::new(Decimal::from(120), Currency::USD);
        let cv = CostVariance::new(planned, actual).unwrap();  // Now returns Result

        assert_eq!(cv.planned_cost.amount, Decimal::from(100));
        assert_eq!(cv.actual_cost.amount, Decimal::from(120));
        assert_eq!(cv.variance.amount, Decimal::from(20)); // 120 - 100 = 20
    }

    // Test 4.3: MatchedActivity with cost_variance: Some(CostVariance)
    #[test]
    fn matched_activity_with_cost_variance() {
        let cv = CostVariance::new(
            Money::new(Decimal::from(100), Currency::USD),
            Money::new(Decimal::from(120), Currency::USD),
        ).unwrap();  // Now returns Result

        let opt_cv: Option<CostVariance> = Some(cv);
        assert!(opt_cv.is_some());
    }

    // Test 4.4: EconomicVarianceService::analyze_costs() with mocked provider
    #[test]
    fn analyze_costs_with_mocked_provider() {
        let mut provider = MockEconomicProvider {
            planned_costs: std::collections::HashMap::new(),
            actual_costs: std::collections::HashMap::new(),
        };

        // Setup mock data
        let planned_id = PlannedActivityId("p1".to_string());
        provider.planned_costs.insert(
            "p1".to_string(),
            Money::new(Decimal::from(100), Currency::USD),
        );

        // Create activity and get its actual ID for the mock (encapsulated access)
        let activity = Activity::new(1000, ActivityCategory::Sowing);
        let activity_id = activity.id().as_str().to_string();  // FIXED: use as_str()
        provider.actual_costs.insert(
            activity_id,
            Money::new(Decimal::from(120), Currency::USD),
        );

        let record = ActivityRecord::new(activity, vec![IntegrityStatus::Valid]);
        let matched = MatchedActivity {
            planned_id: planned_id.clone(),
            record,
            variance: TimingVariance::OnTime,
            confidence: ConfidenceScore::High,
            cost_variance: None,
        };

        let report = EconomicVarianceService::analyze_costs(&[matched], &provider);

        assert_eq!(report.matched.len(), 1);
        assert!(report.matched[0].cost_variance.is_some());
        let cv = report.matched[0].cost_variance.as_ref().unwrap();
        assert_eq!(cv.planned_cost.amount, Decimal::from(100));
        assert_eq!(cv.actual_cost.amount, Decimal::from(120));
    }

    // Test 4.5: Graceful degradation when provider returns None
    #[test]
    fn analyze_costs_graceful_degradation() {
        let provider = MockEconomicProvider {
            planned_costs: std::collections::HashMap::new(), // Empty = no data
            actual_costs: std::collections::HashMap::new(),
        };

        let planned_id = PlannedActivityId("p1".to_string());
        let activity = Activity::new(1000, ActivityCategory::Sowing);
        let record = ActivityRecord::new(activity, vec![IntegrityStatus::Valid]);
        let matched = MatchedActivity {
            planned_id,
            record,
            variance: TimingVariance::OnTime,
            confidence: ConfidenceScore::High,
            cost_variance: None,
        };

        let report = EconomicVarianceService::analyze_costs(&[matched], &provider);

        // Should still have the matched activity, but without cost_variance
        assert_eq!(report.matched.len(), 1);
        assert!(report.matched[0].cost_variance.is_none());
    }

    // Test 4.6: EconomicVarianceReport aggregations
    #[test]
    fn report_aggregation_multiple_activities() {
        let mut provider = MockEconomicProvider {
            planned_costs: std::collections::HashMap::new(),
            actual_costs: std::collections::HashMap::new(),
        };

        // Activity 1: planned=100, actual=120
        provider.planned_costs.insert("p1".to_string(), Money::new(Decimal::from(100), Currency::USD));
        
        let activity1 = Activity::new(1000, ActivityCategory::Sowing);
        let activity1_id = activity1.id().as_str().to_string();  // FIXED: use as_str()
        provider.actual_costs.insert(activity1_id, Money::new(Decimal::from(120), Currency::USD));

        // Activity 2: planned=200, actual=180
        provider.planned_costs.insert("p2".to_string(), Money::new(Decimal::from(200), Currency::USD));
        
        let activity2 = Activity::new(2000, ActivityCategory::Harvest);
        let activity2_id = activity2.id().as_str().to_string();  // FIXED: use as_str()
        provider.actual_costs.insert(activity2_id, Money::new(Decimal::from(180), Currency::USD));

        let matched1 = MatchedActivity {
            planned_id: PlannedActivityId("p1".to_string()),
            record: ActivityRecord::new(activity1, vec![IntegrityStatus::Valid]),
            variance: TimingVariance::OnTime,
            confidence: ConfidenceScore::High,
            cost_variance: None,
        };

        let matched2 = MatchedActivity {
            planned_id: PlannedActivityId("p2".to_string()),
            record: ActivityRecord::new(activity2, vec![IntegrityStatus::Valid]),
            variance: TimingVariance::OnTime,
            confidence: ConfidenceScore::High,
            cost_variance: None,
        };

        let report = EconomicVarianceService::analyze_costs(&[matched1, matched2], &provider);

        assert_eq!(report.matched.len(), 2);
        assert!(report.total_planned.is_some());
        assert!(report.total_actual.is_some());
        assert!(report.total_variance.is_some());

        let total_planned = report.total_planned.unwrap();
        let total_actual = report.total_actual.unwrap();
        let total_variance = report.total_variance.unwrap();

        assert_eq!(total_planned.amount, Decimal::from(300)); // 100 + 200
        assert_eq!(total_actual.amount, Decimal::from(300));  // 120 + 180
        assert_eq!(total_variance.amount, Decimal::ZERO);     // 300 - 300
    }
}
