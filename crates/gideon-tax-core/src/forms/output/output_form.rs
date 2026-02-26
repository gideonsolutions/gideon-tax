use crate::GideonTaxError;

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

    /// Returns `true` if the IRS requires this form for the given input
    /// (e.g., Form 4137 is required when the filer has unreported tip
    /// income). This is a lightweight pre-check; [`try_new`](Self::try_new)
    /// may still succeed even when this returns `false`.
    fn must_file(input: &Self::Input) -> bool;

    /// Construct the form from its input. Returns `Err` if the input
    /// violates a constraint.
    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError>;

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
