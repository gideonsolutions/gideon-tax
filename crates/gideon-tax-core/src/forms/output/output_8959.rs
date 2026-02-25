use crate::Usd;

/// Output fields for IRS Form 8959 (2025) — Additional Medicare Tax.
#[derive(Debug, Clone, Default)]
pub struct Output8959 {
    // -----------------------------------------------------------------------
    // Part I — Additional Medicare Tax on Medicare Wages
    // -----------------------------------------------------------------------
    /// Line 1: Medicare wages and tips from Form W-2, box 5
    pub total_w2_medicare_wages_and_tips_amt: Usd,
    /// Line 2: Unreported tips from Form 4137, line 6
    pub total_unreported_medicare_tips_amt: Usd,
    /// Line 3: Wages from Form 8919, line 6
    pub total_wages_with_no_withholding_amt: Usd,
    /// Line 4: Add lines 1 through 3
    pub total_medicare_wages_and_tips_amt: Usd,
    /// Line 5: Filing status threshold amount
    pub filing_status_threshold_cd: String,
    /// Line 6: Subtract line 5 from line 4 (if zero or less, enter -0-)
    pub wages_tips_subj_to_addl_medcr_tax_amt: Usd,
    /// Line 7: Additional Medicare Tax on Medicare wages (multiply line 6 by 0.9%)
    pub additional_medicare_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Additional Medicare Tax on Self-Employment Income
    // -----------------------------------------------------------------------
    /// Line 8: Self-employment income from Schedule SE, Part I, line 6
    pub total_self_employment_income_amt: Usd,
    /// Line 9: Filing status threshold amount for self-employment
    /// (reduced by Medicare wages, tips, and RRTA compensation on line 4)
    pub medcr_wages_tips_below_thrshld_amt: Usd,
    /// Line 10: Amount from line 4
    /// (used to reduce the threshold for self-employment income)
    pub se_income_subj_to_add_se_tax_amt: Usd,
    /// Line 13: Additional Medicare Tax on self-employment income (multiply line 12 by 0.9%)
    pub addl_self_employment_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Additional Medicare Tax on Railroad Retirement Tax Act (RRTA) Compensation
    // -----------------------------------------------------------------------
    /// Line 14: Railroad retirement (RRTA) compensation and tips from Form W-2, box 14
    pub total_railroad_retirement_comp_amt: Usd,
    /// Line 16: Subtract line 15 from line 14 (if zero or less, enter -0-)
    pub rrt_comp_subj_to_add_rrt_tax_amt: Usd,
    /// Line 17: Additional Medicare Tax on RRTA compensation (multiply line 16 by 0.9%)
    pub addl_railroad_retirement_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Total Additional Medicare Tax
    // -----------------------------------------------------------------------
    /// Line 18: Add lines 7, 13, and 17 (total Additional Medicare Tax)
    pub total_amrrt_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Withholding Reconciliation
    // -----------------------------------------------------------------------
    /// Line 19: Medicare tax withheld from Form W-2, box 6
    pub total_w2_medicare_tax_withheld_amt: Usd,
    /// Line 20: Amount from line 1 (Medicare wages and tips)
    pub total_medicare_tax_amt: Usd,
    /// Line 21: Multiply line 20 by 1.45% (regular Medicare tax withholding)
    pub addnl_medicare_tax_withholding_amt: Usd,
    /// Line 22: Subtract line 21 from line 19 (Additional Medicare Tax withholding on Medicare wages)
    pub addl_medcr_rrt_tax_withholding_amt: Usd,
    /// Line 23: Additional Medicare Tax withholding on RRTA compensation from Form W-2, box 14
    pub total_w2_addl_rrt_tax_amt: Usd,
    // Line 24: Total Additional Medicare Tax withholding (add lines 22 and 23)
    // Include on Form 1040, 1040-SR, or 1040-NR, line 25c
}
