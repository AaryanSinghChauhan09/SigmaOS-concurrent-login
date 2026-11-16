// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-concurrent-login: concurrent login component model.
//!
//! Real implementation: logins, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Login {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcurrentLoginError {
    EmptyLoginName,
    DuplicateLogin(String),
    EmptyEventSubject,
}

impl fmt::Display for ConcurrentLoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLoginName => write!(f, "login name must not be empty"),
            Self::DuplicateLogin(id) => write!(f, "duplicate login {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ConcurrentLoginError {}

pub struct ConcurrentLogin {
    pub name: String,
    logins: BTreeMap<String, Login>,
    events: BTreeMap<String, Event>,
}

impl ConcurrentLogin {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            logins: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_login(&mut self, item: Login) -> Result<(), ConcurrentLoginError> {
        if item.name.trim().is_empty() {
            return Err(ConcurrentLoginError::EmptyLoginName);
        }
        let id = item.id.clone();
        if self.logins.contains_key(&id) {
            return Err(ConcurrentLoginError::DuplicateLogin(id.clone()));
        }
        self.logins.insert(id, item);
        Ok(())
    }

    pub fn login_count(&self) -> usize {
        self.logins.len()
    }

    pub fn get_login(&self, id: &str) -> Option<&Login> {
        self.logins.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ConcurrentLoginError> {
        if event.subject.trim().is_empty() {
            return Err(ConcurrentLoginError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_login(id: &str, name: &str) -> Login {
        Login {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ConcurrentLogin::new("ConcurrentLogin");
        assert_eq!(m.name, "ConcurrentLogin");
        assert_eq!(m.login_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_login() {
        let mut m = ConcurrentLogin::new("ConcurrentLogin");
        m.add_login(sample_login("a1", "Alpha")).unwrap();
        assert_eq!(m.login_count(), 1);
        assert!(m.get_login("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ConcurrentLogin::new("ConcurrentLogin");
        assert!(m.add_login(sample_login("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ConcurrentLogin::new("ConcurrentLogin");
        m.add_login(sample_login("x", "One")).unwrap();
        assert!(m.add_login(sample_login("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ConcurrentLogin::new("ConcurrentLogin");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
