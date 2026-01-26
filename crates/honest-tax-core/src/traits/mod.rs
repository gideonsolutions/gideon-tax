//! Core traits for the tax calculation engine.

mod input;
mod output;
mod rules;

pub use input::{InputForm, InputFormCollection};
pub use output::{FormLine, FormSchema, FormValue, FormValueType, FormLineSpec, OutputForm};
pub use rules::{PhaseOut, SeniorBonusDeduction, TaxBracket, TaxRules};
