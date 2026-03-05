use super::super::form::Form;

/// A [`Form`] whose `form_type()` is [`FormType::Election`].
///
/// Represents an IRS election or application form (e.g., Form 4029,
/// Form 4361) that a taxpayer files once to elect into or out of a
/// particular tax treatment. Election forms are data-collection
/// forms with no computed fields.
pub trait ElectionForm: Form + Sized {}
