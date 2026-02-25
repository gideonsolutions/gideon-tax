use super::super::form::{Form, FormType};

/// A [`Form`] whose `form_type()` is [`FormType::Input`].
///
/// Represents a source document (W-2, 1099, etc.) used as input
/// to the tax computation.
pub trait InputForm: Form {
    /// Always returns [`FormType::Input`].
    fn form_type() -> FormType {
        FormType::Input
    }
}
