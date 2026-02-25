use crate::Usd;

/// Output fields for IRS Form 8839 (2025) — Qualified Adoption Expenses.
#[derive(Debug, Clone, Default)]
pub struct Output8839 {
    // -----------------------------------------------------------------------
    // Part I — Information About Your Eligible Child or Children
    // -----------------------------------------------------------------------
    /// Line 1: Child information (name, year of birth, indicators, identifying number)
    pub adopted_child: String,

    // -----------------------------------------------------------------------
    // Part II — Adoption Credit
    // -----------------------------------------------------------------------
    /// Line 6: Enter the smaller of line 4 or line 5
    pub calculated_adoption_credit_amt: Usd,
    /// Line 7: Enter modified adjusted gross income
    pub adoption_credit_modified_agi_amt: Usd,
    /// Line 8: Is line 7 more than $259,190?
    pub adoption_cr_modif_agi_grtr_amt_ind: bool,
    /// Line 8 (Yes): Subtract $259,190 from line 7
    pub adoption_credit_modif_agi_limit_amt: Usd,
    /// Line 9: Divide line 8 by $40,000 (decimal)
    pub adoption_credit_adj_modif_agi_pct: String,
    /// Line 11b: Enter the smaller of the amount on line 11a or $5,000
    pub net_calculated_adoption_cr_adj_amt: Usd,
    /// Line 11c: Add the amounts on line 11b
    pub net_calculated_adoption_credit_amt: Usd,
    /// Line 12: Add the amounts on line 11a
    pub refundable_adptn_cr_cfwd_excl_amt: Usd,
    /// Line 13: Refundable adoption credit
    pub refundable_adoption_credit_amt: Usd,
    /// Line 14: Subtract line 13 from line 12. If zero or less, enter -0-
    pub net_adoption_credit_excl_cfwd_amt: Usd,
    /// Line 15: Credit carryforward, if any, from prior years
    pub adoption_credit_cfwd_amt: Usd,
    /// Line 16: Add lines 14 and 15
    pub net_adoption_credit_cfwd_amt: Usd,
    /// Line 17: Enter the amount from line 5 of the Credit Limit Worksheet in the instructions
    pub credit_limit_worksheet_amt: Usd,
    /// Line 18: Nonrefundable adoption credit. Enter the smaller of line 16 or line 17
    pub nonrefundable_adoption_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Employer-Provided Adoption Benefits
    // -----------------------------------------------------------------------
    /// Line 23: Add the amounts on line 22
    pub employer_adoption_benefits_amt: Usd,
    /// Line 24: Enter the smaller of line 21 or line 22
    pub excluded_benefits_amt: Usd,
    /// Line 25: Enter modified adjusted gross income (from the worksheet in the instructions)
    pub adoption_benefits_modified_agi_amt: Usd,
    /// Line 26: Is line 25 more than $259,190?
    pub adoption_bnft_modif_agi_grtr_amt_ind: bool,
    /// Line 26 (Yes): Subtract $259,190 from line 25
    pub adoption_bnft_modif_agi_less_lmt_amt: Usd,
    /// Line 27: Divide line 26 by $40,000 (decimal)
    pub adoption_benefit_adj_modif_agi_pct: String,
    /// Line 29: Excluded benefits. Subtract line 28 from line 24
    pub adoption_bnft_agi_pct_expns_amt: Usd,
    /// Line 30: Add the amounts on line 29
    pub total_excluded_benefits_amt: Usd,
    /// Line 31: Taxable benefits. Is line 30 more than line 23?
    pub taxable_benefits_form8839_amt: Usd,
    /// Indicator: Exclude more than employer adoption benefits
    pub excld_more_than_emplr_adptn_bnft_ind: bool,
}
