//! Error types for the tax calculation engine.

use crate::types::{FilingStatus, TaxYear};
use thiserror::Error;

/// Errors that can occur during tax calculation.
#[derive(Debug, Error)]
pub enum TaxError {
    /// Tax year is not supported.
    #[error("tax year {0} is not supported (supported: {1}-{2})")]
    UnsupportedTaxYear(TaxYear, TaxYear, TaxYear),

    /// Invalid filing status for the given situation.
    #[error("invalid filing status {status}: {reason}")]
    InvalidFilingStatus { status: FilingStatus, reason: String },

    /// Missing required input.
    #[error("missing required input: {0}")]
    MissingInput(String),

    /// Invalid input value.
    #[error("invalid value for {field}: {reason}")]
    InvalidValue { field: String, reason: String },

    /// Negative income where not allowed.
    #[error("negative income not allowed for {field}: got {value}")]
    NegativeIncome { field: String, value: String },

    /// SSN format is invalid.
    #[error("invalid SSN format: {0}")]
    InvalidSsn(String),

    /// Date format is invalid.
    #[error("invalid date format: {0} (expected YYYY-MM-DD)")]
    InvalidDate(String),

    /// Inconsistent data between forms.
    #[error("inconsistent data: {0}")]
    InconsistentData(String),

    /// Form schema not found.
    #[error("form schema not found: {form_type} for year {year}")]
    SchemaNotFound { form_type: String, year: TaxYear },

    /// Rule data not found.
    #[error("tax rules not found for year {0}")]
    RulesNotFound(TaxYear),

    /// JSON parsing error.
    #[error("failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Calculation overflow.
    #[error("calculation overflow: {0}")]
    Overflow(String),
}

/// Result type alias for tax operations.
pub type TaxResult<T> = Result<T, TaxError>;

/// Validation error details.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Field that failed validation.
    pub field: String,
    /// Error message.
    pub message: String,
    /// Severity level.
    pub severity: ValidationSeverity,
}

/// Severity of a validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    /// Error: calculation cannot proceed.
    Error,
    /// Warning: calculation can proceed but result may be incorrect.
    Warning,
}

impl ValidationError {
    /// Creates a new error-level validation error.
    pub fn error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            severity: ValidationSeverity::Error,
        }
    }

    /// Creates a new warning-level validation error.
    pub fn warning(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            severity: ValidationSeverity::Warning,
        }
    }
}

/// Collection of validation errors.
#[derive(Debug, Default)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Creates a new empty validation errors collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a validation error.
    pub fn add(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Adds an error-level validation error.
    pub fn add_error(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.add(ValidationError::error(field, message));
    }

    /// Returns true if there are any errors (not warnings).
    pub fn has_errors(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity == ValidationSeverity::Error)
    }

    /// Returns true if there are any warnings.
    pub fn has_warnings(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity == ValidationSeverity::Warning)
    }

    /// Returns true if there are no errors or warnings.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns all errors (not warnings).
    pub fn errors(&self) -> impl Iterator<Item = &ValidationError> {
        self.errors
            .iter()
            .filter(|e| e.severity == ValidationSeverity::Error)
    }

    /// Returns all warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &ValidationError> {
        self.errors
            .iter()
            .filter(|e| e.severity == ValidationSeverity::Warning)
    }

    /// Returns all validation errors.
    pub fn all(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Converts to a TaxError if there are any errors.
    pub fn into_result(self) -> TaxResult<()> {
        if self.has_errors() {
            let messages: Vec<_> = self.errors().map(|e| e.message.clone()).collect();
            Err(TaxError::InvalidValue {
                field: "multiple".to_string(),
                reason: messages.join("; "),
            })
        } else {
            Ok(())
        }
    }
}
