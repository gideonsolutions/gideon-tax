use crate::Usd;

/// Output fields for IRS Form 8978 (2025) — Partner's Additional Reporting Year Tax.
#[derive(Debug, Clone, Default)]
pub struct Output8978 {
    // -----------------------------------------------------------------------
    // Header — Source of review year adjustments
    // -----------------------------------------------------------------------
    /// Source of review year adjustments: BBA Audit checkbox
    pub bba_audit_ind: bool,
    /// Source of review year adjustments: AAR Filing checkbox
    pub aar_filing_ind: bool,
    /// Tax year ended date
    pub tax_year_end_dt: String,

    // -----------------------------------------------------------------------
    // Part I — Computation of Additional Reporting Year Tax
    // -----------------------------------------------------------------------
    /// Line 1a: Total income per original return or as previously adjusted
    pub total_income_originally_rpt_amt: Usd,
    /// Line 1b: Adjustments to income from Schedule A (Form 8978), line 2, columns (a) through (d)
    pub total_adjustments_to_income_amt: Usd,
    /// Line 2: Combine lines 1a and 1b and enter the corrected income
    pub total_income_correct_amt: Usd,
    /// Line 3a: Total deductions per original return or as previously adjusted
    pub total_deduction_originally_rpt_amt: Usd,
    /// Line 3b: Adjustments to deductions from Schedule A (Form 8978), line 4, columns (a) through (d)
    pub total_deduction_net_change_amt: Usd,
    /// Line 4: Combine lines 3a and 3b and enter the corrected deductions
    pub total_deduction_correct_amt: Usd,
    /// Line 5: Corrected taxable income. Subtract line 4 from line 2
    pub taxable_income_correct_amt: Usd,
    /// Line 6: Income tax on line 5 (see instructions)
    pub income_tax_amt: Usd,
    /// Line 7: Alternative minimum tax on line 5 (see instructions)
    pub alternative_minimum_tax_amt: Usd,
    /// Line 8: Total corrected income tax. Add lines 6 and 7
    pub total_tax_correct_amt: Usd,
    /// Line 9a: Total credits per original return or as previously adjusted
    pub total_credit_originally_rpt_amt: Usd,
    /// Line 9b: Adjustments to credits from Schedule A (Form 8978), line 6, columns (a) through (d)
    pub total_adjustments_to_credit_amt: Usd,
    /// Line 10: Combine lines 9a and 9b and enter the corrected credits
    pub total_credits_correct_amt: Usd,
    /// Line 11: Total corrected income tax liability. Subtract line 10 from line 8
    pub total_corr_incm_tax_liab_after_cr_amt: Usd,
    /// Line 12: Total income tax shown on original return or as previously adjusted
    pub total_tax_originally_rpt_amt: Usd,
    /// Line 13: Increase/Decrease to tax. Subtract line 12 from line 11, columns (a) through (d)
    pub tax_increase_decrease_amt: Usd,
    /// Line 14: Total increase/decrease to reporting year tax. Add line 13, columns (a) through (d)
    pub tot_rptg_yr_tx_increase_decrease_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Penalties
    // -----------------------------------------------------------------------
    /// Line 15: Penalties
    pub penalty_amt: Usd,
    /// Line 16: Total penalties. Add line 15, columns (a) through (d)
    pub total_penalty_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Interest
    // -----------------------------------------------------------------------
    /// Line 17: Interest
    pub interest_amt: Usd,
    /// Line 18: Total interest. Add line 17, columns (a) through (d)
    pub total_interest_amt: Usd,
}
