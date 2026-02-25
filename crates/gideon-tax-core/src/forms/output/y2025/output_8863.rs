/// Output fields for IRS Form 8863 (2025) — Education Credits.
#[derive(Debug, Clone, Default)]
pub struct Output8863 {
    // -----------------------------------------------------------------------
    // Part I — Refundable American Opportunity Credit
    // -----------------------------------------------------------------------
    /// Part I: Refundable American opportunity credit group (Lines 1-8)
    pub refundable_amer_opp_credit_group: String,

    // -----------------------------------------------------------------------
    // Part II — Nonrefundable Education Credits
    // -----------------------------------------------------------------------
    /// Part II: Nonrefundable education credit group (Lines 9-19)
    pub nonrefundable_education_cr_group: String,

    // -----------------------------------------------------------------------
    // Part III — Student and Educational Institution Information
    // -----------------------------------------------------------------------
    /// Part III: American opportunity credit group (Lines 20-30)
    pub american_opportunity_credit_group: String,
    /// Part III: Lifetime learning credit group (Line 31)
    pub lifetime_learning_credit_group: String,
}
