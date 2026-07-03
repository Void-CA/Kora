use uuid::Uuid;
use kora_kernel::ids::CycleId;

use super::error::PayrollError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkerId(pub String);

impl WorkerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Operario,
    Supervisor,
    Tractorista,
    Tecnico,
    Otro(String),
}

#[derive(Debug, Clone)]
pub struct Worker {
    id: WorkerId,
    name: String,
    role: Option<Role>,
    active: bool,
}

impl Worker {
    pub fn new(name: String) -> Result<Self, PayrollError> {
        if name.trim().is_empty() {
            return Err(PayrollError::EmptyName);
        }
        Ok(Self {
            id: WorkerId::new(),
            name,
            role: None,
            active: true,
        })
    }

    pub fn with_role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = Some(role);
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn id(&self) -> &WorkerId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn role(&self) -> Option<&Role> {
        self.role.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_creation_with_name() {
        let w = Worker::new("Juan Pérez".into()).unwrap();
        assert_eq!(w.name(), "Juan Pérez");
        assert!(w.is_active());
    }

    #[test]
    fn empty_name_rejected() {
        assert!(matches!(Worker::new("".into()), Err(PayrollError::EmptyName)));
        assert!(matches!(Worker::new("   ".into()), Err(PayrollError::EmptyName)));
    }

    #[test]
    fn with_role_assigns_role() {
        let w = Worker::new("Ana".into()).unwrap().with_role(Role::Supervisor);
        assert_eq!(w.role(), Some(&Role::Supervisor));
    }

    #[test]
    fn deactivate_marks_inactive() {
        let mut w = Worker::new("Pedro".into()).unwrap();
        w.deactivate();
        assert!(!w.is_active());
    }

    #[test]
    fn worker_ids_are_unique() {
        let a = WorkerId::new();
        let b = WorkerId::new();
        assert_ne!(a, b);
    }

    #[allow(dead_code)]
    fn _cycle_marker(_: &CycleId) {}
}
