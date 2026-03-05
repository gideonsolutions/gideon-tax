use us_tax_brackets::TaxYear;

/// Whether a form is an input (source document), output (IRS return form),
/// or election (one-time application/election form).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormType {
    Input,
    Output,
    /// One-time application or election form (e.g., Form 4029, Form 4361).
    Election,
}

/// Shared interface for all tax forms (input, output, and election).
pub trait Form {
    /// IRS form name (e.g., "Form W-2", "Form 1040", "Schedule B").
    fn name() -> &'static str;

    /// Tax year this form applies to.
    fn year(&self) -> TaxYear;

    /// Whether this is an input, output, or election form.
    fn form_type() -> FormType;
}
