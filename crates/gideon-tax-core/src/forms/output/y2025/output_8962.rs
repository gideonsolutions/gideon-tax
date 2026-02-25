use crate::Usd;

/// Output fields for IRS Form 8962 (2025) — Premium Tax Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8962 {
    // -----------------------------------------------------------------------
    // Part I — Annual and Monthly Contribution Amount
    // -----------------------------------------------------------------------
    /// Line 1: Tax family size. Enter your tax family size
    pub family_size_cnt: u32,
    /// Line 2a: Modified AGI. Enter your modified AGI
    pub modified_agi_amt: Usd,
    /// Line 2b: Enter the total of your dependents' modified AGI
    pub total_dependents_modified_agi_amt: Usd,
    /// Line 3: Household income. Add the amounts on lines 2a and 2b
    pub household_income_amt: Usd,
    /// Line 4: Federal poverty line amount from Table 1-1, 1-2, or 1-3
    pub poverty_level_amt: Usd,
    /// Line 4: Federal poverty table location code (a = Alaska, b = Hawaii, c = Other 48 states and DC)
    pub federal_poverty_table_loc_cd: String,
    /// Line 5: Household income as a percentage of federal poverty line
    pub federal_poverty_level_pct: String,
    /// Line 7: Applicable figure. Using your line 5 percentage, locate your "applicable figure"
    pub applicable_figure_rt: String,
    /// Line 8a: Annual contribution amount. Multiply line 3 by line 7. Round to nearest whole dollar amount
    pub annual_contribution_amt: Usd,
    /// Line 8b: Monthly contribution amount. Divide line 8a by 12. Round to nearest whole dollar amount
    pub monthly_contribution_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Premium Tax Credit Claim and Reconciliation of Advance Payment of Premium Tax Credit
    // -----------------------------------------------------------------------
    /// Line 9: Are you allocating policy amounts with another taxpayer or using alternative calculation? (Yes indicator)
    pub share_policy_married_alt_calc_ind: bool,
    /// Line 10: See the instructions to determine if you can use line 11 or must complete lines 12 through 23
    /// (shared policy allocation info indicator)
    pub shared_policy_allocation_info_ind: bool,
    /// Line A: Check the box if you cannot take the PTC because filing status is married filing separately
    pub married_filing_separately_exc_ind: bool,
    /// Full-year coverage on Form 1095-A indicator
    pub full_yr_coverage1095_a_ind: bool,
    /// QSEHRA indicator
    pub qsehra_ind: bool,
    /// Start month number code for monthly calculation
    pub start_month_number_cd: String,
    /// End month number code for monthly calculation
    pub end_month_number_cd: String,
    /// Line 11(a): Annual enrollment premiums (Form(s) 1095-A, line 33A)
    pub annual_premium_amt: Usd,
    /// Line 11(b): Annual applicable SLCSP premium (Form(s) 1095-A, line 33B)
    pub annual_premium_slcsp_amt: Usd,
    /// Line 11(c): Annual contribution amount (line 8a)
    pub annual_max_premium_assistance_amt: Usd,
    /// Line 11(e): Annual PTC allowed (smaller of (a) or (d))
    pub annual_premium_tax_credit_allw_amt: Usd,
    /// Line 11(f): Annual advance payment of PTC (Form(s) 1095-A, line 33C)
    pub annual_advanced_ptc_amt: Usd,
    /// Lines 12-23: Monthly contribution for health care coverage amount
    pub monthly_contri_health_care_cvr_amt: Usd,
    /// Line 24: Total PTC. Enter the amount from line 11(e), or add lines 12 through 23 column (e)
    pub total_premium_tax_credit_amt: Usd,
    /// Line 25: Advance payment of PTC. Enter the amount from line 11(f), or add lines 12 through 23 column (f)
    pub total_advanced_ptc_amt: Usd,
    /// Line 26: Net PTC. If line 24 is greater than line 25, subtract line 25 from line 24
    pub reconciled_premium_tax_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Repayment of Excess Advance Payment of the Premium Tax Credit
    // -----------------------------------------------------------------------
    /// Line 27: Excess advance payment of PTC. If line 25 is greater than line 24, subtract line 24 from line 25
    pub excess_advnc_payment_amt: Usd,
    /// Line 28: Repayment limitation (see instructions)
    pub additional_tax_limitation_amt: Usd,
    /// Line 29: Excess advance PTC repayment. Enter the smaller of line 27 or line 28
    pub premium_tax_credit_tax_liab_amt: Usd,

    // -----------------------------------------------------------------------
    // Exemptions
    // -----------------------------------------------------------------------
    /// Total exemptions count
    pub total_exemptions_cnt: u32,
}
