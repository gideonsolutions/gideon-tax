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

    // ── Output forms ──
    F1040,
    F1040Nr,
    F1040Sr,
    Schedule1,
    Schedule1A,
    Schedule2,
    Schedule3,
    ScheduleA,
    ScheduleB,
    ScheduleC,
    ScheduleD,
    ScheduleE,
    ScheduleF,
    ScheduleH,
    ScheduleJ,
    ScheduleR,
    ScheduleSe,
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
            | Self::F1099R => FormType::Input,

            Self::F1040
            | Self::F1040Nr
            | Self::F1040Sr
            | Self::Schedule1
            | Self::Schedule1A
            | Self::Schedule2
            | Self::Schedule3
            | Self::ScheduleA
            | Self::ScheduleB
            | Self::ScheduleC
            | Self::ScheduleD
            | Self::ScheduleE
            | Self::ScheduleF
            | Self::ScheduleH
            | Self::ScheduleJ
            | Self::ScheduleR
            | Self::ScheduleSe => FormType::Output,
        }
    }
}
