use crate::Usd;

/// Output fields for IRS Form 5329 (2025) — Additional Taxes on Qualified Plans and Other Tax-Favored Accounts.
#[derive(Debug, Clone, Default)]
pub struct Output5329 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of individual subject to additional tax
    pub person_nm: String,
    /// Your social security number
    pub ssn: String,
    /// If this is an amended return, check here
    pub amended_return_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Additional Tax on Early Distributions
    // -----------------------------------------------------------------------
    /// Line 1: Early distributions includible in income
    pub early_distributions_amt: Usd,
    /// Line 2: Early distributions included on line 1 that are not subject to the additional tax.
    /// Enter the appropriate exception number from the instructions
    pub early_distri_not_subject_to_tax_amt: Usd,
    /// Line 2: Exception reason code
    pub early_distri_exception_reason_cd: String,
    /// Line 3: Amount subject to additional tax. Subtract line 2 from line 1
    pub early_distri_subject_to_tax_amt: Usd,
    /// Line 4: Additional tax. Enter 10% (0.10) of line 3
    pub ira_early_distributions_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Additional Tax on Certain Distributions From Education Accounts and ABLE Accounts
    // -----------------------------------------------------------------------
    /// Line 5: Distributions included in income from a Coverdell ESA, a QTP, or an ABLE account
    pub educ_acct_distribution_amt: Usd,
    /// Line 6: Distributions included on line 5 that are not subject to the additional tax
    pub educ_acct_distri_not_subj_to_tax_amt: Usd,
    /// Line 7: Amount subject to additional tax. Subtract line 6 from line 5
    pub educ_acct_distri_subject_to_tax_amt: Usd,
    /// Line 8: Additional tax. Enter 10% (0.10) of line 7
    pub educ_ira_distributions_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Additional Tax on Excess Contributions to Traditional IRAs
    // -----------------------------------------------------------------------
    /// Line 9: Enter your excess contributions from line 16 of your 2024 Form 5329. If zero, go to line 15
    pub ira_excess_contri_prior_year_amt: Usd,
    /// Line 10: If your traditional IRA contributions for 2025 are less than your maximum allowable
    /// contribution, enter the difference. Otherwise, enter -0-
    pub ira_excess_contri_current_year_amt: Usd,
    /// Line 11: 2025 traditional IRA distributions included in income
    pub ira_distri_included_in_income_amt: Usd,
    /// Line 12: 2025 distributions of prior year excess contributions to traditional IRAs
    pub ira_excess_contri_withdrawn_amt: Usd,
    /// Line 13: Add lines 10, 11, and 12
    pub ira_excess_contri_adjustment_amt: Usd,
    /// Line 14: Prior year excess contributions. Subtract line 13 from line 9. If zero or less, enter -0-
    pub ira_excess_contri_pr_yr_adjust_amt: Usd,
    /// Line 15: Excess contributions for 2025
    pub ira_excess_contri_credit_amt: Usd,
    /// Line 16: Total excess contributions. Add lines 14 and 15
    pub ira_excess_contri_total_amt: Usd,
    /// Line 17: Additional tax. Enter 6% (0.06) of the smaller of line 16 or the value of your
    /// traditional IRAs on December 31, 2025
    pub ira_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Additional Tax on Excess Contributions to Roth IRAs
    // -----------------------------------------------------------------------
    /// Line 18: Enter your excess contributions from line 24 of your 2024 Form 5329. If zero, go to line 23
    pub roth_ira_excess_contri_prior_yr_amt: Usd,
    /// Line 19: If your Roth IRA contributions for 2025 are less than your maximum allowable
    /// contribution, enter the difference. Otherwise, enter -0-
    pub roth_ira_excess_contri_cy_amt: Usd,
    /// Line 20: 2025 distributions from your Roth IRAs
    pub roth_ira_distri_included_in_cy_amt: Usd,
    /// Line 21: Add lines 19 and 20
    pub roth_ira_excess_contri_adjust_amt: Usd,
    /// Line 22: Prior year excess contributions. Subtract line 21 from line 18. If zero or less, enter -0-
    pub roth_ira_excess_contri_py_wthdrw_amt: Usd,
    /// Line 23: Excess contributions for 2025
    pub roth_ira_excess_contri_credit_amt: Usd,
    /// Line 24: Total excess contributions. Add lines 22 and 23
    pub roth_ira_excess_contri_total_amt: Usd,
    /// Line 25: Additional tax. Enter 6% (0.06) of the smaller of line 24 or the value of your
    /// Roth IRAs on December 31, 2025
    pub roth_ira_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Additional Tax on Excess Contributions to Coverdell ESAs
    // -----------------------------------------------------------------------
    /// Line 26: Enter the excess contributions from line 32 of your 2024 Form 5329. If zero, go to line 31
    pub esa_excess_contri_prior_year_amt: Usd,
    /// Line 27: If the contributions to your Coverdell ESAs for 2025 were less than the maximum
    /// allowable contribution, enter the difference. Otherwise, enter -0-
    pub esa_excess_contri_cy_amt: Usd,
    /// Line 28: 2025 distributions from your Coverdell ESAs
    pub esa_distri_included_in_cy_amt: Usd,
    /// Line 29: Add lines 27 and 28
    pub esa_excess_contri_adjustment_amt: Usd,
    /// Line 30: Prior year excess contributions. Subtract line 29 from line 26. If zero or less, enter -0-
    pub esa_excess_contri_py_wthdrw_amt: Usd,
    /// Line 31: Excess contributions for 2025
    pub esa_excess_contri_credit_amt: Usd,
    /// Line 32: Total excess contributions. Add lines 30 and 31
    pub esa_excess_contri_total_amt: Usd,
    /// Line 33: Additional tax. Enter 6% (0.06) of the smaller of line 32 or the value of your
    /// Coverdell ESAs on December 31, 2025
    pub educ_ira_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VI — Additional Tax on Excess Contributions to Archer MSAs
    // -----------------------------------------------------------------------
    /// Line 34: Enter the excess contributions from line 40 of your 2024 Form 5329. If zero, go to line 39
    pub archer_msa_excess_contri_pr_yr_amt: Usd,
    /// Line 35: If the contributions to your Archer MSAs for 2025 are less than the maximum
    /// allowable contribution, enter the difference. Otherwise, enter -0-
    pub archer_msa_excess_contri_cy_amt: Usd,
    /// Line 36: 2025 distributions from your Archer MSAs from Form 8853, line 8
    pub archer_msa_excess_contri_adj_amt: Usd,
    /// Line 37: Add lines 35 and 36
    pub archer_msa_excess_contri_credit_amt: Usd,
    /// Line 38: Prior year excess contributions. Subtract line 37 from line 34. If zero or less, enter -0-
    pub archer_msa_ex_contri_py_wthdrw_amt: Usd,
    /// Line 39: Excess contributions for 2025
    pub taxable_archer_msa_distri_amt: Usd,
    /// Line 40: Total excess contributions. Add lines 38 and 39
    pub archer_msa_excess_contri_total_amt: Usd,
    /// Line 41: Additional tax. Enter 6% (0.06) of the smaller of line 40 or the value of your
    /// Archer MSAs on December 31, 2025
    pub msa_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VII — Additional Tax on Excess Contributions to Health Savings Accounts (HSAs)
    // -----------------------------------------------------------------------
    /// Line 42: Enter the excess contributions from line 48 of your 2024 Form 5329. If zero, go to line 47
    pub hsa_excess_contri_prior_year_amt: Usd,
    /// Line 43: If the contributions to your HSAs for 2025 are less than the maximum allowable
    /// contribution, enter the difference. Otherwise, enter -0-
    pub hsa_excess_contri_current_year_amt: Usd,
    /// Line 44: 2025 distributions from your HSAs from Form 8889, line 16
    pub hsa_excess_contri_py_adjusted_amt: Usd,
    /// Line 45: Add lines 43 and 44
    pub hsa_excess_contri_adjustment_amt: Usd,
    /// Line 46: Prior year excess contributions. Subtract line 45 from line 42. If zero or less, enter -0-
    pub hsa_excess_contri_credit_amt: Usd,
    /// Line 47: Excess contributions for 2025
    pub taxable_hsa_distribution_amt: Usd,
    /// Line 48: Total excess contributions. Add lines 46 and 47
    pub hsa_excess_contri_total_amt: Usd,
    /// Line 49: Additional tax. Enter 6% (0.06) of the smaller of line 48 or the value of your
    /// HSAs on December 31, 2025
    pub hsa_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VIII — Additional Tax on Excess Contributions to an ABLE Account
    // -----------------------------------------------------------------------
    /// Line 50: Excess contributions for 2025
    pub able_excess_contri_cy_amt: Usd,
    /// Line 51: Additional tax. Enter 6% (0.06) of the smaller of line 50 or the value of your
    /// ABLE account on December 31, 2025
    pub able_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IX — Additional Tax on Excess Accumulation in Qualified Retirement Plans (Including IRAs)
    // -----------------------------------------------------------------------
    /// Line 52a: Minimum required distribution for 2025 from all qualified plans for which you
    /// received a distribution of the full amount of the excess accumulation during the correction window
    pub qlfy_retire_plan_min_rqr_distri_amt: Usd,
    /// Line 52b: Minimum required distribution for 2025 from all other plans
    pub all_oth_qlfy_plan_min_rqr_distri_amt: Usd,
    /// Line 53a: Amount distributed to you during 2025 from all qualified plans for which you
    /// received a distribution of the full amount of the excess accumulation during the correction window
    pub qlfy_retire_plan_actual_distri_amt: Usd,
    /// Line 53b: Amount distributed to you during 2025 from all other plans
    pub all_oth_qlfy_plan_actual_distri_amt: Usd,
    /// Line 54a: Subtract line 53a from line 52a and multiply the result by 10% (0.10). If zero or less, enter -0-
    pub qlfy_retire_plan_excess_accum_amt: Usd,
    /// Line 54b: Subtract line 53b from line 52b and multiply the result by 25% (0.25). If zero or less, enter -0-
    pub all_oth_qlfy_plan_excess_accum_amt: Usd,
    /// Line 54b: Waiver of tax on excess accumulation statement code
    pub waive_tax_on_ex_accum_qrp_stmt_cd: String,
    /// Line 54b: Waiver of tax on excess accumulation statement amount
    pub waive_tax_on_ex_accum_qrp_stmt_amt: Usd,
    /// Line 55: Add lines 54a and 54b. Include the total on Schedule 2 (Form 1040), line 8, or
    /// Form 1041, Schedule G, line 8
    pub rtmnt_annty_excess_contrib_tax_amt: Usd,
}
