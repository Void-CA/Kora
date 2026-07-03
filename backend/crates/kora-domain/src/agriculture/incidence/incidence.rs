use rust_decimal::Decimal;
use uuid::Uuid;
use kora_kernel::ids::CycleId;
use kora_kernel::money::Money;

use super::error::IncidenceError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SanitaryIncidenceId(pub String);

impl SanitaryIncidenceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncidenceType {
    Pest,
    Disease,
    Otro(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SanitaryIncidence {
    id: SanitaryIncidenceId,
    cycle_id: CycleId,
    kind: IncidenceType,
    severity: Severity,
    description: String,
    action_taken: String,
    detected_at: i64,
    resolved_at: Option<i64>,
    economic_impact: Option<Money>,
}

impl SanitaryIncidence {
    pub fn new(
        cycle_id: CycleId,
        kind: IncidenceType,
        severity: Severity,
        description: String,
        action_taken: String,
        detected_at: i64,
    ) -> Result<Self, IncidenceError> {
        if cycle_id.0.is_empty() {
            return Err(IncidenceError::EmptyCycleId);
        }
        if description.trim().is_empty() {
            return Err(IncidenceError::EmptyDescription);
        }
        if action_taken.trim().is_empty() {
            return Err(IncidenceError::EmptyActionTaken);
        }
        Ok(Self {
            id: SanitaryIncidenceId::new(),
            cycle_id,
            kind,
            severity,
            description,
            action_taken,
            detected_at,
            resolved_at: None,
            economic_impact: None,
        })
    }

    pub fn resolve(&mut self, at: i64) {
        self.resolved_at = Some(at);
    }

    pub fn set_economic_impact(&mut self, impact: Money) {
        if impact.amount > Decimal::from(0) {
            self.economic_impact = Some(impact);
        }
    }

    pub fn is_resolved(&self) -> bool {
        self.resolved_at.is_some()
    }

    pub fn id(&self) -> &SanitaryIncidenceId {
        &self.id
    }

    pub fn cycle_id(&self) -> &CycleId {
        &self.cycle_id
    }

    pub fn kind(&self) -> &IncidenceType {
        &self.kind
    }

    pub fn severity(&self) -> &Severity {
        &self.severity
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn action_taken(&self) -> &str {
        &self.action_taken
    }

    pub fn detected_at(&self) -> i64 {
        self.detected_at
    }

    pub fn resolved_at(&self) -> Option<i64> {
        self.resolved_at
    }

    pub fn economic_impact(&self) -> Option<&Money> {
        self.economic_impact.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kora_kernel::money::Currency;

    fn make_incidence() -> SanitaryIncidence {
        SanitaryIncidence::new(
            CycleId::new(),
            IncidenceType::Pest,
            Severity::High,
            "Pulgón en hojas inferiores".into(),
            "Aplicación de imidacloprid".into(),
            1000,
        ).unwrap()
    }

    #[test]
    fn incidence_creation() {
        let i = make_incidence();
        assert_eq!(i.severity(), &Severity::High);
        assert!(!i.is_resolved());
    }

    #[test]
    fn empty_description_rejected() {
        let result = SanitaryIncidence::new(
            CycleId::new(),
            IncidenceType::Pest,
            Severity::Low,
            "".into(),
            "Acción".into(),
            1000,
        );
        assert!(matches!(result, Err(IncidenceError::EmptyDescription)));
    }

    #[test]
    fn empty_action_rejected() {
        let result = SanitaryIncidence::new(
            CycleId::new(),
            IncidenceType::Pest,
            Severity::Low,
            "Descripción".into(),
            "".into(),
            1000,
        );
        assert!(matches!(result, Err(IncidenceError::EmptyActionTaken)));
    }

    #[test]
    fn resolve_marks_resolved() {
        let mut i = make_incidence();
        i.resolve(1500);
        assert!(i.is_resolved());
        assert_eq!(i.resolved_at(), Some(1500));
    }

    #[test]
    fn economic_impact_set_with_positive_amount() {
        let mut i = make_incidence();
        i.set_economic_impact(Money::new(Decimal::from(200), Currency::USD));
        assert!(i.economic_impact().is_some());
    }

    #[test]
    fn zero_economic_impact_ignored() {
        let mut i = make_incidence();
        i.set_economic_impact(Money::new(Decimal::from(0), Currency::USD));
        assert!(i.economic_impact().is_none());
    }
}
