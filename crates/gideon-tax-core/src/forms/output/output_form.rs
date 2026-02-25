use super::super::dyn_form::DynForm;
use super::super::form::{Form, FormType};

/// A [`Form`] whose `form_type()` is [`FormType::Output`].
///
/// Represents an IRS return form (1040, Schedule B, etc.) produced by the
/// tax computation.
pub trait OutputForm: Form + Sized {
    /// Per-form input struct containing all info needed to complete the form
    /// without redundancy. Any instance of Input is valid.
    type Input;

    /// Construct the form from its input. Returns `None` if the form is
    /// unnecessary for this return (e.g., Schedule B when interest < $1,500).
    fn new(input: Self::Input) -> Option<Self>;

    /// Always returns [`FormType::Output`].
    fn form_type() -> FormType {
        FormType::Output
    }

    /// Other forms this form depends on.
    fn dependencies() -> &'static [DynForm];

    /// Returns `true` if computed fields are internally consistent
    /// (e.g., line 9 == sum of component lines).
    fn is_valid(&self) -> bool;
}
