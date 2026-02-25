mod dyn_form;
mod form;
mod input_form;
pub mod output;

pub use dyn_form::DynForm;
pub use form::{Form, FormType};
pub use input_form::InputForm;
pub use output::{
    Output1040, Output1040Nr, Output1040Sr, OutputForm, OutputSchedule1, OutputSchedule1A,
    OutputSchedule2, OutputSchedule3, OutputScheduleA, OutputScheduleB, OutputScheduleC,
    OutputScheduleD, OutputScheduleE, OutputScheduleF, OutputScheduleH, OutputScheduleJ,
    OutputScheduleR, OutputScheduleSe,
};
