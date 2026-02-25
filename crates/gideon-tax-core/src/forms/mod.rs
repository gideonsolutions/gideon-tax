mod dyn_form;
mod form;
pub mod input;
pub mod output;

pub use dyn_form::DynForm;
pub use form::{Form, FormType};
pub use input::{
    Input1041ScheduleK1, Input1065ScheduleK1, Input1098, Input1098C, Input1098E, Input1098T,
    Input1099B, Input1099Div, Input1099G, Input1099Int, Input1099K, Input1099Misc, Input1099Nec,
    Input1099Oid, Input1099Patr, Input1099R, Input1120SScheduleK1, InputForm, InputW2, InputW2G,
};
pub use output::{
    F4137Input, F4137Line1, Output965A, Output1040, Output1040Nr, Output1040Sr, Output1042S,
    Output1116, Output2106, Output2439, Output2441, Output2555, Output3800, Output3903, Output4136,
    Output4137, Output4255, Output4562, Output4563, Output4684, Output4797, Output4835, Output4952,
    Output4972, Output5329, Output5695, Output6198, Output6251, Output6252, Output6781,
    Output8288A, Output8396, Output8582, Output8611, Output8621, Output8689, Output8697,
    Output8801, Output8805, Output8814, Output8815, Output8824, Output8829, Output8834, Output8839,
    Output8853, Output8859, Output8863, Output8866, Output8880, Output8888, Output8889, Output8912,
    Output8919, Output8936, Output8936ScheduleA, Output8949, Output8959, Output8960, Output8962,
    Output8978, Output8995, Output8995A, OutputForm, OutputSchedule1, OutputSchedule1A,
    OutputSchedule2, OutputSchedule3, OutputSchedule8812, OutputScheduleA, OutputScheduleB,
    OutputScheduleC, OutputScheduleD, OutputScheduleE, OutputScheduleEic, OutputScheduleF,
    OutputScheduleH, OutputScheduleJ, OutputScheduleNec, OutputScheduleOi, OutputScheduleR,
    OutputScheduleSe,
};
