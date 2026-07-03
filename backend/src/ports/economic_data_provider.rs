use crate::shared_kernel::money::Money;
use crate::agriculture::ids::{PlannedActivityId, ActivityRecordId};

/// Provider trait for fetching economic data from finance context.
/// Agriculture uses this to enrich drift analysis with cost data
/// without depending on finance internals.
pub trait EconomicDataProvider {
    fn get_planned_cost(&self, planned_id: &PlannedActivityId) -> Option<Money>;
    fn get_actual_cost(&self, record_id: &ActivityRecordId) -> Option<Money>;
}
