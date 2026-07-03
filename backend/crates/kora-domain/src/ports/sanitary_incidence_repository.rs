use crate::agriculture::incidence::{SanitaryIncidence, SanitaryIncidenceId};
use kora_kernel::ids::CycleId;

pub trait SanitaryIncidenceRepository {
    fn find_by_id(&self, id: &SanitaryIncidenceId) -> Option<SanitaryIncidence>;
    fn save(&mut self, incidence: SanitaryIncidence);
    fn all(&self) -> Vec<SanitaryIncidence>;
    fn for_cycle(&self, cycle_id: &CycleId) -> Vec<SanitaryIncidence>;
}
