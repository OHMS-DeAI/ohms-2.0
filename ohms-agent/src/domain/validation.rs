use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType)]
pub struct ValidationIssue {
    pub field: String,
    pub message: String,
    pub severity: ValidationSeverity,
}

#[derive(Clone, Debug, Serialize, Deserialize, CandidType, Default)]
pub struct ValidationReport {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn ok() -> Self {
        Self {
            valid: true,
            issues: Vec::new(),
        }
    }

    pub fn with_issue(
        mut self,
        field: impl Into<String>,
        message: impl Into<String>,
        severity: ValidationSeverity,
    ) -> Self {
        self.valid = self.valid && !matches!(severity, ValidationSeverity::Error);
        self.issues.push(ValidationIssue {
            field: field.into(),
            message: message.into(),
            severity,
        });
        self
    }
}
