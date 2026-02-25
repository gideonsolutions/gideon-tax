use crate::Usd;

/// Input fields for IRS Form 1098-T (2025) — Tuition Statement.
///
/// Field names are **NOT** derived from the IRS MeF (Modernized e-File) schema.
/// They are Claude-authored from the paper/PDF form and use descriptive names
/// with the same suffix conventions as the rest of the crate (`_amt`, `_ind`,
/// `_dt`, `_txt`, `_nm`).
#[derive(Debug, Clone, Default)]
pub struct Input1098T {
    // -----------------------------------------------------------------------
    // Header — Filer (eligible educational institution) information
    // -----------------------------------------------------------------------
    /// Filer's name
    pub filer_nm: String,
    /// Filer's street address (including apt. no.)
    pub filer_address_txt: String,
    /// Filer's city, state, and ZIP code
    pub filer_city_state_zip_txt: String,
    /// Filer's telephone number
    pub filer_phone_txt: String,
    /// FILER'S TIN
    pub filer_tin: String,

    // -----------------------------------------------------------------------
    // Header — Student information
    // -----------------------------------------------------------------------
    /// STUDENT'S TIN
    pub student_tin: String,
    /// Student's name
    pub student_nm: String,
    /// Student's street address (including apt. no.)
    pub student_address_txt: String,
    /// Student's city, state, and ZIP code
    pub student_city_state_zip_txt: String,
    /// Account number (optional)
    pub account_num: String,
    /// Service provider/acct. no.
    pub service_provider_num: String,

    // -----------------------------------------------------------------------
    // Boxes
    // -----------------------------------------------------------------------
    /// Box 1 — Payments received for qualified tuition and related expenses
    pub qualified_tuition_payments_amt: Usd,
    // Box 2 — Reserved
    // Box 3 — Reserved
    /// Box 4 — Adjustments made for a prior year
    pub prior_year_adjustments_amt: Usd,
    /// Box 5 — Scholarships or grants
    pub scholarships_or_grants_amt: Usd,
    /// Box 6 — Adjustments to scholarships or grants for a prior year
    pub prior_year_scholarship_adjustments_amt: Usd,
    /// Box 7 — Checked if the amount in Box 1 includes amounts for an
    /// academic period beginning January–March of the next year
    pub next_year_academic_period_ind: bool,
    /// Box 8 — Checked if at least half-time student
    pub at_least_half_time_ind: bool,
    /// Box 9 — Checked if graduate student
    pub graduate_student_ind: bool,
    /// Box 10 — Insurance contract reimbursements or refunds
    pub insurance_reimbursements_amt: Usd,
}
