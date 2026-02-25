use us_tax_brackets::TaxYear;

/// Whether a form is an input (source document) or output (IRS return form).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormType {
    Input,
    Output,
}

/// Shared interface for all tax forms (input and output).
pub trait Form {
    /// IRS form name (e.g., "Form W-2", "Form 1040", "Schedule B").
    fn name() -> &'static str;

    /// Tax year this form applies to.
    fn year(&self) -> TaxYear;

    /// Whether this is an input or output form.
    fn form_type() -> FormType;
}
