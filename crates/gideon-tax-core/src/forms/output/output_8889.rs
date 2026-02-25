use crate::Usd;

/// Output fields for IRS Form 8889 (2025) — Health Savings Accounts.
#[derive(Debug, Clone, Default)]
pub struct Output8889 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of HSA beneficiary
    pub person_nm: String,
    /// Social security number of HSA beneficiary
    pub recipient_ssn: String,

    // -----------------------------------------------------------------------
    // Part I — HSA Contributions and Deduction
    // -----------------------------------------------------------------------
    /// Line 1: Check the box — Self-only coverage indicator
    pub hdhp_self_only_coverage_ind: bool,
    /// Line 1: Check the box — Family coverage indicator
    pub hdhp_family_coverage_ind: bool,
    /// Line 2: HSA contributions you made for 2025 (not including employer contributions)
    pub hsa_contribution_amt: Usd,
    /// Line 3: HSA deduction limit based on coverage type and age
    pub hsa_limited_annual_deductible_amt: Usd,
    /// Line 4: Employer contributions to your Archer MSAs for 2025
    pub total_archer_msa_contribution_amt: Usd,
    /// Line 5: Subtract line 4 from line 3 (if zero or less, enter -0-)
    pub hsa_limited_deductible_allwd_amt: Usd,
    /// Line 6: HSA deduction amount for separate HSAs (see instructions)
    pub hsa_limited_contribution_amt: Usd,
    /// Line 7: Additional contribution amount if age 55 or older
    pub hsa_addnl_contribution_amt: Usd,
    /// Line 8: Add lines 6 and 7
    pub hsa_limited_gross_contribution_amt: Usd,
    /// Line 9: Employer contributions made to your HSAs for 2025
    pub hsa_employer_contribution_amt: Usd,
    /// Line 10: Qualified HSA funding distributions
    pub hsa_qualified_funding_distri_amt: Usd,
    /// Line 11: Add lines 9 and 10
    pub total_hsa_contribution_amt: Usd,
    /// Line 12: Subtract line 11 from line 8 (if zero or less, enter -0-)
    pub hsa_family_deductible_amt: Usd,
    /// Line 13: HSA deduction — smaller of line 2 or line 12
    pub total_hsa_deduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — HSA Distributions
    // -----------------------------------------------------------------------
    /// Line 14a: Total distributions you received in 2025 from all HSAs
    pub total_hsa_distribution_amt: Usd,
    /// Line 14b: Rollover and excess contribution distributions included on line 14a
    pub hsa_distribution_rollover_amt: Usd,
    /// Line 14c: Subtract line 14b from line 14a
    pub hsa_net_distribution_amt: Usd,
    /// Line 15: Qualified medical expenses paid using HSA distributions
    pub unreimb_qual_med_and_dental_exp_amt: Usd,
    /// Line 16: Taxable HSA distributions (subtract line 15 from line 14c; if zero or less, enter -0-)
    pub taxable_hsa_distribution_amt: Usd,
    /// Line 17a: Exception to the Additional 20% Tax indicator
    pub hsa_distri_addnl_percent_tax_exc_ind: bool,
    /// Line 17b: Additional 20% tax on taxable distributions
    pub hsa_distri_addnl_percent_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Income and Additional Tax for Failure To Maintain HDHP Coverage
    // -----------------------------------------------------------------------
    /// Line 18: Last-month rule income amount
    pub hdhp_coverage_fail_partial_yr_amt: Usd,
    /// Line 19: Qualified HSA funding distribution income
    pub hdhp_coverage_fail_fund_distri_amt: Usd,
    /// Line 20: Total income (add lines 18 and 19)
    pub hdhp_coverage_income_amt: Usd,
    /// Line 21: Additional tax (multiply line 20 by 10%)
    pub hdhp_coverage_addnl_tax_amt: Usd,
}
