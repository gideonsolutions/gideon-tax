use super::form::FormType;

/// Enumerates every known tax form (input and output).
///
/// Used in [`OutputForm::dependencies`](super::output::OutputForm::dependencies)
/// so that each output form can declare which forms it depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynForm {
    // ── Input forms ──
    W2,
    W2G,
    F1098,
    F1098C,
    F1098E,
    F1098T,
    F1099B,
    F1099Div,
    F1099G,
    F1099Int,
    F1099K,
    F1099Misc,
    F1099Nec,
    F1099Oid,
    F1099Patr,
    F1099R,
    F1041ScheduleK1,
    F1065ScheduleK1,
    F1120SScheduleK1,

    // ── Output forms: 1040 series ──
    F1040,
    F1040Nr,
    F1040Sr,
    Schedule1,
    Schedule1A,
    Schedule2,
    Schedule3,
    Schedule8812,
    ScheduleA,
    ScheduleB,
    ScheduleC,
    ScheduleD,
    ScheduleE,
    ScheduleEic,
    ScheduleF,
    ScheduleH,
    ScheduleJ,
    ScheduleNec,
    ScheduleOi,
    ScheduleR,
    ScheduleSe,
    // ── Output forms: numbered forms ──
    F1042S,
    F1116,
    F2106,
    F2439,
    F2441,
    F2555,
    F3800,
    F3903,
    F4136,
    F4137,
    F4255,
    F4562,
    F4563,
    F4684,
    F4797,
    F4835,
    F4952,
    F4972,
    F5329,
    F5695,
    F6198,
    F6251,
    F6252,
    F6781,
    F8288A,
    F8396,
    F8582,
    F8611,
    F8621,
    F8689,
    F8697,
    F8801,
    F8805,
    F8814,
    F8815,
    F8824,
    F8829,
    F8834,
    F8839,
    F8853,
    F8859,
    F8863,
    F8866,
    F8880,
    F8888,
    F8889,
    F8912,
    F8919,
    F8936,
    F8936ScheduleA,
    F8949,
    F8959,
    F8960,
    F8962,
    F8978,
    F8995,
    F8995A,
    F965A,
}

impl DynForm {
    /// Whether this form is an input or output form.
    pub const fn form_type(self) -> FormType {
        match self {
            Self::W2
            | Self::W2G
            | Self::F1099B
            | Self::F1099Div
            | Self::F1099G
            | Self::F1099Int
            | Self::F1099K
            | Self::F1099Misc
            | Self::F1099Nec
            | Self::F1099Oid
            | Self::F1099Patr
            | Self::F1099R
            | Self::F1098
            | Self::F1098C
            | Self::F1098E
            | Self::F1098T
            | Self::F1041ScheduleK1
            | Self::F1065ScheduleK1
            | Self::F1120SScheduleK1 => FormType::Input,

            _ => FormType::Output,
        }
    }
}
